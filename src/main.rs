#![cfg_attr(not(test), no_std)]
#![no_main]

use core::arch::asm;
use cortex_m_rt::entry;
use rtt_target::rprintln;
use rust_stm32_helloworld::{self, test_kernel};
// use cortex_m;
use rust_stm32_helloworld::{kernel_init, start_scheduler};
// use rust_stm32_helloworld::kernel;

#[entry]
fn main() -> ! {
    kernel_init();

    start_scheduler();
    rprintln!("Scheduler started");

    test_kernel();

    loop {}
}
// 0x61000000
// 134232299