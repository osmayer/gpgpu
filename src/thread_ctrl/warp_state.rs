use core::{fmt, panic};

use crate::{instr_execute::execute_instr, thread_ctrl::{memory_state, thread_state::ThreadState}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WarpState {
    warp_idx:     u32,
    num_threads:  u32, 
    /// is_runnable:  bool,
    threads:      Vec<ThreadState>
}


impl WarpState {
    pub fn new(starting_pc: u32, threads_per_warp: u32,  block_idx: u32, warp_idx:u32) -> Self {
        let mut thread_states = vec![];
        for i in 0..threads_per_warp {
            println!("Created Block {} Warp {} Thread {}", block_idx, warp_idx, i);
            thread_states.push(ThreadState::new(starting_pc, i, warp_idx, block_idx));
        }

        WarpState { 
            warp_idx: warp_idx, 
            num_threads: threads_per_warp, 
            threads:     thread_states
        }
    }

    pub fn check_is_runnable (&self) -> bool {
        for thread in &self.threads {
            if thread.get_waiting_for_mem() | thread.is_halted() {
                return false;
            }
        }
        true
    }

    pub fn run_threads (&mut self, mem_state: &mut memory_state::MemoryState) {
        if !self.check_is_runnable() {
            panic!("Unable to run warp because not all threads are ready.");
        }

        for thread in &mut self.threads {
            execute_instr(thread, mem_state);
        }
    }

    pub fn set_waiting_for_mem (& mut self, thread_idx:u32, new_val: bool) {
        self.threads[thread_idx as usize].set_waiting_for_mem(new_val);
    }

    pub fn is_warp_halted (&self) -> bool {
        for thread in &self.threads {
            if !thread.is_halted() {
                return false;
            }
        }
        true
    }
}

impl fmt::Display for WarpState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for thread_idx in 0..self.num_threads {
            writeln!(f, "TID: {}", thread_idx)?;
            writeln!(f, "{}", self.threads[thread_idx as usize])?;
        }
        writeln!(f, "\n")
    }
}