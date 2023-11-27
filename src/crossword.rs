use std::collections::BTreeSet;

use serde::{Serialize, Deserialize};

use super::word::*;


/// Represents a size constraint on a [crossword](Crossword)
/// ```text
/// //MaxArea(46)        MaxLength(7) 
/// // satisfied         unsatisfied
/// //                
/// //                        8
/// //                 < - - - - - - >
/// //                 ---------------
/// //MaxHeight(6)  ^ |h e l l o      |
/// // satisfied    | |      i        |
/// //            6 | |      k        |
/// //              | |      e n t e r|
/// //              | |      l        |
/// //              v |      y        |
/// //                 ---------------
/// ```
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default, Debug, Serialize, Deserialize)]
pub enum CrosswordSizeConstraint
{
    MaxLength(usize),
    MaxHeight(usize),
    MaxArea(usize),
    #[default]
    None
}

impl CrosswordSizeConstraint 
{
    /// Checks if the [crossword](Crossword) satisfies the [constraint](CrosswordSizeConstraint)
    pub fn is_crossword_valid(&self, cw: &Crossword) -> bool
    {
        let size = cw.get_size();
        match *self
        {
            CrosswordSizeConstraint::MaxLength(l) => size.0 <= l,
            CrosswordSizeConstraint::MaxHeight(h) => size.1 <= h,
            CrosswordSizeConstraint::MaxArea(a) => size.0 * size.1 <= a,
            CrosswordSizeConstraint::None => true
        }
    }
}

/// Represents all settigns for a [crossword](Crossword)
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default, Debug, Serialize, Deserialize)]
pub struct CrosswordSettings
{
    pub size_constraints: Vec<CrosswordSizeConstraint>
}

impl CrosswordSettings
{
    /// Checks if the [crossword](Crossword) satisfies the [constraints](CrosswordSizeConstraint)
    pub fn is_crossword_valid(&self, cw: &Crossword) -> bool
    {
        return self.size_constraints.iter().all(|c| c.is_crossword_valid(cw))
    }
}

/// # Represents a crossword
/// 
/// A crossword can't have two [words](Word) with the same string value in it.
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default, Debug, Serialize, Deserialize)]
pub struct Crossword<'a>
{
    #[serde(borrow)]
    words: BTreeSet<Word<'a>>,
}

impl<'a> Crossword<'a>
{
    /// Creates a new [crossword](Crossword) with the given [words](Word) and [normalizes](Crossword::normalize) it
    /// 
    /// ## Warning
    /// Note that this function does not check if the specified words are for example intersecting correctly.
    /// You can put words cat and dog on each other, even though they don't even have a common letter.
    /// For checking if a word can be added to the crossword correctly, you need to use the function [can_word_be_added](Crossword::can_word_be_added) to check, and [add_word](Crossword::add_word) to add a single word.
    pub fn new(words: &[Word<'a>]) -> Crossword<'a>
    {
        let mut cw = Crossword { words: words.iter().map(|x| x.clone()).collect(), ..Default::default() };
        cw.normalize();

        return cw;
    }

    /// Moves [words'](Word) coordinates so that the minimum x and y coordinates are zeros
    /// 
    /// ## Example
    /// 
    /// ```
    /// # use crossword_generator::word::{Word, WordDirection, WordPosition};
    /// # use crossword_generator::crossword::Crossword;                                                      
    ///                                                                                                      //       0
    ///                                                                                                      //       |
    /// let mut cw = Crossword::new(&[                                                                       //     ---------
    ///     Word{position: WordPosition { x: -1, y: -1 }, direction: WordDirection::Right, value: "hello"},  //    |h e l l o|
    ///     Word{position: WordPosition { x: 1, y: -1 }, direction: WordDirection::Down, value: "local"},    //0 - |    o    |
    ///     Word{position: WordPosition { x: 1, y: 1 }, direction: WordDirection::Right, value: "cat"},      //    |    c a t|
    ///     Word{position: WordPosition { x: 2, y: 1 }, direction: WordDirection::Down, value: "and"},       //    |    a n o|
    ///     Word{position: WordPosition { x: 3, y: 1 }, direction: WordDirection::Down, value: "toy"},       //    |    l d y|
    /// ]);                                                                                                  //     ---------
    /// cw.normalize();                                                                                         
    ///                                                                                                      //     0
    ///                                                                                                      //     | 
    /// let cw_normalized = Crossword::new(&[                                                                //     ---------                 
    ///     Word{position: WordPosition { x: 0, y: 0 }, direction: WordDirection::Right, value: "hello"},    //0 - |h e l l o|
    ///     Word{position: WordPosition { x: 2, y: 0 }, direction: WordDirection::Down, value: "local"},     //    |    o    |
    ///     Word{position: WordPosition { x: 2, y: 2 }, direction: WordDirection::Right, value: "cat"},      //    |    c a t|
    ///     Word{position: WordPosition { x: 3, y: 2 }, direction: WordDirection::Down, value: "and"},       //    |    a n o|
    ///     Word{position: WordPosition { x: 4, y: 2 }, direction: WordDirection::Down, value: "toy"},       //    |    l d y|
    /// ]);                                                                                                  //     ---------
    ///     
    /// assert_eq!(cw, cw_normalized);
    /// ```
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

    /// Adds the [word](Word) to the [crossword](Crossword) if not finded any other word with same string value and [normalizes](Crossword::normalize) it
    /// 
    /// ## Warning
    /// Note that this function does not check if the specified word can be added correctly.
    /// You can put word cat on the word dog, even though they don't even have a common letter.
    /// For checking if a word can be added to the crossword correctly, you need to use the function [can_word_be_added](Crossword::can_word_be_added).
    pub fn add_word(&mut self, word: &Word<'a>)
    {
        if self.words.iter().find(|w| w.value == word.value).is_some() { return; }
        self.words.insert(word.clone());
        self.normalize();
    }

    /// Removes the [word](Word) from the [crossword](Crossword) if finded and [normalizes](Crossword::normalize) it
    pub fn remove_word(&mut self, word: &str)
    {
        if let Some(word) = self.find_word(word).and_then(|w: &Word<'a>| Some(w.clone()))
        {
            self.words.remove(&word);

            self.normalize();
        }
    }

    /// Finds the [word](Word) given its string value.
    pub fn find_word(&self, word: &str) -> Option<&Word<'a>>
    {
        self.words.iter().filter(|w| w.value == word).next()
    }
    
    /// Checks if another [crossword](Crossword) is found inside this crossword.
    /// 
    /// ## Example
    /// 
    /// ```
    /// # use crossword_generator::word::{Word, WordDirection, WordPosition};
    /// # use crossword_generator::crossword::Crossword;                                                      
    /// let mut cw1 = Crossword::new(&[                                                                      //     ---------
    ///     Word{position: WordPosition { x: -1, y: -1 }, direction: WordDirection::Right, value: "hello"},  //    |h e l l o|
    ///     Word{position: WordPosition { x: 1, y: -1 }, direction: WordDirection::Down, value: "local"},    //    |    o    |
    ///     Word{position: WordPosition { x: 1, y: 1 }, direction: WordDirection::Right, value: "cat"},      //    |    c a t|
    ///     Word{position: WordPosition { x: 2, y: 1 }, direction: WordDirection::Down, value: "and"},       //    |    a n o|
    ///     Word{position: WordPosition { x: 3, y: 1 }, direction: WordDirection::Down, value: "toy"},       //    |    l d y|
    /// ]);                                                                                                  //     ---------
    ///                                                                                         
    ///
    /// let cw2 = Crossword::new(&[                                                                          //     -----                 
    ///     Word{position: WordPosition { x: 0, y: 0 }, direction: WordDirection::Right, value: "cat"},      //    |c a t|
    ///     Word{position: WordPosition { x: 1, y: 0 }, direction: WordDirection::Down, value: "and"},       //    |  n o|
    ///     Word{position: WordPosition { x: 2, y: 0 }, direction: WordDirection::Down, value: "toy"},       //    |  d y|
    /// ]);                                                                                                  //     -----
    ///     
    /// assert!(cw1.contains_crossword(&cw2));
    /// ```
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

    /// Returns all possible ways (given some [settings](WordCompatibilitySettings)) to add a [word](Word) into the [crossword](Crossword)
    /// 
    /// ## Example
    /// 
    /// ```
    /// # use crossword_generator::word::{Word, WordDirection, WordPosition, WordCompatibilitySettings};
    /// # use crossword_generator::crossword::Crossword;         
    /// # use std::collections::BTreeSet;                                             
    /// let mut cw = Crossword::new(&[                                                                      //     ---------
    ///     Word{position: WordPosition { x: 0, y: 0 }, direction: WordDirection::Right, value: "hello"},   //    |h e l l o|
    ///     Word{position: WordPosition { x: 2, y: 0 }, direction: WordDirection::Down, value: "local"},    //    |    o    |
    /// ]);                                                                                                 //    |    c    |
    ///                                                                                                     //    |    a    |
    ///                                                                                                     //    |    l    |
    ///                                                                                                     //     ---------
    ///                                                                                             
    /// assert_eq!(cw.calculate_possible_ways_to_add_word("halo", &WordCompatibilitySettings::default()), 
    ///             BTreeSet::from([
    ///     Word{position: WordPosition { x: 0, y: 0 }, direction: WordDirection::Down, value: "halo"},
    ///     Word{position: WordPosition { x: 4, y: -3 }, direction: WordDirection::Down, value: "halo"},
    ///     Word{position: WordPosition { x: 0, y: 4 }, direction: WordDirection::Right, value: "halo"},
    ///     Word{position: WordPosition { x: 1, y: 3 }, direction: WordDirection::Right, value: "halo"},
    /// ]));
    /// ```
    /// 
    /// 
    /// 
    /// Note that for example word halo on position 3 -2 and direction down is not allowed by a setting in word compatibility settings that forbids two words with same direction to be side to side
    pub fn calculate_possible_ways_to_add_word(&self, word: &'a str, word_compatibility_settings: &WordCompatibilitySettings) -> BTreeSet<Word<'a>>
    {
        if self.words.is_empty()
        {
            return vec![Word{ value: word, ..Word::default()}].into_iter().collect()
        }

        self.words.iter()
            .flat_map(|cur_word: &Word<'a>| cur_word.calculate_possible_ways_to_add_word(word))
            .filter(|w: &Word<'a>| self.can_word_be_added(w, word_compatibility_settings))
            .collect()
    }

    /// Checks if a [word](Word) can be added to the [crossword](Crossword) given the [word compatibility settings](WordCompatibilitySettings)
    /// 
    /// ## Example
    /// 
    /// ```
    /// # use crossword_generator::word::{Word, WordDirection, WordPosition, WordCompatibilitySettings};
    /// # use crossword_generator::crossword::Crossword;                                         
    /// let mut cw = Crossword::new(&[                                                                      //     ---------
    ///     Word{position: WordPosition { x: 0, y: 0 }, direction: WordDirection::Right, value: "hello"},   //    |h e l l o|
    ///     Word{position: WordPosition { x: 2, y: 0 }, direction: WordDirection::Down, value: "local"},    //    |    o    |
    /// ]);                                                                                                 //    |    c    |
    ///                                                                                                     //    |    a    |
    ///                                                                                                     //    |    l    |
    ///                                                                                                     //     ---------
    ///                                                                                             
    /// assert!(cw.can_word_be_added(&Word{position: WordPosition { x: 0, y: 0 }, direction: WordDirection::Down, value: "halo"}, &WordCompatibilitySettings::default()));
    /// ```
    /// 
    /// Note that for example word halo on position 3 -2 and direction down is not allowed by a setting in word compatibility settings that forbids two words with same direction to be side to side
    pub fn can_word_be_added(&self, word: &Word<'a>, word_compatibility_settings: &WordCompatibilitySettings) -> bool
    {
        self.words.iter().all(|w: &Word<'a>| word_compatibility_settings.are_words_compatible(w, word))
    }
    
    /// Returns the size of the minimum rectangle that can contain the [crossword](Crossword)
    /// 
    /// ## Example
    /// 
    /// ```
    /// # use crossword_generator::word::{Word, WordDirection, WordPosition};
    /// # use crossword_generator::crossword::Crossword;                                         
    /// let mut cw = Crossword::new(&[                                                                      //     ---------
    ///     Word{position: WordPosition { x: 0, y: 0 }, direction: WordDirection::Right, value: "hello"},   //    |h e l l o|
    ///     Word{position: WordPosition { x: 2, y: 0 }, direction: WordDirection::Down, value: "local"},    //    |    o    |
    /// ]);                                                                                                 //    |    c    |
    ///                                                                                                     //    |    a    |
    ///                                                                                                     //    |    l    |
    ///                                                                                                     //     ---------
    ///                                                                                             
    /// assert_eq!(cw.get_size(), (5, 5));
    /// ```
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
    

    /// Returns a matrix of characters that represent the [crossword](Crossword)
    /// 
    /// ## Example
    /// 
    /// ```
    /// # use crossword_generator::word::{Word, WordDirection, WordPosition};
    /// # use crossword_generator::crossword::Crossword;                                         
    /// let mut cw = Crossword::new(&[                                                                      //     ---------
    ///     Word{position: WordPosition { x: 0, y: 0 }, direction: WordDirection::Right, value: "hello"},   //    |h e l l o|
    ///     Word{position: WordPosition { x: 2, y: 0 }, direction: WordDirection::Down, value: "local"},    //    |    o    |
    /// ]);                                                                                                 //    |    c    |
    ///                                                                                                     //    |    a    |
    ///                                                                                                     //    |    l    |
    ///                                                                                                     //     ---------
    /// 
    /// 
    /// assert_eq!(cw.generate_char_table(), vec!
    /// [
    ///     vec!['h', 'e', 'l', 'l', 'o'],    
    ///     vec![' ', ' ', 'o', ' ', ' '],
    ///     vec![' ', ' ', 'c', ' ', ' '],
    ///     vec![' ', ' ', 'a', ' ', ' '],
    ///     vec![' ', ' ', 'l', ' ', ' ']
    /// ]);                                                 
    /// ```
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
    
    /// Returns a printable [String] representation of the [crossword](Crossword)
    /// 
    /// ## Example
    /// 
    /// ```
    /// # use crossword_generator::word::{Word, WordDirection, WordPosition};
    /// # use crossword_generator::crossword::Crossword;                                         
    /// let mut cw = Crossword::new(&[                                                                      //     ---------
    ///     Word{position: WordPosition { x: 0, y: 0 }, direction: WordDirection::Right, value: "hello"},   //    |h e l l o|
    ///     Word{position: WordPosition { x: 2, y: 0 }, direction: WordDirection::Down, value: "local"},    //    |    o    |
    /// ]);                                                                                                 //    |    c    |
    ///                                                                                                     //    |    a    |
    ///                                                                                                     //    |    l    |
    ///                                                                                                     //     ---------
    /// 
    /// 
    /// assert_eq!(cw.generate_string(), 
    /// "\
    /// -----------
    /// |h e l l o|
    /// |    o    |
    /// |    c    |
    /// |    a    |
    /// |    l    |
    /// -----------\n");                                                 
    /// ```
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
            Word{position: WordPosition { x: 0, y: 0 }, direction: WordDirection::Down, value: new_word},
            //Word{position: WordPosition { x: 1, y: 1 }, direction: WordDirection::Down, value: new_word.clone()},  |-
            //Word{position: WordPosition { x: 1, y: 3 }, direction: WordDirection::Right, value: new_word.clone()}, ||
            //Word{position: WordPosition { x: 3, y: -3 }, direction: WordDirection::Down, value: new_word.clone()}, ||
            Word{position: WordPosition { x: -1, y: 4 }, direction: WordDirection::Right, value: new_word},
            //Word{position: WordPosition { x: -2, y: 1 }, direction: WordDirection::Right, value: new_word.clone()},||
            Word{position: WordPosition { x: 4, y: -4 }, direction: WordDirection::Down, value: new_word},
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
