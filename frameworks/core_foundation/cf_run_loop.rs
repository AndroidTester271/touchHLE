/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `CFRunLoop`.
//!
//! This is not even toll-free bridged to `NSRunLoop` in Apple's implementation,
//! but here it is the same type.

use crate::dyld::{export_c_func, ConstantExports, FunctionExports, HostConstant};
use crate::objc::msg_class;
use crate::{Environment, msg};
use crate::frameworks::core_foundation::cf_string::CFStringRef;
use crate::frameworks::core_foundation::{CFIndex, CFTypeRef};
use crate::frameworks::core_foundation::cf_allocator::CFAllocatorRef;
use crate::frameworks::core_foundation::time::CFTimeInterval;
use crate::mem::{ConstVoidPtr, Ptr};

pub type CFRunLoopRef = super::CFTypeRef;
pub type CFRunLoopMode = super::cf_string::CFStringRef;

fn CFRunLoopGetCurrent(env: &mut Environment) -> CFRunLoopRef {
    msg_class![env; NSRunLoop currentRunLoop]
}

pub fn CFRunLoopGetMain(env: &mut Environment) -> CFRunLoopRef {
    msg_class![env; NSRunLoop mainRunLoop]
}

fn CFRunLoopRunInMode(
    env: &mut Environment, mode: CFRunLoopMode, seconds: CFTimeInterval, returnSomething: bool
) -> i32 {
    // let loop_ = CFRunLoopGetCurrent(env);
    // () = msg![env; loop_ run];
    // 0
    1
}

fn CFPreferencesCopyAppValue(env: &mut Environment, key: CFStringRef, appID: CFStringRef) -> CFTypeRef {
    Ptr::null()
}

fn CFPreferencesSetAppValue(env: &mut Environment, key: CFStringRef, value: CFTypeRef, appID: CFStringRef) {

}

fn CFPreferencesAppSynchronize(env: &mut Environment, appID: CFStringRef) -> bool {
    false
}

const kCFNumberSInt32Type: CFIndex = 3;

fn CFNumberCreate(
    env: &mut Environment, allocator: CFAllocatorRef, type_: CFIndex, value_ptr: ConstVoidPtr
) -> CFTypeRef {
    log!("CFNumberCreate type {}", type_);
    assert_eq!(type_, kCFNumberSInt32Type);
    let val: i32 = env.mem.read(value_ptr.cast());
    msg_class![env; NSNumber numberWithInt:val]
}

pub const kCFRunLoopCommonModes: &str = "kCFRunLoopCommonModes";
pub const kCFRunLoopDefaultMode: &str = "kCFRunLoopDefaultMode";
pub const kCFPreferencesCurrentApplication: &str = "kCFPreferencesCurrentApplication";

pub const CONSTANTS: ConstantExports = &[
    (
        "_kCFRunLoopCommonModes",
        HostConstant::NSString(kCFRunLoopCommonModes),
    ),
    (
        "_kCFRunLoopDefaultMode",
        HostConstant::NSString(kCFRunLoopDefaultMode),
    ),
    (
        "_kCFPreferencesCurrentApplication",
        HostConstant::NSString(kCFPreferencesCurrentApplication),
    ),
];

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CFRunLoopGetCurrent()),
    export_c_func!(CFRunLoopGetMain()),
    export_c_func!(CFRunLoopRunInMode(_, _, _)),
    export_c_func!(CFPreferencesCopyAppValue(_, _)),
    export_c_func!(CFPreferencesSetAppValue(_, _, _)),
    export_c_func!(CFPreferencesAppSynchronize(_)),
    export_c_func!(CFNumberCreate(_, _, _)),
];
