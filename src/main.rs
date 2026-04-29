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
                            user_args.scheduler,
                            user_args.functional_units
                        );

    let num_blocks = user_args.num_blocks;
    let threads_per_warp = user_args.threads_per_warp;
    let warps_per_block = user_args.warps_per_block;
    let mut global_stalls = 0; 
    let mut warps_dispatched = 0; 
    let mut total_mem_wakes = 0; 
    let mut total_cycles_with_wake = 0; 
    
    loop {
        // scheduler does things, decides who's going
        let data = select_warp(& mut system_state);

        // for loop to run warps that have been decided
        match data {
            Some (set) => {
                for i in 0..set.len() {
                    let block = set[i as usize].0;
                    let warp = set[i as usize].1;
                    system_state.run_warp(block, warp); 
                }
                warps_dispatched += set.len();
            }
            None => {
                global_stalls+=1; 
            }
        }

        // update memory request state
        let mut did_have_wake = false; 
        for block in 0..num_blocks {
            for warp in 0..warps_per_block {
                for thread in 0..threads_per_warp {
                    let did_wake = system_state.incr_cycles(thread, warp, block);
                    if did_wake {
                        total_mem_wakes += 1; 
                        did_have_wake = true; 
                    }
                }
            }
        }
        
        if did_have_wake {
            total_cycles_with_wake += 1;
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
    }

    println!("{}", system_state);

    println!("===== Begin System Stats =====");
    println!("Total Cycles Elapsed: {}", system_state.get_total_cycle_count());
    println!("Cycles without any any instructions dispatched {}", global_stalls);
    println!("Warp occupancy rate: {} slots used / {} slots available", warps_dispatched, system_state.get_total_cycle_count() * user_args.functional_units);
    let issue_util: (u32, u32) = system_state.get_overall_thread_util();
    println!("Thread Occupancy Rate: {} slots used / {} slots available", issue_util.0, issue_util.1);
    println!("Total Memory Requests: {}", total_mem_wakes);
    println!("Total Cycles with Wake: {}", total_cycles_with_wake);
    println!("===== End System Stats =====");

    Ok(())
}
