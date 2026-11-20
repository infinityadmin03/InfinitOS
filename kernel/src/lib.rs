#![no_std]

use common::BootInfo;

pub fn kernel_main(info: &BootInfo) {
    let _bootloader = info.bootloader_name;
    let _protocol_version = info.boot_protocol_version;
    let _total_memory = info.memory.total_memory;
}