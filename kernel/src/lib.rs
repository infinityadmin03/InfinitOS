#![no_std]

use common::BootInfo;

pub fn kernel_main(info: &BootInfo) {
    let _bootloader = info.bootloader_name;
    let _version = info.version;
}
