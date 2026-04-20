use core::panic;
use crate::thread_ctrl::{self, Instr, thread_state::ThreadState};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Opcode {
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
    And,
    Tid,
    Bid,
    Gdim,
    Bdim,
    LwS,
    SwS
}

fn execute_r_instr (op: Opcode, instr: riscv_decode::types::RType, thread: &mut ThreadState, _state: &mut thread_ctrl::memory_state::MemoryState) {
    let rs1 = instr.rs1();
    let rs2 = instr.rs2();
    let rs1_data = thread.read_register(rs1) as i32;
    let rs2_data = thread.read_register(rs2) as i32;
    let result;

    match op {
        Opcode::Add => {
            result = rs1_data.overflowing_add(rs2_data).0;
        },
        Opcode::Sub => {
            result = rs1_data.overflowing_sub(rs2_data).0;
        },
        Opcode::And => {
            result = rs1_data & rs2_data;
        },
        Opcode::Or => {
            result = rs1_data | rs2_data;
        }
        Opcode::Srl => {
            result = ((rs1_data as u32) >> ((rs2_data as u32) & 0x1F)) as i32
        }
        Opcode::Sra => {
            result = rs1_data >> ((rs2_data) & 0x1F);
        }
        Opcode::Sll => {
            result = rs1_data << (rs2_data & 0x1F);
        },
        Opcode::Sltu => {
            if (rs1_data as u32) < (rs2_data as u32) {
                result = 1;
            } else {
                result = 0;
            }
        },
        Opcode::Slt => {
            if (rs1_data) < (rs2_data) {
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

    thread.write_register(instr.rd(), result as u32);
    thread.advance_pc();
}


fn execute_imm_instr (op: Opcode, instr: riscv_decode::types::IType, thread: &mut ThreadState, state: &mut thread_ctrl::memory_state::MemoryState) {
    let rs1 = instr.rs1();
    let imm = ((instr.imm() as i32) << 20) >> 20;
    let rd = instr.rd();
    let rs1_data = thread.read_register(rs1) as i32; 

    let thread_idx = thread.get_thread_id();
    let block_idx  = thread.get_block_id();
    let warp_idx  = thread.get_warp_id();

    match op {
        Opcode::Lw => {
            let load_data = state.load_32(thread_idx, warp_idx, block_idx, (rs1_data+imm) as u32);
            match load_data {
                Some(data) => {
                    thread.write_register(rd, data as u32);
                    thread.advance_pc();
                }
                None => {
                    thread.set_waiting_for_mem(true);
                }
            } 
        }
        Opcode::Lh => {
            let load_data = state.load_16(thread_idx, warp_idx, block_idx, (rs1_data+imm) as u32);
            match load_data {
                Some(data) => {
                    let new_data = ((data as i32) << 16) >> 16;
                    thread.write_register(rd, new_data as u32);
                    thread.advance_pc();
                }
                None => {}
            }
        }
        Opcode::Lhu => {
            let load_data = state.load_16(thread_idx, warp_idx, block_idx, (rs1_data+imm) as u32);
            match load_data {
                Some(data) => {
                    thread.write_register(rd, data as u32);
                    thread.advance_pc();
                }
                None => {}
            } 
        }
        Opcode::Lb => {
            let load_data = state.load_8(thread_idx, warp_idx, block_idx, (rs1_data+imm) as u32);
            match load_data {
                Some(data) => {
                    let new_data = ((data as i32) << 24) >> 24;
                    thread.write_register(rd, new_data as u32);
                    thread.advance_pc();
                }
                None => {}
            }
        }
        Opcode::Lbu => {
            let load_data = state.load_8(thread_idx, warp_idx, block_idx, (rs1_data+imm) as u32);
            match load_data {
                Some(data) => {
                    thread.write_register(rd, data as u32);
                    thread.advance_pc();
                }
                None => {}
            } 
        }
        Opcode::Addi => {
            let result = rs1_data.overflowing_add(imm).0;
            thread.write_register(rd, result as u32);
            thread.advance_pc();
        }
        Opcode::Slti => {
            let result;
            if (rs1_data as i32) < (imm as i32) {
                result = 1;
            }
            else {
                result = 0;
            }
            thread.write_register(rd, result as u32);
            thread.advance_pc();
        }
        Opcode::Sltiu => {
            let result;
            if (rs1_data as u32) < (imm as u32) {
                result = 1;
            }
            else {
                result = 0;
            }
            thread.write_register(rd, result as u32);
            thread.advance_pc();
        }
        Opcode::Xori => {
            let result = rs1_data ^ imm;
            thread.write_register(rd, result as u32);
            thread.advance_pc();
        }
        Opcode::Andi => {
            let result = rs1_data & imm;
            thread.write_register(rd, result as u32);
            thread.advance_pc();
        }
        Opcode::Ori => {
            let result = rs1_data | imm;
            thread.write_register(rd, result as u32);
            thread.advance_pc();
        }
        
        Opcode::Jalr => {
            let target_addr: u32 = rs1_data.overflowing_add(imm).0 as u32;
            let result = (thread.get_pc() as i32).overflowing_add(4).0;
            thread.write_register(rd, result as u32);
            thread.set_pc(target_addr);
        }
        _ => {
            panic!("Illegal I-Type Instruction!");
        }
    }
}

fn execute_sb_instr (op: Opcode, instr: riscv_decode::types::BType, thread: &mut ThreadState, _state: &mut thread_ctrl::memory_state::MemoryState) {
    let rs1 = instr.rs1();
    let imm = ((instr.imm() as i32) << 20) >> 20;
    let rs2 = instr.rs2();
    let rs1_data = thread.read_register(rs1) as i32; 
    let rs2_data = thread.read_register(rs2) as i32; 
    let target_addr = (imm.overflowing_add(thread.get_pc() as i32)).0 as u32;

    match op {
        Opcode::Beq => {
            if rs1_data == rs2_data {
                thread.set_pc(target_addr);
            } else {
                thread.advance_pc();
            }
        },
        Opcode::Bne => {
            if rs1_data != rs2_data {
                thread.set_pc(target_addr);
            } else {
                thread.advance_pc();
            }
        },
        Opcode::Blt => {
            if rs1_data < rs2_data {
                thread.set_pc(target_addr);
            } else {
                thread.advance_pc();
            }
        },
        Opcode::Bltu => {
            if (rs1_data as u32) < (rs2_data as u32) {
                thread.set_pc(target_addr);
            } else {
                thread.advance_pc();
            }
        },
        Opcode::Bge => {
            if rs1_data >= rs2_data {
                thread.set_pc( target_addr);
            } else {
                thread.advance_pc();
            }
        },
        Opcode::Bgeu => {
            if (rs1_data as u32) >= (rs2_data as u32) {
                thread.set_pc( target_addr);
            } else {
                thread.advance_pc();
            }
        },
        _ => {
            panic!("Illegal SB_type instruction!")
        }
    }
}

fn execute_s_instr (op: Opcode, instr: riscv_decode::types::SType, thread: &mut ThreadState, state: &mut thread_ctrl::memory_state::MemoryState) {
    let rs1 = instr.rs1();
    let rs2 = instr.rs2();
    let imm = ((instr.imm() as i32) << 20) >> 20;

    let rs1_data = thread.read_register(rs1) as i32;
    let rs2_data = thread.read_register(rs2) as i32; 
    let thread_idx = thread.get_thread_id();
    let block_idx = thread.get_block_id();
    let warp_idx  = thread.get_warp_id();
    let success;
    match op {
        Opcode::Sw => {
            let store_addr = rs1_data.overflowing_add(imm).0;
            success = state.store_32(thread_idx, warp_idx, block_idx, store_addr as u32, rs2_data as u32);
        },
        Opcode::Sh => {
            let store_addr = rs1_data.overflowing_add(imm).0;
            success = state.store_16(thread_idx, warp_idx, block_idx, store_addr as u32, rs2_data as u16);
        },
        Opcode::Sb => {
            let store_addr = rs1_data.overflowing_add(imm).0;
            success = state.store_8(thread_idx, warp_idx, block_idx, store_addr as u32, rs2_data as u8);
        },
        _ => {
            panic!("Illegal I-Type Instruction!");
        }
    }
    if success {
        thread.advance_pc();
    }
}

fn execute_u_instr (op: Opcode, instr: riscv_decode::types::UType, thread: &mut ThreadState) {
    let rd = instr.rd();
    let imm = instr.imm();

    match op {
        Opcode::Lui => {
            thread.write_register(rd, imm);
        },
        Opcode::Auipc => {
            let final_imm = imm.overflowing_add(thread.get_pc()).0;
            thread.write_register(rd, final_imm); 
        }
        _ => {
            panic!("Illegal U Type Instruction!");
        }
    }
    thread.advance_pc();
}

fn execute_shift_instr (op: Opcode, instr: riscv_decode::types::ShiftType, thread: &mut ThreadState) {
    let rd = instr.rd();
    let rs1 = instr.rs1();
    let imm = instr.shamt() & 0x1F;
    
    let rs1_data = thread.read_register(rs1);

    match op {
        Opcode::Srli => {
            let result = rs1_data as u32 >> ((imm as u32) & 0x1F);
            thread.write_register(rd, result);
            thread.advance_pc();
        }
        Opcode::Srai => {
            let result = (rs1_data as i32) >> ((imm as i32) & 0x1F);
            thread.write_register(rd, result as u32);
            thread.advance_pc();
        }
         Opcode::Slli => {
            let result = (rs1_data as u32) << ((imm as u32) & 0x1F);
            thread.write_register(rd, result as u32);
            thread.advance_pc();
        }
        _ => {
            panic!("Illegal U Type Instruction!");
        }
    }
}

fn execute_uj_instr (op: Opcode, instr: riscv_decode::types::JType, thread: &mut ThreadState) {
    let rd = instr.rd();
    let curr_pc = thread.get_pc();
    let imm = ((instr.imm() as i32) << 12) >> 12;
    let target_addr = (imm.overflowing_add(curr_pc as i32)).0 as u32;
    let next_pc = (curr_pc as i32).overflowing_add(4).0 as u32;
    match op {
        Opcode::Jal => {
            thread.set_pc(target_addr);
            thread.write_register(rd, next_pc);
        },
        _ => {
            panic!("Illegal UJ Type Instruction!");
        }
    }
}

fn execute_custom_instr (instr: Instr, thread: &mut ThreadState, state: &mut thread_ctrl::memory_state::MemoryState) {
    match instr {
        Instr::Custom {op, rd, ..} => {
            match op {
                Opcode::Tid => {
                    let idx = thread.get_thread_id();
                    thread.write_register(rd, idx);
                }
                Opcode::Bid => {
                    let idx = thread.get_thread_id();
                    thread.write_register(rd, idx);
                }
                Opcode::Bdim => {
                    thread.write_register(rd, state.get_threads_per_block());
                }
                Opcode::Gdim => {
                    thread.write_register(rd, state.get_num_blocks());
                }
                _ => {
                    panic!("unimplemented custom instr");
                }
            }
        }
        _ => {
            panic!("you idiot");
        }
    }
    thread.advance_pc();
}

pub fn execute_instr (curr_thread: &mut ThreadState, mem_state: &mut thread_ctrl::memory_state::MemoryState) {
    let raw_instr = mem_state.fetch_instr(curr_thread.get_pc());
    let target_instr; 
    match raw_instr {
        Some(i) => target_instr = i,
        None => panic!("Tried to execute an illegal instruction")
    }
    match target_instr {
        Instr::Standard(op) => {
            match op {
                riscv_decode::Instruction::Add(add) => {
                    execute_r_instr(Opcode::Add, add, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Sub(sub) => {
                    execute_r_instr(Opcode::Sub, sub, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Sll(sll) => {
                    execute_r_instr(Opcode::Sll, sll, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Sltu(sltu) => {
                    execute_r_instr(Opcode::Sltu, sltu, curr_thread, mem_state);
                }
                riscv_decode::Instruction::And(and) => {
                    execute_r_instr(Opcode::And, and, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Or(or) => {
                    execute_r_instr(Opcode::Or, or, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Srl(srl) => {
                    execute_r_instr(Opcode::Srl, srl, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Sra(sra) => {
                    execute_r_instr(Opcode::Sra, sra, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Slt(slt) => {
                    execute_r_instr(Opcode::Slt, slt, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Xor(xor) => {
                    execute_r_instr(Opcode::Xor, xor, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Lw(lw) => {
                    execute_imm_instr(Opcode::Lw, lw, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Lh(lh) => {
                    execute_imm_instr(Opcode::Lh, lh, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Lhu(lhu) => {
                    execute_imm_instr(Opcode::Lhu, lhu, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Lb(lb) => {
                    execute_imm_instr(Opcode::Lb, lb, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Lbu(lbu) => {
                    execute_imm_instr(Opcode::Lbu, lbu,curr_thread, mem_state);
                }
                riscv_decode::Instruction::Sw(sw) => {
                    execute_s_instr(Opcode::Sw, sw, curr_thread, mem_state);
                },
                riscv_decode::Instruction::Ecall => {
                    curr_thread.halt();
                }
                riscv_decode::Instruction::Addi(addi) => {
                    execute_imm_instr(Opcode::Addi, addi, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Ori(ori) => {
                    execute_imm_instr(Opcode::Ori, ori, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Xori(xori) => {
                    execute_imm_instr(Opcode::Xori, xori, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Andi(andi) => {
                    execute_imm_instr(Opcode::Andi, andi, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Slti(slti) => {
                    execute_imm_instr(Opcode::Slti, slti, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Sltiu(sltiu) => {
                    execute_imm_instr(Opcode::Sltiu, sltiu, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Slli(slli) => {
                    execute_shift_instr(Opcode::Slli, slli, curr_thread);
                }
                riscv_decode::Instruction::Srli(srli) => {
                    execute_shift_instr(Opcode::Srli, srli, curr_thread);
                }
                riscv_decode::Instruction::Srai(srai) => {
                    execute_shift_instr(Opcode::Srai, srai, curr_thread);
                }
                riscv_decode::Instruction::Lui(lui) => {
                    execute_u_instr(Opcode::Lui, lui, curr_thread);
                }
                riscv_decode::Instruction::Sh(sh) => {
                    execute_s_instr(Opcode::Sh, sh, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Sb(sb) => {
                    execute_s_instr(Opcode::Sb, sb, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Auipc(auipc) => {
                    execute_u_instr(Opcode::Auipc, auipc, curr_thread);
                }
                riscv_decode::Instruction::Bne(bne) => {
                    execute_sb_instr(Opcode::Bne, bne, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Beq(beq) => {
                    execute_sb_instr(Opcode::Beq, beq, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Blt(blt) => {
                    execute_sb_instr(Opcode::Blt, blt, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Bltu(bltu) => {
                    execute_sb_instr(Opcode::Bltu, bltu, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Bge(bge) => {
                    execute_sb_instr(Opcode::Bge, bge, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Bgeu(bgeu) => {
                    execute_sb_instr(Opcode::Bgeu, bgeu, curr_thread, mem_state);
                }
                riscv_decode::Instruction::Jal(jal) => {
                    execute_uj_instr(Opcode::Jal, jal, curr_thread);
                }
                riscv_decode::Instruction::Jalr(jalr) => {
                    execute_imm_instr(Opcode::Jalr, jalr, curr_thread, mem_state);
                }
                _ => {
                    panic!("Unimplemented Instruction {:?}!", target_instr);
                }
            }
        }
        Instr::Custom{op, ..} => {
            match op {
                Opcode::Tid => {
                    execute_custom_instr(target_instr.clone(), curr_thread, mem_state);
                }
                Opcode::Bid => {
                    execute_custom_instr(target_instr.clone(), curr_thread, mem_state);
                }
                Opcode::Gdim => {
                    execute_custom_instr(target_instr.clone(), curr_thread, mem_state);
                }
                Opcode::Bdim => {
                    execute_custom_instr(target_instr.clone(), curr_thread, mem_state);
                }
                _ => {
                    panic!("unimplemented custom instruction")
                }
            }
        }
    }
}