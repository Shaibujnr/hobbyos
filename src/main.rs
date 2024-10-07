#![no_std] // exclude rust standard library as it requires existing os features
#![no_main] // disable rust level entry points
#![feature(custom_test_frameworks)]
#![test_runner(hobbyos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use hobbyos::{
    memory::{translate_addr, BootInfoFrameAllocator},
    println,
};

entry_point!(kernel_main);

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hobbyos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hobbyos::test_panic_handler(info)
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use hobbyos::memory;
    use x86_64::{
        structures::paging::{Page, Translate},
        VirtAddr,
    };

    println!("Hello World{}", "!");
    hobbyos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    hobbyos::hlt_loop();
}
