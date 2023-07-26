#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

pub mod allocator;
pub mod exits;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod test_runner;
pub mod vga_buffer;
extern crate alloc;

#[cfg(test)]
use bootloader::{entry_point, BootInfo};
#[cfg(test)]
use core::panic::PanicInfo;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

/// Entry point for `cargo test`
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use test_runner::test_panic_handler;

    test_panic_handler(info)
}

#[cfg(test)]
entry_point!(test_kernel_main);
