
use byteorder::{ByteOrder, LittleEndian};
use crate::{instr_execute::Opcode, program_loader::{self, Segment, SegmentMetadata}, thread_ctrl::{Instr, mem_request::MemRequest}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MemoryState {
    memory_state:  Vec<SectionImage>,
    pub memory_requests: Vec<Vec<Vec<MemRequest>>>,
    pub threads_per_warp: u32,
    pub warps_per_block: u32,
    pub num_blocks: u32
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SectionData {
    Data (Vec<u8>),
    Instruction (Vec<Instr>)
}



#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct  SectionImage {
    metadata: SegmentMetadata,
    data:     SectionData
}

impl MemoryState {
    pub fn new (program_image: &Vec<Segment>, num_blocks: u32,  warps_per_block: u32, threads_per_warp: u32, mem_delay: u32) -> Self {

        let mut new_state = MemoryState { 
            memory_state: vec![], 
            memory_requests: vec![],
            num_blocks,
            threads_per_warp,
            warps_per_block
        };
        // Make a system state and request array for all threads
        for _i in 0..(num_blocks) {
            let mut requests_vec = vec![];
            for _j in 0..(warps_per_block) {
                let mut warp_requests_vec = vec![];
                for _k in 0..threads_per_warp {
                    warp_requests_vec.push(MemRequest::new(mem_delay));
                }
                requests_vec.push(warp_requests_vec);
            }
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
                                        panic!("Illegal instr found!");
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

        // println!("{:?}", new_state.memory_state);
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

    pub fn check_if_ready (&self, thread_idx:u32, warp_idx:u32, block_idx:u32) -> bool {
        self.memory_requests[block_idx as usize][warp_idx as usize][thread_idx as usize].check_if_ready()
    }

    pub fn check_if_valid (&self, thread_idx:u32, warp_idx:u32, block_idx:u32) -> bool {
        self.memory_requests[block_idx as usize][warp_idx as usize][thread_idx as usize].check_if_valid()
    }

    pub fn fetch_instr (&self, curr_pc: u32) -> Option<Instr> {
        let mem_loc = self.get_effective_addr(curr_pc, 1, false);
        let seg_idx                  = mem_loc.0;
        let byte_idx                 = mem_loc.1;

        match (seg_idx, byte_idx) {
            (Some(seg_idx), Some(byte_idx)) => {
                match &self.memory_state[seg_idx].data {
                    SectionData::Instruction(i) => {
                        let pc_idx = byte_idx / 4;
                        Some(i[pc_idx].clone())
                    },
                    SectionData::Data(_) => {
                        None
                    }
                }
            },
            _ => {
                None
            }
        }
    }

     pub fn read_request (& mut self, thread_idx:u32, warp_idx:u32, block_idx:u32) {
        self.memory_requests[block_idx as usize][warp_idx as usize][thread_idx as usize].read_request();
    }

    pub fn write_request (& mut self, thread_idx:u32, warp_idx:u32, block_idx:u32) {
        self.memory_requests[block_idx as usize][warp_idx as usize][thread_idx as usize].write_request();
    }

    pub fn reset_request (& mut self, thread_idx:u32, warp_idx:u32, block_idx:u32) {
        self.memory_requests[block_idx as usize][warp_idx as usize][thread_idx as usize].reset_request();
    }

    pub fn load_32 (& mut self, thread_idx: u32, warp_idx:u32, block_idx: u32, req_addr: u32) -> Option<u32> {
        let ready = self.check_if_ready(thread_idx, warp_idx, block_idx);
        let valid = self.check_if_valid(thread_idx, warp_idx, block_idx);
        if !valid {
            self.read_request(thread_idx, warp_idx, block_idx);
            None
        }
        else if !ready {
            None
        } else {
            self.reset_request(thread_idx, warp_idx, block_idx);
            let (segment, ea) = self.get_effective_addr(req_addr, 4, false); 
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

    pub fn load_16 (& mut self, thread_idx: u32, warp_idx:u32, block_idx: u32, req_addr: u32) -> Option<u16> {
        let ready = self.check_if_ready(thread_idx, warp_idx, block_idx);
        let valid = self.check_if_valid(thread_idx, warp_idx, block_idx);
        if !valid {
            self.read_request(thread_idx, warp_idx, block_idx);
            None
        }
        else if !ready {
            None
        } else {
            self.reset_request(thread_idx, warp_idx, block_idx);
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

    pub fn load_8 (& mut self, thread_idx: u32, warp_idx:u32, block_idx: u32, req_addr: u32) -> Option<u8> {
        let ready = self.check_if_ready(thread_idx, warp_idx, block_idx);
        let valid = self.check_if_valid(thread_idx, warp_idx, block_idx);
        if !valid {
            self.read_request(thread_idx, warp_idx, block_idx);
            None
        }
        else if !ready {
            None
        } else {
            self.reset_request(thread_idx, warp_idx, block_idx);
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

    pub fn store_32 (&mut self, thread_idx: u32, warp_idx:u32, block_idx: u32, req_addr: u32, new_val: u32) -> bool {
        let ready = self.check_if_ready(thread_idx, warp_idx, block_idx);
        let valid = self.check_if_valid(thread_idx, warp_idx, block_idx);
        if !valid {
            self.write_request(thread_idx, warp_idx, block_idx);
            false
        }
        else if !ready {
            false
        } else {
            self.reset_request(thread_idx, warp_idx, block_idx);
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

     pub fn store_16 (&mut self, thread_idx: u32, warp_idx:u32, block_idx: u32, req_addr: u32, new_val: u16) -> bool {
        let ready = self.check_if_ready(thread_idx, warp_idx, block_idx);
        let valid = self.check_if_valid(thread_idx, warp_idx, block_idx);
        if !valid {
            self.write_request(thread_idx, warp_idx, block_idx);
            false
        }
        else if !ready {
            false
        } else {
            self.reset_request(thread_idx, warp_idx, block_idx);
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

     pub fn store_8 (&mut self, thread_idx: u32, warp_idx:u32, block_idx: u32, req_addr: u32, new_val: u8) -> bool {
        let ready = self.check_if_ready(thread_idx, warp_idx, block_idx);
        let valid = self.check_if_valid(thread_idx, warp_idx, block_idx);
        if !valid {
            self.write_request(thread_idx, warp_idx, block_idx);
            false
        }
        else if !ready {
            false
        } else {
            self.reset_request(thread_idx, warp_idx, block_idx);
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

    pub fn get_threads_per_block(&self) -> u32 {
        self.threads_per_warp * self.warps_per_block
    }

    pub fn get_num_blocks(&self) -> u32 {
        self.num_blocks
    }

    pub fn incr_cycles (& mut self, thread_idx:u32, warp_idx:u32, block_idx:u32) {
        self.memory_requests[block_idx as usize][warp_idx as usize][thread_idx as usize].incr_cycles();
    }
}