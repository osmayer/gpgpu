pub mod system_state;
pub mod thread_state; 
pub mod mem_request;
pub mod block_state;
pub mod warp_state;
pub mod memory_state;

use crate::{instr_execute::Opcode, program_loader::{self, Segment, SegmentMetadata}};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instr {
    Custom {
        op: Opcode,
        rd: u32, 
        rs1: u32, 
        rs2: u32
    },
    Standard (riscv_decode::Instruction)
}