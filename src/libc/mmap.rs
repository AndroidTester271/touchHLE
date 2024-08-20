/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::dyld::FunctionExports;
use crate::environment::Environment;
use crate::export_c_func;
use crate::libc::posix_io;
use crate::libc::posix_io::{off_t, FileDescriptor, SEEK_SET};
use crate::mem::{GuestUSize, MutPtr, MutVoidPtr, Ptr};

#[allow(dead_code)]
const MAP_FILE: i32 = 0x0000;
const MAP_ANON: i32 = 0x1000;

/// Our implementation of mmap is really simple: it's just load entirety of
/// file in memory!
fn mmap(
    env: &mut Environment,
    addr: MutVoidPtr,
    len: GuestUSize,
    _prot: i32,
    flags: i32,
    fd: FileDescriptor,
    offset: off_t,
) -> MutVoidPtr {
    //assert!(addr.is_null());
    assert_eq!(offset, 0);
    //assert_eq!((flags & MAP_ANON), 0);
    let mut ptr = env.mem.alloc(len);
    if (flags & MAP_ANON) != 0 {
        // // This prevents "GC_unix_get_mem: Memory returned by mmap is not aligned to HBLKSIZE."
        // env.mem.write(Ptr::from_bits(0x0063de64), 0x00f020e3);
        // // This bypass "Duplicate large block deallocation"
        // env.mem.write(Ptr::from_bits(0x006353d8), 0x00f020e3);
        // // * Assertion at gc.c:205, condition `GC_base (obj) == (char*)obj - offset' not met
        // env.mem.write(Ptr::from_bits(0x0059c318), 0x00f020e3);
        log!("mmap ANON ptr {:?}", ptr);
        let mut to_free = vec![];
        while ptr.to_bits() & 0xfff != 0 {
            to_free.push(ptr);
            ptr = env.mem.alloc(len);
            log!("mmap ANON ptr {:?}", ptr);
        }
        for x in to_free {
            env.mem.free(x);
        }
        assert_eq!(fd, -1);
        return ptr;
    }
    let new_offset = posix_io::lseek(env, fd, offset, SEEK_SET);
    assert_eq!(new_offset, offset);
    let read = posix_io::read(env, fd, ptr, len);
    assert_eq!(read as u32, len);
    ptr
}

pub const FUNCTIONS: FunctionExports = &[export_c_func!(mmap(_, _, _, _, _, _))];
