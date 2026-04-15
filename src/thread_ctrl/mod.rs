pub mod system_state;
pub mod thread_state; 
pub mod mem_request;

use crate::{instr_execute::Opcode};


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