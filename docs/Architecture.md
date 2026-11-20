# InfinitOS

## Platform

- UEFI
- x86_64
- GPT

## Language

- Rust

## Build System

- Cargo
- LLVM

## Test Platform

- QEMU

## Components

### InfinitBoot

Responsibilities:

- Load kernel
- Obtain memory map
- Setup framebuffer
- Transfer control

### InfinitKernel

Responsibilities:

- Memory management
- Task scheduling
- Driver framework