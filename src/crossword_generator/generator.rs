use std::collections::BTreeSet;
use std::iter;
use serde::{Serialize, Deserialize};

#[cfg(feature = "rec-iter")]
use corosensei::CoroutineResult;
#[cfg(feature = "rec-iter")]
use corosensei::ScopedCoroutine;
#[cfg(feature = "rec-iter")]
use corosensei::Yielder;
#[cfg(feature = "rec-iter")]
use corosensei::stack::DefaultStack;

use super::word::*;
use super::crossword::*;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default, Debug, Serialize, Deserialize)]
pub struct CrosswordGeneratorSettings
{
    pub word_compatibility_settings: WordCompatibilitySettings,
    pub crossword_settings: CrosswordSettings
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default, Debug, Serialize, Deserialize)]
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

    #[cfg(feature = "rec-iter")]
    fn crossword_iter_rec_impl<'a>(&self, yielder: &Yielder<(), Crossword<'a>>, current_crossword: &mut Crossword<'a>, remained_words: &BTreeSet<&'a str>, full_created_crossword_bases: &mut BTreeSet<Crossword<'a>>)
    {
        if !self.settings.crossword_settings.is_crossword_valid(&current_crossword) 
        {
            return; 
        }

        if full_created_crossword_bases.iter().any(|cw| current_crossword.contains_crossword(cw))
        {
            return;
        }
        
        if remained_words.is_empty()
        {
            yielder.suspend(current_crossword.clone());
            return;
        }
        for current_word in remained_words.iter()
        {
            let mut new_remained_words = remained_words.clone();
            new_remained_words.remove(current_word);
            for step in current_crossword.calculate_possible_ways_to_add_word(current_word, &self.settings.word_compatibility_settings).iter()
            {
                current_crossword.add_word(step);

                self.crossword_iter_rec_impl(yielder, current_crossword, &new_remained_words, full_created_crossword_bases);

                let to_remove: Vec<Crossword<'a>> = full_created_crossword_bases.clone().into_iter().filter(|cw| cw.contains_crossword(&current_crossword)).collect();
                to_remove.into_iter().for_each(|cw| {full_created_crossword_bases.remove(&cw);});
                
                full_created_crossword_bases.insert(current_crossword.clone());

                current_crossword.remove_word(&step.value);
            }
        }
    }

    #[cfg(feature = "rec-iter")]
    pub fn crossword_iter_rec(&self) -> CrosswordIteratorRecursive 
    {
        return CrosswordIteratorRecursive
        {
            generating_coroutine: ScopedCoroutine::new(|yielder, _|
            {
                let mut crossword = Crossword::new(&[]);
                let words = self.words.iter().map(|s| s.as_str()).collect::<BTreeSet<&str>>();

                let mut full_created_crossword_bases = BTreeSet::new();

                self.crossword_iter_rec_impl(yielder,&mut crossword, &words, &mut full_created_crossword_bases);

            })
        }
    } 


    pub fn crossword_iter(&self) -> CrosswordIterator
    {
        CrosswordIterator
        {
            settings: self.settings.clone(),
            current_crossword: Crossword::default(),
            full_created_crossword_bases: BTreeSet::new(),
            frame_stack: vec!
            [
                Frame
                {
                    remained_words: self.words.iter().map(|s| s.as_str()).collect(),
                    ..Frame::new()
                }
            ],
            started: false,
            ended: false
        }
    }
}

struct Frame<'a>
{
    remained_words: BTreeSet<&'a str>,
    new_remained_words: BTreeSet<&'a str>,
    current_word_iterator: Box<dyn Iterator<Item = &'a str> + 'a>,
    current_word: Option<&'a str>,
    current_step_iterator: Box<dyn Iterator<Item = Word<'a>> + 'a>,
    current_step: Option<Word<'a>>,
}

impl<'a> Frame<'a>
{
    fn new() -> Frame<'a>
    {
        Frame
        {
            remained_words: BTreeSet::new(),
            new_remained_words: BTreeSet::new(),
            current_word_iterator: Box::new(iter::empty()),
            current_word: None,
            current_step_iterator: Box::new(iter::empty()),
            current_step: None,
        }
    }
}

pub struct CrosswordIterator<'a>
{
    settings: CrosswordGeneratorSettings,
    current_crossword: Crossword<'a>,
    full_created_crossword_bases: BTreeSet<Crossword<'a>>,
    frame_stack: Vec<Frame<'a>>,
    started: bool,
    ended: bool,
}

impl<'a> CrosswordIterator<'a>
{
    fn current_frame(&mut self) -> &mut Frame<'a>
    {
        self.frame_stack.last_mut().expect("Frame stack must have at least one frame in it.")
    }
}

impl<'a> Iterator for CrosswordIterator<'a>
{
    type Item = Crossword<'a>;
    fn next(&mut self) -> Option<Self::Item>
    {
        if self.ended
        {
            return None;
        }

        if !self.started
        {
            self.started = true;
            self.current_frame().current_word_iterator = Box::new(self.current_frame().remained_words.clone().into_iter());
        }
        else
        {
            self.frame_stack.pop();
            
            let to_remove: Vec<Crossword<'a>> = self.full_created_crossword_bases.iter().filter_map(|cw| cw.contains_crossword(&self.current_crossword).then_some(cw.clone())).collect();
            to_remove.into_iter().for_each(|cw| {self.full_created_crossword_bases.remove(&cw);});
            
            self.full_created_crossword_bases.insert(self.current_crossword.clone());

            let step_to_remove = self.current_frame().current_step.as_ref().unwrap().value;
            self.current_crossword.remove_word(step_to_remove);

        }

        loop 
        {
            let not_none = loop
            {
                if self.current_frame().current_step != None
                {
                    break true;
                }
                self.current_frame().current_word = self.current_frame().current_word_iterator.next();
                if self.current_frame().current_word == None
                {
                    break false;
                }
                self.current_frame().new_remained_words = self.current_frame().remained_words.clone();
                let word_to_remove = self.current_frame().current_word.unwrap();
                self.current_frame().new_remained_words.remove(word_to_remove);
                
                let curr_word = self.current_frame().current_word.unwrap();
                self.current_frame().current_step_iterator = Box::new(self.current_crossword.calculate_possible_ways_to_add_word(curr_word, &self.settings.word_compatibility_settings).into_iter());
                self.current_frame().current_step = self.current_frame().current_step_iterator.next();
            };
            
            if !not_none
            {
                self.frame_stack.pop();
                if self.frame_stack.is_empty()
                {
                    self.ended = true;
                    return None;
                }
                else 
                {
                    let to_remove: Vec<Crossword<'a>> = self.full_created_crossword_bases.iter().filter_map(|cw| cw.contains_crossword(&self.current_crossword).then_some(cw.clone())).collect();
                    to_remove.into_iter().for_each(|cw| {self.full_created_crossword_bases.remove(&cw);});
                    
                    self.full_created_crossword_bases.insert(self.current_crossword.clone());
        
                    let step_to_remove = self.current_frame().current_step.as_ref().unwrap().value;
                    self.current_crossword.remove_word(step_to_remove);
        
                    self.current_frame().current_step = self.current_frame().current_step_iterator.next();
                    continue;    
                }
            }

            let curr_step = &self.current_frame().current_step.clone().unwrap();
            self.current_crossword.add_word(curr_step);

            let new_rem_words = self.current_frame().new_remained_words.clone();
            self.frame_stack.push(Frame
            {
                remained_words: new_rem_words,
                ..Frame::new()
            });

            if !self.settings.crossword_settings.is_crossword_valid(&self.current_crossword) { continue; }

            if self.full_created_crossword_bases.iter().any(|cw| self.current_crossword.contains_crossword(cw)) { continue; }

            if !self.current_frame().remained_words.is_empty() 
            {
                self.current_frame().current_word_iterator = Box::new(self.current_frame().remained_words.clone().into_iter());
                continue; 
            }

            return Some(self.current_crossword.clone());
        }
    }
}


#[cfg(feature = "rec-iter")]
pub struct CrosswordIteratorRecursive<'a>
{
    generating_coroutine: ScopedCoroutine<'a, (), Crossword<'a>, (), DefaultStack>,
}

#[cfg(feature = "rec-iter")]
impl<'a> Iterator for CrosswordIteratorRecursive<'a>
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



#[cfg(all(test, feature = "rec-iter"))]
mod tests {
    

    use super::*;

    #[test]
    fn test_run() {
        let mut generator = CrosswordGenerator::default();
        generator.settings = CrosswordGeneratorSettings::default();
        generator.settings.crossword_settings.size_constraints.push(CrosswordSizeConstrain::MaxLength(13));
        generator.settings.word_compatibility_settings.side_by_head = true;
        generator.words = vec!["Hello", "world", "asdf", "myname", "sesame", "yeeee"].into_iter().map(|s| s.to_lowercase()).collect();
        assert_eq!(generator.crossword_iter().count(), generator.crossword_iter_rec().count());
    }

}