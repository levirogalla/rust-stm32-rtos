use super::utils::Queue;

use core::cell::RefCell;
use core::sync::atomic::AtomicU32;

use critical_section::Mutex;

pub const MSP_STACK_SIZE: u32 = 0x2000;
pub static THREADS: Mutex<RefCell<Queue<TCB, 20>>> = Mutex::new(RefCell::new(Queue::new()));
pub static LAST_STACK_END: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Clone, Copy)]
pub struct TCB {
    pub stack_ptr: u32,
    pub stack_size: u32,
    pub stack_start: u32,
    pub id: u32,
    // state: ThreadState,
}

