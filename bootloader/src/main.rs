#![no_main]
#![no_std]

use uefi::prelude::*;

#[entry]
fn main() -> Status {
    Status::SUCCESS
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}