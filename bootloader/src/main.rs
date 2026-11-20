#![no_main]
#![no_std]

use common::{BootInfo, MemoryInfo};
use uefi::boot;                        
use uefi::mem::memory_map::{MemoryMap, MemoryType};
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::println!("InfinitBoot loaded!");
    uefi::println!("Entering kernel...");

    let memory_map = boot::memory_map(MemoryType::LOADER_DATA).unwrap();
    let memory_regions = memory_map.entries().count() as u64;

    uefi::println!("Memory regions: {}", memory_regions);

    let boot_info = BootInfo {
     bootloader_name: "InfinitBoot",
     version: 1,
     memory: MemoryInfo {
       total_memory: 0,
       memory_regions,
       },
    };


    kernel::kernel_main(&boot_info);

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}