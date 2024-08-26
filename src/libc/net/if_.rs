/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `net/if.h`

use crate::dyld::FunctionExports;
use crate::export_c_func;
use crate::mem::{ConstPtr, MutVoidPtr, Ptr};
use crate::Environment;
use crate::frameworks::core_foundation::cf_allocator::CFAllocatorRef;
use crate::frameworks::core_foundation::CFTypeRef;
use crate::objc::nil;

// TODO: struct definition
#[allow(non_camel_case_types)]
struct if_nameindex {}

fn if_nameindex(_env: &mut Environment) -> ConstPtr<if_nameindex> {
    // TODO: implement
    Ptr::null()
}

fn SCNetworkReachabilityCreateWithAddress(_env: &mut Environment, allocator: CFAllocatorRef, socket: MutVoidPtr) -> CFTypeRef {
    nil
}

fn SCNetworkReachabilityGetFlags(_env: &mut Environment, target: CFTypeRef, flags: MutVoidPtr) -> bool {
    false
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(if_nameindex()),
    export_c_func!(SCNetworkReachabilityCreateWithAddress(_, _)),
    export_c_func!(SCNetworkReachabilityGetFlags(_, _)),
];
