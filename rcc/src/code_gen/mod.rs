pub mod riscv32;
pub(crate) mod simple_allocator;

use strenum::StrEnum;
use crate::ir::cfg::CFG;
use crate::rcc::OptimizeLevel;
use crate::code_gen::simple_allocator::SimpleAllocator;
use crate::ir::IRType;

#[derive(StrEnum)]
pub enum TargetPlatform {
    Riscv32
}

pub trait Allocator {
    fn get_frame_size(&self) -> u32;

    /// Return offset from fp
    fn get_fp_offset(&mut self, var_name: &str, ir_type: &IRType) -> u32;
}

pub fn create_allocator<'cfg>(opt_level: OptimizeLevel, cfg: &'cfg CFG, addr_size: u32) -> Box<dyn Allocator + 'cfg>  {
    match opt_level {
        OptimizeLevel::Zero => Box::new(SimpleAllocator::new(cfg, addr_size)),
        OptimizeLevel::One => todo!()
    }
}