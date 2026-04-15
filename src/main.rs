pub mod program_state;
pub mod instr_execute;
pub mod program_loader;

use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use clap::Parser;
use object::{self, Endian, LittleEndian, Object, ObjectSection};

use crate::instr_execute::execute_instr;


#[derive(Parser)]
struct Parameters {
    #[arg(short, long)]
    code_path: String,
    #[arg(short, long, default_value_t = 1)]
    threads_per_block: u32,
    #[arg(short, long, default_value_t = 1)]
    num_blocks: u32,
    #[arg(short, long, default_value_t = 100)]
    memory_delay: u32
}

fn thread_execute_instr (thread_idx: u32, block_idx: u32, system_state: &mut program_state::SystemState ) {
    if system_state.is_thread_halted(thread_idx, block_idx) {
        return;
    }
    let curr_instr = system_state.fetch_instr(thread_idx, block_idx);
    match curr_instr.1 {
        Some(instr) => {
            println!("pc: {:x} instr: {:?}", curr_instr.0, instr);
            execute_instr(instr, curr_instr.0, thread_idx, block_idx, system_state);
        },
        _ => {     
            panic!("Illegal PC Value!");
        }
    }
}


fn main() -> io::Result<()> {
	println!("Hello, world!");
    let user_args = Parameters::parse(); 
  //  let file = File::open(user_args.code_path)?;
    // let reader = BufReader::new(file);

    let bin_data = std::fs::read(Path::new(&user_args.code_path))?;
    let obj_file = object::File::parse(&*bin_data).expect("Skill issue");

    
    let image = program_loader::file_to_image(&obj_file);
    let mut system_state = program_state::SystemState::new(&image, user_args.threads_per_block, user_args.num_blocks, user_args.memory_delay);

    let num_blocks = system_state.get_num_blocks();
    let threads_per_block = system_state.get_threads_per_block();
    
    loop {
        // scheduler does things, decides who's going
        
        // for loop to run threads that have been decided
        for block in 0..num_blocks {
            for thread in 0..threads_per_block {
                thread_execute_instr(thread, block, &mut system_state);
            }
        }

        // update memory request state
        for block in 0..num_blocks {
            for thread in 0..threads_per_block {
                system_state.incr_cycles(thread, block);
            }
        }

        // check if entire program is done
        let mut halted = true;
        for block in 0..num_blocks {
            for thread in 0..threads_per_block {
                if !system_state.is_thread_halted(thread, block) {
                    halted = false;
                } 
            }
        }

        system_state.update_total_cycle_count();
        
        if halted {
            break;
        }

        // println!("Per Cycle Register Trace:");
        // println!("{}", system_state.thread_states[0]);
    }

    println!("{}", system_state);

    Ok(())
}
