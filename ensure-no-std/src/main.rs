#![no_std]
#![no_main]

// #![feature(default_alloc_error_handler)]

use core::panic::PanicInfo;

#[allow(unused_imports)]
use poseidon;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
