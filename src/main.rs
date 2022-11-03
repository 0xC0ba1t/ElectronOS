#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(electron_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use electron_os::println;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    electron_os::init();

    // fn stack_overflow() {
    //    stack_overflow();
    // }

    // stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    electron_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    electron_os::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    electron_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
