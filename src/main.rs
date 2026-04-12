pub mod instr_parser;
pub mod program_state;
pub mod instr_execute;
pub mod program_loader;

use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use clap::Parser;
use object::{self, Endian, LittleEndian, Object, ObjectSection};

use crate::instr_parser::parse_instruction;
use crate::instr_execute::execute_instr;


#[derive(Parser)]
struct Parameters {
    #[arg(short, long)]
    code_path: String,
}


fn main() -> io::Result<()> {
	println!("Hello, world!");
    let user_args = Parameters::parse(); 
    let file = File::open(user_args.code_path)?;
    let reader = BufReader::new(file);

    let bin_data = std::fs::read("memDist1_nocache.elf")?;
    let obj_file = object::File::parse(&*bin_data).expect("Skill issue");

    
    let mut image = program_loader::file_to_image(&obj_file);
    println!("{:?}", program_state::SystemState::new(&image, 1));
    
    Ok(())
}
