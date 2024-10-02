mod idt;

use crate::println;
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: idt::Idt = {
        let mut idt = idt::Idt::new();

        idt.set_handler(0, divide_by_zero_handler);
        idt.set_handler(3, breakpoint_handler);

        idt
    };
}

extern "C" fn divide_by_zero_handler() -> ! {
    println!("EXCEPTION: DIVIDE BY ZERO");
    loop {}
}

extern "C" fn breakpoint_handler() -> ! {
    println!("EXCEPTION: BREAKPOINT");
    loop {}
}

pub fn init_idt() {
    IDT.load();
}
