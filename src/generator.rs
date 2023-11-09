use std::collections::BTreeSet;
use corosensei::CoroutineResult;
use corosensei::ScopedCoroutine;
use corosensei::Yielder;
use corosensei::stack::DefaultStack;
use serde::{Serialize, Deserialize};

use crate::word::*;
use crate::crossword::*;

#[derive(Serialize, Deserialize)]
pub struct CrosswordGeneratorSettings
{
    pub word_compatibility_settings: WordCompatibilitySettings,
    pub crossword_settings: CrosswordSettings
}

#[derive(Serialize, Deserialize)]
pub struct CrosswordGenerator
{
    pub words: BTreeSet<String>,
    pub settings: CrosswordGeneratorSettings,
}

impl CrosswordGenerator
{
    pub fn generate_crosswords(&self) -> BTreeSet<Crossword>
    {
        self.crossword_iter().collect::<BTreeSet<Crossword>>()
    }

    fn generate_crosswords_impl<'a>(&self, yielder: &Yielder<(), Crossword<'a>>, current_crossword: &mut Crossword<'a>, remained_words: &BTreeSet<&'a str>, crosswords: &mut BTreeSet<Crossword<'a>>, full_created_crossword_bases: &mut BTreeSet<Crossword<'a>>)
    {
        if !self.settings.crossword_settings.is_crossword_valid(&current_crossword) { return; }

        if remained_words.is_empty()
        {
            if crosswords.insert(current_crossword.clone())
            {
                yielder.suspend(current_crossword.clone());
            }
            return;
        }
        
        if full_created_crossword_bases.iter().any(|cw| current_crossword.contains_crossword(cw))
        {
            return;
        }
        
        for current_word in remained_words.iter()
        {
            let mut new_remained_words = remained_words.clone();
            new_remained_words.remove(current_word);
            for step in current_crossword.calculate_possible_ways_to_add_word(current_word, &self.settings.word_compatibility_settings).iter()
            {
                current_crossword.add_word(step);

                self.generate_crosswords_impl(yielder, current_crossword, &new_remained_words, crosswords, full_created_crossword_bases);

                let to_remove: Vec<Crossword<'a>> = full_created_crossword_bases.clone().into_iter().filter(|cw| cw.contains_crossword(&current_crossword)).collect();
                to_remove.into_iter().for_each(|cw| {full_created_crossword_bases.remove(&cw);});
                
                full_created_crossword_bases.insert(current_crossword.clone());

                current_crossword.remove_word(&step.value);
            }
        }
    }

    pub fn crossword_iter(&self) -> CrosswordIterator 
    {
        return CrosswordIterator
        {
            generating_coroutine: ScopedCoroutine::new(|yielder, _|
            {
                let mut crossword = Crossword::new(&[]);
                let mut crosswords = BTreeSet::new();
                let words = self.words.iter().map(|s| s.as_str()).collect::<BTreeSet<&str>>();

                let mut full_created_crossword_bases = BTreeSet::new();

                self.generate_crosswords_impl(yielder,&mut crossword, &words, &mut crosswords, &mut full_created_crossword_bases);

            })
        }
    }   
}

pub struct CrosswordIterator<'a>
{
    generating_coroutine: ScopedCoroutine<'a, (), Crossword<'a>, (), DefaultStack>,
}

impl<'a> Iterator for CrosswordIterator<'a>
{
    type Item = Crossword<'a>;
    fn next(&mut self) -> Option<Self::Item>
    {
        match self.generating_coroutine.resume(()) {
            CoroutineResult::Yield(crossword) => Some(crossword),
            CoroutineResult::Return(()) => None,
        }
    }
}
