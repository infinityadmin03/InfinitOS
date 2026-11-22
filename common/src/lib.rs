#![no_std]

pub struct MemoryRegion {
    pub start: u64,
    pub pages: u64,
}

pub struct MemoryInfo {
    pub total_memory: u64,
    pub memory_regions: u64,
}

pub struct BootInfo {
    pub bootloader_name: &'static str,
    pub version: u32,
    pub memory: MemoryInfo,
    pub first_region: MemoryRegion,
}