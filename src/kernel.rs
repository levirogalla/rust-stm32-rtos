
use super::state;
use super::utils::{CalleeRegisters, Queue, ScratchRegisters};

use rtt_target::rprintln;

use core::arch::asm;
use core::cell::RefMut;
use core::sync::atomic::Ordering;

unsafe extern "C" {
    pub unsafe fn initial_context_switch(psp: *const u32) -> !;
}

pub const DEFAULT_SYSTICK_INTERVAL: u32 = 1_000; // 1ms in microseconds, this is very not accurate tho, not sure if I am doing something wrong or its something else to do with the clock.
pub const CPU_CLOCK_HZ: u32 = 16_000_000; // 16MHz, this is the default clock speed for most STM32 MCUs

static mut ID_COUNTER: u32 = 0;
#[derive(Debug, Copy)]
pub struct TCB {
    pub stack_ptr: u32,
    pub stack_size: u32,
    pub stack_start: u32,
    pub timeout: u32,
    pub args: u32,
    pub priority: u32,
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
            timeout: self.timeout,
            args: self.args,
            priority: self.priority,
            id: unsafe { ID_COUNTER },
        }
    }
}

impl TCB {
    /// Creates a new task control block (TCB) for a task with the given stack size in bytes. This function will reserve stack space, create a fake context, and initialize the TCB with a unique ID. This does not add the TCB to the scheduler's queue.
    /// # Arguments
    /// * `task` - The task function to be executed, represented as a u32
    /// * `stack_size` - The size of the stack for the task in bytes
    /// * `timeout` - The timeout for the task in milliseconds, 0 for no timout, timout will be checked in intervals set by the set timeout svcall, the default is 10ms
    /// * `args` - Arguments to be passed to the task function, represented as a u32, it should be a pointer to some user defined data structure
    /// * `priority` - The priority of the task, represented as a u32,
    pub fn new_task(
        task: u32,
        stack_size: u32,
        timeout: u32,
        args: u32,
        priority: u32,
    ) -> Option<TCB> {
        let stack_start = reserve_stack_space(stack_size)?; // reserve space and get the psp
        let fake_context = ScratchRegisters::new_fake(task); // make a fake context
        let psp = unsafe {
            let sp = fake_context.push_to_sp(stack_start as *mut u32);
            CalleeRegisters::new_fake().push_to_sp(sp as *mut u32)
        };
        let tcb = TCB {
            stack_ptr: psp as u32,
            stack_size,
            timeout,
            args,
            priority,
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

/// Get next task to run from the ready queues
pub fn get_next_task() -> TCB {
    critical_section::with(|cs_token| {
        // Initialize the threads queue to lowest priority queue since this will always have the idle task
        let mut threads_queue: RefMut<Queue<TCB, 20>> =
            state::READY_TASKS[0].borrow(cs_token).borrow_mut();

        // Find the highest priority queue with tasks
        for i in (0..state::NUM_PRIORITIES).rev() {
            threads_queue = state::READY_TASKS[i].borrow(cs_token).borrow_mut();
            if threads_queue.length() > 0 {
                break;
            }
        }

        // Dequeue the next task to run
        threads_queue.dequeue().unwrap()
    })
}

pub fn idle() -> ! {
    rprintln!("Here");
    loop {
        rprintln!("Idle\n");
        for _ in 0..200000 {
            unsafe { asm!("nop") }
        }
    }
}
