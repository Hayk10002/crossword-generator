use std::fs::File;
use std::io::{Write, Error, BufReader};

mod generator;
mod word;
mod crossword;

use generator::*;
use itertools::Itertools;

fn main() -> Result<(), Error>
{
    let generator: CrosswordGenerator = serde_json::from_reader(BufReader::new(File::open("input.json")?))?;

    let mut output = File::create("output.txt")?;

    let cw_iter = generator.crossword_iter();
    for (ind, cw) in cw_iter.sorted_by_key(|a| 
        { 
            let (x, y) = a.get_size();
            x * y
        }).enumerate()
    {
        write!(output, "{}.\n{}\n\n\n", ind + 1, cw.generate_string())?;
    }

    Ok(())
    
    
}
