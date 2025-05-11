#![no_std]
#![no_main]

mod lang_items;
mod sbi;
mod console;
//mod logging;
#[path = "boards/qemu.rs"]
mod board;

const SYSCALL_EXIT: usize = 93;
const SYSCALL_WRITE: usize = 64;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id,
        )
    }
    ret
}

pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

core::arch::global_asm!(include_str!("entry.asm"));

pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
pub fn rust_main() {
    clear_bss();

    println!("hello, world!");
    use crate::board::QEMUExit;
    crate::board::QEMU_EXIT_HANDLE.exit_success(); 
}
