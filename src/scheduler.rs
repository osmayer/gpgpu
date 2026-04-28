use crate::thread_ctrl::{system_state::SystemState, scheduler_state:: {SchedulerData}};



pub fn select_warp (state: & mut SystemState) -> Option<Vec<(u32, u32)>> {
    let data = state.get_scheduler_data();
    let functional_units = state.functional_units;
    let mut return_set = vec![];
    let mut num_selected = 0;
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
                    num_selected += 1;
                    return_set.push((block_idx, warp_idx));
                    if num_selected == functional_units {
                        break;
                    }
                }
            }
        }
        SchedulerData::Chaos => {
            for block in 0..state.num_blocks {
                let (warps, size) = state.get_runnable_warps(block);
                for i in 0..size {
                    return_set.push((block, warps[i as usize]));
                    num_selected += 1;
                    if num_selected == functional_units {
                        break;
                    }
                }
                if num_selected == functional_units {
                    break;
                }
            }
        }
        SchedulerData::Lru { history } => {
            let mut history_index = 0;
            let mut h = history.clone();
            for _tries in 0..(state.num_blocks*state.warps_per_block) {
                let (b, w) = h[history_index];
                if state.is_warp_runnable(b, w) {
                    num_selected += 1;
                    return_set.push((b, w));
                    h.remove(history_index as usize);
                    h.push((b,w));
                    if num_selected == functional_units {
                        break;
                    }
                } else {
                    history_index += 1;
                }
            }
            state.set_scheduler_data(SchedulerData::Lru{history: h});
        }
    }
    if num_selected != 0 {
        Some(return_set)
    } else {
        None
    }
}