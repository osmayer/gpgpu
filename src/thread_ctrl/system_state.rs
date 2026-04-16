use core::num;
use std::fmt;
use byteorder::{ByteOrder, LittleEndian};
use crate::{instr_execute::Opcode, program_loader::{self, Segment, SegmentMetadata}, thread_ctrl::{Instr, block_state::BlockState, mem_request::MemRequest, memory_state::MemoryState, thread_state::ThreadState}}; 

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemState {
    pub block_states: Vec<BlockState>,
    memory_state:  MemoryState,
    pub threads_per_block: u32,
    pub num_blocks: u32,
    pub memory_requests: Vec<Vec<MemRequest>>,
    pub cycles_elapsed: u32
}


impl SystemState {
    pub fn new(program_image: &Vec<Segment>, num_blocks: u32,  threads_per_block: u32, threads_per_warp: u32, mem_delay: u32, starting_pc: u32) -> Self {
        println!("Creating a new system with {} blocks and {} threads per block", num_blocks, threads_per_block);
        let mut new_state = SystemState {
            block_states: vec![],
            memory_state:  MemoryState::new(program_image, num_blocks, threads_per_block, mem_delay),
            threads_per_block: threads_per_block,
            num_blocks: num_blocks,
            memory_requests: vec![], 
            cycles_elapsed: 0
        };

        for i in 0..num_blocks {
            new_state.block_states.push(BlockState::new(i, threads_per_block, threads_per_warp, starting_pc));
        }

        println!("{:?}", new_state.memory_state);
        new_state
    }


    pub fn get_num_blocks(&self) -> u32 {
        self.num_blocks
    }

    pub fn get_threads_per_block (&self) -> u32 {
        self.threads_per_block
    }

    pub fn incr_cycles (& mut self, thread_idx:u32, block_idx:u32) {
        self.memory_state.incr_cycles(thread_idx, block_idx);
    }

    pub fn run_if_able (&mut self, block_idx: u32) -> bool {
        if self.block_states[block_idx as usize].check_is_runnable() {
            self.block_states[block_idx as usize].run_block(&mut self.memory_state);
            return true;
        }
        false
    }

    pub fn is_block_halted (&mut self, block_idx: u32) -> bool {
        println!("Block {} is halted {}", block_idx, self.block_states[block_idx as usize].is_block_halted());
        self.block_states[block_idx as usize].is_block_halted()
    }

    pub fn update_total_cycle_count(&mut self) {
        self.cycles_elapsed += 1; 
    }
}

impl fmt::Display for SystemState {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        println!("Printing out the system state!");
        for block in 0..self.get_num_blocks() {
            writeln!(f, "BID: {}", block)?;
            writeln!(f, "{}", self.block_states[block as usize])?;
        }
        // Use the write! macro to define the string representation
        writeln!(f, "Cycles Elapsed {}", self.cycles_elapsed)
    }
}