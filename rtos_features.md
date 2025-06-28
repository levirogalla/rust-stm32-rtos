# Essential RTOS Features Reference

This document summarizes the essential features of a basic Real-Time Operating System (RTOS) for embedded systems, particularly for single-core microcontrollers like STM32.

## Core Features

### 1. Task Management
- **Task Creation**: Ability to create tasks with specific entry points and stack sizes.
- **Task Termination**: Mechanism to terminate tasks and free resources.
- **Task States**: Tasks can be in states such as Running, Ready, or Blocked.

### 2. Scheduler
- **Round-Robin Scheduling**: Basic time-slice-based scheduling for fairness.
- **Priority-Based Scheduling**: Option to assign priorities to tasks.
- **Context Switching**: Efficient switching between tasks.

### 3. Synchronization
- **Mutexes**: Mutual exclusion locks to protect shared resources.
- **Semaphores**: Counting semaphores for signaling and resource management.
- **Event Flags**: Mechanism to signal events between tasks.

### 4. Inter-Task Communication
- **Message Queues**: FIFO queues for passing messages between tasks.
- **Mailboxes**: Fixed-size message buffers for task communication.

### 5. Timing and Delays
- **System Ticks**: Timer-based system tick for scheduling and delays.
- **Delays**: Functions like `nanosleep` to pause tasks for a duration.
- **Timeouts**: Support for timeouts in synchronization primitives.

### 6. Interrupt Handling
- **Interrupt Service Routines (ISRs)**: Mechanism to handle hardware interrupts.
- **Deferred Interrupt Processing**: Use of tasks or queues to handle interrupt-related work.

### 7. Memory Management
- **Stack Management**: Allocate and manage task stacks.
- **Dynamic Memory Allocation**: Optional support for heap-based memory allocation.

### 8. Debugging and Diagnostics
- **Task Monitoring**: APIs to inspect task states and stack usage.
- **System Logs**: Logging mechanisms for debugging.
- **Performance Metrics**: Measure CPU usage, task execution time, etc.

### 9. Power Management
- **Idle Task**: Low-power idle task when no other tasks are ready.
- **Sleep Modes**: Integration with microcontroller sleep modes.

### 10. Error Handling
- **Fault Recovery**: Mechanisms to recover from task or system faults.
- **Assertions**: Debugging aids to catch programming errors.

## Notes
- This list is not exhaustive but covers the foundational features expected in a basic RTOS.
- Advanced RTOS implementations may include features like dynamic priority adjustment, real-time clocks, and more.
