//! Minimal RTOS Boilerplate for a Single-Core STM32
//! Implements basic threading, synchronization, and scheduling

// #![no_std]
#![no_main]
#![no_std]

mod interupts;
mod kernel;
pub mod registers;
mod state;
mod synchronization;
mod syscalls;
mod utils;

use core::sync::atomic::Ordering;
use core::arch::asm;

use kernel::TCB;
use rtt_target::{rprintln, rtt_init_print};

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

/// Initializes the kernel, setting up the initial stack pointer and preparing for task scheduling.
pub fn kernel_init() {
    // Initialize the scheduler
    rtt_init_print!();
    state::LAST_STACK_END.store(
        utils::VectorTable::get_sp() - state::MSP_STACK_SIZE,
        Ordering::SeqCst,
    );
    registers::systick::enable_counter();
    registers::systick::enable_systick_interrupt();

    let reload_value = kernel::DEFAULT_SYSTICK_INTERVAL * (kernel::CPU_CLOCK_HZ / 1_000_000);
    rprintln!("Setting SysTick reload value to: {}", reload_value);
    registers::systick::set_reload_value(reload_value);

    rprintln!("Kernel initialized");
}

/// Starts the idle task and switches to it using the initial context switch.
pub fn start_scheduler() {
    unsafe { asm!("svc #0") }
}

/// Creates a new task control block (TCB) for a task with the given stack size in bytes.
pub fn create_task(
    task: fn() -> !,
    stack_size: u32,
    timeout: u32,
    args: u32,
    priority: u32,
) -> Option<TCB> {
    let tcb = TCB::new_task(task as u32, stack_size, timeout, args, priority)?;
    critical_section::with(|cs_token| {
        state::READY_TASKS[priority as usize]
            .borrow(cs_token)
            .borrow_mut()
            .enqueue(tcb);
    });
    Some(tcb)
}

/// Yields the CPU to allow other tasks to run, triggering a context switch.
pub fn yield_cpu() {
    unsafe {
        asm!("svc #1");
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
