use std::collections::BTreeSet;
use std::fs::File;
use std::io::{Write, Error};

mod crossword;
use crate::crossword::*;


fn main() -> Result<(), Error>
{
    
    let words = vec!
        [
            "Անապատ", 
            "Էնդեմիկ", 
            "Մայրցամաք", 
            "Քամի",
            "Ամազոն",
            "Օվկիանոս",
            "Սահարա"
        ].into_iter().map(|w| w.to_lowercase()).collect::<BTreeSet<String>>();

    let mut output = File::create("output.txt")?;

    let word_settings = WordCompatibilitySettings { side_by_side: false, head_by_head: false, side_by_head: false, corner_by_corner: true };
    let crossword_settings = CrosswordSettings { size_constraints: vec![CrosswordSizeConstrain::MaxArea(110)] };

    let cws = generate_crosswords(&words, &word_settings, &crossword_settings);
    for (ind, cw) in cws.iter().enumerate()
    {
        write!(output, "{}.\n{}\n\n\n", ind + 1, cw.generate_string())?;
    }

    println!("found {} crosswords", cws.len());

    Ok(())
    
    
}
