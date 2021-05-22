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

#[test]
fn run_line_break_test() {
    // no failed tests
    let failed = [];

    let f = File::open("./tests/testdata/LineBreakTest.txt");
    let f = BufReader::new(f.unwrap());
    for line in f.lines() {
        let line = line.unwrap();
        if line.starts_with('#') {
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
        let s: String = char_vec.into_iter().collect();
        let mut iter = LineBreakIterator::new(&s);
        if failed.contains(&&s.as_str()) {
            assert_ne!(iter.next(), Some(char_break[0]), "{}", line);
            continue;
        }

        {
            println!("UTF8: {}", line);
            let result: Vec<usize> = iter.collect();
            assert_eq!(result, char_break, "{}", line);
        }

        {
            println!("UTF16: {}", line);
            let iter = LineBreakIteratorUtf16::new(&u16_vec);
            let result: Vec<usize> = iter.collect();
            assert_eq!(result, u16_break, "UTF16: {}", line);
        }

        if ascii_only {
            println!("Latin1: {}", line);
            let iter = LineBreakIteratorLatin1::new(&u8_vec);
            let result: Vec<usize> = iter.collect();
            assert_eq!(result, u8_break, "Latin1: {}", line);
        }
    }
}
