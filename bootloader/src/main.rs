#![no_main]
#![no_std]

use common::{
    BootInfo,
    MemoryInfo,
    MemoryMap,
    MemoryRegion,
    MAX_MEMORY_REGIONS,
};
use uefi::boot;                        
use uefi::mem::memory_map::{MemoryMap as UefiMemoryMap, MemoryType};
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::println!("InfinitBoot loaded!");
    uefi::println!("Entering kernel...");

    let memory_map = boot::memory_map(MemoryType::LOADER_DATA).unwrap();
    let memory_regions = memory_map.entries().count() as u64;

    uefi::println!("Memory regions: {}", memory_regions);

    let mut total_memory = 0u64;

    let mut region_index = 0usize;

    let mut first_region = MemoryRegion {
      start: 0,
      pages: 0,
    };


    
    let mut memory_map_info = MemoryMap {
       count: 0,
       regions: [MemoryRegion {
          start: 0,
          pages: 0,
        }; MAX_MEMORY_REGIONS],
    };

    

    for region in memory_map.entries() {
       if region.ty == uefi::mem::memory_map::MemoryType::CONVENTIONAL {


       if region_index < MAX_MEMORY_REGIONS {
          memory_map_info.regions[region_index] = MemoryRegion {
             start: region.phys_start,
             pages: region.page_count,
           };

           region_index += 1;
        }


        if first_region.pages == 0 {
          first_region = MemoryRegion {
             start: region.phys_start,
             pages: region.page_count,
            };
        }



        total_memory += region.page_count * 4096;


        }
    }

    memory_map_info.count = region_index;

    uefi::println!(
     "Stored regions: {}",
      memory_map_info.count
    );

    uefi::println!(
      "First region: start={:#x} pages={}",
      first_region.start,
      first_region.pages
    );


 
    uefi::println!("Total memory: {} MB", total_memory / 1024 / 1024);


    let boot_info = BootInfo {
      bootloader_name: "InfinitBoot",
      version: 1,
      memory: MemoryInfo {
         total_memory,
         memory_regions,
        },
       first_region,
       memory_map: memory_map_info,
    };



    let kernel_state = kernel::kernel_main(&boot_info);

    uefi::println!(
      "Kernel memory: {} MB",
       kernel_state.total_memory / 1024 / 1024
    );

    uefi::println!(
      "Kernel stored regions: {}",
       kernel_state.stored_regions
    );


    uefi::println!(
      "Kernel regions: {}",
      kernel_state.memory_regions
    );

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}