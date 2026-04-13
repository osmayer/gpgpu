use core::panic;
use std::thread::Thread;

use object::elf::R_NIOS2_TLS_DTPMOD;
use riscv_decode::types::ShiftType;

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
    let rs1_data = state.read_thread_register(thread_idx, rs1) as i32;
    let rs2_data = state.read_thread_register(thread_idx, rs2) as i32;
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

    state.write_thread_register(thread_idx, instr.rd(), result as u32);
    state.incr_pc(thread_idx);
}


fn execute_imm_instr (op: Opcode, instr: riscv_decode::types::IType, thread_idx: u32, curr_pc: u32, state: &mut program_state::SystemState) {
    let rs1 = instr.rs1();
    let imm = ((instr.imm() as i32) << 20) >> 20;
    let rd = instr.rd();

    let rs1_data = state.read_thread_register(thread_idx, rs1) as i32; 

    match op {
        Opcode::Lw => {
            let load_data = state.load_32(thread_idx, (rs1_data+imm) as u32);
            state.write_thread_register(thread_idx, rd, load_data);
        }
        Opcode::Lh => {
            let load_data = (((state.load_16(thread_idx, (rs1_data+imm) as u32) as i32) << 16) >> 16) as u32;
            state.write_thread_register(thread_idx, rd, load_data);
        }
        Opcode::Lhu => {
            let load_data = state.load_16(thread_idx, (rs1_data+imm) as u32) as u32;
            state.write_thread_register(thread_idx, rd, load_data);
        }
        Opcode::Lb => {
            let load_data = (((state.load_8(thread_idx, (rs1_data+imm) as u32) as i32) << 24) >> 24) as u32;
            state.write_thread_register(thread_idx, rd, load_data);
        }
        Opcode::Lbu => {
            let load_data = state.load_8(thread_idx, (rs1_data+imm) as u32) as u32;
            state.write_thread_register(thread_idx, rd, load_data);
        }
        Opcode::Addi => {
            let result = rs1_data + imm;
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

fn execute_sb_instr (op: Opcode, instr: riscv_decode::types::BType, thread_idx: u32, curr_pc: u32, state: &mut program_state::SystemState) {
    let rs1 = instr.rs1();
    let imm = ((instr.imm() as i32) << 20) >> 20;
    let rs2 = instr.rs2();
    let rs1_data = state.read_thread_register(thread_idx, rs1) as i32; 
    let rs2_data = state.read_thread_register(thread_idx, rs2) as i32; 
    let target_addr = (imm.overflowing_add(curr_pc as i32)).0 as u32;

    match op {
        Opcode::Beq => {
            if rs1_data == rs2_data {
                state.update_pc(thread_idx, target_addr);
            } else {
                state.incr_pc(thread_idx);
            }
        },
        Opcode::Bne => {
            if rs1_data != rs2_data {
                state.update_pc(thread_idx, target_addr);
            } else {
                state.incr_pc(thread_idx);
            }
        },
        Opcode::Blt => {
            if rs1_data < rs2_data {
                state.update_pc(thread_idx, target_addr);
            } else {
                state.incr_pc(thread_idx);
            }
        },
        Opcode::Bltu => {
            if (rs1_data as u32) < (rs2_data as u32) {
                state.update_pc(thread_idx, target_addr);
            } else {
                state.incr_pc(thread_idx);
            }
        },
        Opcode::Bge => {
            if rs1_data > rs2_data {
                state.update_pc(thread_idx, target_addr);
            } else {
                state.incr_pc(thread_idx);
            }
        },
        Opcode::Bgeu => {
            if (rs1_data as u32) > (rs2_data as u32) {
                state.update_pc(thread_idx, target_addr);
            } else {
                state.incr_pc(thread_idx);
            }
        },
        _ => {
            panic!("Illegal SB_type instruction!")
        }
    }
}

fn execute_s_instr (op: Opcode, instr: riscv_decode::types::SType, thread_idx: u32, curr_pc: u32, state: &mut program_state::SystemState) {
    let rs1 = instr.rs1();
    let rs2 = instr.rs2();
    let imm = ((instr.imm() as i32) << 20) >> 20;

    let rs1_data = state.read_thread_register(thread_idx, rs1) as i32;
    let rs2_data = state.read_thread_register(thread_idx, rs2) as i32; 

    match op {
        Opcode::Sw => {
            let store_addr = rs1_data + imm;
            state.store_32(thread_idx, store_addr as u32, rs2_data as u32);
        },
        Opcode::Sh => {
            let store_addr = rs1_data + imm;
            state.store_16(thread_idx, store_addr as u32, rs2_data as u16);
        },
        Opcode::Sb => {
            let store_addr = rs1_data + imm;
            state.store_8(thread_idx, store_addr as u32, rs2_data as u8);
        },
        _ => {
            panic!("Illegal I-Type Instruction!");
        }
    }
    state.incr_pc(thread_idx);
}

fn execute_u_instr (op: Opcode, instr: riscv_decode::types::UType, thread_idx: u32, curr_pc: u32, state: &mut program_state::SystemState) {
    let rd = instr.rd();
    let imm = instr.imm();
    println!("{}", imm as i32);

    match op {
        Opcode::Lui => {
            state.write_thread_register(thread_idx, rd, imm);
        },
        Opcode::Auipc => {
            let final_imm = imm.overflowing_add(curr_pc).0;
            state.write_thread_register(thread_idx, rd, final_imm); 
        }
        _ => {
            panic!("Illegal U Type Instruction!");
        }
    }
    state.incr_pc(thread_idx);
}

pub fn execute_instr (target_instr: riscv_decode::Instruction, curr_pc: u32, thread_idx: u32,  state: &mut program_state::SystemState) {
    match target_instr {
        riscv_decode::Instruction::Add(add) => {
            execute_r_instr(Opcode::Add, add, thread_idx, curr_pc, state);
        }
        riscv_decode::Instruction::Lw(ld) => {
            execute_imm_instr(Opcode::Lw, ld, thread_idx, curr_pc, state);
        }
        riscv_decode::Instruction::Lh(lh) => {
            execute_imm_instr(Opcode::Lh, lh, thread_idx, curr_pc, state);
        }
        riscv_decode::Instruction::Lhu(lhu) => {
            execute_imm_instr(Opcode::Lhu, lhu, thread_idx, curr_pc, state);
        }
        riscv_decode::Instruction::Lb(lb) => {
            execute_imm_instr(Opcode::Lb, lb, thread_idx, curr_pc, state);
        }
        riscv_decode::Instruction::Lbu(lbu) => {
            execute_imm_instr(Opcode::Lbu, lbu, thread_idx, curr_pc, state);
        }
        riscv_decode::Instruction::Sw(sw) => {
            execute_s_instr(Opcode::Sw, sw, thread_idx, curr_pc, state);
        },
        riscv_decode::Instruction::Ecall => {
            state.halt_thread(thread_idx);
        },
        riscv_decode::Instruction::Addi(addi) => {
            execute_imm_instr(Opcode::Addi, addi, thread_idx, curr_pc, state);
        },
        riscv_decode::Instruction::Lui(lui) => {
            execute_u_instr(Opcode::Lui, lui, thread_idx, curr_pc, state);
        },
        riscv_decode::Instruction::Sh(sh) => {
            execute_s_instr(Opcode::Sh, sh, thread_idx, curr_pc, state);
        },
        riscv_decode::Instruction::Sb(sb) => {
            execute_s_instr(Opcode::Sb, sb, thread_idx, curr_pc, state);
        },
        riscv_decode::Instruction::Auipc(auipc) => {
            execute_u_instr(Opcode::Auipc, auipc, thread_idx, curr_pc, state);
        },
        riscv_decode::Instruction::Bne(bne) => {
            execute_sb_instr(Opcode::Bne, bne, thread_idx, curr_pc, state);
        },
        riscv_decode::Instruction::Beq(beq) => {
            execute_sb_instr(Opcode::Beq, beq, thread_idx, curr_pc, state);
        },
        riscv_decode::Instruction::Blt(blt) => {
            execute_sb_instr(Opcode::Blt, blt, thread_idx, curr_pc, state);
        },
        riscv_decode::Instruction::Bltu(bltu) => {
            execute_sb_instr(Opcode::Bltu, bltu, thread_idx, curr_pc, state);
        },
        riscv_decode::Instruction::Bge(bge) => {
            execute_sb_instr(Opcode::Bge, bge, thread_idx, curr_pc, state);
        },
        riscv_decode::Instruction::Bgeu(bgeu) => {
            execute_sb_instr(Opcode::Bgeu, bgeu, thread_idx, curr_pc, state);
        },
        _ => {
            panic!("Unimplemented Instruction {:?}!", target_instr);
        }
    }
}