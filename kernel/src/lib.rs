#![no_std]

pub struct KernelInfo {
    pub name: &'static str,
    pub version: &'static str,
}

pub fn kernel_main() -> KernelInfo {
    KernelInfo {
        name: "InfinitOS",
        version: "0.1",
    }
}