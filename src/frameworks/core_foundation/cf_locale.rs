/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::{Environment, export_c_func, msg, msg_class};
use crate::dyld::FunctionExports;
use crate::frameworks::foundation::{ns_string, NSUInteger};
use crate::objc::id;
use super::cf_allocator::CFAllocatorRef;
use super::cf_array::CFArrayRef;
use super::cf_string::CFStringRef;

type CFLocaleIdentifier = CFStringRef;

fn CFLocaleCopyPreferredLanguages(env: &mut Environment) -> CFArrayRef {
    let arr = msg_class![env; NSLocale preferredLanguages];
    msg![env; arr copy]
}

fn CFLocaleCreateCanonicalLocaleIdentifierFromString(
    env: &mut Environment,
    allocator: CFAllocatorRef,
    localeIdentifier: CFStringRef
) -> CFLocaleIdentifier {
    assert!(allocator.is_null());
    let len: NSUInteger = msg![env; localeIdentifier length];
    assert_eq!(len, 2);
    let ns_string: id = msg_class![env; NSString alloc];
    msg![env; ns_string initWithString:localeIdentifier]
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(CFLocaleCopyPreferredLanguages()),
    export_c_func!(CFLocaleCreateCanonicalLocaleIdentifierFromString(_, _)),
];