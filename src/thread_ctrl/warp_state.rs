use core::{num, panic};

use crate::{instr_execute::execute_instr, thread_ctrl::thread_state::ThreadState};

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
        for _ in 0..threads_per_warp {
            thread_states.push(ThreadState::new(starting_pc, starting_thread_idx + 1, block_idx));
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

    pub fn run_threads (&mut self, sys_state:&mut super::system_state::SystemState) {
        if !self.check_is_runable() {
            panic!("Unnable to run warp because not all threads are ready.");
        }

        for thread in &mut self.threads {
            execute_instr(thread, sys_state);
        }
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