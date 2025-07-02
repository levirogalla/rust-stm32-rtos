#![cfg_attr(not(test), no_std)]
#![no_main]

use core::arch::asm;
use cortex_m_rt::entry;
use rtt_target::rprintln;
use rust_stm32_helloworld::{self, create_task, test_kernel, yield_cpu};
// use cortex_m;
use rust_stm32_helloworld::{kernel_init, start_scheduler};
// use rust_stm32_helloworld::kernel;

fn task1() -> ! {
    loop {
        for _ in 0..3 {
            rprintln!("Task 1 is running");
        }
        yield_cpu();
    }
}

fn task2() -> ! {
    loop {
        for _ in 0..3 {
            rprintln!("Task 2 is running");
        }
        yield_cpu();
    }
}

#[entry]
fn main() -> ! {
    kernel_init();

    create_task(task1, 1000);
    create_task(task2, 1000);

    start_scheduler();


    rprintln!("Scheduler started");


    // test_kernel();


    loop {}
}
// 0x61000000
// 134232299