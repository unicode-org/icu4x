// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_segmenter::LineBreakIterator;
use icu_segmenter::LineBreakIteratorLatin1;
use icu_segmenter::LineBreakIteratorUtf16;
use std::char;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::u32;

struct TestContentIterator {
    reader: std::io::BufReader<File>,
}

struct TestData {
    original_line: String,
    utf8_vec: Vec<char>,
    utf16_vec: Vec<u16>,
    latin1_vec: Vec<u8>,
    break_result_utf8: Vec<usize>,
    break_result_utf16: Vec<usize>,
    break_result_latin1: Option<Vec<usize>>,
}

impl TestContentIterator {
    pub fn new(filename: &str) -> Self {
        let f = File::open(filename);
        Self {
            reader: BufReader::new(f.unwrap()),
        }
    }
}

impl Iterator for TestContentIterator {
    type Item = TestData;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut line = String::new();
            let len = self.reader.read_line(&mut line).ok()?;
            if len == 0 {
                // EOF
                return None;
            }
            if line.starts_with('#') {
                // Comment
                continue;
            }

            let mut r = line.split('#');
            let r = r.next();
            let v: Vec<_> = r.unwrap().split_ascii_whitespace().collect();
            let mut char_break: Vec<_> = Vec::new();
            let mut u8_break: Vec<_> = Vec::new();
            let mut u16_break: Vec<_> = Vec::new();
            let mut char_vec: Vec<_> = Vec::new();
            let mut u8_vec: Vec<_> = Vec::new();
            let mut u16_vec: Vec<_> = Vec::new();
            let mut count = 0;

            let mut char_len = 0;
            let mut u8_len = 0;
            let mut u16_len = 0;

            let mut ascii_only = true;
            loop {
                if count >= v.len() {
                    break;
                }
                if count % 2 == 1 {
                    let ch = char::from_u32(u32::from_str_radix(v[count], 16).unwrap()).unwrap();
                    char_vec.push(ch);
                    char_len += ch.len_utf8();

                    if ch as u32 >= 0x100 {
                        ascii_only = false;
                    } else {
                        u8_vec.push(ch as u8);
                        u8_len += 1;
                    }

                    if ch as u32 >= 0x10000 {
                        u16_vec.push((((ch as u32 - 0x10000) >> 10) | 0xd800) as u16);
                        u16_vec.push((((ch as u32) & 0x3ff) | 0xdc00) as u16);
                        u16_len += 2;
                    } else {
                        u16_vec.push(ch as u16);
                        u16_len += 1;
                    }
                } else if v[count] != "\u{00d7}" {
                    assert_eq!(v[count], "\u{00f7}");
                    char_break.push(char_len);
                    u8_break.push(u8_len);
                    u16_break.push(u16_len);
                }
                count += 1
            }
            return Some(Self::Item {
                original_line: line,
                utf8_vec: char_vec,
                utf16_vec: u16_vec,
                latin1_vec: u8_vec,
                break_result_utf8: char_break,
                break_result_utf16: u16_break,
                break_result_latin1: if ascii_only { Some(u8_break) } else { None },
            });
        }
    }
}

#[test]
fn run_line_break_test() {
    let test_iter = TestContentIterator::new("./tests/testdata/LineBreakTest.txt");
    for test in test_iter {
        let s: String = test.utf8_vec.into_iter().collect();
        let iter = LineBreakIterator::new(&s);
        let result: Vec<usize> = iter.collect();
        assert_eq!(result, test.break_result_utf8, "{}", test.original_line);

        let iter = LineBreakIteratorUtf16::new(&test.utf16_vec);
        let result: Vec<usize> = iter.collect();
        assert_eq!(
            result, test.break_result_utf16,
            "UTF16: {}",
            test.original_line
        );

        // Test data is Latin-1 character only, it can run for Latin-1 segmenter test.
        if let Some(break_result_latin1) = test.break_result_latin1 {
            let iter = LineBreakIteratorLatin1::new(&test.latin1_vec);
            let result: Vec<usize> = iter.collect();
            assert_eq!(
                result, break_result_latin1,
                "Latin1: {}",
                test.original_line
            );
        }
    }
}
