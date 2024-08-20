/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! Miscellaneous parts of `unistd.h`

use crate::dyld::{export_c_func, FunctionExports};
use crate::fs::GuestPath;
use crate::libc::posix_io::{FileDescriptor, O_RDONLY, STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO};
use crate::mem::{ConstPtr, ConstVoidPtr, GuestISize, GuestUSize, MutPtr, MutVoidPtr, Ptr};
use crate::Environment;
use std::time::Duration;
use crate::libc::posix_io;
use crate::libc::stdio::{FILE, fread};

#[allow(non_camel_case_types)]
type useconds_t = u32;

const F_OK: i32 = 0;
const R_OK: i32 = 4;

fn sleep(env: &mut Environment, seconds: u32) -> u32 {
    env.sleep(Duration::from_secs(seconds.into()), true);
    // sleep() returns the amount of time remaining that should have been slept,
    // but wasn't, if the thread was woken up early by a signal.
    // touchHLE never does that currently, so 0 is always correct here.
    0
}

fn usleep(env: &mut Environment, useconds: useconds_t) -> i32 {
    env.sleep(Duration::from_micros(useconds.into()), true);
    0 // success
}

#[allow(non_camel_case_types)]
type pid_t = i32;

fn getpid(_env: &mut Environment) -> pid_t {
    // Not a real value, since touchHLE only simulates a single process.
    // PID 0 would be init, which is a bit unrealistic, so let's go with 1.
    1
}
fn getppid(_env: &mut Environment) -> pid_t {
    // Included just for completeness. Surely no app ever calls this.
    0
}

fn isatty(_env: &mut Environment, fd: FileDescriptor) -> i32 {
    if [STDIN_FILENO, STDOUT_FILENO, STDERR_FILENO].contains(&fd) {
        1
    } else {
        0
    }
}

fn access(env: &mut Environment, path: ConstPtr<u8>, mode: i32) -> i32 {
    let binding = env.mem.cstr_at_utf8(path).unwrap();
    let guest_path = GuestPath::new(&binding);
    let (exists, r, _, _) = env.fs.access(guest_path);
    // TODO: set errno
    match mode {
        F_OK => {
            if exists {
                0
            } else {
                -1
            }
        }
        R_OK => {
            if r {
                0
            } else {
                -1
            }
        }
        _ => unimplemented!("{}", mode),
    }
}

fn uname(_env: &mut Environment, name: MutVoidPtr) -> i32 {
    -1
}

fn getpagesize(env: &mut Environment) -> i32 {
    4096
}

fn get_etext(env: &mut Environment) -> u32 {
    4096
}

fn get_end(env: &mut Environment) -> u32 {
    927506432
}

// int sigaction(int sig, const struct sigaction *restrict act, struct sigaction *restrict oact);
fn sigaction(env: &mut Environment, sig: i32, act: ConstVoidPtr, oact: MutVoidPtr) -> i32 {
    0
}

// int sigprocmask(int how, const sigset_t *restrict set, sigset_t *restrict oset);
fn sigprocmask(env: &mut Environment, how: i32, set: ConstVoidPtr, oact: MutVoidPtr) -> i32 {
    0
}

// sig_t signal(int sig, sig_t func);
fn signal(env: &mut Environment, sig: i32, func: MutVoidPtr) -> MutVoidPtr {
    Ptr::null()
}

// ssize_t readlink(const char *restrict path, char *restrict buf, size_t bufsize)
fn readlink(env: &mut Environment, path: ConstPtr<u8>, buf: MutPtr<u8>, bufsize: GuestISize) -> GuestISize {
    log!("Failing readlink() for {}", env.mem.cstr_at_utf8(path).unwrap());
    // TODO: set errno
    -1
    // let file: MutPtr<FILE> = match posix_io::open_direct(env, path, O_RDONLY) {
    //     -1 => Ptr::null(),
    //     fd => env.mem.alloc_and_write(FILE { fd }),
    // };
    // if file.is_null() {
    //     // TODO: set errno
    //     return -1;
    // }
    // fread(env, buf.cast(), 1, bufsize.try_into().unwrap(), file) as GuestISize
}

fn getdtablesize(env: &mut Environment) -> i32 {
    2
}

fn gethostname(env: &mut Environment, name: ConstPtr<u8>, namelen: GuestUSize) -> i32 {
    -1
}

fn sysconf(env: &mut Environment, name: i32) -> i32 {
    log!("sysconf {}", name);
    match name {
        // _SC_PAGESIZE => 4 Kib
        29 => 4096,
        // _SC_NPROCESSORS_ONLN => 1
        58 => 1,
        _ => -1
    }
}

fn waitpid(env: &mut Environment, pid: pid_t, stat_loc: MutPtr<i32>, options: i32) -> i32 {
    log!("waitpid pid {}, options {}", pid, options);
    // we do not have any other processes really
    assert_eq!(pid, getpid(env));
    // WNOHANG
    assert_eq!(options, 1);
    0
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(getpagesize()),
    export_c_func!(get_etext()),
    export_c_func!(get_end()),
    export_c_func!(sleep(_)),
    export_c_func!(usleep(_)),
    export_c_func!(getpid()),
    export_c_func!(getppid()),
    export_c_func!(isatty(_)),
    export_c_func!(access(_, _)),
    export_c_func!(uname(_)),
    export_c_func!(sigaction(_, _, _)),
    export_c_func!(sigprocmask(_, _, _)),
    export_c_func!(signal(_, _)),
    export_c_func!(readlink(_, _, _)),
    export_c_func!(getdtablesize()),
    export_c_func!(gethostname(_, _)),
    export_c_func!(sysconf(_)),
    export_c_func!(waitpid(_, _, _)),
];
