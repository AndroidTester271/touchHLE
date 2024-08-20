/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `mach/thread_info.h`
//!
//! This is extremely undocumented. :(

#![allow(non_camel_case_types)]

use crate::dyld::{export_c_func, FunctionExports};
use crate::mem::{guest_size_of, GuestUSize, MutPtr, MutVoidPtr, Ptr, SafeRead};
use crate::Environment;
use crate::environment::ThreadBlock::Suspended;
use crate::environment::ThreadId;

type kern_return_t = i32;
type mach_msg_return_t = kern_return_t;
const KERN_SUCCESS: kern_return_t = 0;

type mach_port_t = u32;

type natural_t = u32;
type integer_t = i32;
type boolean_t = i32;

type task_t = mach_port_t;
type thread_act_t = mach_port_t;
type thread_act_array_t = MutPtr<thread_act_t>;
type ipc_space_t = mach_port_t;
type mach_port_name_t = natural_t;
type mach_port_right_t = natural_t;

type vm_map_t = mach_port_t;
type mach_vm_address_t = u32;
type mach_vm_size_t = u32;

type thread_inspect_t = mach_port_t;
type thread_flavor_t = natural_t;
type thread_info_t = MutPtr<integer_t>;
type thread_state_flavor_t = i32;
type thread_state_t = MutPtr<natural_t>;

type mach_msg_type_number_t = natural_t;
type mach_msg_type_name_t = u32;

type mach_msg_option_t = integer_t;
type mach_msg_size_t = natural_t;
type mach_msg_timeout_t = natural_t;

type exception_mask_t = u32;
type exception_behavior_t = i32;

type policy_t = i32;
const POLICY_TIMESHARE: policy_t = 1;

const THREAD_BASIC_INFO: thread_flavor_t = 3;
const THREAD_SCHED_TIMESHARE_INFO: thread_flavor_t = 10;

const ARM_THREAD_STATE: thread_state_flavor_t = 1;
const MACHINE_THREAD_STATE: thread_state_flavor_t = ARM_THREAD_STATE;

#[repr(C, packed)]
struct arm_thread_state {
    r: [u32; 13], // General purpose register r0-r12
    sp: u32, // Stack pointer r13
    lr: u32, // Link register r14
    pc: u32, // Program counter r15
    cpsr: u32 // Current program status register
}
unsafe impl SafeRead for arm_thread_state {}

#[repr(C, packed)]
struct time_value_t {
    seconds: integer_t,
    microseconds: integer_t,
}
unsafe impl SafeRead for time_value_t {}

#[repr(C, packed)]
struct thread_basic_info {
    user_time: time_value_t,
    system_time: time_value_t,
    cpu_usage: integer_t,
    policy: policy_t,
    run_state: integer_t,
    flags: integer_t,
    suspend_count: integer_t,
    sleep_time: integer_t,
}
unsafe impl SafeRead for thread_basic_info {}

#[repr(C, packed)]
struct policy_timeshare_info {
    max_priority: integer_t,
    base_priority: integer_t,
    cur_priority: integer_t,
    depressed: boolean_t,
    depress_priority: integer_t,
}
unsafe impl SafeRead for policy_timeshare_info {}

const TH_STATE_RUNNING: integer_t = 1;
const TH_STATE_STOPPED: integer_t = 2;

/// Undocumented Darwin function that returns information about a thread.
///
/// I swear these are the correct type names, the API is just... like this.
fn thread_info(
    env: &mut Environment,
    target_act: thread_inspect_t,
    flavor: thread_flavor_t,
    thread_info_out: thread_info_t,
    thread_info_out_count: MutPtr<mach_msg_type_number_t>,
) -> kern_return_t {
    let thread = env.threads.get(target_act as usize).unwrap();

    let out_size_available = env.mem.read(thread_info_out_count);

    match flavor {
        THREAD_BASIC_INFO => {
            let out_size_expected =
                guest_size_of::<thread_basic_info>() / guest_size_of::<integer_t>();
            assert!(out_size_expected <= out_size_available);
            env.mem.write(
                thread_info_out.cast(),
                thread_basic_info {
                    user_time: time_value_t {
                        seconds: 0,
                        microseconds: 0,
                    },
                    system_time: time_value_t {
                        seconds: 0,
                        microseconds: 0,
                    },
                    cpu_usage: 0,
                    policy: POLICY_TIMESHARE, // no idea if this is realistic
                    run_state: if thread.active {
                        TH_STATE_RUNNING
                    } else {
                        TH_STATE_STOPPED
                    },
                    flags: 0, // FIXME
                    suspend_count: if thread.blocked_by == Suspended {
                        1
                    } else {
                        0
                    },
                    sleep_time: 0,
                },
            );
            env.mem.write(thread_info_out_count, out_size_expected);
        }
        THREAD_SCHED_TIMESHARE_INFO => {
            let out_size_expected =
                guest_size_of::<policy_timeshare_info>() / guest_size_of::<integer_t>();
            assert!(out_size_expected <= out_size_available);
            env.mem.write(
                thread_info_out.cast(),
                policy_timeshare_info {
                    max_priority: 0,
                    base_priority: 0,
                    cur_priority: 0,
                    depressed: 0,
                    depress_priority: 0,
                },
            );
            env.mem.write(thread_info_out_count, out_size_expected);
        }
        _ => unimplemented!("TODO: flavor {:?}", flavor),
    }

    KERN_SUCCESS
}

type thread_t = mach_port_t;
type thread_policy_flavor_t = natural_t;
type thread_policy_t = MutPtr<integer_t>;

// This is actually from the thread policy file.
fn thread_policy_set(
    _env: &mut Environment,
    thread: thread_t,
    flavor: thread_policy_flavor_t,
    policy_info: thread_policy_t,
    count: mach_msg_type_number_t,
) -> kern_return_t {
    log!(
        "TODO: thread_policy_set({}, {}, {:?}, {}) (ignored)",
        thread,
        flavor,
        policy_info,
        count
    );
    KERN_SUCCESS
}

fn mach_thread_self(env: &mut Environment) -> i32 {
    //assert_eq!(env.current_thread, 0);
    env.current_thread as i32
}

fn task_threads(
    env: &mut Environment,
    task: task_t,
    thread_list: MutPtr<thread_act_array_t>,
    thread_count_: MutPtr<mach_msg_type_number_t>
) -> kern_return_t {
    assert_eq!(task, 0); // mach_task_self_
    let thread_count = env.threads.len() as GuestUSize;
    let arr: MutPtr<thread_act_t> = env.mem.alloc(thread_count * guest_size_of::<thread_act_t>()).cast();
    for i in 0..thread_count {
        env.mem.write(arr + i, i);
    }
    env.mem.write(thread_list, arr);
    env.mem.write(thread_count_, thread_count);
    KERN_SUCCESS
}

fn mach_msg(
    env: &mut Environment,
    msg: MutVoidPtr, // MutPtr<mach_msg_header_t>,
    option: mach_msg_option_t,
    send_size: mach_msg_size_t,
    rcv_size: mach_msg_size_t,
    rcv_name: mach_port_name_t,
    timeout: mach_msg_timeout_t,
    notify: mach_port_name_t,
) -> mach_msg_return_t {
    log_dbg!("TOD0: mach_msg send/rcv");
    KERN_SUCCESS
}

fn mach_port_allocate(
    env: &mut Environment,
    task: ipc_space_t,
    right: mach_port_right_t,
    name: MutPtr<mach_port_name_t>
) -> kern_return_t {
    // TODO: implement
    KERN_SUCCESS
}

fn mach_port_deallocate(
    env: &mut Environment,
    task: ipc_space_t,
    name: mach_port_name_t
) -> kern_return_t {
    // TODO: implement
    KERN_SUCCESS
}

fn mach_port_insert_right(
    env: &mut Environment,
    task: ipc_space_t,
    name: mach_port_name_t,
    poly: mach_port_t,
    polyPoly: mach_msg_type_name_t
) -> kern_return_t {
    // TODO: implement
    KERN_SUCCESS
}

fn vm_deallocate(
    env: &mut Environment,
    target_task: vm_map_t,
    address: mach_vm_address_t,
    size: mach_vm_size_t
) -> kern_return_t {
    // Is it OK? vm_deallocate() can be called to free a list created by task_threads()
    // But in general there is no guarantee what memory was previously allocated by malloc!
    env.mem.free(Ptr::from_bits(address));
    KERN_SUCCESS
}

fn exc_server(
    env: &mut Environment,
    request_msg: MutVoidPtr, // MutPtr<mach_msg_header_t>,
    reply_ms: MutVoidPtr, // MutPtr<mach_msg_header_t>,
) -> boolean_t {
    1 // FALSE
}

fn task_set_exception_ports(
    env: &mut Environment,
    task: task_t,
    exception_mask: exception_mask_t,
    new_port: mach_port_t,
    behavior: exception_behavior_t,
    new_flavor: thread_state_flavor_t
) -> kern_return_t {
    KERN_SUCCESS
}

fn thread_suspend(
    env: &mut Environment,
    target_thread: thread_inspect_t
) -> kern_return_t {
    env.suspend_thread(target_thread as ThreadId);
    0
}

fn thread_resume(
    env: &mut Environment,
    target_thread: thread_inspect_t
) -> kern_return_t {
    env.resume_thread(target_thread as ThreadId);
    0
}

fn thread_get_state(
    env: &mut Environment,
    target_thread: thread_act_t,
    flavor: thread_state_flavor_t,
    old_state: thread_state_t,
    old_state_count: MutPtr<mach_msg_type_number_t>,
) -> kern_return_t {
    assert_eq!(flavor, MACHINE_THREAD_STATE);
    let old_thread = env.current_thread;
    env.switch_thread(target_thread as ThreadId);

    let out_size_available = env.mem.read(old_state_count);
    let out_size_expected =
        guest_size_of::<arm_thread_state>() / guest_size_of::<integer_t>();
    assert!(out_size_expected <= out_size_available);
    let state = arm_thread_state {
        r: env.cpu.regs()[..13].try_into().unwrap(),
        sp: env.cpu.regs()[crate::cpu::Cpu::SP],
        lr: env.cpu.regs()[crate::cpu::Cpu::LR],
        pc: env.cpu.regs()[crate::cpu::Cpu::PC],
        cpsr: env.cpu.cpsr()
    };
    env.mem.write(old_state.cast(), state);
    env.mem.write(old_state_count, out_size_expected);

    env.switch_thread(old_thread);
    0
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(thread_info(_, _, _, _)),
    export_c_func!(thread_policy_set(_, _, _, _)),
    export_c_func!(mach_thread_self()),
    export_c_func!(task_threads(_, _, _)),
    export_c_func!(mach_msg(_, _, _, _, _, _, _)),
    export_c_func!(mach_port_allocate(_, _, _)),
    export_c_func!(mach_port_deallocate(_, _)),
    export_c_func!(mach_port_insert_right(_, _, _, _)),
    export_c_func!(vm_deallocate(_, _, _)),
    export_c_func!(exc_server(_, _)),
    export_c_func!(task_set_exception_ports(_, _, _, _, _)),
    export_c_func!(thread_suspend(_)),
    export_c_func!(thread_resume(_)),
    export_c_func!(thread_get_state(_, _, _, _)),
];
