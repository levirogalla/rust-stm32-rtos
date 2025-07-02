use crate::yield_cpu;

use super::state;
use super::utils::{CalleeRegisters, ScratchRegisters};

use rtt_target::rprintln;

use core::arch::asm;
use core::sync::atomic::Ordering;

extern "C" {
    pub fn initial_context_switch(psp: *const u32) -> !;
}

static mut ID_COUNTER: u32 = 0;
#[derive(Debug, Copy)]
pub struct TCB {
    pub stack_ptr: u32,
    pub stack_size: u32,
    pub stack_start: u32,
    pub id: u32,
    // state: ThreadState,
}

impl Clone for TCB {
    fn clone(&self) -> Self {
        unsafe {
            ID_COUNTER += 1; // increment the ID counter
        }
        TCB {
            stack_ptr: self.stack_ptr,
            stack_size: self.stack_size,
            stack_start: self.stack_start,
            id: unsafe { ID_COUNTER },
        }
    }
}

impl TCB {
    /// Creates a new task control block (TCB) for a task with the given stack size in bytes. This function will reserve stack space, create a fake context, and initialize the TCB with a unique ID. This does not add the TCB to the scheduler's queue.
    pub fn new_task(task: u32, stack_size: u32) -> Option<TCB> {
        let stack_start = reserve_stack_space(stack_size)?; // reserve space and get the psp
        let fake_context = ScratchRegisters::new_fake(task as u32); // make a fake context
        let psp = unsafe {
            let sp = fake_context.push_to_sp(stack_start as *mut u32);
            CalleeRegisters::new_fake().push_to_sp(sp as *mut u32)
        }; 
        let tcb = TCB {
            stack_ptr: psp as u32,
            stack_size: stack_size,
            stack_start,
            id: unsafe { 
                ID_COUNTER += 1;
                ID_COUNTER
            },
        };
        Some(tcb)
    }
}

/// Reserves space on the stack for a new thread and return the stack pointer, this will also make sure the stack is 8-byte aligned
pub fn reserve_stack_space(stack_size: u32) -> Option<u32> {
    let aligned_stack_size = (stack_size + 7) & !7; // round up to the nearest 8 aligned size
    let sp = state::LAST_STACK_END.fetch_sub(aligned_stack_size, Ordering::SeqCst);
    if sp - aligned_stack_size < 0x20000000 {
        // make sure not to overflow the stack
        // restore the last stack end var
        state::LAST_STACK_END.fetch_add(aligned_stack_size, Ordering::SeqCst);
        None
    } else {
        Some(sp)
    }
}

pub fn idle() -> ! {
    rprintln!("Here");
    loop {
        rprintln!("Idle\n");
        for _ in 0..200000 {
            unsafe { asm!("nop") }
        }
        yield_cpu();
        // unsafe {asm!("wfi", options(nomem, nostack))}
        let x = 0x20015f98;
    }
}
