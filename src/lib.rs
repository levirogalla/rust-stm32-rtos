//! Minimal RTOS Boilerplate for a Single-Core STM32
//! Implements basic threading, synchronization, and scheduling

// #![no_std]
#![no_main]
#![no_std]

mod interupts;
mod state;
mod syscalls;
mod utils;
mod synchronization;


use core::{arch::asm, ptr};
use core::sync::atomic::Ordering;

use rtt_target::{rprintln, rtt_init_print};
use utils::InterruptContext;

extern "C" {
    fn runFirstThread();
    // fn idleThread();
}
// /// Represents a thread/task in the RTOS
// #[repr(C)]
// pub struct Thread {
//     stack_ptr: *mut u8, // Stack pointer
//     id: u32,            // Thread ID
//     state: ThreadState, // Thread state (Running, Ready, Blocked)
// }

// /// Possible states of a thread
// #[derive(Clone, Copy, PartialEq)]
// pub enum ThreadState {
//     Running,
//     Ready,
//     Blocked,
// }

pub fn kernel_init() {
    // Initialize the scheduler
    rtt_init_print!();
    state::LAST_STACK_END.store(
        utils::VectorTable::get_sp() - state::MSP_STACK_SIZE,
        Ordering::SeqCst,
    );
    rprintln!("Kernel initialized");
}
#[no_mangle]
pub fn start_scheduler() {
    unsafe { asm!("svc #0"); };
    // let tcb = create_task(idle as u32, 0x400).unwrap(); // create the idle task
    // unsafe { 
    //     asm!("msr psp, {0}", in(reg) tcb.stack_ptr);
    //     runFirstThread(); 
    // }
    // rprintln!("here")
    // unsafe {
    //     let control: u32;
    //     asm!(
    //         "mov r0, #0x0",
    //         "msr CONTROL, r0",

    //     );
    // //     rprintln!("Control: {:x}", control);
    // }

    // rprintln!("TCB: {:?}", tcb);
    // unsafe {
    // unsafe {
    //     let psp: u32;
    //     let msp: u32;
    //     let sp: u32;
    //     asm!(
    //         "mrs {0}, psp",
    //         "mrs {1}, msp",
    //         "mov {2}, sp",
    //         out(reg) psp,
    //         out(reg) msp,
    //         out(reg) sp
    //     );
    //     rprintln!("PSP: {:x}, MSP: {:x}, SP: {:x}, TSB: {:x}", psp, msp, sp, tcb.stack_ptr);
    // }

//     rprintln!("{:x}, {:x}", *((tcb.stack_start as *const u32).offset(-2)), idle as u32);
    //     asm!(
    //         "mov pc, {0}",
    //         in(reg) idle as u32
    //     )
    // };
    // unsafe {
    //     rprintln!("{}", (tcb.stack_ptr as *const u32) as u32);
    //     rprintln!("{:X}", *(tcb.stack_start as *const u32).offset(-1));
    //     rprintln!("{:X}, {:X}", *(tcb.stack_start as *const u32).offset(-2), idle as u32);
    //     asm!(
    //         "mov r0, {0}",
    //         // "mov lr, #0xFFFFFFFD", // load context from psp
    //         "msr psp, r0",
    //         // "bx lr",
    //         in(reg) tcb.stack_ptr,
    //         // options(noreturn)
    //     );
    //     rprintln!("{:?}", InterruptContext::load());
    //     if (tcb.stack_ptr as u32) & 0x7 != 0 {
    //         rprintln!("ERROR: PSP Misaligned! Fixing...");
    //     }
    // }
    // rprintln!("Hre");
    // unsafe {
    //     asm!(
    //         // Set lr to return to the caller after context switch
            
    //         // Set psp (Process Stack Pointer)
    //         "mov r0, {0}",
    //         "msr psp, r0",  // Set PSP from the value passed into r0
    //         "ldr r0, =0xFFFFFFFD",  
    
    //         // Return to thread execution using bx lr
    //         "bx r0",
            
    //         in(reg) tcb.stack_ptr,  // Pass the stack pointer value
    //         options(noreturn, preserves_flags)  // Don't let compiler optimize or clobber flags
    //     );
    // }
    // unsafe {
    //     asm!(
    //         "mov pc, {0}",
    //         in(reg) tcb.
    //     )
    // }    

    // unsafe {
    //     let pc: u32;
    //     let pc_addr: u32;
    //     let r11: u32;
    //     asm!(
    //         "mov r0, {0}", // load psp into r0
    //         "mov lr, #0xFFFFFFFD", // this tells the cpu to restore the interupt state from the psp stack
    //         "ldmia r0!, {{r4-r11}}", // load value r0 is pointing into r4-11, increment r0 to do this
    //         "msr psp, r0", // restore the correct psp since we updated the stack
    //         "mov r4, r0",
    //         "ldmia r4!, {{r0-r3, r6, r7}}",
    //         "mov {1}, r7",
    //         "mov {2}, r4",
    //         "mov {3}, r11",
    //         // "bx lr", // special return to start executing the idle task
    //         in(reg) tcb.stack_ptr,
    //         out(reg) pc,
    //         out(reg) pc_addr,
    //         out(reg) r11
    //     );
    //     rprintln!("In sched: pc: {:x}, pc_addr: {:x}, r11: {}", pc, pc_addr, r11);

    //     let v = *(tcb.stack_ptr as *const u32).offset(0);
    //     rprintln!("In sched: {}", v);

    //     let addr = 0x20015ff8;
    //     rprintln!("Addr: {:x}", *(addr as *const u32).offset(0));
    // }
    // loop {}
    // sp: 20017BC0
    // start: 20017C00
    // size: 1000
}

pub fn create_task(task: u32, stack_size: u32) -> Option<state::TCB> {
    let stack_start = reserve_stack_space(stack_size)?; // reserve space and get the psp
    let fake_context = utils::InterruptContext::new_fake(task as u32); // make a fake context
    let psp = unsafe {
        let sp = fake_context.push_to_sp(stack_start as *mut u32);
        rprintln!("sp in create task: {:x}, {}", sp, *(sp as *const u32));
        sp
        // utils::CalleeRegisters::new_fake().push_to_sp((sp as *mut u32).offset(-1))
    }; // push the context to the new reserved stack space, this is okay since we know that reserve stack space returns a valid stack
       // although we can still overflow if there is not enough space. I need to check for overflow.
    let tcb = state::TCB {
        stack_ptr: psp,
        stack_size: stack_size,
        stack_start,
        id: 0,
    };
    critical_section::with(|cs_token| state::THREADS.borrow(cs_token).borrow_mut().enqueue(tcb))?;
    Some(tcb)
}

/// Reserves space on the stack for a new thread and return the stack pointer, this will also make sure the stack is 8-byte aligned
fn reserve_stack_space(stack_size: u32) -> Option<u32> {
    let aligned_stack_size = (stack_size + 7) & !7; // round up to the nearest 8 aligned size
    let sp = state::LAST_STACK_END.fetch_sub(aligned_stack_size, Ordering::SeqCst);
    if sp - stack_size < 0x20000000 { // make sure not to overflow the stack
        None
    } else {
        Some(sp)
    }
}

fn idle() -> ! {
    
    // for _ in 0..1000000 {
    //     unsafe { asm!("nop") }
    // }
    rprintln!("Here");
    loop {
        rprintln!("Idle\n");
        for _ in 0..1000000 {
            unsafe { asm!("nop") }
        }
        // unsafe {asm!("wfi", options(nomem, nostack))}
    }
}

pub fn test_kernel() {
    test_queue();
}

fn test_queue() {
    rprintln!("Running queue tests");
    rprintln!("Creating queue...");
    let mut queue: utils::Queue<u8, 3> = utils::Queue::new();

    rprintln!("Enqueueing, starting with {:?}", queue);
    assert_eq!(queue.enqueue(10), Some(1));
    assert_eq!(queue.enqueue(20), Some(2));
    assert_eq!(queue.enqueue(30), Some(3));
    assert_eq!(queue.peek(), Some(&10));

    assert_eq!(queue.length(), 3);
    assert!(queue.enqueue(40).is_none());

    rprintln!("Dequeueing, starting with {:?}", queue);
    assert_eq!(queue.dequeue(), Some(10));
    assert_eq!(queue.dequeue(), Some(20));
    assert_eq!(queue.dequeue(), Some(30));
    assert_eq!(queue.peek(), None);

    rprintln!("Cycling, starting with {:?}", queue);
    assert!(queue.dequeue().is_none());
    assert_eq!(queue.length(), 0);
    queue.enqueue(10);
    queue.enqueue(20);
    queue.dequeue();
    queue.enqueue(30);
    assert_eq!(queue.dequeue(), Some(20));
    queue.enqueue(40);
    assert_eq!(queue.dequeue(), Some(30));
    rprintln!("Queue tests passed");
}

// /// Creates a new thread
// pub fn pthread_create(_thread: &mut Thread, _stack: &mut [u8], _entry: fn()) {
//     // TODO: Initialize the thread structure and set up its stack
// }

// /// Terminates the calling thread
// pub fn pthread_exit() -> ! {
//     // TODO: Remove the thread from the scheduler and switch context
//     loop {} // Infinite loop to prevent returning
// }

// /// Returns the ID of the calling thread
// pub fn pthread_self() -> u32 {
//     // TODO: Return the currently running thread ID
//     0
// }

// /// Yields CPU to another thread (voluntary context switch)
// pub fn pthread_yield() {
//     // TODO: Trigger a scheduler context switch
// }

// /// Mutex structure for thread synchronization
// pub struct Mutex {
//     locked: AtomicBool,
// }

// /// Initializes a mutex
// pub fn pthread_mutex_init(mutex: &mut Mutex) {
//     mutex.locked = AtomicBool::new(false);
// }

// /// Locks a mutex (blocks if already locked)
// pub fn pthread_mutex_lock(mutex: &mut Mutex) {
//     // TODO: Implement spinlock or blocking wait
// }

// /// Unlocks a mutex
// pub fn pthread_mutex_unlock(mutex: &mut Mutex) {
//     // TODO: Release the lock
// }

// /// Semaphore structure
// pub struct Semaphore {
//     count: u32,
// }

// /// Initializes a semaphore
// pub fn sem_init(sem: &mut Semaphore, _value: u32) {
//     // TODO: Initialize semaphore count
// }

// /// Decrements (waits) on a semaphore
// pub fn sem_wait(sem: &mut Semaphore) {
//     // TODO: Block if count is zero, otherwise decrement
// }

// /// Increments (signals) a semaphore
// pub fn sem_post(sem: &mut Semaphore) {
//     // TODO: Increase the count and wake up blocked thread if any
// }

// /// Retrieves the current system time
// pub fn clock_gettime() -> u32 {
//     // TODO: Return the current system tick count
//     0
// }

// /// Sleeps the calling thread for a duration (in ms)
// pub fn nanosleep(_duration: u32) {
//     // TODO: Implement sleep using system timer
// }

// /// Switches to the next thread in the scheduler
// pub fn sched_yield() {
//     let icsr = 0xE000ED04 as *mut u32; // Address of SCB_ICSR
//     unsafe {
//         core::ptr::write_volatile(icsr, 1 << 28); // Set PENDSVSET (bit 28)
//     }
// }

// /// Basic Round-Robin Scheduler
// pub fn scheduler_run() {
//     // TODO: Implement simple round-robin or priority-based scheduler
// }

// #[cfg(test)]
// #[defmt_test::tests]
// mod queue_tests {
//     use super::kernel;

//     #[test]
//     fn test_queue() {

// }
