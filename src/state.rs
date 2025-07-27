use super::kernel::TCB;
use super::utils::Queue;

use core::cell::RefCell;
use core::sync::atomic::AtomicU32;

use critical_section::Mutex;

pub const MSP_STACK_SIZE: u32 = 0x2000;
pub const NUM_PRIORITIES: usize = 5; // Number of task priorities
                                     // TODO: once I implement the heap, the queue implementation should be done on the heap so that it only has to store references to each tcb rather than copying it everytime

/// A global queue of threads (TCBs) that are ready to run.
// pub static READY_TASKS: Mutex<RefCell<Queue<TCB, 20>>> = Mutex::new(RefCell::new(Queue::new()));
pub static READY_TASKS: [Mutex<RefCell<Queue<TCB, 20>>>; NUM_PRIORITIES] = [
    Mutex::new(RefCell::new(Queue::new())), // Priority 0 Idle task
    Mutex::new(RefCell::new(Queue::new())), // Priority 1 Low priority tasks
    Mutex::new(RefCell::new(Queue::new())), // Priority 2 Normal priority tasks
    Mutex::new(RefCell::new(Queue::new())), // Priority 3 High priority tasks
    Mutex::new(RefCell::new(Queue::new())), // Priority 4 Urgent tasks
];
pub static SLEEPING_TASKS: [Mutex<RefCell<Queue<TCB, 20>>>; NUM_PRIORITIES] = [
    Mutex::new(RefCell::new(Queue::new())), // Priority 0
    Mutex::new(RefCell::new(Queue::new())), // Priority 1
    Mutex::new(RefCell::new(Queue::new())), // Priority 2
    Mutex::new(RefCell::new(Queue::new())), // Priority 3
    Mutex::new(RefCell::new(Queue::new())), // Priority 4
];
pub static BLOCKED_TASKS: [Mutex<RefCell<Queue<TCB, 20>>>; NUM_PRIORITIES] = [
    Mutex::new(RefCell::new(Queue::new())), // Priority 0
    Mutex::new(RefCell::new(Queue::new())), // Priority 1
    Mutex::new(RefCell::new(Queue::new())), // Priority 2
    Mutex::new(RefCell::new(Queue::new())), // Priority 3
    Mutex::new(RefCell::new(Queue::new())), // Priority 4
];
// pub static THREADS: Mutex<RefCell<Queue<TCB, 20>>> = Mutex::new(RefCell::new(Queue::new()));

/// memory location where the last stack end is stored
pub static LAST_STACK_END: AtomicU32 = AtomicU32::new(0);

/// A mutex that holds the currently running task control block (TCB).
pub static RUNNING_TASK: Mutex<RefCell<Option<TCB>>> = Mutex::new(RefCell::new(None));
// thread states: new ready running blocked terminated
