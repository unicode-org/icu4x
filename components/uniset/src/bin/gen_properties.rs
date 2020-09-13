use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, Error};
use std::u32;
use std::iter::Iterator;
use std::path::Path;

use icu_unicodeset::UnicodeSet;


//
// Can run with command in root of icu_unicodeset crate:
//   cargo run --bin genprops
//

fn get_line_str_opt(line_result: Result<String, Error>) -> Option<String> {
    if let Ok(line) = line_result {
        Some(line)
    } else {
        None
    }
}

fn split_line(line: &String) -> Vec<&str> {
    line.split(";").collect::<Vec<_>>()
}

fn get_data_line_prop_vals(data_line_parts: &Vec<&str>) -> HashMap<String, String> {
    let mut m = HashMap::new();
    // idx 0 = line type identifier (property, value, defaults, blk, cp)
    // idx 1 = data line code point value or code point range
    let mut line_parts = data_line_parts.clone();
    line_parts.drain(0..2);
    for prop_str in line_parts {
        let first_equals_idx = prop_str.find("=");
        if let Some(idx) = first_equals_idx {
            let prop_key_val = prop_str.split_at(idx);
            let key = String::from(prop_key_val.0);
            let mut val = String::from(prop_key_val.1);
            val.drain(0..1); // get rid of first "=" that we split on
            &m.insert(key, val);
        }
    }
    m
}

fn is_skip_ppucd_line(line: &String) -> bool {
    line.starts_with("#")
    || line.starts_with("ucd")
    || line.trim().len() == 0
}

fn is_property_line(line: &String) -> bool {
    line.starts_with("property;")
}

fn update_aliases(prop_aliases: &mut HashMap<String, HashSet<String>>, line: &String) {
    // println!("{}", &line);

    let mut line_parts = split_line(&line);
    assert_eq!(&"property", &line_parts[0]);
    let prop_type = &line_parts[1];
    if prop_type == &"Binary" {
        &line_parts.drain(0..2);

        // TODO: ask Markus what to do with the property lines that appear to have
        // no canonical name
        // property;Binary;;alnum
        // property;Binary;;blank
        // property;Binary;;graph
        // property;Binary;;print
        // property;Binary;;xdigit
        if &line_parts[0] == &"" {
            &line_parts.drain(0..1);
        }

        let canonical_name = String::from(*&line_parts[0]);
        let all_names: Vec<String> = line_parts.iter().map(|line| String::from(*line)).collect();
        let all_names_set: HashSet<String> = all_names.into_iter().collect();
        &prop_aliases.insert(canonical_name, all_names_set);
    }
}

fn is_defaults_line(line: &String) -> bool {
    line.starts_with("defaults;")
}

fn get_defaults_prop_vals(line: &String) -> HashMap<String, String> {
    let line_parts = split_line(&line);
    assert_eq!(&"defaults", &line_parts[0]);
    let props_vals = get_data_line_prop_vals(&line_parts);
    props_vals
}

fn get_block_range_prop_vals(line: &String) -> (UnicodeSet, HashMap<String, String>) {
    let line_parts = split_line(&line);
    assert_eq!(&"block", &line_parts[0]);

    let range_str = &line_parts[1];
    let range_bound_strs = range_str.split("..").collect::<Vec<_>>();
    let range_start: &u32 = &u32::from_str_radix(&range_bound_strs[0], 16).unwrap();
    let range_end: &u32 = &u32::from_str_radix(&range_bound_strs[1], 16).unwrap(); // inclusive end val in PPUCD
    let inv_list_start: u32 = *range_start;
    let inv_list_end: u32 = *range_end + 1;
    let inv_list: Vec<u32> = vec![inv_list_start, inv_list_end];
    let range_result = UnicodeSet::from_inversion_list(inv_list);

    let props_vals = get_data_line_prop_vals(&line_parts);

    let range =
        if let Ok(range) = range_result {
            range
        } else {
            let inv_list: Vec<u32> = Vec::default();
            let empty_range = UnicodeSet::from_inversion_list(inv_list).unwrap();
            empty_range
        };
    return (range, props_vals);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let filename = "ppucd-test.txt";

    // println!("In file {}", filename);

    if let Ok(lines) = read_lines(filename) {

        let line_opts = lines.map(get_line_str_opt);
        let line_strs = line_opts.flatten();
        let parseable_lines = line_strs.filter(|line| !is_skip_ppucd_line(line));

        let mut prop_aliases: HashMap<String, HashSet<String>> = HashMap::new();
        
        let mut defaults: HashMap<String, String> = HashMap::new();

        for line in parseable_lines {
            if is_property_line(&line) {
                update_aliases(&mut prop_aliases, &line);
            } else if is_defaults_line(&line) {
                defaults = get_defaults_prop_vals(&line);
            } else {
                println!("{}", &line);
            }
        }

        // let upper_aliases_opt: Option<&HashSet<String>> = prop_aliases.get("Upper");
        // if let Some(upper_aliases) = upper_aliases_opt {
        //     for alias in upper_aliases {
        //         println!("Upper aka: {}", alias);
        //     }   
        // }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn skip_ppucd_line_test() {
        assert_eq!(true, is_skip_ppucd_line(&String::from("ucd;13.0.0")));
        assert_eq!(false, is_skip_ppucd_line(&String::from("value;InSC;Gemination_Mark;Gemination_Mark")));
    }

    #[test]
    fn split_line_test() {
        let line = String::from("cp;0020;bc=WS;gc=Zs;lb=SP;na=SPACE;Name_Alias=abbreviation=SP;Pat_WS;SB=SP;WB=WSegSpace;WSpace");
        let exp_parts = vec![
            String::from("cp"),
            String::from("0020"),
            String::from("bc=WS"),
            String::from("gc=Zs"),
            String::from("lb=SP"),
            String::from("na=SPACE"),
            String::from("Name_Alias=abbreviation=SP"),
            String::from("Pat_WS"),
            String::from("SB=SP"),
            String::from("WB=WSegSpace"),
            String::from("WSpace"),
        ];
        let line_parts = split_line(&line);
        assert_eq!(exp_parts, line_parts);
    }

    #[test]
    fn get_data_line_prop_vals_test() {
        let line = String::from("cp;0020;bc=WS;gc=Zs;lb=SP;na=SPACE;Name_Alias=abbreviation=SP;Pat_WS;SB=SP;WB=WSegSpace;WSpace");

        let mut exp_prop_vals: HashMap<String, String> = HashMap::new();
        &exp_prop_vals.insert(
            String::from("bc"),
            String::from("WS"));
        &exp_prop_vals.insert(
            String::from("gc"),
            String::from("Zs"));
        &exp_prop_vals.insert(    
            String::from("lb"),
            String::from("SP"));
        &exp_prop_vals.insert(
            String::from("na"),
            String::from("SPACE"));
        &exp_prop_vals.insert(
            String::from("Name_Alias"),
            String::from("abbreviation=SP"));
        // &exp_prop_vals.insert(
        //     String::from("Pat_WS"),
        //     String::from("Pat_WS"),
        &exp_prop_vals.insert(
            String::from("SB"),
            String::from("SP"));
        &exp_prop_vals.insert(
            String::from("WB"),
            String::from("WSegSpace"));
        // &exp_prop_vals.insert(
        //     String::from("WSpace"),
        //     String::from("WSpace"));

        let line_parts = split_line(&line);
        let act_props_vals = get_data_line_prop_vals(&line_parts);

        assert_eq!(exp_prop_vals, act_props_vals);
    }

    #[test]
    fn update_aliases_test() {
        let line = String::from("property;Binary;Upper;Uppercase");
        let mut prop_aliases: HashMap<String, HashSet<String>> = HashMap::new();

        update_aliases(&mut prop_aliases, &line);

        let mut exp_prop_aliases: HashMap<String, HashSet<String>> = HashMap::new();
        let exp_aliases = vec![String::from("Uppercase"), String::from("Upper")]; // order won't matter
        let exp_aliases: HashSet<String> = exp_aliases.into_iter().collect();
        &exp_prop_aliases.insert(String::from("Upper"), exp_aliases);

        assert_eq!(&exp_prop_aliases, &prop_aliases);
    }

    #[test]
    fn block_range_prop_vals_test() {
        let line = String::from("block;0000..007F;age=1.1;blk=ASCII;ea=Na;gc=Cc;Gr_Base;lb=AL;sc=Zyyy");

        let mut exp_prop_vals: HashMap<String, String> = HashMap::new();
        &exp_prop_vals.insert(
            String::from("age"),
            String::from("1.1"));
        &exp_prop_vals.insert(
            String::from("blk"),
            String::from("ASCII"));
        &exp_prop_vals.insert(
            String::from("ea"),
            String::from("Na"));
        &exp_prop_vals.insert(
            String::from("gc"),
            String::from("Cc"));
        &exp_prop_vals.insert(
            String::from("lb"),
            String::from("AL"));
        &exp_prop_vals.insert(
            String::from("sc"),
            String::from("Zyyy"));

        let exp_range_inv_list: Vec<u32> = vec![0, 128]; // PPUCD: end val is inclusive;
                                                         // inversion list: end val exclusive
        let exp_range = UnicodeSet::from_inversion_list(exp_range_inv_list).unwrap();

        let (range, prop_vals) = get_block_range_prop_vals(&line);

        assert_eq!(exp_range, range);
        assert_eq!(exp_prop_vals, prop_vals);
    }
}