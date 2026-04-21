pub mod instr_execute;
pub mod program_loader;
pub mod thread_ctrl; 
pub mod scheduler;

use std::io::{self};
use std::path::Path;
use clap::Parser;

use crate::scheduler::select_warp;


#[derive(Parser)]
struct Parameters {
    #[arg(short, long)]
    code_path: String,
    #[arg(short, long, default_value_t = 1)]
    threads_per_warp: u32,
    #[arg(short, long, default_value_t = 1)]
    num_blocks: u32,
    #[arg(short, long, default_value_t = 100)]
    memory_delay: u32,
    #[arg(short, long, default_value_t = 1)]
    warps_per_block: u32,
    #[arg(short, long, default_value_t = 1)]
    functional_units: u32,
    #[arg(short, long, default_value_t = 0)]
    scheduler: u32
}


fn main() -> io::Result<()> {
	println!("Hello, world!");
    let user_args = Parameters::parse(); 
  //  let file = File::open(user_args.code_path)?;
    // let reader = BufReader::new(file);

    let bin_data = std::fs::read(Path::new(&user_args.code_path))?;
    let obj_file = object::File::parse(&*bin_data).expect("Skill issue");

    
    let image = program_loader::file_to_image(&obj_file);
    let mut system_state = 
                        thread_ctrl::system_state::SystemState::new(&image, 
                            user_args.num_blocks, 
                            user_args.threads_per_warp, 
                            user_args.warps_per_block, 
                            user_args.memory_delay, 
                            4194304,
                            user_args.scheduler
                        );

    let num_blocks = user_args.num_blocks;
    let threads_per_warp = user_args.threads_per_warp;
    let warps_per_block = user_args.warps_per_block;
    
    loop {
        // scheduler does things, decides who's going
        let (block, warp) = select_warp(& mut system_state);

        // for loop to run warps that have been decided
        match block {
            Some (b) => {system_state.run_warp(b, warp); println!("{}, {}", b, warp);}
            None => {}
        }

        // update memory request state
        for block in 0..num_blocks {
            for warp in 0..warps_per_block {
                for thread in 0..threads_per_warp {
                    system_state.incr_cycles(thread, warp, block);
                }
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
