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
    num_blocks: u32
}


fn main() -> io::Result<()> {
	println!("Hello, world!");
    let user_args = Parameters::parse(); 
  //  let file = File::open(user_args.code_path)?;
    // let reader = BufReader::new(file);

    let bin_data = std::fs::read(Path::new(&user_args.code_path))?;
    let obj_file = object::File::parse(&*bin_data).expect("Skill issue");

    
    let image = program_loader::file_to_image(&obj_file);
    let mut system_state = program_state::SystemState::new(&image, user_args.threads_per_block, user_args.num_blocks);
    
    loop {
        // scheduler does things, decides who's going

        // for loop to run threads that have been decided

        // update memory queue

        // check if entire program is done
        let mut halted = false;
        for i in 0..system_state.num_blocks {
            for j in 0..system_state.threads_per_block {
                if system_state.is_thread_halted(i, j) {
                    halted = true;
                } 
            }
        }
        
        let curr_instr = system_state.fetch_instr(0, 0);
        match curr_instr.1 {
            Some(instr) => {
                println!("pc: {:x} instr: {:?}", curr_instr.0, instr);
                execute_instr(instr, curr_instr.0, 0,0, &mut system_state);
            },
            _ => {     
                panic!("Illegal PC Value!");
            }
        }
        
        if halted {
            break;
        }

        // println!("Per Cycle Register Trace:");
        // println!("{}", system_state.thread_states[0]);
    }

    println!("{}", system_state.thread_states[0][0]);

    
    Ok(())
}
