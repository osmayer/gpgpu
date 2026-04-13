use core::panic;
use std::thread::Thread;

use object::elf::R_NIOS2_TLS_DTPMOD;

use crate::program_state;
enum Opcode {
    Lui,
    Auipc,
    Jal,
    Jalr,
    Beq,
    Bne,
    Blt, 
    Bge,
    Bltu,
    Bgeu,
    Lb,
    Lh,
    Lw,
    Lbu,
    Lhu,
    Lwu,
    Sb,
    Sh,
    Sw,
    Addi,
    Slti, 
    Sltiu,
    Xori,
    Ori, 
    Andi,
    Slli,
    Srli,
    Srai, 
    Add,
    Sub,
    Sll,
    Slt,
    Sltu, 
    Xor,
    Srl,
    Sra,
    Or, 
    And
}

fn execute_r_instr (op: Opcode, instr: riscv_decode::types::RType, thread_idx: u32, curr_pc: u32, state: &mut program_state::SystemState) {
    let rs1 = instr.rs1();
    let rs2 = instr.rs2();
    let rs1_data = state.read_thread_register(thread_idx, rs1);
    let rs2_data = state.read_thread_register(thread_idx, rs2);
    let result;

    match op {
        Opcode::Add => {
            result = rs1_data + rs2_data;
        },
        Opcode::Sub => {
            result = rs1_data - rs2_data;
        },
        Opcode::Sll => {
            result = rs1_data << (rs2_data & 0x1F);
        },
        Opcode::Sltu => {
            if rs1_data < rs2_data {
                result = 1;
            } else {
                result = 0;
            }
        },
        Opcode::Xor => {
            result = rs1_data ^ rs2_data;
        },
        _ => {
            panic!("Invalidate R-Type Instruction");
        }
    }

    state.write_thread_register(thread_idx, instr.rd(), result);
    state.incr_pc(thread_idx);
}


fn execute_imm_instr (op: Opcode, instr: riscv_decode::types::IType, thread_idx: u32, curr_pc: u32, state: &mut program_state::SystemState) {
    let rs1 = instr.rs1();
    let imm = ((instr.imm() as i32) << 20) >> 20;
    let rd = instr.rd();

    let rs1_data = state.read_thread_register(thread_idx, rs1) as i32; 

    match op {
        Opcode::Lw => {
            let load_data = state.load(thread_idx, (rs1_data+imm) as u32);
            state.write_thread_register(thread_idx, rd, load_data);
        },
        Opcode::Addi => {
            let result = rs1_data + imm;
            println!("Sum: {}, Imm: {}", result, imm);
            state.write_thread_register(thread_idx, rd, result as u32);
        }
        Opcode::Slti => {
            let result;
            if ((rs1_data as i32) < (imm as i32)) {
                result = 1;
            }
            else {
                result = 0;
            }
            state.write_thread_register(thread_idx, rd, result);
        }
        Opcode::Sltiu => {
            let result;
            if (rs1_data < imm) {
                result = 1;
            }
            else {
                result = 0;
            }
            state.write_thread_register(thread_idx, rd, result);
        }
        Opcode::Xori => {
            let result = rs1_data ^ imm;
            state.write_thread_register(thread_idx, rd, result as u32);
        }
        _ => {
            panic!("Illegal I-Type Instruction!");
        }
    }

    state.incr_pc(thread_idx);
}

fn execute_s_instr (op: Opcode, instr: riscv_decode::types::SType, thread_idx: u32, curr_pc: u32, state: &mut program_state::SystemState) {
    let rs1 = instr.rs1();
    let rs2 = instr.rs2();
    let imm = instr.imm();

    let rs1_data = state.read_thread_register(thread_idx, rs1);
    let rs2_data = state.read_thread_register(thread_idx, rs2); 

    match op {
        Opcode::Sw => {
            let store_addr = rs1_data + imm;
            state.store(thread_idx, store_addr, rs2_data);
        },
        _ => {
            panic!("Illegal I-Type Instruction!");
        }
    }
    state.incr_pc(thread_idx);
}

pub fn execute_instr (target_instr: riscv_decode::Instruction, curr_pc: u32, thread_idx: u32,  state: &mut program_state::SystemState) {
    match target_instr {
        riscv_decode::Instruction::Add(add) => {
            execute_r_instr(Opcode::Add, add, thread_idx, curr_pc, state);
        },
        riscv_decode::Instruction::Lw(ld) => {
            execute_imm_instr(Opcode::Lw, ld, thread_idx, curr_pc, state);
        },
        riscv_decode::Instruction::Sw(sw) => {
            execute_s_instr(Opcode::Sw, sw, thread_idx, curr_pc, state);
        },
        riscv_decode::Instruction::Ecall => {
            state.halt_thread(thread_idx);
        },
        riscv_decode::Instruction::Addi(addi) => {
            execute_imm_instr(Opcode::Addi, addi, thread_idx, curr_pc, state);
        },
        _ => {
            panic!("Unimplemented Instruction!");
        }
    }
}