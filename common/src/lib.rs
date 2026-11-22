#![no_std]

#[derive(Clone, Copy)]
pub struct MemoryRegion {
    pub start: u64,
    pub pages: u64,
}


#[derive(Clone, Copy)]
pub struct MemoryMap {
    pub count: usize,
    pub regions: [MemoryRegion; MAX_MEMORY_REGIONS],
}

pub const MAX_MEMORY_REGIONS: usize = 128;

#[derive(Clone, Copy)]
pub struct MemoryInfo {
    pub total_memory: u64,
    pub memory_regions: u64,
}

pub struct BootInfo {
    pub bootloader_name: &'static str,
    pub version: u32,
    pub memory: MemoryInfo,
    pub first_region: MemoryRegion,
    pub memory_map: MemoryMap,
}