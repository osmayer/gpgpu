pub mod instr_parser;
pub mod program_state;
pub mod instr_execute;

use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use clap::Parser;
use object::{self, Endian, LittleEndian, Object, ObjectSection};

use crate::instr_parser::parse_instruction;
use crate::program_state::ProgramState;
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
    let mut instructions = Vec::new(); 
    let mut program_state = ProgramState::new(0);

    let bin_data = std::fs::read("memDist1_nocache.elf")?;
    let obj_file = object::File::parse(&*bin_data).expect("Skill issue");

    /* 
    // 2. Locate the .text section
    if let Some(text_section) = obj_file.section_by_name(".text") {
        let address = text_section.address();
        let data = text_section.data().expect("Skill isue");

        println!("Found .text at: {:#x}, size: {:#x}", address, data.len());

        // 3. Interpret bytes as instructions (considering LittleEndian)
        // RISC-V instructions are 32-bit aligned.
        for (i, chunk) in data.chunks_exact(4).enumerate() {
            println!("Instr {}: {:?}", i, chunk);
        }
    }
    */
    for line in reader.lines() {
        let line = line?; // Handle potential I/O errors for each line
        let str_line: &str = line.as_str();
        instructions.push(parse_instruction(str_line))
    }
    
    while !program_state.is_halted() {
        execute_instr(&instructions[program_state.get_instr_idx() as usize], &mut program_state);
    }

    println!("{}", program_state.to_string());

    Ok(())
}
