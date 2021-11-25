// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! TODO: module documentation
#![warn(missing_docs)]

mod error;
mod internals;
pub mod provider;

pub use error::Error as CaseMappingError;
pub use internals::CaseMapping;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::collections::HashSet;

//     pub(crate) fn get_case_mapping() -> CaseMapping<'static> {
// 	let trie = super::test_data::get_case_trie();
// 	let exceptions = CaseMappingExceptions {
// 	    raw: ZeroVec::from_slice(&test_data::UCASE_PROPS_EXCEPTIONS),
// 	};
// 	let unfold = CaseMappingUnfoldData::try_from_raw(&test_data::UCASE_PROPS_UNFOLD).unwrap();
// 	CaseMapping { trie, exceptions, unfold }
//     }

//     #[test]
//     fn test_upper() {
// 	let case_mapping = get_case_mapping();
// 	assert_eq!(case_mapping.to_upper('a'), 'A');
// 	assert_eq!(case_mapping.to_upper('\u{1c4}'), '\u{1c4}');
// 	assert_eq!(case_mapping.to_title('\u{1c4}'), '\u{1c5}');
// 	assert_eq!(case_mapping.to_lower('\u{1c4}'), '\u{1c6}');
// 	assert_eq!(case_mapping.to_upper('\u{1c5}'), '\u{1c4}');
// 	assert_eq!(case_mapping.to_title('\u{1c5}'), '\u{1c5}');
// 	assert_eq!(case_mapping.to_lower('\u{1c5}'), '\u{1c6}');
// 	assert_eq!(case_mapping.to_upper('\u{1c6}'), '\u{1c4}');
// 	assert_eq!(case_mapping.to_title('\u{1c6}'), '\u{1c5}');
// 	assert_eq!(case_mapping.to_lower('\u{1c6}'), '\u{1c6}');
//     }

//     #[test]
//     fn test_softdotted() {
// 	let case_mapping = get_case_mapping();
// 	assert_eq!(case_mapping.is_soft_dotted('a'), false);
// 	assert_eq!(case_mapping.dot_type('i'), DotType::SoftDotted);
// 	assert_eq!(case_mapping.dot_type('j'), DotType::SoftDotted);
//     }

//     #[derive(Eq,PartialEq,Default)]
//     struct SimpleSet {
// 	chars: HashSet<char>,
// 	strings: HashSet<String>,
//     }

//     impl SimpleSet {
// 	pub fn chars(&self) -> Vec<char> {
// 	    let mut result: Vec<char> = self.chars.iter().map(|&c| c).collect();
// 	    result.sort();
// 	    result
// 	}
// 	pub fn strings(&self) -> Vec<String> {
// 	    let mut result: Vec<String> = self.strings.iter().map(|c| c.clone()).collect();
// 	    result.sort();
// 	    result
// 	}
// 	pub fn clear(&mut self) {
// 	    self.chars.clear();
// 	    self.strings.clear();
// 	}
//     }

//     impl SetAdder for SimpleSet {
// 	fn add_char(&mut self, c: char) {
// 	    self.chars.insert(c);
// 	}
// 	fn add_string(&mut self, s: &str) {
// 	    self.strings.insert(String::from(s));
// 	}
// 	fn add_range(&mut self, start: char, end: char) {
// 	    for c in start..=end {
// 		self.chars.insert(c);
// 	    }
// 	}
//     }

//     #[test]
//     fn test_closure() {
// 	let case_mapping = get_case_mapping();
// 	let mut closure = SimpleSet::default();

// 	case_mapping.add_case_closure('i', &mut closure);
// 	assert_eq!(closure.chars(), vec!['I']);
// 	assert!(closure.strings().is_empty());
// 	closure.clear();

// 	case_mapping.add_case_closure('k', &mut closure);
// 	assert_eq!(closure.chars(), vec!['K', '\u{212a}']); // Kelvin sign
// 	assert!(closure.strings().is_empty());
// 	closure.clear();

// 	case_mapping.add_case_closure('s', &mut closure);
// 	assert_eq!(closure.chars(), vec!['S', '\u{17f}']); // long S
// 	assert!(closure.strings().is_empty());
// 	closure.clear();

// 	case_mapping.add_case_closure('\u{df}', &mut closure); // lowercase sharp s
// 	assert_eq!(closure.chars(), vec!['\u{1e9e}']); // uppercase sharp s
// 	assert_eq!(closure.strings(), vec![String::from("ss")]);
// 	closure.clear();
//     }
// }
