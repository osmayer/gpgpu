use core::{fmt, panic};

use crate::{instr_execute::execute_instr, thread_ctrl::{memory_state, thread_state::ThreadState}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WarpState {
    warp_idx:     u32,
    num_threads:  u32, 
    threads:      Vec<ThreadState>,
    run_status:   Vec<bool> 
}


impl WarpState {
    pub fn new(starting_pc: u32, threads_per_warp: u32,  block_idx: u32, warp_idx:u32) -> Self {
        let mut thread_states = vec![];
        
        for i in 0..threads_per_warp {
            thread_states.push(ThreadState::new(starting_pc, i, warp_idx, block_idx));
        }
        let run_status = vec![ false; thread_states.len() ];
        
        WarpState { 
            warp_idx:    warp_idx, 
            num_threads: threads_per_warp, 
            threads:     thread_states,
            run_status:  run_status
        }
    }

    pub fn check_is_runnable (&self) -> bool {
        for thread in &self.threads {
            if thread.get_waiting_for_mem() {
                return false;
            }
        }
        true
    }

    pub fn run_threads (&mut self, mem_state: &mut memory_state::MemoryState) {
        if !self.check_is_runnable() {
            panic!("Unable to run warp because not all threads are ready.");
        }

        // for thread in &mut self.threads {
        //    execute_instr(thread, mem_state);
        // }
        let mut first_pc = 0; 
        for i in 0..self.num_threads {
            if !self.run_status[i as usize] && !self.threads[i as usize].is_halted() {
                first_pc = self.threads[i as usize].get_pc();
            }
        }

        for i in 0..self.num_threads {
            if self.threads[i as usize].get_pc() == first_pc && !self.threads[i as usize].is_halted() {
                execute_instr(&mut self.threads[i as usize], mem_state);
                self.run_status[i as usize] = true; 
            }
        }

        let mut is_done = true;
        for i in 0..self.num_threads {
            if !self.run_status[i as usize] && !self.threads[i as usize].is_halted() {
                is_done = false; 
            }
        }

        if is_done {
            for i in 0..self.num_threads {
                self.run_status[i as usize] = false;
            }
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