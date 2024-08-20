/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `sys/sysctl.h`

use crate::dyld::{export_c_func, FunctionExports};
use crate::mem::{ConstPtr, GuestUSize, MutPtr, MutVoidPtr};
use crate::Environment;
use crate::libc::errno::ENOENT;

fn sysctl(
    env: &mut Environment,
    name: MutPtr<i32>,
    name_len: u32,
    oldp: MutVoidPtr,
    oldlenp: MutPtr<GuestUSize>,
    newp: MutVoidPtr,
    newlen: GuestUSize,
) -> i32 {
    log!(
        "TODO: sysctl({:?}, {:#x}, {:?}, {:?}, {:?}, {:x})",
        name,
        name_len,
        oldp,
        oldlenp,
        newp,
        newlen
    );
    assert!(!oldp.is_null() && !oldlenp.is_null()); // TODO
    assert!(newp.is_null()); // TODO
    env.mem.write(oldlenp, 0);
    0 // success
}

fn sysctlbyname(
    env: &mut Environment,
    name: ConstPtr<u8>,
    oldp: MutVoidPtr,
    oldlenp: MutPtr<GuestUSize>,
    newp: MutVoidPtr,
    newlen: GuestUSize,
) -> i32 {
    let name_str = env.mem.cstr_at_utf8(name).unwrap();
    log_dbg!(
        "TODO: sysctlbyname({:?}, {:?}, {:?}, {:?}, {:x})",
        name_str,
        oldp,
        oldlenp,
        newp,
        newlen
    );
    // reference https://www.mail-archive.com/misc@openbsd.org/msg80988.html
    let (val, len): (&[u8], GuestUSize) = match name_str {
        "hw.machine" => (b"iPhone1,1", 10),
        "hw.model" => (b"M68AP", 6),
        "hw.ncpu" => (b"1", 2),
        "hw.cputype" => (b"12", 3),
        "hw.cpusubtype" => (b"6", 2),
        "hw.cpufrequency" => (b"412000000", 10),
        "hw.busfrequency" => (b"103000000", 10),
        "hw.physmem" => (b"121634816", 10),
        "hw.usermem" => (b"93564928", 9),
        "hw.memsize" => (b"121634816", 10),
        "hw.pagesize" => (b"4096", 5),
        "kern.ostype" => (b"Darwin", 7),
        "kern.osrelease" => (b"10.0.0d3", 9),
        "kern.version" => (b"Darwin Kernel Version 10.0.0d3: Wed May 13 22:11:58 PDT 2009; root:xnu-1357.2.89~4/RELEASE_ARM_S5L8900X", 104),
        "hw.optional.mmx" => return ENOENT,
        "hw.optional.sse" => return ENOENT,
        _str => unimplemented!("{}", _str)
    };
    if oldp.is_null() && newp.is_null() {
        env.mem.write(oldlenp, len);
        return 0;
    }
    assert!(!oldp.is_null() && !oldlenp.is_null());
    assert!(newp.is_null());
    let sysctl_str = env.mem.alloc_and_write_cstr(val);
    // assert_eq!(env.mem.read(oldlenp), len);
    env.mem
        .memmove(oldp, sysctl_str.cast().cast_const(), len);
    env.mem.free(sysctl_str.cast());
    0 // success
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(sysctl(_, _, _, _, _, _)),
    export_c_func!(sysctlbyname(_, _, _, _, _)),
];
