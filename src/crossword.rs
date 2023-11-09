use std::collections::BTreeSet;

use serde::{Serialize, Deserialize};

use super::word::*;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default, Debug, Serialize, Deserialize)]
pub enum CrosswordSizeConstrain
{
    MaxLength(usize),
    MaxHeight(usize),
    MaxArea(usize),
    #[default]
    None
}

impl CrosswordSizeConstrain 
{
    pub fn is_crossword_valid(&self, cw: &Crossword) -> bool
    {
        let size = cw.get_size();
        match *self
        {
            CrosswordSizeConstrain::MaxLength(l) => size.0 <= l,
            CrosswordSizeConstrain::MaxHeight(h) => size.1 <= h,
            CrosswordSizeConstrain::MaxArea(a) => size.0 * size.1 <= a,
            CrosswordSizeConstrain::None => true
        }
    }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default, Debug, Serialize, Deserialize)]
pub struct CrosswordSettings
{
    pub size_constraints: Vec<CrosswordSizeConstrain>
}

impl CrosswordSettings
{
    pub fn is_crossword_valid(&self, cw: &Crossword) -> bool
    {
        return self.size_constraints.iter().all(|c| c.is_crossword_valid(cw))
    }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default, Debug)]
pub struct Crossword<'a>
{
    words: BTreeSet<Word<'a>>,
}

impl<'a> Crossword<'a>
{
    pub fn new(words: &[Word<'a>]) -> Crossword<'a>
    {
        let mut cw = Crossword { words: words.iter().map(|x| x.clone()).collect(), ..Default::default() };
        cw.normalize();

        return cw;
    }
    pub fn normalize(&mut self)
    {
        let mut min_corner = (isize::MAX, isize::MAX);
        let mut new_set = BTreeSet::new();

        for word in self.words.iter()
        {
            min_corner.0 = min_corner.0.min(word.position.x);
            min_corner.1 = min_corner.1.min(word.position.y);
        }

        for word in self.words.iter()
        {
            new_set.insert(Word{ position: WordPosition { x: word.position.x - min_corner.0, y: word.position.y - min_corner.1}, ..word.clone() });
        }

        self.words = new_set;
    }

    pub fn add_word(&mut self, word: &Word<'a>)
    {
        if self.words.iter().find(|w| w.value == word.value).is_some() { return; }
        self.words.insert(word.clone());
        self.normalize();
    }

    pub fn remove_word(&mut self, word: &str)
    {
        let mut word_to_remove = Word::default();

        self.words.iter().for_each(|w| if w.value == word { word_to_remove = w.clone() });
        self.words.remove(&word_to_remove);

        self.normalize();
    }

    pub fn find_word(&self, word: &str) -> Option<&Word>
    {
        self.words.iter().filter(|w| w.value == word).next()
    }
    

    pub fn contains_crossword(&self, other: &Crossword) -> bool 
    {
        if other.words.len() > self.words.len() { return false; }
        let mut offset: Option<(isize, isize)> = None;
        for other_word in other.words.iter()
        {
            let cur_word = self.find_word(&other_word.value);
            if let None = cur_word
            {
                return false;
            }
            let cur_word = cur_word.unwrap();
            if cur_word.direction != other_word.direction
            {
                return false;
            }

            match &offset
            {
                None => offset = Some((cur_word.position.x - other_word.position.x, cur_word.position.y - other_word.position.y)),
                Some(offset) => 
                {
                    if *offset != (cur_word.position.x - other_word.position.x, cur_word.position.y - other_word.position.y)
                    {
                        return false;
                    }
                }
            }

        }
        true
    }

    pub fn calculate_possible_ways_to_add_word(&self, word: &'a str, word_compatibility_settings: &WordCompatibilitySettings) -> BTreeSet<Word<'a>>
    {
        if self.words.is_empty()
        {
            dbg!(word);
            return vec![Word{ value: word, ..Word::default()}].into_iter().collect()
        }

        self.words.iter()
            .flat_map(|cur_word: &Word<'a>| cur_word.calculate_possible_ways_to_add_word(word))
            .filter(|w: &Word<'a>| self.can_word_be_added(w, word_compatibility_settings))
            .collect()
    }

    pub fn can_word_be_added(&self, word: &Word<'a>, word_compatibility_settings: &WordCompatibilitySettings) -> bool
    {
        self.words.iter().all(|w: &Word<'a>| word_compatibility_settings.are_words_compatible(w, word))
    }
    
    pub fn get_size(&self) -> (usize, usize)
    {
        let mut max_corner = (0isize, 0isize);
    
        for word in self.words.iter()
        {
            max_corner.0 = max_corner.0.max(word.position.x + 1);
            max_corner.1 = max_corner.1.max(word.position.y + 1);
            match word.direction
            {
                WordDirection::Right => max_corner.0 = max_corner.0.max(word.position.x + word.value.chars().count() as isize),
                WordDirection::Down => max_corner.1 = max_corner.1.max(word.position.y + word.value.chars().count() as isize), 
            }
        }
    
        (max_corner.0 as usize, max_corner.1 as usize)
    }
    
    pub fn generate_char_table(&self) ->Vec<Vec<char>>
    {
        let size = self.get_size();
        let mut table = vec![vec![' '; size.0]; size.1];
        for word in self.words.iter()
        {
            for (index, char) in word.value.chars().enumerate()
            {
                match word.direction
                {
                    WordDirection::Right => table[word.position.y as usize][word.position.x as usize + index] = char,
                    WordDirection::Down => table[word.position.y as usize + index][word.position.x as usize] = char,
                }
            }
        }
    
        table
    }
    
    pub fn generate_string(&self) -> String
    {
        let table = self.generate_char_table();
        let size = table[0].len() * 2 + 1;
        let result: String = vec![vec!['-'; size], vec!['\n']].concat().into_iter().chain(table
            .into_iter()
            .map(|mut el| 
            {
                el = el.into_iter().flat_map(|ch| [ch, ' ']).collect();
                el.insert(0, '|');
                el.pop();
                el.push('|');
                el.push('\n');
                el
            })
            .flatten()).chain(vec![vec!['-'; size], vec!['\n']].concat().into_iter())
            .collect();
    
    
        result
    }
}



#[cfg(test)]
mod tests {
    

    use super::*;

    #[test]
    fn test_crossword_contains_crossword() {
        let cw = Crossword::new(
            &[
                Word{position: WordPosition { x: -1, y: -1 }, direction: WordDirection::Right, value: "hello"},
                Word{position: WordPosition { x: 1, y: -1 }, direction: WordDirection::Down, value: "local"},
                Word{position: WordPosition { x: 1, y: 1 }, direction: WordDirection::Right, value: "cat"},
                Word{position: WordPosition { x: 2, y: 1 }, direction: WordDirection::Down, value: "and"},
                Word{position: WordPosition { x: 3, y: 1 }, direction: WordDirection::Down, value: "toy"},

            ]);

        let mut containing_crossword_1 = Crossword::new(
            &[
                Word{position: WordPosition { x: -1, y: -1 }, direction: WordDirection::Right, value: "hello"},
                Word{position: WordPosition { x: 1, y: -1 }, direction: WordDirection::Down, value: "local"},
                Word{position: WordPosition { x: 1, y: 1 }, direction: WordDirection::Right, value: "cat"},
                Word{position: WordPosition { x: 2, y: 1 }, direction: WordDirection::Down, value: "and"},
                Word{position: WordPosition { x: 3, y: 1 }, direction: WordDirection::Down, value: "toy"},

            ]);

        let mut containing_crossword_2 = Crossword::new(
            &[
                Word{position: WordPosition { x: 2, y: 1 }, direction: WordDirection::Right, value: "cat"},
                Word{position: WordPosition { x: 3, y: 1 }, direction: WordDirection::Down, value: "and"},
                Word{position: WordPosition { x: 4, y: 1 }, direction: WordDirection::Down, value: "toy"},

            ]);

        let mut containing_crossword_3 = Crossword::new(
            &[
                Word{position: WordPosition { x: 2, y: 2 }, direction: WordDirection::Down, value: "and"},
                Word{position: WordPosition { x: 3, y: 1 }, direction: WordDirection::Down, value: "toy"},

            ]);

        containing_crossword_1.normalize();    
        containing_crossword_2.normalize();    
        containing_crossword_3.normalize();    

        assert_eq!([cw.contains_crossword(&containing_crossword_1), cw.contains_crossword(&containing_crossword_2), cw.contains_crossword(&containing_crossword_3)], [true, true, false]);
    }

    #[test]
    fn test_crossword_generate_string() {
        let cw = Crossword::new(
            &[
                Word{position: WordPosition { x: -1, y: -1 }, direction: WordDirection::Right, value: "hello"},
                Word{position: WordPosition { x: 1, y: -1 }, direction: WordDirection::Down, value: "local"},
                Word{position: WordPosition { x: 1, y: 1 }, direction: WordDirection::Right, value: "cat"},
                Word{position: WordPosition { x: 2, y: 1 }, direction: WordDirection::Down, value: "and"},
                Word{position: WordPosition { x: 3, y: 1 }, direction: WordDirection::Down, value: "toy"},
    
            ]);


        assert_eq!(cw.generate_string(), 
        "\
-----------
|h e l l o|
|    o    |
|    c a t|
|    a n o|
|    l d y|
-----------\n")
    }

    #[test]
    fn test_crossword_normalize() {
        let mut cw = Crossword::new(
            &[
                Word{position: WordPosition { x: -1, y: -1 }, direction: WordDirection::Right, value: "hello"},
                Word{position: WordPosition { x: 1, y: -1 }, direction: WordDirection::Down, value: "local"},
                Word{position: WordPosition { x: 1, y: 1 }, direction: WordDirection::Right, value: "cat"},
                Word{position: WordPosition { x: 2, y: 1 }, direction: WordDirection::Down, value: "and"},
                Word{position: WordPosition { x: 3, y: 1 }, direction: WordDirection::Down, value: "toy"},
    
            ]);
        
        cw.normalize();

        let cw_normalized = Crossword::new(
            &[
                Word{position: WordPosition { x: 0, y: 0 }, direction: WordDirection::Right, value: "hello"},
                Word{position: WordPosition { x: 2, y: 0 }, direction: WordDirection::Down, value: "local"},
                Word{position: WordPosition { x: 2, y: 2 }, direction: WordDirection::Right, value: "cat"},
                Word{position: WordPosition { x: 3, y: 2 }, direction: WordDirection::Down, value: "and"},
                Word{position: WordPosition { x: 4, y: 2 }, direction: WordDirection::Down, value: "toy"},

            ]);

        assert_eq!(cw, cw_normalized);
    }

    #[test]
    fn test_crossword_remove_word() {
        let mut cw = Crossword::new(
            &[
                Word{position: WordPosition { x: -1, y: -1 }, direction: WordDirection::Right, value: "hello"},
                Word{position: WordPosition { x: 1, y: -1 }, direction: WordDirection::Down, value: "local"},
                Word{position: WordPosition { x: 1, y: 1 }, direction: WordDirection::Right, value: "cat"},
                Word{position: WordPosition { x: 2, y: 1 }, direction: WordDirection::Down, value: "and"},
                Word{position: WordPosition { x: 3, y: 1 }, direction: WordDirection::Down, value: "toy"},
    
            ]);
        
        cw.remove_word("toy");

        let cw_word_removed = Crossword::new(
            &[
                Word{position: WordPosition { x: 0, y: 0 }, direction: WordDirection::Right, value: "hello"},
                Word{position: WordPosition { x: 2, y: 0 }, direction: WordDirection::Down, value: "local"},
                Word{position: WordPosition { x: 2, y: 2 }, direction: WordDirection::Right, value: "cat"},
                Word{position: WordPosition { x: 3, y: 2 }, direction: WordDirection::Down, value: "and"},

            ]);

        assert_eq!(cw, cw_word_removed);
    }

    #[test]
    fn test_crossword_calculate_possible_ways_to_add_word() {
        let cw = Crossword::new(
            &[
                Word{position: WordPosition { x: 0, y: 0 }, direction: WordDirection::Right, value: "hello"},
                Word{position: WordPosition { x: 2, y: 0 }, direction: WordDirection::Down, value: "local"},
                Word{position: WordPosition { x: 0, y: 2 }, direction: WordDirection::Right, value: "tac"}
            ]);

        let new_word = "hatlo";

        assert_eq!(cw.calculate_possible_ways_to_add_word(&new_word, &WordCompatibilitySettings::default()), vec![
            Word{position: WordPosition { x: 0, y: 0 }, direction: WordDirection::Down, value: new_word.clone()},
            //Word{position: WordPosition { x: 1, y: 1 }, direction: WordDirection::Down, value: new_word.clone()},  |-
            //Word{position: WordPosition { x: 1, y: 3 }, direction: WordDirection::Right, value: new_word.clone()}, ||
            //Word{position: WordPosition { x: 3, y: -3 }, direction: WordDirection::Down, value: new_word.clone()}, ||
            Word{position: WordPosition { x: -1, y: 4 }, direction: WordDirection::Right, value: new_word.clone()},
            //Word{position: WordPosition { x: -2, y: 1 }, direction: WordDirection::Right, value: new_word.clone()},||
            Word{position: WordPosition { x: 4, y: -4 }, direction: WordDirection::Down, value: new_word.clone()},
            ].into_iter().collect());

        // assert_eq!(cw.generate_string(), 
        // "\
        // ---------------------\n\
        // | h | e | l | l | o |\n\
        // ---------------------\n\
        // |   |   | o |   |   |\n\
        // ---------------------\n\
        // | t | a | c |   |   |\n\
        // ---------------------\n\
        // |   |   | a |   |   |\n\
        // ---------------------\n\
        // |   |   | l |   |   |\n\
        // ---------------------\n".to_owned())
    }


}
