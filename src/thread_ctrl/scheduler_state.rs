
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SchedulerType {
    RoundRobin,
    Chaos
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SchedulerData {
    RoundRobin {
        curr_block: u32,
        curr_warp: u32
    },
    Chaos
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SchedulerState {
    pub scheduler_type: SchedulerType, 
    pub scheduler_data: SchedulerData
}

impl SchedulerState {
    pub fn new(scheduler_type: u32) -> Self {
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