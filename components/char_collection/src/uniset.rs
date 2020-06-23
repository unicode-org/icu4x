use std::{boxed::Box, error::Error, iter::Iterator, slice::Iter, vec::Vec};

const UNICODESET_MAX: u32 = 0x10FFFF; // does max imply inclusive? else should be 10FFFF
const UNICODESET_MIN: u32 = 0x000000;
const BMP_MAX: u32 = 0xFFFF;

/// Given string representation of inversion list create set
///
/// Requires starting capacity integer, followed by space delimited integer code points.
/// There must be an even number of elements (not including the capacity int), and must be
/// in ascending sorted order.
///
/// Example String: `4 0 5 10 15` designates a capacity of size 4, followed by 2 ranges
/// The ranges are {0, 4} and {10, 14} inclusive
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
        if parsed <= prev {
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

/// UnicodeSet membership wrapper
///
/// Provides exposure to membership functions and constructors from serialized UnicodeSets
/// and predefined ranges.
//#[derive(Copy, Clone, Debug, Eq)]
pub struct UnicodeSet {
    // If we wanted to use an array to keep the memory on the stack, there is an unsafe nightly feature
    // https://doc.rust-lang.org/nightly/core/array/trait.FixedSizeArray.html
    // Allows for traits of fixed size arrays
    set: Vec<u32>, // is set misleading? could be uset
}

impl UnicodeSet {
    /// Returns Result of UnicodeSet from serialized string
    ///
    /// Returns an error if the serialized string fails to parse.
    /// The serialized string requires starting capacity integer, followed by space delimited
    /// integer code points. There must be an even number of elements (not including the
    /// capacity int), and must be in ascending sorted order.
    ///
    /// Example String: `"4 0 5 10 15"` designates a capacity of size `4`, followed by 2 ranges
    /// The ranges are `{0, 4}` and `{10, 14}` inclusive
    pub fn new(serialize: &str) -> Result<UnicodeSet, Box<dyn Error>> {
        match parse_serial_string(serialize) {
            Ok(serialize) => Ok(UnicodeSet { set: serialize }),
            Err(e) => Err(e),
        }
    }

    /// Returns Result of UnicodeSet from a single pair of integers defining a range
    ///
    /// `start`: inclusive, `end`: exclusive
    ///
    /// Returns an error if the range is invalid (out of order and out of bounds).
    ///
    /// Example Call: `UnicodeSet::from_range(&0, &15)`
    pub fn from_range(start: &u32, end: &u32) -> Result<UnicodeSet, Box<dyn Error>> {
        if start > end {
            return Err("Range is out of order".into());
        }
        if start < &UNICODESET_MIN || end > &UNICODESET_MAX {
            return Err("Range is out of bounds".into());
        }
        Ok(UnicodeSet {
            set: vec![*start, *end],
        })
    }

    /// Returns UnicodeSet spanning entire Unicode range
    ///
    /// The range spans from `0x0 -> 0x10FFFF` inclusive
    pub fn all() -> UnicodeSet {
        UnicodeSet {
            set: vec![UNICODESET_MIN, UNICODESET_MAX + 1],
        }
    }

    /// Returns UnicodeSet spanning BMP range
    ///
    /// The range spans from `0x0 -> 0xFFFF` inclusive
    pub fn bmp() -> UnicodeSet {
        UnicodeSet {
            set: vec![UNICODESET_MIN, BMP_MAX + 1],
        }
    }
    /// Returns an `Iter` of start and stop `u32` points of the UnicodeSet
    pub fn iter(&self) -> Iter<u32> {
        self.set.iter()
    }

    /// Returns the cardinality of the UnicodeSet
    pub fn size(&self) -> Result<usize, Box<dyn Error>> {
        if self.set.len() < 2 {
            return Err("UnicodeSet length < 2".into());
        }
        let end: u32 = self.iter().skip(1).step_by(2).sum::<u32>();
        let start: u32 = self.iter().step_by(2).sum::<u32>();
        Ok((end - start) as usize)
    }

    /// Returns whether or not the UnicodeSet is empty
    pub fn is_empty(&self) -> bool {
        self.set.len() < 2 // unsure if this is appropriate definition of just self.set.is_empty()
    }

    /// Checks to see the query is in the UnicodeSet
    ///
    /// Runs a binary search in `O(log(n))` where `n` is the number of start and end points
    /// on the set using `std::vec::Vec` implementation
    pub fn contains(&self, query: &u32) -> bool {
        match self.set.binary_search(query) {
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
                    if pos >= self.set.len() {
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
    use super::{parse_serial_string, UnicodeSet, BMP_MAX, UNICODESET_MAX, UNICODESET_MIN};
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
    fn test_parse_serial_string_wrong_order() {
        assert!(parse_serial_string("4 1 0 4 2").is_err());
    }
    #[test]
    fn test_parse_serial_string_single_char_error() {
        assert!(parse_serial_string("4 1 1 2 2").is_err());
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
    #[test]
    fn test_unicodeset_new() {
        let expected = vec![2, 3, 4, 5];
        let actual = UnicodeSet::new("4 2 3 4 5").unwrap().set;
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_unicodeset_new_error() {
        assert!(UnicodeSet::new("3 2 4 3").is_err());
    }
    #[test]
    fn test_unicodeset_from_range() {
        let expected = vec![4, 10];
        let actual = UnicodeSet::from_range(&4, &10).unwrap().set;
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_unicodeset_from_range_bad_order() {
        assert!(UnicodeSet::from_range(&10, &5).is_err());
    }
    #[test]
    fn test_unicodeset_from_range_out_of_bounds() {
        assert!(UnicodeSet::from_range(&0, &0x110000).is_err());
    }
    #[test]
    fn test_unicodeset_all() {
        let expected = vec![UNICODESET_MIN, UNICODESET_MAX + 1];
        assert_eq!(UnicodeSet::all().set, expected);
    }
    #[test]
    fn test_unicodeset_bmp() {
        let expected = vec![UNICODESET_MIN, BMP_MAX + 1];
        assert_eq!(UnicodeSet::bmp().set, expected);
    }
    #[test]
    fn test_unicodeset_contains() {
        let check = UnicodeSet::new("4 2 5 10 15").unwrap();
        assert!(check.contains(&2));
        assert!(check.contains(&4));
        assert!(check.contains(&10));
        assert!(check.contains(&14));
    }
    #[test]
    fn test_unicodeset_contains_false() {
        let check = UnicodeSet::new("4 2 5 10 15").unwrap();
        assert!(!check.contains(&1));
        assert!(!check.contains(&5));
        assert!(!check.contains(&9));
        assert!(!check.contains(&15));
        assert!(!check.contains(&16));
    }
    #[test]
    fn test_unicodeset_size() {
        let check = UnicodeSet::new("4 2 5 10 15").unwrap();
        assert_eq!(8, check.size().unwrap());
        let check = UnicodeSet::all();
        let expected = UNICODESET_MAX + 1 - UNICODESET_MIN;
        assert_eq!(expected as usize, check.size().unwrap());
    }
    #[test]
    fn test_unicodeset_size_error() {
        let check = UnicodeSet { set: vec![0] };
        assert!(check.size().is_err());
    }
    #[test]
    fn test_unicodeset_is_empty() {
        let check = UnicodeSet { set: vec![] };
        assert!(check.is_empty());
        let check = UnicodeSet { set: vec![0] };
        assert!(check.is_empty());
        let check = UnicodeSet::all();
        assert!(!check.is_empty());
    }
}
// impl From<io:: // need to define an error
