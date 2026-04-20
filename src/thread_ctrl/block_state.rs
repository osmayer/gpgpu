use core::{fmt, panic};

use crate::thread_ctrl::{memory_state, warp_state::WarpState};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockState {
    warps:               Vec<WarpState>,
    num_warps:           u32,
    num_threads:         u32,
}


impl BlockState {
    pub fn new(block_idx: u32, warps_per_thread: u32, threads_per_warp: u32, starting_pc: u32) -> Self {
        let num_warps= warps_per_thread;

        let mut warps = vec![];
        for i in 0..num_warps {
            let warp_idx = i; 
            warps.push(WarpState::new(starting_pc, threads_per_warp, block_idx, warp_idx));
        }

        BlockState { 
            warps: warps,
            num_warps: num_warps, 
            num_threads: num_warps * threads_per_warp, 
        }
    }

    pub fn check_is_runnable(&self) -> bool {
        for warp in &self.warps {
            if !warp.check_is_runnable() {
                return false; 
            }
        }
        true
    }

    pub fn get_runnable_warps (&self) -> (Vec<u32>, u32) {
        let mut warp_list = vec![];
        let mut size = 0;
        for warp in 0..self.num_warps {
            if self.warps[warp as usize].check_is_runnable() {
                warp_list.push(warp);
                size += 1;
            }       
        }
        (warp_list, size)
    }

    pub fn set_waiting_for_mem (&mut self, thread_idx:u32, warp_idx:u32, new_val:bool) {
        self.warps[warp_idx as usize].set_waiting_for_mem(thread_idx, new_val);
    }

    pub fn run_block (&mut self, mem_state: &mut memory_state::MemoryState)  {
        if !self.check_is_runnable() {
            panic!("Tried to run a block with no runnable warps! ");
        }

        for warp in &mut self.warps {
            if warp.check_is_runnable() {
                warp.run_threads(mem_state);
            }
        }
    }

    pub fn run_warp (& mut self, warp_idx: u32, mem_state: &mut memory_state::MemoryState) -> bool {
        if self.warps[warp_idx as usize].check_is_runnable() {
            self.warps[warp_idx as usize].run_threads(mem_state);
            true
        } else {
            false
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
        for warp in 0..self.num_warps {
            writeln!(f, "WID: {}", warp)?;
            writeln!(f, "{}", self.warps[warp as usize])?;
        }
        writeln!(f, "\n")
    }

}