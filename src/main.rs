pub mod instr_execute;
pub mod program_loader;
pub mod thread_ctrl; 

use std::io::{self};
use std::path::Path;
use clap::Parser;

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
    memory_delay: u32,
    #[arg(short, long, default_value_t = 1)]
    threads_per_warp: u32
}


fn main() -> io::Result<()> {
	println!("Hello, world!");
    let user_args = Parameters::parse(); 
  //  let file = File::open(user_args.code_path)?;
    // let reader = BufReader::new(file);

    let bin_data = std::fs::read(Path::new(&user_args.code_path))?;
    let obj_file = object::File::parse(&*bin_data).expect("Skill issue");

    
    let image = program_loader::file_to_image(&obj_file);
    let mut system_state = thread_ctrl::system_state::SystemState::new(&image, user_args.num_blocks, user_args.num_blocks, user_args.threads_per_warp, user_args.memory_delay);

    let num_blocks = system_state.get_num_blocks();
    let threads_per_block = system_state.get_threads_per_block();
    
    loop {
        // scheduler does things, decides who's going
        
        // for loop to run threads that have been decided
        for block_idx in 0..num_blocks {
            let block = system_state.is_block_runnable(block_idx);
            let block_data; 
            match block {
                Some(b) => {
                    block_data = b;
                },
                None => {
                    continue;
                }
            }

            block_data.run_block(&mut system_state);
            
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
            if !system_state.is_block_halted(block) {
                halted = false;
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
