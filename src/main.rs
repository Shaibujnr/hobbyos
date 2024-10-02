#![no_std] // exclude rust standard library as it requires existing os features
#![no_main] // disable rust level entry points
#![feature(custom_test_frameworks)]
#![test_runner(hobbyos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use hobbyos::println;

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hobbyos::test_panic_handler(info)
}

#[no_mangle] // don't mangle the name of this function as most linkers expect the entry point name to be '_start'
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    hobbyos::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}
