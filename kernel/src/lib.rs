#![no_std]

use common::BootInfo;

pub struct KernelState {
    pub total_memory: u64,
    pub memory_regions: u64,
}

pub fn kernel_main(info: &BootInfo) -> KernelState {
    KernelState {
        total_memory: info.memory.total_memory,
        memory_regions: info.memory.memory_regions,
    }
}