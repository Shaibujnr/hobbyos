#![no_std] // exclude rust standard library as it requires existing os features
#![no_main] // disable rust level entry points

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle the name of this function as most linkers expect the entry point name to be '_start'
pub extern "C" fn _start() -> ! {
    loop {}
}
