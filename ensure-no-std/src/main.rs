#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// #![feature(default_alloc_error_handler)]

#[allow(unused_imports)]
use poseidon;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
