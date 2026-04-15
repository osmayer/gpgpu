#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MemRequestType {
    Read,
    Write,
    NoReq
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MemRequest {
    cycles_waited: u32,
    req_type: MemRequestType,
    valid: bool,
    ready: bool,
    mem_delay: u32
}


impl MemRequest {
    pub fn new (mem_delay: u32) -> MemRequest{
        let new_state=MemRequest {
            ready: false,
            cycles_waited: 0,
            req_type: MemRequestType::NoReq,
            valid: false,
            mem_delay: mem_delay
        };
        new_state
    }

    pub fn incr_cycles (& mut self) {
        if self.valid {
            self.cycles_waited += 1;
            // FIX THIS NUMBER TO CHANGE MEM DELAY
            if self.cycles_waited == self.mem_delay {
                self.ready = true;
            }
        }
    }

    pub fn check_if_ready (& self) -> bool {
        self.ready
    }

    pub fn check_if_valid (& self) -> bool {
        self.valid
    }

    pub fn read_request (& mut self) {
        self.valid = true;
        self.cycles_waited = 0;
        self.req_type = MemRequestType::Read;
    }

    pub fn reset_request (& mut self) {
        self.valid = false;
        self.ready = false;
        self.req_type = MemRequestType::NoReq;
    }

    pub fn write_request (& mut self) {
        self.req_type = MemRequestType::Write;
        self.valid = true;
        self.cycles_waited = 0;
    }
}