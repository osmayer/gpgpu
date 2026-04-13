use core::{fmt, num, panic};
use std::{os::unix::thread, thread::Thread};
use byteorder::{ByteOrder, LittleEndian};
use riscv_decode;
use crate::program_loader::{self, Segment, SegmentMetadata};

const INITIAL_SP: u32 = 0x7ff00000;
const INITIAL_GP: u32 = 0x10000000; 

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThreadState {
    pc:         u32,
    registers:  [u32; 32],
    halted:     bool
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SectionData {
    Data (Vec<u8>),
    Instruction (Vec<riscv_decode::Instruction>)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct  SectionImage {
    metadata: SegmentMetadata,
    data:     SectionData
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemState {
    pub thread_states: Vec<ThreadState>,
    memory_state:  Vec<SectionImage>
}

impl ThreadState {
    pub fn new(starting_pc: u32) -> Self {
        let mut new_state= ThreadState { 
            pc: starting_pc, 
            registers: [0; 32],
            halted: false
        };
        new_state.registers[2] = INITIAL_SP;
        new_state.registers[3] = INITIAL_GP;
        new_state
    }

    pub fn get_pc(&self) -> u32 {
        self.pc
    }

    pub fn set_pc(&mut self, new_pc: u32) {
        self.pc = new_pc;
    }

    pub fn advance_pc (&mut self) {
        self.pc += 4; 
    }

    pub fn read_register(&self, idx: u32) -> u32 {
        if idx < 0  || idx > 31 {
            assert!(false, "Tried to read invalid index");
        }
        self.registers[idx as usize]
    }

    pub fn write_register(&mut self, idx: u32, new_val: u32) {
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
        writeln!(f, "PC: {:#x}", self.get_pc())?;
        for i in 0..32 {
            writeln!(f, "x{:<2} \t {:#010X} ({})", i, self.read_register(i), self.read_register(i) as i32)?;
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

    fn get_effective_addr (&self, req_addr: u32, req_size: usize) -> (Option<usize>, Option<usize>) {
        let mut seg_id: usize = 0; 
        for segment in &self.memory_state {
            let segment_start = segment.metadata.base_addr as usize;
            let segment_end   = (segment.metadata.base_addr + segment.metadata.size) as usize;
            let req_start_addr= req_addr as usize;
            let req_end_addr  = (req_start_addr + 4) as usize;

            if req_start_addr >= segment_start && req_end_addr <= segment_end {
                let effective_start = req_start_addr - segment_start;
                let effective_end   = effective_start + req_size; 
            
                if effective_end as u32 > segment.metadata.allocated_size {
                    return (None, None);
                }

                return (Some(seg_id), Some(effective_start));
            }
            seg_id+=1;
        }
        (None, None)
    }

    pub fn fetch_instr (&self, thread_idx: u32) -> (u32, Option<riscv_decode::Instruction>) {
        let curr_pc                            = self.thread_states[thread_idx as usize].get_pc();
        let mem_loc = self.get_effective_addr(curr_pc, 1);
        let seg_idx                  = mem_loc.0;
        let byte_idx                 = mem_loc.1;

        match (seg_idx, byte_idx) {
            (Some(seg_idx), Some(byte_idx)) => {
                match &self.memory_state[seg_idx].data {
                    SectionData::Instruction(i) => {
                        let pc_idx = byte_idx / 4;
                        (curr_pc, Some(i[pc_idx].clone()))
                    },
                    SectionData::Data(_) => {
                        (curr_pc, None)
                    }
                }
            },
            _ => {
                (curr_pc, None)
            }
        }
    }

    pub fn incr_pc(&mut self, thread_idx: u32)  {
        self.thread_states[thread_idx as usize].advance_pc();
    }

    pub fn update_pc(&mut self, thread_idx: u32, new_pc: u32) {
        self.thread_states[thread_idx as usize].set_pc(new_pc);
    }

    pub fn halt_thread(&mut self, thread_idx: u32) {
        self.thread_states[thread_idx as usize].halt();
    }

    pub fn is_thread_halted(&self, thread_idx: u32) -> bool {
        self.thread_states[thread_idx as usize].is_halted()
    }

    pub fn read_thread_register(&self, thread_idx: u32, register_idx: u32) -> u32 {
        self.thread_states[thread_idx as usize].read_register(register_idx)
    }

    pub fn write_thread_register(&mut self, thread_idx: u32, register_idx: u32, new_val: u32) {
        self.thread_states[thread_idx as usize].write_register(register_idx, new_val);
    }

    pub fn load (&self, thread_idx: u32, req_addr: u32) -> u32 {
        let (segment, ea) = self.get_effective_addr(req_addr, 4); 
        match (segment, ea) {
            (Some(s), Some(a)) => {
                match &self.memory_state[s as usize].data {
                    SectionData::Data(d) => {
                        LittleEndian::read_u32(&d[a..a + 4])
                    },
                    _ => {
                        panic!("Illegal Read from Instruction Memory");
                    }
                }
            },
            _ => {
                panic!("Illegal load address!");
            }
        }
    }

    pub fn store (&mut self, thread_idx: u32, req_addr: u32, new_val: u32)  {
        let (segment, ea) = self.get_effective_addr(req_addr, 4); 
        match (segment, ea) {
            (Some(s), Some(a)) => {
                match &mut self.memory_state[s as usize].data {
                    SectionData::Data(d) => {
                        LittleEndian::write_u32(&mut d[a..a + 4], new_val)
                    },
                    _ => {
                        panic!("Illegal Write to Instruction Memory");
                    }
                }
            },
            _ => {
                panic!("Illegal store address!");
            }
        }
    }
}
