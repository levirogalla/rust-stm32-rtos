use super::utils::Queue;
use super::kernel::TCB;

use core::cell::RefCell;
use core::sync::atomic::AtomicU32;

use critical_section::Mutex;

pub const MSP_STACK_SIZE: u32 = 0x2000;
// TODO: once I implement the heap, the queue implementation should be done on the heap so that it only has to store references to each tcb rather than copying it everytime

/// A global queue of threads (TCBs) that are ready to run.
pub static THREADS: Mutex<RefCell<Queue<TCB, 20>>> = Mutex::new(RefCell::new(Queue::new()));

/// memory location where the last stack end is stored
pub static LAST_STACK_END: AtomicU32 = AtomicU32::new(0);

/// A mutex that holds the currently running task control block (TCB).
pub static RUNNING_TASK: Mutex<RefCell<Option<TCB>>> = Mutex::new(RefCell::new(None));
// thread states: new ready running blocked terminated