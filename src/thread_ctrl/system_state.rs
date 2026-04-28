use std::fmt;
use crate::{program_loader::{Segment}, thread_ctrl::{block_state::BlockState, memory_state::MemoryState, scheduler_state::{SchedulerState, SchedulerData}}}; 

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SystemState {
    pub block_states: Vec<BlockState>,
    memory_state:  MemoryState,
    pub threads_per_warp: u32,
    pub warps_per_block: u32,
    pub num_blocks: u32,
    pub cycles_elapsed: u32,
    pub scheduler: SchedulerState,
    pub functional_units: u32
}


impl SystemState {
    pub fn new(program_image: &Vec<Segment>, num_blocks: u32,  threads_per_warp: u32, warps_per_block: u32, mem_delay: u32, starting_pc: u32, scheduler: u32, functional_units: u32) -> Self {
        let mut new_state = SystemState {
            block_states: vec![],
            memory_state:  MemoryState::new(program_image, num_blocks, warps_per_block, threads_per_warp, mem_delay),
            warps_per_block: warps_per_block,
            threads_per_warp: threads_per_warp,
            num_blocks: num_blocks,
            cycles_elapsed: 0,
            scheduler: SchedulerState::new(scheduler,num_blocks, warps_per_block),
            functional_units: functional_units
        };

        for i in 0..num_blocks {
            new_state.block_states.push(BlockState::new(i, warps_per_block, threads_per_warp, starting_pc));
        }

        new_state
    }


    pub fn get_num_blocks(&self) -> u32 {
        self.num_blocks
    }

    pub fn get_threads_per_warp(&self) -> u32 {
        self.threads_per_warp
    }

    pub fn get_warps_per_block(&self) -> u32 {
        self.warps_per_block
    }

    pub fn get_threads_per_block (&self) -> u32 {
        self.threads_per_warp * self.warps_per_block
    }

    pub fn set_waiting_for_mem (&mut self, thread_idx:u32, warp_idx:u32, block_idx:u32, new_val:bool) {
        self.block_states[block_idx as usize].set_waiting_for_mem(thread_idx, warp_idx, new_val);
    }

    pub fn incr_cycles (& mut self, thread_idx:u32, warp_idx:u32, block_idx:u32) {
        self.memory_state.incr_cycles(thread_idx, warp_idx, block_idx);
        if self.memory_state.check_if_ready(thread_idx, warp_idx, block_idx) {
            self.set_waiting_for_mem(thread_idx, warp_idx, block_idx, false);
        }
    }

    pub fn run_warp (& mut self, block_idx: u32, warp_idx: u32) {
        self.block_states[block_idx as usize].run_warp(warp_idx, &mut self.memory_state);
    }

    pub fn get_runnable_warps (&self, block_idx: u32) -> (Vec<u32>, u32) {
        self.block_states[block_idx as usize].get_runnable_warps()
    }

    pub fn is_warp_runnable (&self, block_idx: u32, warp_idx:u32) -> bool {
        self.block_states[block_idx as usize].is_warp_runnable(warp_idx)
    }

    pub fn is_block_halted (&mut self, block_idx: u32) -> bool {
        self.block_states[block_idx as usize].is_block_halted()
    }

    pub fn update_total_cycle_count(&mut self) {
        self.cycles_elapsed += 1; 
    }

    pub fn set_scheduler_data (& mut self, data: SchedulerData) {
        self.scheduler.set_scheduler_data(data);
    }

    pub fn get_scheduler_data (&self) -> SchedulerData {
        self.scheduler.get_scheduler_data()
    }
}

impl fmt::Display for SystemState {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for block in 0..self.get_num_blocks() {
            writeln!(f, "BID: {}", block)?;
            writeln!(f, "{}", self.block_states[block as usize])?;
        }
        // Use the write! macro to define the string representation
        writeln!(f, "Cycles Elapsed {}", self.cycles_elapsed)
    }
}