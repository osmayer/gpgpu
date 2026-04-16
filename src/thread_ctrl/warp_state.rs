use core::{fmt, num, panic};

use crate::{instr_execute::execute_instr, thread_ctrl::{memory_state, thread_state::ThreadState}};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WarpState {
    warp_idx:     u32,
    num_threads:  u32, 
    /// is_runnable:  bool,
    threads:      Box<[ThreadState]>
}


impl WarpState {
    pub fn new(warp_idx: u32, starting_thread_idx: u32, threads_per_block: u32, threads_per_warp: u32, starting_pc: u32, block_idx: u32) -> Self {
        let mut thread_states = vec![];
        for i in 0..threads_per_warp {
            thread_states.push(ThreadState::new(starting_pc, starting_thread_idx + i, block_idx));
        }

        WarpState { 
            warp_idx, 
            num_threads: threads_per_warp, 
            threads:     thread_states.into_boxed_slice()
        }
    }

    pub fn check_is_runable (&self) -> bool {
        for thread in &self.threads {
            if thread.get_waiting_for_mem() {
                return false;
            }
        }
        true
    }

    pub fn run_threads (&mut self, mem_state: &mut memory_state::MemoryState) {
        if !self.check_is_runable() {
            panic!("Unnable to run warp because not all threads are ready.");
        }

        for thread in &mut self.threads {
            execute_instr(thread, mem_state);
        }
    }

    pub fn is_warp_halted (&self) -> bool {
        println!("Polled for halted ");
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
        for thread in &self.threads {
             writeln!(f, "{}", thread)?;
        }
        writeln!(f, "\n")
    }

}