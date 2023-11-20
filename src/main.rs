use crossword_generator::crossword_generator::{generator::{CrosswordGenerator, CrosswordGeneratorSettings}, crossword::CrosswordSizeConstrain};


fn main()
{
    let mut generator = CrosswordGenerator::default();
    generator.settings = CrosswordGeneratorSettings::default();
    generator.settings.crossword_settings.size_constraints.push(CrosswordSizeConstrain::MaxLength(13));
    generator.settings.word_compatibility_settings.side_by_head = true;
    generator.words = vec!["Hello", "world", "asdf", "myname", "sesame", "yeeee"].into_iter().map(|s| s.to_lowercase()).collect();
    
}