use core::{fmt, num};
use std::thread::Thread;
use riscv_decode;
use crate::program_loader::{self, Segment, SegmentMetadata};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThreadState {
    pc:         i32,
    registers:  [i32; 32],
    halted:     bool
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SectionData {
    Data (Vec<u8>),
    Instruction (Vec<riscv_decode::Instruction>)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct  SectionImage {
    metadata: SegmentMetadata,
    data:     SectionData
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemState {
    thread_states: Vec<ThreadState>,
    memory_state:  Vec<SectionImage>
}

impl ThreadState {
    pub fn new(starting_pc: i32) -> Self {
        ThreadState { 
            pc: starting_pc, 
            registers: [0; 32],
            halted: false
        }
    }

    pub fn get_pc(&self) -> i32 {
        self.pc
    }

    pub fn get_instr_idx(&self) -> i32 {
        self.pc / 4
    }

    pub fn set_pc(&mut self, new_pc: i32) {
        self.pc = new_pc;
    }

    pub fn advance_pc (&mut self) {
        self.pc += 4; 
    }

    pub fn read_register(&self, idx: i32) -> i32 {
        if idx < 0  || idx > 31 {
            assert!(false, "Tried to read invalid index");
        }
        self.registers[idx as usize]
    }

    pub fn write_register(&mut self, idx: i32, new_val: i32) {
        if idx < 0  || idx > 31 {
            assert!(false, "Tried to read invalid index");
        }
        
        if idx != 0 {
            self.registers[idx as usize] = new_val;
        }
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

}

impl fmt::Display for ThreadState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use the write! macro to define the string representation
        writeln!(f, "PC: {}", self.get_pc())?;
        for i in 0..32 {
            writeln!(f, "x{}: {}", i, self.read_register(i))?;
        }
        writeln!(f, "Halted: {}", self.is_halted())
    }

}

impl SystemState {
    pub fn new(program_image: &Vec<Segment>, num_threads: u32) -> Self {
        let mut new_state = SystemState {
            thread_states: vec![],
            memory_state:  vec![]
        };
        
        // Make a system state for all threads
        for _i in 0..num_threads {
            new_state.thread_states.push(ThreadState::new(4194304));
        }

        // parse memory 
        for section in program_image {
            
            if section.metadata.executable {
                let mut curr_loc = 4194304;
                let mut curr_instructions = vec![];
                loop {
                    let curr_instr = program_loader::get_u32(program_image, curr_loc);
                    match curr_instr {
                        Some(u) => {
                            curr_instructions.push(riscv_decode::decode(u).ok().expect("Found illegal instruction"));
                            curr_loc += 4;
                        },
                        _ => {
                            break;
                        }
                    }
                }

                new_state.memory_state.push(
                    SectionImage { 
                        metadata: section.metadata.clone(), 
                        data: SectionData::Instruction(curr_instructions)
                    }
                );
            } else {
                new_state.memory_state.push(
                    SectionImage { 
                        metadata: section.metadata.clone(), 
                        data: SectionData::Data(section.values.clone())
                    }
                );
            }
        }

        new_state
    }
}
