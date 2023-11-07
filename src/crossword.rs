use std::collections::BTreeSet;

use itertools::Itertools;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
struct WordBoundingBox
{
    x: isize,
    y: isize,
    w: usize, 
    h: usize
}

impl WordBoundingBox
{
    fn same_direction_as(&self, other: &WordBoundingBox) -> bool
    {
        (self.w == 1 && other.w == 1) || (self.h == 1 && other.h == 1)
    }

    fn intersects(&self, other: &WordBoundingBox) -> bool 
    {
        (self.x < other.x + other.w as isize && self.x + self.w as isize > other.x) &&
        (self.y < other.y + other.h as isize && self.y + self.h as isize > other.y)
    }

    fn side_touches_side(&self, other: &WordBoundingBox) -> bool
    {
        if !self.same_direction_as(other) { return false; }

        if self.h == 1
        {
            self.y.abs_diff(other.y) == 1 && (self.x < other.x + other.w as isize && self.x + self.w as isize > other.x)
        }
        else
        {
            self.x.abs_diff(other.x) == 1 && (self.y < other.y + other.h as isize && self.y + self.h as isize > other.y)
        }
    }

    fn side_touches_head(&self, other: &WordBoundingBox) -> bool
    {
        if self.same_direction_as(other) { return false; }

        let hor: &WordBoundingBox;
        let ver: &WordBoundingBox;

        if self.h == 1
        {
            hor = self;
            ver = other;
        }
        else
        {
            ver = self;
            hor = other;
        }

        (hor.x + hor.w as isize >= ver.x) &&
        (hor.x <= ver.x + 1) &&
        (hor.y + 1 >= ver.y) &&
        (hor.y <= ver.y + ver.h as isize) &&
        
        ((hor.x + hor.w as isize == ver.x) as u8 + 
        (hor.x == ver.x + 1) as u8 + 
        (hor.y + 1 == ver.y) as u8 + 
        (hor.y == ver.y + ver.h as isize) as u8) == 1u8
    }

    fn head_touches_head(&self, other: &WordBoundingBox) -> bool
    {
        if !self.same_direction_as(other) { return false; }

        if self.h == 1
        {
            self.y == other.y && (self.x + self.w as isize == other.x || other.x + other.w as isize == self.x)
        }
        else
        {
            self.x == other.x && (self.y + self.h as isize == other.y || other.y + other.h as isize == self.y)
        }
    }

    fn corners(&self, other: &WordBoundingBox) -> bool
    {
        (self.x == other.x + other.w as isize && self.y == other.y + other.h as isize) ||
        (self.x + self.w as isize == other.x && self.y == other.y + other.h as isize) ||
        (self.x + self.w as isize == other.x && self.y + self.h as isize == other.y) ||
        (self.x == other.x + other.w as isize && self.y + self.h as isize == other.y)
    }

    fn get_intersection_indices(&self, other: &WordBoundingBox) -> Option<(usize, usize)>
    {
        if !self.intersects(other) { return None; }
        if self.same_direction_as(other) { return None; }
        if self.h == 1 
        {
            Some(((other.x - self.x) as usize, (self.y - other.y) as usize))
        }
        else
        {
            Some(((other.y - self.y) as usize, (self.x - other.x) as usize))
        }  
    }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct WordCompatibilitySettings
{
    pub side_by_side: bool,
    pub head_by_head: bool,
    pub side_by_head: bool,
    pub corner_by_corner: bool
}

impl WordCompatibilitySettings 
{
    fn are_words_compatible(&self, first: &Word, second: &Word) -> bool
    {
        let first_bb = first.get_bounding_box();
        let second_bb = second.get_bounding_box();

        if first_bb.corners(&second_bb) && !self.corner_by_corner { return false; }

        if first.direction == second.direction
        {
            if first_bb.head_touches_head(&second_bb) && !self.head_by_head { return false; }
            if first_bb.side_touches_side(&second_bb) && !self.side_by_side { return false; }
            if first_bb.intersects(&second_bb) { return false; }

            true
        }
        else
        {
            if first_bb.side_touches_head(&second_bb) && !self.side_by_head { return false; }
            if first_bb.intersects(&second_bb)
            {
                let (first_ind, second_ind) = first_bb.get_intersection_indices(&second_bb).unwrap();
                let first_char = first.value.chars().nth(first_ind);
                let second_char = second.value.chars().nth(second_ind);
        
                return first_char.is_some() && second_char.is_some() && (first_char == second_char);
            }

            true
        }
    }
}

impl Default for WordCompatibilitySettings 
{
    fn default() -> Self 
    {
        return WordCompatibilitySettings 
        {
            side_by_side: false,
            head_by_head: false,
            side_by_head: false,
            corner_by_corner: true
        }    
    }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default, Debug)]
pub struct WordPosition
{
    pub x: isize,
    pub y: isize,  
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default, Debug)]
pub enum WordDirection
{
    #[default]
    Right,
    Down
}

impl WordDirection 
{
    fn opposite(&self) -> WordDirection
    {
        match *self
        {
            WordDirection::Down => WordDirection::Right,
            WordDirection::Right => WordDirection::Down
        }
    } 
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default, Debug)]
pub struct Word
{
    pub position: WordPosition,
    pub direction: WordDirection,
    pub value: String
}

impl Word
{
    fn get_bounding_box(&self) -> WordBoundingBox
    {
        match self.direction 
        {
            WordDirection::Right => WordBoundingBox { x: self.position.x, y: self.position.y, w: self.value.len(), h: 1 },
            WordDirection::Down => WordBoundingBox { x: self.position.x, y: self.position.y, w: 1, h: self.value.len() },
        }
    }

    fn calculate_possible_ways_to_add_word(&self, word: &str) -> BTreeSet<Word>
    {
        let mut pos_ways = BTreeSet::new();
        let common_chars = word.chars().filter(|c| self.value.contains(*c)).collect::<Vec<char>>();

        for char in common_chars
        {
            for (word_ind, self_ind) in word.chars().enumerate().filter_map(|c| if c.1 == char { Some(c.0) } else { None } ).cartesian_product(self.value.chars().enumerate().filter_map(|c| if c.1 == char { Some(c.0) } else { None } ))
            {
                pos_ways.insert(
                    Word
                    {
                        position: match self.direction
                        {
                            WordDirection::Right => WordPosition{ x: self.position.x + self_ind as isize, y: self.position.y - word_ind as isize},
                            WordDirection::Down  => WordPosition{ x: self.position.x - word_ind as isize, y: self.position.y + self_ind as isize},
                        },
                        direction: self.direction.opposite(),
                        value: word.to_owned() 
                    }
                );
            }
        }

        pos_ways
    }
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default, Debug)]
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
    fn is_crossword_valid(&self, cw: &Crossword) -> bool
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

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default, Debug)]
pub struct CrosswordSettings
{
    pub size_constraints: Vec<CrosswordSizeConstrain>
}

impl CrosswordSettings
{
    fn is_crossword_valid(&self, cw: &Crossword) -> bool
    {
        return self.size_constraints.iter().all(|c| c.is_crossword_valid(cw))
    }
}


#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default, Debug)]
pub struct Crossword
{
    words: BTreeSet<Word>,
    pub word_compatibitity_settings: WordCompatibilitySettings,
    pub crossword_settings: CrosswordSettings
}

impl Crossword
{
    fn new(words: &[Word]) -> Crossword
    {
        let mut cw = Crossword { words: words.iter().map(|x| x.clone()).collect(), ..Default::default() };
        cw.normalize();

        return cw;
    }
    fn normalize(&mut self)
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

    fn add_word(&mut self, word: &Word)
    {
        if self.words.iter().find(|w| w.value == word.value).is_some() { return; }
        self.words.insert(word.clone());
        self.normalize();
    }

    fn remove_word(&mut self, word: &str)
    {
        let mut word_to_remove = Word::default();

        self.words.iter().for_each(|w| if w.value == word { word_to_remove = w.clone() });
        self.words.remove(&word_to_remove);

        self.normalize();
    }

    fn find_word(&self, word: &str) -> Option<&Word>
    {
        self.words.iter().filter(|w| w.value == word).next()
    }
    

    fn contains_crossword(&self, other: &Crossword) -> bool 
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

    fn calculate_possible_ways_to_add_word(&self, word: &str) -> BTreeSet<Word>
    {
        if self.words.is_empty()
        {
            dbg!(word);
            return vec![Word{ value: word.to_owned(), ..Word::default()}].into_iter().collect()
        }

        self.words.iter()
            .flat_map(|cur_word| cur_word.calculate_possible_ways_to_add_word(word))
            .filter(|w| self.can_word_be_added(w))
            .collect()
    }

    fn can_word_be_added(&self, word: &Word) -> bool
    {
        self.words.iter().all(|w| self.word_compatibitity_settings.are_words_compatible(w, word))
    }
    
    fn get_size(&self) -> (usize, usize)
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
    
    fn generate_char_table(&self) ->Vec<Vec<char>>
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


pub fn generate_crosswords(words: &BTreeSet<String>, word_compatibility_settings: &WordCompatibilitySettings, crossword_settings: &CrosswordSettings) -> BTreeSet<Crossword>
{
    let mut crossword = Crossword::default();
    crossword.word_compatibitity_settings = word_compatibility_settings.clone();
    crossword.crossword_settings = crossword_settings.clone();
    let mut crosswords = BTreeSet::new();

    let mut full_created_crossword_bases = BTreeSet::new();

    generate_crosswords_impl(&mut crossword, words, &mut crosswords, &mut full_created_crossword_bases);
    
    crosswords
}

fn generate_crosswords_impl(current_crossword: &mut Crossword, remained_words: &BTreeSet<String>, crosswords: &mut BTreeSet<Crossword>, full_created_crossword_bases: &mut BTreeSet<Crossword>)
{
    if !current_crossword.crossword_settings.is_crossword_valid(&current_crossword) { return; }

    if remained_words.is_empty()
    {
        crosswords.insert(current_crossword.clone());
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
        for step in current_crossword.calculate_possible_ways_to_add_word(current_word).iter()
        {
            current_crossword.add_word(step);

            generate_crosswords_impl(current_crossword, &new_remained_words, crosswords, full_created_crossword_bases);

            let to_remove: Vec<Crossword> = full_created_crossword_bases.clone().into_iter().filter(|cw| cw.contains_crossword(&current_crossword)).collect();
            to_remove.into_iter().for_each(|cw| {full_created_crossword_bases.remove(&cw);});
            
            full_created_crossword_bases.insert(current_crossword.clone());

            current_crossword.remove_word(&step.value);
        }
    }
}




#[cfg(test)]
mod tests {
    use itertools::iproduct;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_word_bounding_box_same_direction_as()
    {
        let mut first = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "hayastan".to_owned() };
        let mut second = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "arcax".to_owned() };

        assert!(first.get_bounding_box().same_direction_as(&second.get_bounding_box()));

        first.direction = WordDirection::Down;

        assert!(!first.get_bounding_box().same_direction_as(&second.get_bounding_box()));

        second.direction = WordDirection::Down;
        
        assert!(first.get_bounding_box().same_direction_as(&second.get_bounding_box()));
    }

    #[test]
    fn test_word_bounding_box_intersects()
    {
        let mut first = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "hayastan".to_owned() };
        let mut second = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "arcax".to_owned() };
        
        let mut comp = vec![];
        for y in -2isize..=2
        {
            for x in -6isize..=9
            {
                second.position = WordPosition {x, y};
                comp.push(first.get_bounding_box().intersects(&second.get_bounding_box()) as isize);
            }
        }
    
        assert_eq!(comp, vec![  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "hor_hor");
        
        first.direction = WordDirection::Down;
        second.direction = WordDirection::Down;
        comp = vec![];
        for y in -6isize..=9
        {
            for x in -2isize..=2
            {
                second.position = WordPosition {x, y};
                comp.push(first.get_bounding_box().intersects(&second.get_bounding_box()) as isize);
            }
        }

        assert_eq!(comp, vec![  0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 1, 0, 0,
                                0, 0, 1, 0, 0,
                                0, 0, 1, 0, 0,
                                0, 0, 1, 0, 0,
                                0, 0, 1, 0, 0,
                                0, 0, 1, 0, 0,
                                0, 0, 1, 0, 0,
                                0, 0, 1, 0, 0,
                                0, 0, 1, 0, 0,
                                0, 0, 1, 0, 0,
                                0, 0, 1, 0, 0,
                                0, 0, 1, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0], "ver_ver");

        first.direction = WordDirection::Right;
        comp = vec![];
        for y in -6isize..=2
        {
            for x in -2isize..=9
            {
                second.position = WordPosition {x, y};
                comp.push(first.get_bounding_box().intersects(&second.get_bounding_box()) as isize);
            }
        }

        assert_eq!(comp, vec![  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
                                0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
                                0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
                                0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
                                0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "hor_ver");
    }

    #[test]
    fn test_word_bounding_box_side_touches_side()
    {
        let mut first = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "hayastan".to_owned() };
        let mut second = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "arcax".to_owned() };
        
        let mut comp = vec![];
        for y in -2isize..=2
        {
            for x in -6isize..=9
            {
                second.position = WordPosition {x, y};
                comp.push(first.get_bounding_box().side_touches_side(&second.get_bounding_box()) as isize);
            }
        }
    
        assert_eq!(comp, vec![  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "hor_hor");
        
        first.direction = WordDirection::Down;
        second.direction = WordDirection::Down;
        comp = vec![];
        for y in -6isize..=9
        {
            for x in -2isize..=2
            {
                second.position = WordPosition {x, y};
                comp.push(first.get_bounding_box().side_touches_side(&second.get_bounding_box()) as isize);
            }
        }

        assert_eq!(comp, vec![  0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 1, 0, 1, 0,
                                0, 1, 0, 1, 0,
                                0, 1, 0, 1, 0,
                                0, 1, 0, 1, 0,
                                0, 1, 0, 1, 0,
                                0, 1, 0, 1, 0,
                                0, 1, 0, 1, 0,
                                0, 1, 0, 1, 0,
                                0, 1, 0, 1, 0,
                                0, 1, 0, 1, 0,
                                0, 1, 0, 1, 0,
                                0, 1, 0, 1, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0], "ver_ver");

        first.direction = WordDirection::Right;
        comp = vec![];
        for y in -6isize..=2
        {
            for x in -2isize..=9
            {
                second.position = WordPosition {x, y};
                comp.push(first.get_bounding_box().side_touches_side(&second.get_bounding_box()) as isize);
            }
        }

        assert_eq!(comp, vec![  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "hor_ver");
    }

    #[test]
    fn test_word_bounding_box_side_touches_head()
    {
        let mut first = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "hayastan".to_owned() };
        let mut second = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "arcax".to_owned() };
        
        let mut comp = vec![];
        for y in -2isize..=2
        {
            for x in -6isize..=9
            {
                second.position = WordPosition {x, y};
                comp.push(first.get_bounding_box().side_touches_head(&second.get_bounding_box()) as isize);
            }
        }
    
        assert_eq!(comp, vec![  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "hor_hor");
        
        first.direction = WordDirection::Down;
        second.direction = WordDirection::Down;
        comp = vec![];
        for y in -6isize..=9
        {
            for x in -2isize..=2
            {
                second.position = WordPosition {x, y};
                comp.push(first.get_bounding_box().side_touches_head(&second.get_bounding_box()) as isize);
            }
        }

        assert_eq!(comp, vec![  0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0], "ver_ver");

        first.direction = WordDirection::Right;
        comp = vec![];
        for y in -6isize..=2
        {
            for x in -2isize..=9
            {
                second.position = WordPosition {x, y};
                comp.push(first.get_bounding_box().side_touches_head(&second.get_bounding_box()) as isize);
            }
        }

        assert_eq!(comp, vec![  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
                                0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
                                0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
                                0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
                                0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
                                0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
                                0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "hor_ver");
    }
    
    #[test]
    fn test_word_bounding_box_head_touches_head()
    {
        let mut first = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "hayastan".to_owned() };
        let mut second = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "arcax".to_owned() };
        
        let mut comp = vec![];
        for y in -2isize..=2
        {
            for x in -6isize..=9
            {
                second.position = WordPosition {x, y};
                comp.push(first.get_bounding_box().head_touches_head(&second.get_bounding_box()) as isize);
            }
        }
    
        assert_eq!(comp, vec![  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "hor_hor");
        
        first.direction = WordDirection::Down;
        second.direction = WordDirection::Down;
        comp = vec![];
        for y in -6isize..=9
        {
            for x in -2isize..=2
            {
                second.position = WordPosition {x, y};
                comp.push(first.get_bounding_box().head_touches_head(&second.get_bounding_box()) as isize);
            }
        }

        assert_eq!(comp, vec![  0, 0, 0, 0, 0,
                                0, 0, 1, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 1, 0, 0,
                                0, 0, 0, 0, 0], "ver_ver");

        first.direction = WordDirection::Right;
        comp = vec![];
        for y in -6isize..=2
        {
            for x in -2isize..=9
            {
                second.position = WordPosition {x, y};
                comp.push(first.get_bounding_box().head_touches_head(&second.get_bounding_box()) as isize);
            }
        }

        assert_eq!(comp, vec![  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "hor_ver");
    }

    #[test]
    fn test_word_bounding_box_corners()
    {
        let mut first = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "hayastan".to_owned() };
        let mut second = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "arcax".to_owned() };
        
        let mut comp = vec![];
        for y in -2isize..=2
        {
            for x in -6isize..=9
            {
                second.position = WordPosition {x, y};
                comp.push(first.get_bounding_box().corners(&second.get_bounding_box()) as isize);
            }
        }
    
        assert_eq!(comp, vec![  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "hor_hor");
        
        first.direction = WordDirection::Down;
        second.direction = WordDirection::Down;
        comp = vec![];
        for y in -6isize..=9
        {
            for x in -2isize..=2
            {
                second.position = WordPosition {x, y};
                comp.push(first.get_bounding_box().corners(&second.get_bounding_box()) as isize);
            }
        }

        assert_eq!(comp, vec![  0, 0, 0, 0, 0,
                                0, 1, 0, 1, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0,
                                0, 1, 0, 1, 0,
                                0, 0, 0, 0, 0], "ver_ver");

        first.direction = WordDirection::Right;
        comp = vec![];
        for y in -6isize..=2
        {
            for x in -2isize..=9
            {
                second.position = WordPosition {x, y};
                comp.push(first.get_bounding_box().corners(&second.get_bounding_box()) as isize);
            }
        }

        assert_eq!(comp, vec![  0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                                0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
                                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "hor_ver");
    }

    #[test]
    fn test_word_bounding_box_get_intersection_indices()
    {
        let mut first = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "hayastan".to_owned() };
        let mut second = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "arcax".to_owned() };

        assert_eq!(first.get_bounding_box().get_intersection_indices(&second.get_bounding_box()), None);

        first.direction = WordDirection::Down;
        assert_eq!(first.get_bounding_box().get_intersection_indices(&second.get_bounding_box()), Some((0, 0)));

        second.position = WordPosition {x: -1, y: 2};
        assert_eq!(first.get_bounding_box().get_intersection_indices(&second.get_bounding_box()), Some((2, 1)));

        second.position.x = 2;
        assert_eq!(first.get_bounding_box().get_intersection_indices(&second.get_bounding_box()), None);
    }



    #[test]
    fn test_word_compatibility_settings_are_words_compatible() {

        for (a, b, c, d) in iproduct!((0isize..2), (0isize..2), (0isize..2), (0isize..2))
        {
            let settings = WordCompatibilitySettings { side_by_side: a != 0, head_by_head: b != 0, side_by_head: c != 0, corner_by_corner: d != 0 };

            let mut first = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "hayastan".to_owned() };
            let mut second = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "arcax".to_owned() };
            
            let mut comp = vec![];
            for y in -2isize..=2
            {
                for x in -6isize..=9
                {
                    second.position = WordPosition {x, y};
                    comp.push(settings.are_words_compatible(&first, &second) as isize);
                }
            }
        
            assert_eq!(comp, vec![  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                                    1, d, a, a, a, a, a, a, a, a, a, a, a, a, d, 1,
                                    1, b, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, b, 1,
                                    1, d, a, a, a, a, a, a, a, a, a, a, a, a, d, 1,
                                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], "hor_hor with settings {:?}", settings);
            
            first.direction = WordDirection::Down;
            second.direction = WordDirection::Down;
            comp = vec![];
            for y in -6isize..=9
            {
                for x in -2isize..=2
                {
                    second.position = WordPosition {x, y};
                    comp.push(settings.are_words_compatible(&first, &second) as isize);
                }
            }

            assert_eq!(comp, vec![  1, 1, 1, 1, 1,
                                    1, d, b, d, 1,
                                    1, a, 0, a, 1,
                                    1, a, 0, a, 1,
                                    1, a, 0, a, 1,
                                    1, a, 0, a, 1,
                                    1, a, 0, a, 1,
                                    1, a, 0, a, 1,
                                    1, a, 0, a, 1,
                                    1, a, 0, a, 1,
                                    1, a, 0, a, 1,
                                    1, a, 0, a, 1,
                                    1, a, 0, a, 1,
                                    1, a, 0, a, 1,
                                    1, d, b, d, 1,
                                    1, 1, 1, 1, 1], "ver_ver with settings {:?}", settings);

            first.direction = WordDirection::Right;
            comp = vec![];
            for y in -6isize..=2
            {
                for x in -2isize..=9
                {
                    second.position = WordPosition {x, y};
                    comp.push(settings.are_words_compatible(&first, &second) as isize);
                }
            }

            assert_eq!(comp, vec![  1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                                    1, d, c, c, c, c, c, c, c, c, d, 1,
                                    1, c, 0, 0, 0, 0, 0, 0, 0, 0, c, 1,
                                    1, c, 0, 1, 0, 1, 0, 0, 1, 0, c, 1,
                                    1, c, 0, 0, 0, 0, 0, 0, 0, 0, c, 1,
                                    1, c, 0, 0, 0, 0, 0, 0, 0, 0, c, 1,
                                    1, c, 0, 1, 0, 1, 0, 0, 1, 0, c, 1,
                                    1, d, c, c, c, c, c, c, c, c, d, 1,
                                    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], "hor_ver with settings {:?}", settings);
        }

    }


    #[test]
    fn test_crossword_contains_crossword() {
        let cw = Crossword::new(
            &[
                Word{position: WordPosition { x: -1, y: -1 }, direction: WordDirection::Right, value: "hello".to_owned()},
                Word{position: WordPosition { x: 1, y: -1 }, direction: WordDirection::Down, value: "local".to_owned()},
                Word{position: WordPosition { x: 1, y: 1 }, direction: WordDirection::Right, value: "cat".to_owned()},
                Word{position: WordPosition { x: 2, y: 1 }, direction: WordDirection::Down, value: "and".to_owned()},
                Word{position: WordPosition { x: 3, y: 1 }, direction: WordDirection::Down, value: "toy".to_owned()},

            ]);

        let mut containing_crossword_1 = Crossword::new(
            &[
                Word{position: WordPosition { x: -1, y: -1 }, direction: WordDirection::Right, value: "hello".to_owned()},
                Word{position: WordPosition { x: 1, y: -1 }, direction: WordDirection::Down, value: "local".to_owned()},
                Word{position: WordPosition { x: 1, y: 1 }, direction: WordDirection::Right, value: "cat".to_owned()},
                Word{position: WordPosition { x: 2, y: 1 }, direction: WordDirection::Down, value: "and".to_owned()},
                Word{position: WordPosition { x: 3, y: 1 }, direction: WordDirection::Down, value: "toy".to_owned()},

            ]);

        let mut containing_crossword_2 = Crossword::new(
            &[
                Word{position: WordPosition { x: 2, y: 1 }, direction: WordDirection::Right, value: "cat".to_owned()},
                Word{position: WordPosition { x: 3, y: 1 }, direction: WordDirection::Down, value: "and".to_owned()},
                Word{position: WordPosition { x: 4, y: 1 }, direction: WordDirection::Down, value: "toy".to_owned()},

            ]);

        let mut containing_crossword_3 = Crossword::new(
            &[
                Word{position: WordPosition { x: 2, y: 2 }, direction: WordDirection::Down, value: "and".to_owned()},
                Word{position: WordPosition { x: 3, y: 1 }, direction: WordDirection::Down, value: "toy".to_owned()},

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
                Word{position: WordPosition { x: -1, y: -1 }, direction: WordDirection::Right, value: "hello".to_owned()},
                Word{position: WordPosition { x: 1, y: -1 }, direction: WordDirection::Down, value: "local".to_owned()},
                Word{position: WordPosition { x: 1, y: 1 }, direction: WordDirection::Right, value: "cat".to_owned()},
                Word{position: WordPosition { x: 2, y: 1 }, direction: WordDirection::Down, value: "and".to_owned()},
                Word{position: WordPosition { x: 3, y: 1 }, direction: WordDirection::Down, value: "toy".to_owned()},
    
            ]);


        assert_eq!(cw.generate_string(), 
        "\
-----------
|h e l l o|
|    o    |
|    c a t|
|    a n o|
|    l d y|
-----------\n".to_owned())
    }

    #[test]
    fn test_crossword_normalize() {
        let mut cw = Crossword::new(
            &[
                Word{position: WordPosition { x: -1, y: -1 }, direction: WordDirection::Right, value: "hello".to_owned()},
                Word{position: WordPosition { x: 1, y: -1 }, direction: WordDirection::Down, value: "local".to_owned()},
                Word{position: WordPosition { x: 1, y: 1 }, direction: WordDirection::Right, value: "cat".to_owned()},
                Word{position: WordPosition { x: 2, y: 1 }, direction: WordDirection::Down, value: "and".to_owned()},
                Word{position: WordPosition { x: 3, y: 1 }, direction: WordDirection::Down, value: "toy".to_owned()},
    
            ]);
        
        cw.normalize();

        let cw_normalized = Crossword::new(
            &[
                Word{position: WordPosition { x: 0, y: 0 }, direction: WordDirection::Right, value: "hello".to_owned()},
                Word{position: WordPosition { x: 2, y: 0 }, direction: WordDirection::Down, value: "local".to_owned()},
                Word{position: WordPosition { x: 2, y: 2 }, direction: WordDirection::Right, value: "cat".to_owned()},
                Word{position: WordPosition { x: 3, y: 2 }, direction: WordDirection::Down, value: "and".to_owned()},
                Word{position: WordPosition { x: 4, y: 2 }, direction: WordDirection::Down, value: "toy".to_owned()},

            ]);

        assert_eq!(cw, cw_normalized);
    }

    #[test]
    fn test_crossword_remove_word() {
        let mut cw = Crossword::new(
            &[
                Word{position: WordPosition { x: -1, y: -1 }, direction: WordDirection::Right, value: "hello".to_owned()},
                Word{position: WordPosition { x: 1, y: -1 }, direction: WordDirection::Down, value: "local".to_owned()},
                Word{position: WordPosition { x: 1, y: 1 }, direction: WordDirection::Right, value: "cat".to_owned()},
                Word{position: WordPosition { x: 2, y: 1 }, direction: WordDirection::Down, value: "and".to_owned()},
                Word{position: WordPosition { x: 3, y: 1 }, direction: WordDirection::Down, value: "toy".to_owned()},
    
            ]);
        
        cw.remove_word("toy");

        let cw_word_removed = Crossword::new(
            &[
                Word{position: WordPosition { x: 0, y: 0 }, direction: WordDirection::Right, value: "hello".to_owned()},
                Word{position: WordPosition { x: 2, y: 0 }, direction: WordDirection::Down, value: "local".to_owned()},
                Word{position: WordPosition { x: 2, y: 2 }, direction: WordDirection::Right, value: "cat".to_owned()},
                Word{position: WordPosition { x: 3, y: 2 }, direction: WordDirection::Down, value: "and".to_owned()},

            ]);

        assert_eq!(cw, cw_word_removed);
    }

    #[test]
    fn test_crossword_calculate_possible_ways_to_add_word() {
        let cw = Crossword::new(
            &[
                Word{position: WordPosition { x: 0, y: 0 }, direction: WordDirection::Right, value: "hello".to_owned()},
                Word{position: WordPosition { x: 2, y: 0 }, direction: WordDirection::Down, value: "local".to_owned()},
                Word{position: WordPosition { x: 0, y: 2 }, direction: WordDirection::Right, value: "tac".to_owned()}
            ]);

        let new_word = "hatlo".to_owned();

        assert_eq!(cw.calculate_possible_ways_to_add_word(&new_word), vec![
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
