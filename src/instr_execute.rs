use std::thread::Thread;

use crate::{instr_parser::{GTypeOpcode, ITypeOpcode, Instruction, RTypeOpcode}, program_state::ThreadState};

pub fn execute_instr (target_instr: &Instruction, program_state: &mut ThreadState) {
    match target_instr {
        Instruction::RType { opcode, rs1, rs2, rd } => {
            let rs1_data = program_state.read_register(*rs1);
            let rs2_data = program_state.read_register(*rs2);
            match opcode {
                RTypeOpcode::Add => {
                    program_state.write_register(*rd, rs1_data + rs2_data);
                    program_state.advance_pc();
                }
                _ => {

                }
            }
        },
        Instruction::IType { opcode, rs1, imm, rd } => {
            let rs1_data = program_state.read_register(*rs1);
            match opcode {
                ITypeOpcode::Addi => {
                    program_state.write_register(*rd, rs1_data + imm);
                    program_state.advance_pc();
                },
                _ => {

                }
            }
        },
        Instruction::GType { opcode } => {
            match opcode {
                GTypeOpcode::Halt => {
                    program_state.halt();
                }
            }
        }
        _ => {

        }
    }
}