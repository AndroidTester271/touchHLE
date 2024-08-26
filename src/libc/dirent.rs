/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `dirent.h`

use std::collections::HashMap;
use crate::dyld::FunctionExports;
use crate::{Environment, export_c_func, impl_GuestRet_for_large_struct};
use crate::fs::GuestPath;
use crate::mem::{ConstPtr, MutPtr, Ptr, SafeRead};

struct DIR {
    idx: usize
}
unsafe impl SafeRead for DIR {}

const MAXPATHLEN: usize = 1024;

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C, packed)]
struct dirent {
    d_ino: u64,
    d_seekoff: u64,
    d_reclen: u16,
    d_namlen: u16,
    d_type: u8,
    d_name: [u8; MAXPATHLEN]
}
unsafe impl SafeRead for dirent {}
impl_GuestRet_for_large_struct!(dirent);

#[derive(Default)]
pub struct State {
    open_dirs: HashMap<MutPtr<DIR>, Vec<String>>,
}
impl State {
    fn get(env: &Environment) -> &Self {
        &env.libc_state.dirent
    }
    fn get_mut(env: &mut Environment) -> &mut Self {
        &mut env.libc_state.dirent
    }
}

fn opendir(env: &mut Environment, filename: ConstPtr<u8>) -> MutPtr<DIR> {
    let path_string = env.mem.cstr_at_utf8(filename).unwrap().to_owned();
    log_dbg!("opendir {}", path_string);
    let guest_path = GuestPath::new(&path_string);
    let is_dir = env.fs.is_dir(guest_path);
    if is_dir {
        let dir = env.mem.alloc_and_write(DIR { idx: 0 });
        let iter = env.fs.enumerate(guest_path).unwrap();
        let vec = iter.map(|str| str.to_string()).collect();
        State::get_mut(env).open_dirs.insert(dir, vec);
        dir
    } else {
        Ptr::null()
    }
}

fn readdir(env: &mut Environment, dirp: MutPtr<DIR>) -> MutPtr<dirent> {
    let mut dir = env.mem.read(dirp);
    let vec = env.libc_state.dirent.open_dirs.get(&dirp).unwrap();
    log_dbg!("readdir {:?}", vec.get(dir.idx));
    if let Some(str) = vec.get(dir.idx) {
        dir.idx += 1;
        env.mem.write(dirp, dir);

        let len = str.len();
        let mut res = dirent {
            d_ino: 0,
            d_seekoff: 0,
            d_reclen: 0,
            d_namlen: len as u16,
            d_type: 0,
            d_name: [b'\0'; 1024],
        };
        res.d_name[..len].copy_from_slice(&str.as_bytes());
        // FIXME: free those on closedir
        env.mem.alloc_and_write(res)
    } else {
        Ptr::null()
    }
}

fn closedir(env: &mut Environment, dirp: MutPtr<DIR>) -> i32 {
    env.libc_state.dirent.open_dirs.remove(&dirp);
    env.mem.free(dirp.cast());
    0 // Success
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(opendir(_)),
    export_c_func!(readdir(_)),
    export_c_func!(closedir(_)),
];