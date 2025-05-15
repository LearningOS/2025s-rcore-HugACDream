//! Process management syscalls
use crate::{
    task::{exit_current_and_run_next, suspend_current_and_run_next, current_task},
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

// TODO: implement the syscall
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 => {
            // 读取用户空间 id 地址的 u8
            let ptr = id as *const u8;
            unsafe { *ptr as isize }
        }
        1 => {
            // 写入 data 的低 8 位到用户空间 id 地址
            let ptr = id as *mut u8;
            unsafe { *ptr = data as u8; }
            0
        }
        2 => {
            // 查询当前任务调用编号为 id 的 syscall 次数
            if let Some(task) = current_task() {
                if id < 512 {
                return task.syscall_counts[id] as isize;
            }
            }
            -1
        }
        _ => -1,
    }
}
