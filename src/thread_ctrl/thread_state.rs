use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThreadState {
    pc:              u32,
    thread_id:       u32, 
    block_id:        u32,
    warp_id:         u32,
    registers:       [u32; 32],
    halted:          bool,
    waiting_for_mem: bool
}

const INITIAL_SP: u32 = 0x7ff00000;
const INITIAL_GP: u32 = 0x10000000; 

impl ThreadState {
    pub fn new(starting_pc: u32, thread_id: u32, warp_id:u32, block_id: u32) -> Self {
        // println!("Creating new thread with idx: {}", thread_id);
        let mut new_state= ThreadState { 
            pc:              starting_pc, 
            thread_id:       thread_id,
            block_id:        block_id,
            warp_id:         warp_id,
            registers:       [0; 32],
            halted:          false,
            waiting_for_mem: false
        };
        new_state.registers[2] = INITIAL_SP;
        new_state.registers[3] = INITIAL_GP;
        new_state
    }

    pub fn get_pc(&self) -> u32 {
        self.pc
    }

    pub fn set_pc(&mut self, new_pc: u32) {
        self.pc = new_pc;
    }

    pub fn advance_pc (&mut self) {
        self.pc += 4; 
    }

    pub fn read_register(&self, idx: u32) -> u32 {
        if idx > 31 {
            assert!(false, "Tried to read invalid index");
        }
        self.registers[idx as usize]
    }

    pub fn write_register(&mut self, idx: u32, new_val: u32) {
        if idx > 31 {
            assert!(false, "Tried to read invalid index");
        }
        
        if idx != 0 {
            self.registers[idx as usize] = new_val;
        }
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn set_waiting_for_mem(&mut self, new_val: bool) {
        self.waiting_for_mem = new_val;
    }

    pub fn get_waiting_for_mem(&self) -> bool {
        self.waiting_for_mem
    }

    pub fn get_thread_id(&self) -> u32 {
        self.thread_id
    }

    pub fn get_block_id(&self) -> u32 {
        self.block_id
    }

    pub fn get_warp_id(&self) -> u32 {
        self.warp_id
    }
}

impl fmt::Display for ThreadState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use the write! macro to define the string representation
        writeln!(f, "PC: {:#x}", self.get_pc())?;
        writeln!(f, "===== BEGIN REGISTER DUMP =====")?;
        for i in 0..32 {
            writeln!(f, "x{:<2} \t {:#010X} ({})", i, self.read_register(i), self.read_register(i) as i32)?;
        }
        writeln!(f, "===== END REGISTER DUMP =====")?;
        writeln!(f, "Halted: {}", self.is_halted())
    }

}