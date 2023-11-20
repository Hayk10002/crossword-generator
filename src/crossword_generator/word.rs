use std::collections::BTreeSet;
use itertools::Itertools;

use serde::{Serialize, Deserialize};

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct WordCompatibilitySettings
{
    pub side_by_side: bool,
    pub head_by_head: bool,
    pub side_by_head: bool,
    pub corner_by_corner: bool
}

impl WordCompatibilitySettings 
{
    pub fn are_words_compatible(&self, first: &Word, second: &Word) -> bool
    {
        if first.corners_touch(&second) && !self.corner_by_corner { return false; }

        if first.direction == second.direction
        {
            if first.head_touches_head(&second) && !self.head_by_head { return false; }
            if first.side_touches_side(&second) && !self.side_by_side { return false; }
            if first.intersects(&second) { return false; }

            true
        }
        else
        {
            if first.side_touches_head(&second) && !self.side_by_head { return false; }
            if first.intersects(&second)
            {
                let (first_ind, second_ind) = first.get_intersection_indices(&second).unwrap();
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

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default, Debug, Serialize, Deserialize)]
struct WordBoundingBox
{
    x: isize,
    y: isize,
    w: usize, 
    h: usize
}

impl WordBoundingBox
{
    fn intersects(&self, other: &WordBoundingBox) -> bool 
    {
        (self.x < other.x + other.w as isize && self.x + self.w as isize > other.x) &&
        (self.y < other.y + other.h as isize && self.y + self.h as isize > other.y)
    }

    fn sides_touch(&self, other: &WordBoundingBox) -> bool
    {
        ((self.x + self.w as isize > other.x && self.x < other.x + other.w as isize) && (self.y + self.h as isize == other.y || other.y + other.h as isize == self.y)) || 
        ((self.y + self.h as isize > other.y && self.y < other.y + other.h as isize) && (self.x + self.w as isize == other.x || other.x + other.w as isize == self.x))
    }

    fn corners_touch(&self, other: &WordBoundingBox) -> bool
    {
        (self.x == other.x + other.w as isize && self.y == other.y + other.h as isize) ||
        (self.x + self.w as isize == other.x && self.y == other.y + other.h as isize) ||
        (self.x + self.w as isize == other.x && self.y + self.h as isize == other.y) ||
        (self.x == other.x + other.w as isize && self.y + self.h as isize == other.y)
    }

}


#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default, Debug, Serialize, Deserialize)]
pub struct WordPosition
{
    pub x: isize,
    pub y: isize,  
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default, Debug, Serialize, Deserialize)]
pub enum WordDirection
{
    #[default]
    Right,
    Down
}

impl WordDirection 
{
    pub fn opposite(&self) -> WordDirection
    {
        match *self
        {
            WordDirection::Down => WordDirection::Right,
            WordDirection::Right => WordDirection::Down
        }
    } 
}

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Default, Debug, Serialize, Deserialize)]
pub struct Word<'a>
{
    pub position: WordPosition,
    pub direction: WordDirection,
    pub value: &'a str
}

impl<'a> Word<'a>
{
    fn get_bounding_box(&self) -> WordBoundingBox
    {
        match self.direction 
        {
            WordDirection::Right => WordBoundingBox { x: self.position.x, y: self.position.y, w: self.value.len(), h: 1 },
            WordDirection::Down => WordBoundingBox { x: self.position.x, y: self.position.y, w: 1, h: self.value.len() },
        }
    }

    fn get_parallel_coordinate(&self) -> isize
    {
        match self.direction
        {
            WordDirection::Right => self.position.y,
            WordDirection::Down => self.position.x,
        }
    }

    #[allow(dead_code)]
    fn get_perpendicular_coordinate(&self) -> isize
    {
        match self.direction
        {
            WordDirection::Right => self.position.x,
            WordDirection::Down => self.position.y,
        }
    }

    fn intersects(&self, other: &Word) -> bool 
    {
        self.get_bounding_box().intersects(&other.get_bounding_box())
    }

    fn sides_touch(&self, other: &Word) -> bool
    {
        self.get_bounding_box().sides_touch(&other.get_bounding_box())
    }

    fn corners_touch(&self, other: &Word) -> bool
    {
        self.get_bounding_box().corners_touch(&other.get_bounding_box())
    }

    fn side_touches_side(&self, other: &Word) -> bool
    {
        self.direction == other.direction &&
        self.sides_touch(other) && 
        self.get_parallel_coordinate() != other.get_parallel_coordinate()
    }

    fn side_touches_head(&self, other: &Word) -> bool
    {
        self.direction != other.direction &&
        self.sides_touch(other)
    }

    fn head_touches_head(&self, other: &Word) -> bool
    {
        self.direction == other.direction &&
        self.sides_touch(other) && 
        self.get_parallel_coordinate() == other.get_parallel_coordinate()
    }

    fn get_intersection_indices(&self, other: &Word) -> Option<(usize, usize)>
    {
        if !self.intersects(other) { return None; }
        if self.direction == other.direction { return None; }

        match self.direction
        {
            WordDirection::Right => Some(((other.position.x - self.position.x) as usize, (self.position.y - other.position.y) as usize)),
            WordDirection::Down => Some(((other.position.y - self.position.y) as usize, (self.position.x - other.position.x) as usize))
        }
    }

    pub fn calculate_possible_ways_to_add_word(&self, word: &'a str) -> BTreeSet<Word<'a>>
    {
        let mut pos_ways: BTreeSet<Word<'a>> = BTreeSet::new();
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
                        value: word
                    }
                );
            }
        }

        pos_ways
    }
}


#[cfg(test)]
mod tests
{
    use itertools::iproduct;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_word_bounding_box_intersects()
    {
        let mut first = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "hayastan" };
        let mut second = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "arcax" };
        
        let mut comp = vec![];
        for y in -2isize..=2
        {
            for x in -6isize..=9
            {
                second.position = WordPosition {x, y};
                comp.push(first.intersects(&second) as isize);
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
                comp.push(first.intersects(&second) as isize);
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
                comp.push(first.intersects(&second) as isize);
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
        let mut first = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "hayastan" };
        let mut second = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "arcax" };
        
        let mut comp = vec![];
        for y in -2isize..=2
        {
            for x in -6isize..=9
            {
                second.position = WordPosition {x, y};
                comp.push(first.side_touches_side(&second) as isize);
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
                comp.push(first.side_touches_side(&second) as isize);
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
                comp.push(first.side_touches_side(&second) as isize);
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
        let mut first = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "hayastan" };
        let mut second = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "arcax" };
        
        let mut comp = vec![];
        for y in -2isize..=2
        {
            for x in -6isize..=9
            {
                second.position = WordPosition {x, y};
                comp.push(first.side_touches_head(&second) as isize);
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
                comp.push(first.side_touches_head(&second) as isize);
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
                comp.push(first.side_touches_head(&second) as isize);
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
        let mut first = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "hayastan" };
        let mut second = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "arcax" };
        
        let mut comp = vec![];
        for y in -2isize..=2
        {
            for x in -6isize..=9
            {
                second.position = WordPosition {x, y};
                comp.push(first.head_touches_head(&second) as isize);
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
                comp.push(first.head_touches_head(&second) as isize);
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
                comp.push(first.head_touches_head(&second) as isize);
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
        let mut first = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "hayastan" };
        let mut second = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "arcax" };
        
        let mut comp = vec![];
        for y in -2isize..=2
        {
            for x in -6isize..=9
            {
                second.position = WordPosition {x, y};
                comp.push(first.corners_touch(&second) as isize);
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
                comp.push(first.corners_touch(&second) as isize);
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
                comp.push(first.corners_touch(&second) as isize);
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
        let mut first = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "hayastan" };
        let mut second = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "arcax" };

        assert_eq!(first.get_intersection_indices(&second), None);

        first.direction = WordDirection::Down;
        assert_eq!(first.get_intersection_indices(&second), Some((0, 0)));

        second.position = WordPosition {x: -1, y: 2};
        assert_eq!(first.get_intersection_indices(&second), Some((2, 1)));

        second.position.x = 2;
        assert_eq!(first.get_intersection_indices(&second), None);
    }



    #[test]
    fn test_word_compatibility_settings_are_words_compatible() {

        for (a, b, c, d) in iproduct!((0isize..2), (0isize..2), (0isize..2), (0isize..2))
        {
            let settings = WordCompatibilitySettings { side_by_side: a != 0, head_by_head: b != 0, side_by_head: c != 0, corner_by_corner: d != 0 };

            let mut first = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "hayastan" };
            let mut second = Word{ position: WordPosition{ x: 0, y: 0 }, direction: WordDirection::Right, value: "arcax" };
            
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
}
