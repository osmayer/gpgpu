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
}


fn main() -> io::Result<()> {
	println!("Hello, world!");
    let user_args = Parameters::parse(); 
  //  let file = File::open(user_args.code_path)?;
    // let reader = BufReader::new(file);

    let bin_data = std::fs::read(Path::new(&user_args.code_path))?;
    let obj_file = object::File::parse(&*bin_data).expect("Skill issue");

    
    let image = program_loader::file_to_image(&obj_file);
    let mut system_state = program_state::SystemState::new(&image, 1);
    
    loop {
        if system_state.is_thread_halted(0) {
            break;
        } 
        let curr_instr = system_state.fetch_instr(0);
        match curr_instr.1 {
            Some(instr) => {
                execute_instr(instr, curr_instr.0, 0, &mut system_state);
            },
            _ => {
                panic!("Illegal PC Value!");
            }
        }
    }

    println!("{}", system_state.thread_states[0]);
    
    Ok(())
}
