use core::{fmt, num, panic};

use crate::thread_ctrl::{memory_state, warp_state::WarpState};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockState {
    warps:               Box<[WarpState]>,
    num_warps:           u32,
    num_threads:         u32,
    starting_thread_idx: u32
}


impl BlockState {
    pub fn new(block_idx: u32, threads_per_block: u32, threads_per_warp: u32, starting_pc: u32) -> Self {
        let num_warps= threads_per_block / threads_per_warp;
        let starting_thread_idx = block_idx * threads_per_block;
        let starting_warp_ids = starting_thread_idx / threads_per_warp; 

        let mut warps = vec![];
        for i in 0..num_warps {
            let warp_idx = starting_warp_ids + i; 
            let curr_thread_start = starting_thread_idx + i * threads_per_warp;
            warps.push(WarpState::new(warp_idx, curr_thread_start, threads_per_block, threads_per_warp, starting_pc, block_idx));
        }

        BlockState { 
            warps: warps.into_boxed_slice(), 
            num_warps: num_warps, 
            num_threads: num_warps * threads_per_warp, 
            starting_thread_idx: starting_thread_idx
        }
    }

    pub fn check_is_runnable(&self) -> bool {
        for warp in &self.warps {
            if !warp.check_is_runable() {
                return false; 
            }
        }
        true
    }

    pub fn run_block (&mut self, mem_state: &mut memory_state::MemoryState)  {
        if !self.check_is_runnable() {
            panic!("Tried to run a block with no runnable warps! ");
        }

        for warp in &mut self.warps {
            if warp.check_is_runable() {
                warp.run_threads(mem_state);
            }
        }
    }

    pub fn is_block_halted(&self) -> bool {
        for warp in &self.warps {
            if !warp.is_warp_halted() {
                return false;

            }
        }
        true
    }
}

impl fmt::Display for BlockState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for warp in &self.warps {
             writeln!(f, "{}", warp)?;
        }
        writeln!(f, "\n")
    }

}