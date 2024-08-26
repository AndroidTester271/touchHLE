/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSDate`.

use super::NSTimeInterval;
use crate::frameworks::core_foundation::time::apple_epoch;
use crate::objc::{autorelease, id, objc_classes, ClassExports, HostObject};
use crate::objc::nil;

use std::time::SystemTime;
use crate::frameworks::foundation::ns_string;

struct NSDateHostObject {
    instant: NSTimeInterval,
}
impl HostObject for NSDateHostObject {}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSDate: NSObject

+ (id)date {
    // "Date objects are immutable, representing an invariant time interval
    // relative to an absolute reference date (00:00:00 UTC on 1 January 2001)."
    let instant = SystemTime::now()
        .duration_since(apple_epoch())
        .unwrap()
        .as_secs_f64();
    let host_object = Box::new(NSDateHostObject {
        instant
    });
    let new = env.objc.alloc_object(this, host_object, &mut env.mem);

    log_dbg!("[(NSDate*){:?} date]: New date {:?}", this, new);

    autorelease(env, new)
}

- (NSTimeInterval)timeIntervalSinceDate:(id)anotherDate {
    assert!(!anotherDate.is_null());
    let host_object = env.objc.borrow::<NSDateHostObject>(this);
    let another_date_host_object = env.objc.borrow::<NSDateHostObject>(anotherDate);
    let result = another_date_host_object.instant - host_object.instant;
    log_dbg!("[(NSDate*){:?} timeIntervalSinceDate:{:?}]: result {} seconds", this, anotherDate, result);
    result
}

- (NSTimeInterval)timeIntervalSinceReferenceDate {
    env.objc.borrow::<NSDateHostObject>(this).instant
}

- (NSTimeInterval)timeIntervalSinceNow {
    let instant2 = SystemTime::now()
        .duration_since(apple_epoch())
        .unwrap()
        .as_secs_f64();
    instant2 - env.objc.borrow::<NSDateHostObject>(this).instant
}

- (id)addTimeInterval:(NSTimeInterval)seconds {
    let host_object = env.objc.borrow::<NSDateHostObject>(this);
    let new_host_object = Box::new(NSDateHostObject {
        instant: host_object.instant + seconds
    });
    let isa = env
        .objc
        .get_known_class("NSDate", &mut env.mem);
    let new = env.objc.alloc_object(isa, new_host_object, &mut env.mem);
    autorelease(env, new)
}

@end

@implementation NSTimeZone: NSObject

+ (id)localTimeZone {
    nil
}

+ (id)timeZoneWithName:(id)_name { // NSString*
    nil
}

@end

@implementation NSScanner: NSObject

+ (id)scannerWithString:(id)str { // NSString*
    log!("scannerWithString: {}", ns_string::to_rust_string(env, str));
    nil
}

@end

};
