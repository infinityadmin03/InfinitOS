#![no_main]
#![no_std]

use common::{BootInfo, MemoryInfo};
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::println!("InfinitBoot loaded!");
    uefi::println!("Entering kernel...");

    let boot_info = BootInfo {
     bootloader_name: "InfinitBoot",
     version: 1,
     memory: MemoryInfo {
       total_memory: 0,
       memory_regions: 0,
       },
    };


    kernel::kernel_main(&boot_info);

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}