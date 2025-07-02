//! This file contains the system call implementations for the kernel. All these function are designed to be run in privileged mode.

use rtt_target::rprintln;
use crate::state;

use super::kernel::{TCB, idle, initial_context_switch};

use core::arch::asm;

pub fn start_scheduler() {
    // TODO: maybe this should running the scheduler first instead of going directly into the idle task?
    let tcb = TCB::new_task(idle as u32, 0x1000, 1, 0, 0).unwrap(); // create the idle task
    critical_section::with(|cs_token| {
        state::RUNNING_TASK.borrow(cs_token).replace(Some(tcb));
    });
    unsafe { initial_context_switch(tcb.stack_ptr as *const u32) };
}

// pub fn yield_cpu(sp: *const u32) -> Option<()> {

// }

pub fn hello_world() {
    rprintln!("Hello, world!");
    let x = 0x20015f98;
}



// pub fn start_scheduler() {
//     let tcb = create_task(idle, 0x1000).unwrap(); // create the idle task
//     rprintln!("TCB: {:?}", tcb);
//     unsafe {
//         asm!(
//             "mov r0, {}", // load psp into r0
//             "mov lr, #0xFFFFFFFD", // this tells the cpu to restore the interupt state from the msp stack
//             "ldmia r0!, {{r4-r11}}", // load value r0 is pointing into r4-11, increment r0 to do this
//             "msr psp, R0", // restore the correct psp since we updated the stack
//             "bx lr", // special return to start executing the idle task
//             in(reg) tcb.stack_ptr
//         )
//     }
//     loop {}
// }