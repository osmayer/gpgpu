use core::{fmt, num, panic};
use byteorder::{ByteOrder, LittleEndian};
use riscv_decode;
use crate::{instr_execute::Opcode, program_loader::{self, Segment, SegmentMetadata}};

const INITIAL_SP: u32 = 0x7ff00000;
const INITIAL_GP: u32 = 0x10000000; 

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThreadState {
    pc:         u32,
    registers:  [u32; 32],
    halted:     bool,
    waiting_for_mem: bool
}

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SectionData {
    Data (Vec<u8>),
    Instruction (Vec<Instr>)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MemRequestType {
    Read,
    Write,
    NoReq
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct  SectionImage {
    metadata: SegmentMetadata,
    data:     SectionData
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MemRequest {
    cycles_waited: u32,
    req_type: MemRequestType,
    valid: bool,
    ready: bool,
    mem_delay: u32
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemState {
    pub thread_states: Vec<Vec<ThreadState>>,
    memory_state:  Vec<SectionImage>,
    pub threads_per_block: u32,
    pub num_blocks: u32,
    pub memory_requests: Vec<Vec<MemRequest>>,
    pub cycles_elapsed: u32
}

impl ThreadState {
    pub fn new(starting_pc: u32) -> Self {
        let mut new_state= ThreadState { 
            pc: starting_pc, 
            registers: [0; 32],
            halted: false,
            waiting_for_mem: false
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

    pub fn set_waiting_for_mem(&mut self, new_val: bool) {
        self.waiting_for_mem = new_val;
    }

    pub fn get_waiting_for_mem(&mut self) -> bool {
        self.waiting_for_mem
    }

}

impl fmt::Display for ThreadState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use the write! macro to define the string representation
        writeln!(f, "PC: {:#x}", self.get_pc())?;
        writeln!(f, "===== BEGIN REGISTER DUMP =====")?;
        for i in 0..32 {
            writeln!(f, "x{:<2} \t {:#010X} ({})", i, self.read_register(i), self.read_register(i) as i32)?;
        }
        writeln!(f, "===== END REGISTER DUMP =====")?;
        writeln!(f, "Halted: {}", self.is_halted())
    }

}

impl SystemState {
    pub fn new(program_image: &Vec<Segment>, threads_per_block: u32, num_blocks: u32, mem_delay: u32) -> Self {
        let mut new_state = SystemState {
            thread_states: vec![],
            memory_state:  vec![],
            threads_per_block: threads_per_block,
            num_blocks: num_blocks,
            memory_requests: vec![], 
            cycles_elapsed: 0
        };
        
        // Make a system state and request array for all threads
        for _i in 0..(num_blocks) {
            let mut curr_vec = vec![];
            let mut requests_vec = vec![];
            for _j in 0..(threads_per_block) {
                curr_vec.push(ThreadState::new(4194304));
                requests_vec.push(MemRequest::new(mem_delay));
            }
            new_state.thread_states.push(curr_vec);
            new_state.memory_requests.push(requests_vec);
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
                            println!("{:x}", u);
                            
                            let instr = riscv_decode::decode(u).ok();
                            match instr {
                                Some(i) => {
                                    curr_instructions.push(Instr::Standard(i));
                                }
                                _ => {
                                    if u & 0x3F == 0xF {
                                        let funct3 = (u >> 12) & 0x7;
                                        let rs1 = (u >> 15) & 0x1F;
                                        let rs2 = (u >> 20) & 0x1F;
                                        let rd = (u >> 7) & 0x1F;
                                        match funct3 {
                                            0 => {
                                                curr_instructions.push(Instr::Custom{op:Opcode::Tid, rd:rd, rs1:rs1, rs2:rs2});
                                            }
                                            1 => {
                                                curr_instructions.push(Instr::Custom{op:Opcode::Bid, rd:rd, rs1:rs1, rs2:rs2});
                                            }
                                            2 => {
                                                curr_instructions.push(Instr::Custom{op:Opcode::Bdim, rd:rd, rs1:rs1, rs2:rs2});
                                            }
                                            3 => {
                                                curr_instructions.push(Instr::Custom{op:Opcode::Gdim, rd:rd, rs1:rs1, rs2:rs2});
                                            }
                                            4 => {
                                                curr_instructions.push(Instr::Custom{op:Opcode::LwS, rd:rd, rs1:rs1, rs2:rs2});
                                            }
                                            5 => {
                                                curr_instructions.push(Instr::Custom{op:Opcode::SwS, rd:rd, rs1:rs1, rs2:rs2});
                                            }
                                            _ => {
                                                panic!("Illegal new instr found!");
                                            }
                                        }
                                    } else {
                                        println!("Illegal instr found!");
                                    }
                                }
                            }
                            
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

        println!("{:?}", new_state.memory_state);
        new_state
    }

    fn get_effective_addr (&self, req_addr: u32, req_size: usize, can_resize: bool) -> (Option<usize>, Option<usize>) {
        let mut seg_id: usize = 0; 
        for segment in &self.memory_state {
            let segment_start = segment.metadata.base_addr as usize;
            let segment_end   = (segment.metadata.base_addr + segment.metadata.size) as usize;
            let req_start_addr= req_addr as usize;
            let req_end_addr  = (req_start_addr + 4) as usize;

            if req_start_addr >= segment_start && req_end_addr <= segment_end {
                let effective_start = req_start_addr - segment_start;
                let effective_end   = effective_start + req_size; 
            
                if effective_end as u32 > segment.metadata.allocated_size && !can_resize {
                    return (None, None);
                }

                return (Some(seg_id), Some(effective_start));
            }
            seg_id+=1;
        }
        (None, None)
    }

    pub fn fetch_instr (&self, thread_idx: u32, block_idx: u32) -> (u32, Option<Instr>) {
        let curr_pc                            = self.thread_states[block_idx as usize][thread_idx as usize].get_pc();
        let mem_loc = self.get_effective_addr(curr_pc, 1, false);
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

    pub fn incr_pc(&mut self, thread_idx: u32, block_idx: u32)  {
        self.thread_states[block_idx as usize][thread_idx as usize].advance_pc();
    }

    pub fn update_pc(&mut self, thread_idx: u32, block_idx: u32, new_pc: u32) {
        self.thread_states[block_idx as usize][thread_idx as usize].set_pc(new_pc);
    }

    pub fn update_total_cycle_count(&mut self) {
        self.cycles_elapsed += 1;
    }

    pub fn halt_thread(&mut self, thread_idx: u32, block_idx: u32) {
        self.thread_states[block_idx as usize][thread_idx as usize].halt();
    }

    pub fn is_thread_waiting(&mut self, thread_idx: u32, block_idx: u32) -> bool {
        self.thread_states[block_idx as usize][thread_idx as usize].get_waiting_for_mem()
    }

    pub fn set_thread_waiting(&mut self, thread_idx: u32, block_idx: u32, new_val: bool) {
        self.thread_states[block_idx as usize][thread_idx as usize].set_waiting_for_mem(new_val);
    }

    pub fn is_thread_halted(&self, thread_idx: u32, block_idx: u32) -> bool {
        self.thread_states[block_idx as usize][thread_idx as usize].is_halted()
    }

    pub fn read_thread_register(&self, thread_idx: u32, block_idx: u32, register_idx: u32) -> u32 {
        self.thread_states[block_idx as usize][thread_idx as usize].read_register(register_idx)
    }

    pub fn write_thread_register(&mut self, thread_idx: u32, block_idx: u32, register_idx: u32, new_val: u32) {
        self.thread_states[block_idx as usize][thread_idx as usize].write_register(register_idx, new_val);
    }

    pub fn get_num_blocks(&self) -> u32 {
        self.num_blocks
    }

    pub fn get_threads_per_block (&self) -> u32 {
        self.threads_per_block
    }

    pub fn incr_cycles (& mut self, thread_idx:u32, block_idx:u32) {
        self.memory_requests[block_idx as usize][thread_idx as usize].incr_cycles();
    }

    pub fn check_if_ready (&self, thread_idx:u32, block_idx:u32) -> bool {
        self.memory_requests[block_idx as usize][thread_idx as usize].check_if_ready()
    }

    pub fn check_if_valid (&self, thread_idx:u32, block_idx:u32) -> bool {
        self.memory_requests[block_idx as usize][thread_idx as usize].check_if_valid()
    }

    pub fn read_request (& mut self, thread_idx:u32, block_idx:u32) {
        self.memory_requests[block_idx as usize][thread_idx as usize].read_request();
    }

    pub fn write_request (& mut self, thread_idx:u32, block_idx:u32) {
        self.memory_requests[block_idx as usize][thread_idx as usize].write_request();
    }

    pub fn reset_request (& mut self, thread_idx:u32, block_idx:u32) {
        self.memory_requests[block_idx as usize][thread_idx as usize].reset_request();
    }

    pub fn load_32 (& mut self, thread_idx: u32, block_idx: u32, req_addr: u32) -> Option<u32> {
        let ready = self.check_if_ready(thread_idx, block_idx);
        let valid = self.check_if_valid(thread_idx, block_idx);
        if !valid {
            self.read_request(thread_idx, block_idx);
            None
        }
        else if !ready {
            None
        } else {
            self.reset_request(thread_idx, block_idx);
            let (segment, ea) = self.get_effective_addr(req_addr, 4, false); 
            println!("{:?} {:x}", ea, req_addr);
            match (segment, ea) {
                (Some(s), Some(a)) => {
                    match &self.memory_state[s as usize].data {
                        SectionData::Data(d) => {
                            Some(LittleEndian::read_u32(&d[a..a + 4]))
                        },
                        _ => {
                            panic!("Illegal Read from Data Memory of size 4 at {:x}", req_addr);
                        }
                    }
                },
                _ => {
                    panic!("Illegal load address!");
                }
            }
        }
    }

    pub fn load_16 (& mut self, thread_idx: u32, block_idx: u32, req_addr: u32) -> Option<u16> {
        let ready = self.check_if_ready(thread_idx, block_idx);
        let valid = self.check_if_valid(thread_idx, block_idx);
        if !valid {
            self.read_request(thread_idx, block_idx);
            None
        }
        else if !ready {
            None
        } else {
            self.reset_request(thread_idx, block_idx);
            let (segment, ea) = self.get_effective_addr(req_addr, 4, false); 
            match (segment, ea) {
                (Some(s), Some(a)) => {
                    match &self.memory_state[s as usize].data {
                        SectionData::Data(d) => {
                            Some(LittleEndian::read_u16(&d[a..a + 2]))
                        },
                        _ => {
                            panic!("Illegal Read from Data Memory of size 2 at {:x}", req_addr);
                        }
                    }
                },
                _ => {
                    panic!("Illegal load address!");
                }
            }
        }
    }

    pub fn load_8 (& mut self, thread_idx: u32, block_idx: u32, req_addr: u32) -> Option<u8> {
        let ready = self.check_if_ready(thread_idx, block_idx);
        let valid = self.check_if_valid(thread_idx, block_idx);
        if !valid {
            self.read_request(thread_idx, block_idx);
            None
        }
        else if !ready {
            None
        } else {
            self.reset_request(thread_idx, block_idx);
            let (segment, ea) = self.get_effective_addr(req_addr, 1, false); 
            match (segment, ea) {
                (Some(s), Some(a)) => {
                    match &self.memory_state[s as usize].data {
                        SectionData::Data(d) => {
                            Some(d[a])
                        },
                        _ => {
                            panic!("Illegal Read from Data Memory of Size 1 at {:x}", req_addr);
                        }
                    }
                },
                _ => {
                    panic!("Illegal load address!");
                }
            }
        }
    }

    pub fn store_32 (&mut self, thread_idx: u32, block_idx: u32, req_addr: u32, new_val: u32) -> bool {
        println!("{}", req_addr);
        let ready = self.check_if_ready(thread_idx, block_idx);
        let valid = self.check_if_valid(thread_idx, block_idx);
        if !valid {
            self.write_request(thread_idx, block_idx);
            false
        }
        else if !ready {
            false
        } else {
            self.reset_request(thread_idx, block_idx);
            let (segment, ea) = self.get_effective_addr(req_addr, 4, true); 
            let mut did_resize = false; 
            let mut new_len = 0; 
            let mut seg_id = 0; 
            match (segment, ea) {
                (Some(s), Some(a)) => {
                    match &mut self.memory_state[s as usize].data {
                        SectionData::Data(d) => {
                            let len = d.len();
                            if a+4 >= len {
                                d.resize(a+4, 0);
                                did_resize = true; 
                                new_len = a + 4; 
                                seg_id = s;
                            }

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

            if did_resize {
                self.memory_state[seg_id as usize].metadata.allocated_size = new_len as u32;
            }
            true
        }
    }

     pub fn store_16 (&mut self, thread_idx: u32, block_idx: u32, req_addr: u32, new_val: u16) -> bool {
        println!("Storing to {:x}", req_addr);
        let ready = self.check_if_ready(thread_idx, block_idx);
        let valid = self.check_if_valid(thread_idx, block_idx);
        if !valid {
            self.write_request(thread_idx, block_idx);
            false
        }
        else if !ready {
            false
        } else {
            self.reset_request(thread_idx, block_idx);
            let (segment, ea) = self.get_effective_addr(req_addr, 2,true); 
            let mut did_resize = false; 
            let mut new_len = 0; 
            let mut seg_id = 0; 
            match (segment, ea) {
                (Some(s), Some(a)) => {
                    match &mut self.memory_state[s as usize].data {
                        SectionData::Data(d) => {
                            let len = d.len();
                            if a+2 >= len {
                                d.resize(a+2, 0);
                                did_resize = true; 
                                new_len = a + 4; 
                                seg_id = s;
                            }

                            LittleEndian::write_u16(&mut d[a..a + 2], new_val)
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
            if did_resize {
                self.memory_state[seg_id as usize].metadata.allocated_size = new_len as u32;
            }
            true
        }
    }

     pub fn store_8 (&mut self, thread_idx: u32, block_idx: u32, req_addr: u32, new_val: u8) -> bool {
        let ready = self.check_if_ready(thread_idx, block_idx);
        let valid = self.check_if_valid(thread_idx, block_idx);
        if !valid {
            self.write_request(thread_idx, block_idx);
            false
        }
        else if !ready {
            false
        } else {
            self.reset_request(thread_idx, block_idx);
            let (segment, ea) = self.get_effective_addr(req_addr, 1, true); 
            let mut did_resize = false; 
            let mut new_len = 0; 
            let mut seg_id = 0; 
            match (segment, ea) {
                (Some(s), Some(a)) => {
                    match &mut self.memory_state[s as usize].data {
                        SectionData::Data(d) => {
                            let len = d.len();
                            if a+1 >= len {
                                d.resize(a+1, 0);
                                did_resize = true; 
                                new_len = a + 4; 
                                seg_id = s;
                            }

                            // LittleEndian::write_u32(&mut d[a..a + 4], new_val)
                            d[a] = new_val;
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
            if did_resize {
                self.memory_state[seg_id as usize].metadata.allocated_size = new_len as u32;
            }
            true
        }
    }


}

impl fmt::Display for SystemState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for block in 0..self.get_num_blocks() {
            for thread in 0..self.get_threads_per_block() {
                writeln!(f, "BID: {} TID: {}", block, thread)?;
                writeln!(f, "{}", self.thread_states[block as usize][thread as usize]);
            }
        }
        // Use the write! macro to define the string representation
        writeln!(f, "Cycles Elapsed {}", self.cycles_elapsed)
    }
}

impl MemRequest {
    pub fn new (mem_delay: u32) -> MemRequest{
        let new_state=MemRequest {
            ready: false,
            cycles_waited: 0,
            req_type: MemRequestType::NoReq,
            valid: false,
            mem_delay: mem_delay
        };
        new_state
    }

    pub fn incr_cycles (& mut self) {
        if self.valid {
            self.cycles_waited += 1;
            // FIX THIS NUMBER TO CHANGE MEM DELAY
            if self.cycles_waited == self.mem_delay {
                self.ready = true;
            }
        }
    }

    pub fn check_if_ready (& self) -> bool {
        self.ready
    }

    pub fn check_if_valid (& self) -> bool {
        self.valid
    }

    pub fn read_request (& mut self) {
        self.valid = true;
        self.cycles_waited = 0;
        self.req_type = MemRequestType::Read;
    }

    pub fn reset_request (& mut self) {
        self.valid = false;
        self.ready = false;
        self.req_type = MemRequestType::NoReq;
    }

    pub fn write_request (& mut self) {
        self.req_type = MemRequestType::Write;
        self.valid = true;
        self.cycles_waited = 0;
    }
}