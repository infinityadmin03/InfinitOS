#![no_std]

pub struct BootInfo {
    pub bootloader_name: &'static str,
    pub version: u32,
}