use core::panic;
use crate::thread_ctrl::{system_state::SystemState, scheduler_state:: {SchedulerData}};



pub fn select_warp (state: & mut SystemState) -> Option<(u32, u32)> {
    let data = state.get_scheduler_data();
    match data {
        SchedulerData::RoundRobin {curr_block, curr_warp, ..} => {
            let mut warp_idx = curr_warp;
            let mut block_idx = curr_block;
            for _i in 0..(state.num_blocks*state.warps_per_block) {
                warp_idx += 1;
                if warp_idx == state.warps_per_block {
                    warp_idx = 0;
                    block_idx += 1;
                    if block_idx == state.num_blocks {
                        block_idx = 0;
                    }
                }
                if state.is_warp_runnable(block_idx, warp_idx) {
                    let new_data = SchedulerData::RoundRobin{curr_warp: warp_idx, curr_block: block_idx};
                    state.set_scheduler_data(new_data);
                    return Some((block_idx, warp_idx));
                }
            }
            None
           
        }
        SchedulerData::Chaos => {
            for block in 0..state.num_blocks {
                let (warps, size) = state.get_runnable_warps(block);
                if size != 0 {
                    return Some((block, warps[0]));
                }
            }
            None
        }
    }
}