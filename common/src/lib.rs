#![no_std]

pub struct MemoryInfo {
    pub total_memory: u64,
}

pub struct BootInfo {
    pub boot_protocol_version: u32,
    pub bootloader_name: &'static str,
    pub memory: MemoryInfo,
}