#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

pub mod exits;
pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod test_runner;
pub mod vga_buffer;

#[cfg(test)]
use core::panic::PanicInfo;

pub fn init() {
    gdt::init();
    interrupts::init_idt()
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use test_runner::test_panic_handler;

    test_panic_handler(info)
}
