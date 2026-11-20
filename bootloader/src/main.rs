#![no_main]
#![no_std]

use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::println!("InfinitBoot loaded!");
    uefi::println!("Entering kernel...");

    let info = kernel::kernel_main();

    uefi::println!("Kernel: {}", info.name);
    uefi::println!("Version: {}", info.version);

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}