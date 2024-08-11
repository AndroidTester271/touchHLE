/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! The `NSDictionary` class cluster, including `NSMutableDictionary`.

use super::ns_property_list_serialization::deserialize_plist_from_file;
use super::{ns_array, ns_string, ns_url, NSInteger, NSUInteger};
use crate::abi::VaList;
use crate::frameworks::foundation::ns_array::from_vec;
use crate::frameworks::foundation::ns_property_list_serialization::NSPropertyListXMLFormat_v1_0;
use crate::frameworks::foundation::ns_string::{from_rust_string, get_static_str, to_rust_string};
use crate::fs::GuestPath;
use crate::mem::MutPtr;
use crate::objc::{
    autorelease, id, msg, msg_class, nil, objc_classes, release, retain, ClassExports, HostObject,
    NSZonePtr,
};
use crate::Environment;
use std::collections::HashMap;

/// Alias for the return type of the `hash` method of the `NSObject` protocol.
type Hash = NSUInteger;

/// Belongs to _touchHLE_NSDictionary, also used by _touchHLE_NSSet
#[derive(Debug, Default)]
pub(super) struct DictionaryHostObject {
    /// Since we need custom hashing and custom equality, and these both need a
    /// `&mut Environment`, we can't just use a `HashMap<id, id>`.
    /// So here we are using a `HashMap` as a primitive for implementing a
    /// hash-map, which is not ideally efficient. :)
    /// The keys are the hash values, the values are a list of key-value pairs
    /// where the keys have the same hash value.
    pub(super) map: HashMap<Hash, Vec<(id, id)>>,
    pub(super) count: NSUInteger,
}
impl HostObject for DictionaryHostObject {}
impl DictionaryHostObject {
    pub(super) fn lookup(&self, env: &mut Environment, key: id) -> id {
        let hash: Hash = msg![env; key hash];
        let Some(collisions) = self.map.get(&hash) else {
            return nil;
        };
        for &(candidate_key, value) in collisions {
            if candidate_key == key || msg![env; candidate_key isEqualTo:key] {
                return value;
            }
        }
        nil
    }
    pub(super) fn insert(&mut self, env: &mut Environment, key: id, value: id, copy_key: bool) {
        let key: id = if copy_key {
            msg![env; key copy]
        } else {
            retain(env, key)
        };
        let hash: Hash = msg![env; key hash];

        let value = retain(env, value);

        let Some(collisions) = self.map.get_mut(&hash) else {
            self.map.insert(hash, vec![(key, value)]);
            self.count += 1;
            return;
        };
        for &mut (candidate_key, ref mut existing_value) in collisions.iter_mut() {
            if candidate_key == key || msg![env; candidate_key isEqualTo:key] {
                release(env, *existing_value);
                release(env, key);
                *existing_value = value;
                return;
            }
        }
        collisions.push((key, value));
        self.count += 1;
    }
    pub(super) fn remove(&mut self, env: &mut Environment, key: id) {
        let hash: Hash = msg![env; key hash];
        let Some(collisions) = self.map.get_mut(&hash) else {
            return;
        };
        let idx = collisions.iter().position(|&(candidate_key, _)| {
            candidate_key == key || msg![env; candidate_key isEqualTo:key]
        }).unwrap();
        let (existing_key, value) = collisions[idx];
        release(env, existing_key);
        release(env, value);
        collisions.remove(idx);
    }
    pub(super) fn release(&mut self, env: &mut Environment) {
        for collisions in self.map.values() {
            for &(key, value) in collisions {
                release(env, key);
                release(env, value);
            }
        }
    }
    pub(super) fn iter_keys(&self) -> impl Iterator<Item = id> + '_ {
        self.map.values().flatten().map(|&(key, _value)| key)
    }
}

/// Helper to enable sharing `dictionaryWithObjectsAndKeys:` and
/// `initWithObjectsAndKeys:`' implementations without vararg passthrough.
pub fn init_with_objects_and_keys(
    env: &mut Environment,
    this: id,
    first_object: id,
    mut va_args: VaList,
) -> id {
    let first_key: id = va_args.next(env);
    assert!(first_key != nil); // TODO: raise proper exception

    let mut host_object = <DictionaryHostObject as Default>::default();
    host_object.insert(env, first_key, first_object, /* copy_key: */ true);

    loop {
        let object: id = va_args.next(env);
        if object == nil {
            break;
        }
        let key: id = va_args.next(env);
        assert!(key != nil); // TODO: raise proper exception
        host_object.insert(env, key, object, /* copy_key: */ true);
    }

    *env.objc.borrow_mut(this) = host_object;

    this
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

// NSDictionary is an abstract class. A subclass must provide:
// - (id)initWithObjects:(id*)forKeys:(id*)count:(NSUInteger)
// - (NSUInteger)count
// - (id)objectForKey:(id)
// - (NSEnumerator*)keyEnumerator
// We can pick whichever subclass we want for the various alloc methods.
// For the time being, that will always be _touchHLE_NSDictionary.
@implementation NSDictionary: NSObject

+ (id)allocWithZone:(NSZonePtr)zone {
    // NSDictionary might be subclassed by something which needs allocWithZone:
    // to have the normal behaviour. Unimplemented: call superclass alloc then.
    assert!(this == env.objc.get_known_class("NSDictionary", &mut env.mem));
    msg_class![env; _touchHLE_NSDictionary allocWithZone:zone]
}

+ (id)dictionary {
    let new_dict: id = msg![env; this alloc];
    let new_dict: id = msg![env; new_dict init];
    autorelease(env, new_dict)
}

+ (id)dictionaryWithObject:(id)object forKey:(id)key {
    assert_ne!(key, nil); // TODO: raise proper exception

    let new_dict = dict_from_keys_and_objects(env, &[(key, object)]);
    autorelease(env, new_dict)
}

+ (id)dictionaryWithObjectsAndKeys:(id)first_object, ...dots {
    let new_dict: id = msg![env; this alloc];
    let new_dict = init_with_objects_and_keys(env, new_dict, first_object, dots.start());
    autorelease(env, new_dict)
}

// These probably comes from some category related to plists.
+ (id)dictionaryWithContentsOfFile:(id)path { // NSString*
    let path = ns_string::to_rust_string(env, path);
    let res = deserialize_plist_from_file(
        env,
        GuestPath::new(&path),
        /* array_expected: */ false,
    );
    autorelease(env, res)
}
+ (id)dictionaryWithContentsOfURL:(id)url { // NSURL*
    let path = ns_url::to_rust_path(env, url);
    let res = deserialize_plist_from_file(env, &path, /* array_expected: */ false);
    autorelease(env, res)
}

- (id)init {
    todo!("TODO: Implement [dictionary init] for custom subclasses")
}

// These probably comes from some category related to plists.
- (id)initWithContentsOfFile:(id)path { // NSString*
    release(env, this);
    let path = ns_string::to_rust_string(env, path);
    deserialize_plist_from_file(
        env,
        GuestPath::new(&path),
        /* array_expected: */ false,
    )
}
- (id)initWithContentsOfURL:(id)url { // NSURL*
    release(env, this);
    let path = ns_url::to_rust_path(env, url);
    deserialize_plist_from_file(env, &path, /* array_expected: */ false)
}

// NSCopying implementation
- (id)copyWithZone:(NSZonePtr)_zone {
    // TODO: override this once we have NSMutableDictionary!
    retain(env, this)
}

- (id)mutableCopy{
    let ko = dict_to_keys_and_objects(env, this);
    dict_from_keys_and_objects(env, &ko)
}
    
- (id)allKeys {
    let keys = env.objc.borrow::<DictionaryHostObject>(this).iter_keys().collect::<Vec<_>>();
    for key in &keys {
        retain(env, *key);
    }
    let ns_keys = from_vec(env, keys);
    autorelease(env, ns_keys)
}

- (id)fileSize {
    let key = get_static_str(env, "NSFileSize");
    msg![env; this objectForKey:key]
}

// FIXME: those are from NSUserDefaults!
- (())setObject:(id)anObject
         forKey:(id)key { // NSString*
    assert!(!anObject.is_null());
    assert!(anObject != nil);
    assert!(!key.is_null());
    assert!(key != nil);
    let mut host_obj: DictionaryHostObject = std::mem::take(env.objc.borrow_mut(this));
    host_obj.insert(env, key, anObject, false);
    *env.objc.borrow_mut(this) = host_obj;
}
- (())setInteger:(NSInteger)value forKey:(id)defaultName {
    let mut host_obj: DictionaryHostObject = std::mem::take(env.objc.borrow_mut(this));
    let value_id: id = msg_class![env; NSNumber numberWithInteger:value];
    host_obj.insert(env, defaultName, value_id, false);
    *env.objc.borrow_mut(this) = host_obj;
}
- (())setDouble:(f64)value forKey:(id)defaultName {
    let mut host_obj: DictionaryHostObject = std::mem::take(env.objc.borrow_mut(this));
    // TODO: do not down cast
    let float: f32 = value as f32;
    let value_id: id = msg_class![env; NSNumber numberWithFloat:float];
    host_obj.insert(env, defaultName, value_id, false);
    *env.objc.borrow_mut(this) = host_obj;
}
- (())setFloat:(f32)value forKey:(id)defaultName {
    let mut host_obj: DictionaryHostObject = std::mem::take(env.objc.borrow_mut(this));
    let value_id: id = msg_class![env; NSNumber numberWithFloat:value];
    host_obj.insert(env, defaultName, value_id, false);
    *env.objc.borrow_mut(this) = host_obj;
}
- (())removeObjectForKey:(id)defaultName {
    let mut host_obj: DictionaryHostObject = std::mem::take(env.objc.borrow_mut(this));
    host_obj.remove(env, defaultName);
    *env.objc.borrow_mut(this) = host_obj;
}
    
// TODO

- (id)valueForKey:(id)key { // NSString*
    let key_str = to_rust_string(env, key);
    // TODO: strip '@' and call super
    assert!(!key_str.starts_with('@'));
    msg![env; this objectForKey:key]
}

@end

// NSMutableDictionary is an abstract class. A subclass must provide everything
// NSDictionary provides, plus:
// - (void)setObject:(id)object forKey:(id)key;
// - (void)removeObjectForKey:(id)key;
// Note that it inherits from NSDictionary, so we must ensure we override
// any default methods that would be inappropriate for mutability.
@implementation NSMutableDictionary: NSDictionary

+ (id)allocWithZone:(NSZonePtr)zone {
    // NSDictionary might be subclassed by something which needs allocWithZone:
    // to have the normal behaviour. Unimplemented: call superclass alloc then.
    // assert!(this == env.objc.get_known_class("NSMutableDictionary", &mut env.mem));
    msg_class![env; _touchHLE_NSMutableDictionary allocWithZone:zone]
}

+ (id)dictionaryWithCapacity:(NSUInteger)cap {
    let new = msg![env; this alloc];
    let new = msg![env; new initWithCapacity: cap];
    autorelease(env, new)
}

// NSCopying implementation
- (id)copyWithZone:(NSZonePtr)_zone {
    let entries: Vec<_> =
        env.objc.borrow_mut::<DictionaryHostObject>(this).map.values().flatten().copied().collect();
    dict_from_keys_and_objects(env, &entries)
}

- (bool)writeToFile:(id)path
         atomically:(bool)atomic {
    let data = msg_class![env;
        NSPropertyListSerialization dataFromPropertyList: this
                                                  format: NSPropertyListXMLFormat_v1_0
                                        errorDescription: (MutPtr::<id>::null())
    ];
    if data == nil {
        return false;
    }
    msg![env; data writeToFile: path atomically: atomic]
}

@end

@implementation NSMutableDictionary: NSDictionary

+ (id)allocWithZone:(NSZonePtr)zone {
    // NSDictionary might be subclassed by something which needs allocWithZone:
    // to have the normal behaviour. Unimplemented: call superclass alloc then.
    assert!(this == env.objc.get_known_class("NSMutableDictionary", &mut env.mem));
    msg_class![env; _touchHLE_NSDictionary allocWithZone:zone]
}

+ (id)dictionaryWithCapacity:(NSUInteger)cap {
    let new = msg![env; this alloc];
    let new = msg![env; new initWithCapacity: cap];
    autorelease(env, new)
}

- (())setValue:(id)value
       forKey:(id)key {
    msg![env; this setObject: value forKey: key]
}

-(())removeAllObjects {
    let mut objects = std::mem::take(env.objc.borrow_mut::<DictionaryHostObject>(this));
    objects.release(env);
}

- (())setInteger:(NSInteger)value forKey:(id)defaultName {
    let mut host_obj: DictionaryHostObject = std::mem::take(env.objc.borrow_mut(this));
    let value_id: id = msg_class![env; NSNumber numberWithInteger:value];
    host_obj.insert(env, defaultName, value_id, false);
    *env.objc.borrow_mut(this) = host_obj;
}

@end
    
// Our private subclass that is the single implementation of NSDictionary for
// the time being.
@implementation _touchHLE_NSDictionary: NSMutableDictionary

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<DictionaryHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

+ (id)dictionaryWithCapacity:(NSUInteger)cap {
    let new = msg![env; this alloc];
    let new = msg![env; new initWithCapacity: cap];
    autorelease(env, new)
}

- (())dealloc {
    std::mem::take(env.objc.borrow_mut::<DictionaryHostObject>(this)).release(env);

    env.objc.dealloc_object(this, &mut env.mem)
}

- (id)initWithObjectsAndKeys:(id)first_object, ...dots {
    init_with_objects_and_keys(env, this, first_object, dots.start())
}

- (id)init {
    *env.objc.borrow_mut(this) = <DictionaryHostObject as Default>::default();
    this
}

- (id)initWithCapacity:(NSUInteger)cap {
    env.objc.borrow_mut::<DictionaryHostObject>(this).map.reserve(cap as usize);
    this
}

// TODO: enumeration, more init methods, etc

- (NSUInteger)count {
    env.objc.borrow::<DictionaryHostObject>(this).count
}
- (id)objectForKey:(id)key {
    let host_obj: DictionaryHostObject = std::mem::take(env.objc.borrow_mut(this));
    let res = host_obj.lookup(env, key);
    *env.objc.borrow_mut(this) = host_obj;
    res
}

- (NSInteger)fileSize {
    let file_size_key = ns_string::get_static_str(env, "fileSize");
    let host_obj: DictionaryHostObject = std::mem::take(env.objc.borrow_mut(this));
    let res = host_obj.lookup(env, file_size_key);
    *env.objc.borrow_mut(this) = host_obj;
    msg![env; res intValue]
}

-(())setObject:(id)value forKey:(id)key {
    let mut host_obj: DictionaryHostObject = std::mem::take(env.objc.borrow_mut(this));
    host_obj.insert(env, key, value, true);
    *env.objc.borrow_mut(this) = host_obj;
}
    
- (id)description {
    // According to docs, this description should be formatted as property list.
    // But by the same docs, it's meant to be used for debugging purposes only.
    let desc: id = msg_class![env; NSMutableString new];
    let prefix: id = from_rust_string(env, "{\n".to_string());
    () = msg![env; desc appendString:prefix];
    release(env, prefix);
    let keys: Vec<id> = env.objc.borrow_mut::<DictionaryHostObject>(this).iter_keys().collect();
    for key in keys {
        let key_desc: id = msg![env; key description];
        let value: id = msg![env; this objectForKey:key];
        let val_desc: id = msg![env; value description];
        // TODO: respect nesting and padding
        let format = format!("\t{} = {};\n", to_rust_string(env, key_desc), to_rust_string(env, val_desc));
        let format = from_rust_string(env, format);
        () = msg![env; desc appendString:format];
        release(env, format);
    }
    let suffix: id = from_rust_string(env, "}".to_string());
    () = msg![env; desc appendString:suffix];
    release(env, suffix);
    // TODO: return an immutable copy once supported
    autorelease(env, desc)
}

// FIXME: those are from NSUserDefaults!
- (NSInteger)integerForKey:(id)defaultName {
    let val: id = msg![env; this objectForKey:defaultName];
    msg![env; val integerValue]
}
- (bool)boolForKey:(id)defaultName {
    let val: id = msg![env; this objectForKey:defaultName];
    msg![env; val boolValue]
}

@end

@implementation CLLocationManager: NSMutableDictionary
@end

@implementation NSNetServiceBrowser: NSMutableDictionary
@end

// Our private subclass that is the single implementation of
// NSMutableDictionary for the time being.
@implementation _touchHLE_NSMutableDictionary: NSMutableDictionary

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::<DictionaryHostObject>::default();
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (())dealloc {
    std::mem::take(env.objc.borrow_mut::<DictionaryHostObject>(this)).release(env);

    env.objc.dealloc_object(this, &mut env.mem)
}

- (id)dictionaryRepresentation {
    this
}
- (id)headingAvailable {
    nil
}

- (id)locationServicesEnabled {
    nil
}

- (())synchronize {
}

- (bool)persistentDomainForName:(id)defaultName {
    let val: id = msg![env; this objectForKey:defaultName];
    msg![env; val boolValue]
}

- (bool)arrayForKey:(id)defaultName {
    let val: id = msg![env; this objectForKey:defaultName];
    msg![env; val boolValue]
}

- (bool)dataForKey:(id)defaultName {
    let val: id = msg![env; this objectForKey:defaultName];
    msg![env; val boolValue]
}

- (id)initWithCapacity:(NSUInteger)cap {
    env.objc.borrow_mut::<DictionaryHostObject>(this).map.reserve(cap as usize);
    this
}

- (id)initWithObjectsAndKeys:(id)first_object, ...dots {
    init_with_objects_and_keys(env, this, first_object, dots.start())
}

- (id)init {
    *env.objc.borrow_mut(this) = <DictionaryHostObject as Default>::default();
    this
}

// TODO: enumeration, more init methods, etc

- (NSUInteger)count {
    env.objc.borrow::<DictionaryHostObject>(this).count
}
- (id)objectForKey:(id)key {
    let host_obj: DictionaryHostObject = std::mem::take(env.objc.borrow_mut(this));
    let res = host_obj.lookup(env, key);
    *env.objc.borrow_mut(this) = host_obj;
    res
}

- (())setBool:(bool)value
       forKey:(id)key { // NSString*
    let num: id = msg_class![env; NSNumber numberWithBool:value];
    msg![env; this setObject:num forKey:key]
}

- (())setObject:(id)object
         forKey:(id)key {
    // TODO: raise NSInvalidArgumentException
    assert_ne!(object, nil);
    // TODO: raise NSInvalidArgumentException
    // assert_ne!(key, nil);
    let mut host_obj: DictionaryHostObject = std::mem::take(env.objc.borrow_mut(this));
    host_obj.insert(env, object, key, /* copy_key: */ true);
    *env.objc.borrow_mut(this) = host_obj;
}

- (())setInteger:(NSInteger)value forKey:(id)defaultName {
    let mut host_obj: DictionaryHostObject = std::mem::take(env.objc.borrow_mut(this));
    let value_id: id = msg_class![env; NSNumber numberWithInteger:value];
    host_obj.insert(env, defaultName, value_id, false);
    *env.objc.borrow_mut(this) = host_obj;
}

- (())setDelegate:(bool)delegate {
    log!("TODO: setDelegate:{}", delegate);
}

- (())setValue:(id)value
        forKey:(id)key { // NSString*
    assert!(!key.is_null());
    let mut host_obj: DictionaryHostObject = std::mem::take(env.objc.borrow_mut(this));
    host_obj.insert(env, key, value, false);
    *env.objc.borrow_mut(this) = host_obj;
}

// FIXME: those are from NSUserDefaults!
- (NSInteger)integerForKey:(id)defaultName {
    let val: id = msg![env; this objectForKey:defaultName];
    msg![env; val integerValue]
}

- (())registerDefaults:(id)defaults {
    let mut host_obj: DictionaryHostObject = std::mem::take(env.objc.borrow_mut(defaults));
    for (_, key_value) in host_obj.map {
        let key = key_value[0].0;
        let value = key_value[0].1;
        let mut host_obj: DictionaryHostObject = std::mem::take(env.objc.borrow_mut(this));
        host_obj.insert(env, key, value, false);
        *env.objc.borrow_mut(this) = host_obj;
    }
}

- (f32)floatForKey:(id)defaultName {
    let val: id = msg![env; this objectForKey:defaultName];
    msg![env; val floatValue]
}

- (bool)boolForKey:(id)defaultName {
    let val: id = msg![env; this objectForKey:defaultName];
    msg![env; val boolValue]
}

- (id)boolValue {
     nil
}

- (bool)dictionaryForKey:(id)defaultName {
    let val: id = msg![env; this objectForKey:defaultName];
    msg![env; val boolValue]
}

- (id)stringForKey:(id)defaultName {
    msg![env; this objectForKey:defaultName]
}

@end

};

/// Direct constructor for use by host code, similar to
/// `[[NSDictionary alloc] initWithObjectsAndKeys:]` but without variadics and
/// with a more intuitive argument order. Unlike [super::ns_array::from_vec],
/// this **does** copy and retain!
pub fn dict_from_keys_and_objects(env: &mut Environment, keys_and_objects: &[(id, id)]) -> id {
    let dict: id = msg_class![env; NSMutableDictionary alloc];

    let mut host_object = <DictionaryHostObject as Default>::default();
    for &(key, object) in keys_and_objects {
        host_object.insert(env, key, object, /* copy_key: */ true);
    }
    *env.objc.borrow_mut(dict) = host_object;

    dict
}

pub fn dict_to_keys_and_objects(env: &mut Environment, dict: id) -> Vec<(id, id)> {
    let host = env.objc.borrow::<DictionaryHostObject>(dict);
    let mut ret = Vec::with_capacity(host.count as usize);
    for collisions in host.map.values() {
        for &(key, value) in collisions {
            ret.push((key, value));
        }
    }
    ret
}
