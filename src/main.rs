use std::fs::File;
use std::io::{Write, Error, BufReader};

mod generator;
mod word;
mod crossword;

use generator::*;

fn main() -> Result<(), Error>
{
    let generator: CrosswordGenerator = serde_json::from_reader(BufReader::new(File::open("input.json")?))?;

    let mut output = File::create("output.txt")?;

    let cws = generator.generate_crosswords();
    for (ind, cw) in cws.iter().enumerate()
    {
        write!(output, "{}.\n{}\n\n\n", ind + 1, cw.generate_string())?;
    }

    println!("found {} crosswords", cws.len());

    Ok(())
    
    
}
