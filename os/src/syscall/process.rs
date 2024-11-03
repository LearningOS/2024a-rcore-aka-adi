//! Process management syscalls
// use alloc::vec::Vec;

use crate::{
    config::MAX_SYSCALL_NUM, mm::{translated_byte_buffer, MapPermission, PageTable, StepByOne, VirtAddr}, task::{
        change_program_brk, current_user_token, drop_current_task_frame, exit_current_and_run_next, get_current_task_start_time, get_current_task_syscall_times, insert_current_task_frame, suspend_current_and_run_next, TaskStatus
    }, timer::get_time_us
};

use core::mem;
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let ts = TimeVal{sec: us / 1000000, usec: us % 1000000};

    unsafe {
        let ptr = &ts as *const TimeVal as *const u8;
        let mut index = 0;
        let len = mem::size_of::<TimeVal>();
        let buffers = translated_byte_buffer(current_user_token(), _ts as *const u8, len);
        for it in buffers {
            for i in it.iter_mut() {
                *i = *ptr.add(index);
                index += 1;
            }
        }
    if index != len {
        return -1;
    }
    }

    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let ti = TaskInfo{status: TaskStatus::Running, syscall_times: get_current_task_syscall_times(), time: get_time_us() / 1000 - get_current_task_start_time()};

    unsafe {
        let ptr = &ti as *const TaskInfo as *const u8;
        let mut index = 0;
        let len = mem::size_of::<TaskInfo>();
        let buffers = translated_byte_buffer(current_user_token(), _ti as *const u8, len);
        for it in buffers {
            for i in it.iter_mut() {
                *i = *ptr.add(index);
                index += 1;
            }
        }
    if index != 2016 {
        return -1;
    }
    }
    
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap");
    let start_va = VirtAddr::from(_start);
    let end_va = VirtAddr::from(_start + _len);
    if start_va.page_offset() != 0 || _port & !0x7 != 0 || _port & 0x7 == 0 {
        return -1;
    }

    // check exist
    let mut start_vpn = start_va.floor();
    let end_vpn = end_va.ceil();
    info!("wmap {} : {}", start_vpn.0, end_vpn.0);
    let pt = PageTable::from_token(current_user_token());
    while start_vpn.0 != end_vpn.0 {
        info!("wmap1 {} : {}", start_vpn.0, end_vpn.0);
        match pt.translate(start_vpn) {
            Some(pte) => {
                if pte.is_valid() {
                    return -1;
                }
            }
            None => {}
        }
        info!("wmap2 {} : {}", start_vpn.0, end_vpn.0);
        start_vpn.step();
    }
    let permissions = MapPermission::from_bits_truncate((_port as u8 | 0x8) << 1);
    insert_current_task_frame(start_va, end_va, permissions);
    0
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap");
    let start_va = VirtAddr::from(_start);
    let end_va = VirtAddr::from(_start + _len);
    if start_va.page_offset() != 0 {
        return -1;
    }

    // check exist
    let mut start_vpn = start_va.floor();
    let end_vpn = end_va.ceil();
    let pt = PageTable::from_token(current_user_token());
    while start_vpn.0 != end_vpn.0 {
        match pt.translate(start_vpn) {
            Some(pte) => {
                if !pte.is_valid() {
                    return -1;
                }
            }
            None => return -1,
        }
        start_vpn.step();
    }

    drop_current_task_frame(start_va, end_va);
    0
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
