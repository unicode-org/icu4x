use std::{
    boxed::Box,
    char,
    clone::Clone,
    cmp::Ordering,
    convert::From, // https://doc.rust-lang.org/std/convert/trait.From.html rust practice says do not use Into
    error::Error,
    hash::{Hash, Hasher},
    iter::Iterator,
    num::ParseIntError,
    ops::Range,
    str::Split,
    vec::Vec,
};

const UNICODESET_MAX: u32 = 0x110000; // does max imply inclusive? else should be 10FFFF
const UNICODESET_MIN: u32 = 0x000000;

/// Given string representation of inversion list create set
/// Check if sorted during iteration
fn parse_serial_string(serialize_str: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    // wondering how much this method catches in tests
    // let split_serialize: Split<&str> = serialize.split(" ");
    // let capacity: u8 = split_serialize.next().unwrap().
    let mut serialize = serialize_str.split(" ");
    let capacity: usize = serialize.next().unwrap().parse()?;
    if capacity % 2 != 0 {
        return Err("Capacity must be even".into());
    }
    let mut serialized_vec: Vec<u32> = Vec::with_capacity(capacity);
    let mut prev: u32 = 0;
    for str_ele in serialize {
        // unsure if the capacity matters if we can expand, but that might be an issue if you expand into too much memory
        // otherwise shrink_to_fit is possible
        let parsed: u32 = str_ele.parse()?;
        if serialized_vec.len() + 1 > serialized_vec.capacity() {
            return Err("Serialization capacity is too small".into());
        }
        if parsed < prev {
            return Err("Serialization must be sorted".into());
        }
        serialized_vec.push(parsed);
        prev = parsed;
    }
    if serialized_vec.len() % 2 != 0 {
        return Err("Serialization must be even".into());
    }
    serialized_vec.shrink_to_fit(); // necessary if the length < capacity
    Ok(serialized_vec)
}

//#[derive(Copy, Clone, Debug, Eq)]
pub struct UnicodeSet {
    // If we wanted to use an array to keep the memory on the stack, there is an unsafe nightly feature
    // https://doc.rust-lang.org/nightly/core/array/trait.FixedSizeArray.html
    // Allows for traits of fixed size arrays
    set: Vec<u32>, // is set misleading? could be uset
}

impl UnicodeSet {
    pub fn new(serialize: &str) -> Result<UnicodeSet, Box<dyn Error>> {
        match parse_serial_string(serialize) {
            Ok(serialize) => Ok(UnicodeSet { set: serialize }),
            Err(e) => Err(e),
        }
    }

    pub fn from_range(start: &u32, end: &u32) -> UnicodeSet {
        UnicodeSet {
            set: vec![*start, *end],
        }
    }

    pub fn all() -> UnicodeSet {
        UnicodeSet {
            set: vec![UNICODESET_MIN, UNICODESET_MAX],
        }
    }

    pub fn bmp() -> UnicodeSet {
        UnicodeSet {
            set: vec![UNICODESET_MIN, 0xFFFF],
        }
    }

    pub fn contains(&self, query: &u32) -> bool {
        // need an enforcement of pattern
        //Need to evaluate
        // let mut low = 0;
        // let mut high = self.set.len() - 1;
        // if low >= high || query > self.set[high] || query < self.set[low]{
        //     false
        // }
        // // [2, 5, 10, 12] => [2, 4], [10, 11]
        // // [2, 5, 10] => [2, 4], [10]
        // // [2, 5, 10, 10, 12]
        // // [1, 1, 0]
        // // 5, 9
        // let mut pos: i8 = -1;
        // while low <= high {
        //     let middle = (low + high) >> 1;
        //     let check = self.set[middle];
        //     if middle == low {
        //         pos = middle;
        //         break
        //     }
        //     if check < query {
        //         low = middle + 1;
        //     }
        //     else {
        //         high = middle - 1;
        //     }
        // }
        // if pos == -1 {
        //     pos = middle + 1;
        // }
        // [2, 5, 10, 15]
        match self.set.binary_search(query) {
            // relies on having even # elements
            Ok(pos) => {
                if pos % 2 == 0 {
                    true
                } else {
                    if pos > 0 && &self.set[pos - 1] == query {
                        true
                    } else {
                        false
                    }
                }
            }
            Err(pos) => {
                if pos % 2 == 0 {
                    false
                } else {
                    if pos >= self.set.len() - 1 {
                        false
                    } else {
                        true
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::{parse_serial_string, UnicodeSet},
        std::num::ParseIntError,
    };
    // parse_serial_string
    #[test]
    fn test_parse_serial_string() {
        let expected = vec![2, 3, 4, 5];
        let actual = parse_serial_string("4 2 3 4 5").unwrap();
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_parse_serial_string_no_char() {
        assert!(parse_serial_string("4 2 A 3 4 5").is_err());
    }
    #[test]
    fn test_parse_serial_string_empty() {
        assert!(parse_serial_string("").is_err());
    }
    #[test]
    fn test_parse_serial_string_wrong_format() {
        assert!(parse_serial_string("[4, 2, 3, 4, 5  ]").is_err());
    }
    #[test]
    fn test_parse_serial_string_capacity_not_even() {
        assert!(parse_serial_string("3 2 3 4").is_err());
    }
    #[test]
    fn test_parse_serial_string_size_not_even() {
        assert!(parse_serial_string("4 3 2 1").is_err());
    }

    // UnicodeSet constructors
}
// impl From<io:: // need to define an error
