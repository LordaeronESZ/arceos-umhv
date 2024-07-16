#![no_std]
#![feature(asm_const)]
#![feature(concat_idents)]
#![feature(naked_functions)]
#![feature(const_trait_impl)]
#![feature(riscv_ext_intrinsics)]
extern crate alloc;
#[macro_use]
extern crate log;

mod hal;
mod mm;
mod percpu;
// mod vcpu;
mod vm;

pub mod arch;
pub mod config;

// pub use arch::AxArchVCpu;
pub use hal::AxVMHal;
pub use mm::{AxNestedPageTable, NestedPageFaultInfo};
pub use mm::{GuestPhysAddr, GuestVirtAddr, HostPhysAddr, HostVirtAddr};
// pub use percpu::AxVMPerCpu;
// pub use vcpu::AxVCpu;
pub type AxVMPerCpu<H> = percpu::AxVMPerCpu<arch::AxVMArchPerCpuImpl<H>>;
pub use vm::AxVM;

/// Whether the hardware has virtualization support.
pub fn has_hardware_support() -> bool {
    arch::has_hardware_support()
}
