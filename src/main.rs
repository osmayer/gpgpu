use std::fs::File;
use std::io::{self, BufRead, BufReader};
use clap::Parser;


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

    for line in reader.lines() {
        let line = line?; // Handle potential I/O errors for each line
        println!("{}", line);
    }

    Ok(())
}
