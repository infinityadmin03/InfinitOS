#![no_std]

use common::BootInfo;

pub struct KernelState {
    pub total_memory: u64,
    pub memory_regions: u64,
    pub first_region_start: u64,
    pub first_region_pages: u64,
    pub stored_regions: usize,
}

pub fn kernel_main(info: &BootInfo) -> KernelState {
    KernelState {
        total_memory: info.memory.total_memory,
        memory_regions: info.memory.memory_regions,
        first_region_start: info.first_region.start,
        first_region_pages: info.first_region.pages,
        stored_regions: info.memory_map.count,
    }
}