
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SchedulerType {
    Chaos,
    RoundRobin,
    Lru
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SchedulerData {
    Chaos,
    RoundRobin {
        curr_block: u32,
        curr_warp: u32
    },
    Lru {
        history: Vec<(u32, u32)>
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SchedulerState {
    pub scheduler_type: SchedulerType, 
    pub scheduler_data: SchedulerData
}

impl SchedulerState {
    pub fn new(scheduler_type: u32, num_blocks:u32, warps_per_block: u32) -> Self {
        let mut new_state = SchedulerState {
            scheduler_type: SchedulerType::Chaos,
            scheduler_data: SchedulerData::Chaos
        };

        match scheduler_type {
            0 => {}
            1 => {
                new_state.scheduler_data = SchedulerData::RoundRobin{curr_block: 0, curr_warp: 0};
                new_state.scheduler_type = SchedulerType::RoundRobin;
            }
            2 => {
                let mut history = vec![];
                for i in 0..num_blocks {
                    for j in 0..warps_per_block {
                        history.push((i, j));
                    }
                }
                new_state.scheduler_data = SchedulerData::Lru { history: history }
            }
            _ => {
                println!("Unknown scheduler");
            }
        }
        new_state
    }

    pub fn set_scheduler_data (& mut self, data: SchedulerData) {
        self.scheduler_data = data;
    }

    pub fn get_scheduler_data (&self) -> SchedulerData {
        self.scheduler_data.clone()
    }
}