use crossword_generator::{generator::{CrosswordGenerator, CrosswordGeneratorSettings}, crossword::CrosswordSizeConstraint};


fn main()
{
    let mut generator = CrosswordGenerator::default();
    generator.settings = CrosswordGeneratorSettings::default();
    generator.settings.crossword_settings.size_constraints.push(CrosswordSizeConstraint::MaxLength(13));
    generator.settings.word_compatibility_settings.side_by_head = true;
    generator.words = vec!["Hello", "world", "asdf", "myname", "sesame", "yeeee"].into_iter().map(|s| s.to_lowercase()).collect();
    
}