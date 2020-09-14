use std::char;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Error};
use std::iter::Iterator;
use std::panic;
use std::path::Path;
use std::u32;

use icu_unicodeset::{UnicodeSet, UnicodeSetBuilder};

//
// Can run with command in root of icu_unicodeset crate:
//   cargo run --bin genprops <ppucd-file>
//

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_line_str_opt(line_result: Result<String, Error>) -> Option<String> {
    if let Ok(line) = line_result {
        Some(line)
    } else {
        None
    }
}

fn split_line(line: &str) -> Vec<&str> {
    line.split(';').collect::<Vec<_>>()
}

fn get_data_line_prop_vals(data_line_parts: &[&str]) -> HashMap<String, String> {
    let mut m = HashMap::new();
    // idx 0 = line type identifier (property, value, defaults, blk, cp)
    // idx 1 = data line code point value or code point range
    let mut line_parts = data_line_parts.to_owned();
    line_parts.drain(0..2);
    for prop_str in line_parts {
        let first_equals_idx = prop_str.find('=');
        if let Some(idx) = first_equals_idx {
            // Properties with an "=" in their string have values associated
            // (ex: enumerated properties).
            let prop_key_val = prop_str.split_at(idx);
            let key = String::from(prop_key_val.0);
            let mut val = String::from(prop_key_val.1);
            val.drain(0..1); // get rid of first "=" that we split on
            m.insert(key, val);
        } else {
            // For properties that don't take values, let their value in the map be the prop name itself
            // This applies to binary properties.
            m.insert(String::from(prop_str), String::from(prop_str));
        }
    }
    m
}

fn is_skip_ppucd_line(line: &str) -> bool {
    line.starts_with('#') || line.starts_with("ucd") || line.is_empty()
}

fn is_property_line(line: &str) -> bool {
    line.starts_with("property;")
}

/// For a property definition line, update the property aliases map.
/// Only operate on binary properties, currently.
fn update_aliases(prop_aliases: &mut HashMap<String, HashSet<String>>, line: &str) {
    // println!("{}", &line);

    let mut line_parts = split_line(&line);
    assert_eq!(&"property", &line_parts[0]);
    let prop_type = &line_parts[1];
    if prop_type == &"Binary" {
        line_parts.drain(0..2);

        // TODO: ask Markus what to do with the property lines that appear to have
        // no canonical name
        // property;Binary;;alnum
        // property;Binary;;blank
        // property;Binary;;graph
        // property;Binary;;print
        // property;Binary;;xdigit
        if line_parts[0] == "" {
            line_parts.drain(0..1);
        }

        let canonical_name = String::from(line_parts[0]);
        let all_names: Vec<String> = line_parts.iter().map(|line| String::from(*line)).collect();
        let all_names_set: HashSet<String> = all_names.into_iter().collect();
        prop_aliases.insert(canonical_name, all_names_set);
    }
}

fn is_defaults_line(line: &str) -> bool {
    line.starts_with("defaults;")
}

fn get_defaults_prop_vals(line: &str) -> HashMap<String, String> {
    let line_parts = split_line(&line);
    assert_eq!(&"defaults", &line_parts[0]);
    get_data_line_prop_vals(&line_parts)
}

fn is_block_line(line: &str) -> bool {
    line.starts_with("block;")
}

fn get_block_range_prop_vals(line: &str) -> (UnicodeSet, HashMap<String, String>) {
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

    let range = if let Ok(range) = range_result {
        range
    } else {
        let inv_list: Vec<u32> = Vec::default();
        UnicodeSet::from_inversion_list(inv_list).unwrap()
    };
    (range, props_vals)
}

fn is_code_point_line(line: &str) -> bool {
    line.starts_with("cp;")
}

fn get_code_point_overrides(line: &str) -> (UnicodeSet, HashMap<String, String>) {
    let line_parts = split_line(&line);
    assert_eq!(&"cp", &line_parts[0]);

    let range_str = &line_parts[1];
    let range_bound_strs = &range_str.split("..").collect::<Vec<_>>();
    let range_result = if range_bound_strs.len() > 1 {
        let range_start: &u32 = &u32::from_str_radix(&range_bound_strs[0], 16).unwrap();
        let range_end: &u32 = &u32::from_str_radix(&range_bound_strs[1], 16).unwrap(); // inclusive end val in PPUCD
        let inv_list_start: u32 = *range_start;
        let inv_list_end: u32 = *range_end + 1;
        let inv_list: Vec<u32> = vec![inv_list_start, inv_list_end];
        UnicodeSet::from_inversion_list(inv_list)
    } else {
        let code_point_str = range_str;
        let code_point: u32 = u32::from_str_radix(code_point_str, 16).unwrap();
        let inv_list: Vec<u32> = vec![code_point, code_point + 1];
        UnicodeSet::from_inversion_list(inv_list)
    };

    let props_vals = get_data_line_prop_vals(&line_parts);

    let range = if let Ok(range) = range_result {
        range
    } else {
        let inv_list: Vec<u32> = Vec::default();
        UnicodeSet::from_inversion_list(inv_list).unwrap()
    };
    (range, props_vals)
}

fn get_code_point_prop_vals(
    code_point: u32,
    code_point_overrides: &HashMap<UnicodeSet, HashMap<String, String>>,
    blocks: &HashMap<UnicodeSet, HashMap<String, String>>,
    defaults: &HashMap<String, String>,
) -> HashMap<String, String> {
    // create the map of applicable property values, and initialize it to
    // defaults for the whole Unicode range
    let mut prop_vals: HashMap<String, String> = HashMap::new();
    prop_vals.clone_from(defaults);

    // determine if this code point matches any of the block-wide defaults
    // ("overrides")
    for (range, block_prop_vals) in blocks {
        if range.contains(char::from_u32(code_point).unwrap()) {
            let block_prop_vals_clone = block_prop_vals.iter().map(|(k, v)| (k.clone(), v.clone()));
            prop_vals.extend(block_prop_vals_clone);
        }
    }

    // finally, apply the overrides for this code point
    for (range, code_point_prop_vals) in code_point_overrides {
        if range.contains(char::from_u32(code_point).unwrap()) {
            let code_point_override_prop_vals_clone = code_point_prop_vals
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()));
            prop_vals.extend(code_point_override_prop_vals_clone);
        }
    }

    prop_vals
}

fn get_binary_prop_unisets(
    prop_aliases: &HashMap<String, HashSet<String>>,
    code_points: &HashMap<u32, HashMap<String, String>>,
) -> HashMap<String, UnicodeSet> {
    let mut m: HashMap<String, UnicodeSet> = HashMap::new();

    for (canonical_name, all_names) in prop_aliases {
        let mut uniset_builder = UnicodeSetBuilder::new();
        for (code_point, code_point_prop_vals) in code_points {
            let code_point_prop_names: HashSet<String> =
                code_point_prop_vals.keys().cloned().collect();
            if !all_names.is_disjoint(&code_point_prop_names) {
                uniset_builder.add_char(char::from_u32(*code_point).unwrap());
            }
        }
        if !&uniset_builder.is_empty() {
            let uniset = uniset_builder.build();
            m.insert(canonical_name.clone(), uniset);
        }
    }

    m
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    if let Ok(lines) = read_lines(filename) {
        let line_opts = lines.map(get_line_str_opt);
        let line_strs = line_opts.flatten();
        let parseable_lines = line_strs.filter(|line| !is_skip_ppucd_line(line));

        let mut prop_aliases: HashMap<String, HashSet<String>> = HashMap::new();

        let mut defaults: HashMap<String, String> = HashMap::new();

        // TODO: need advice - how should we not add Hash and Eq from UnicodeSet unless we need it?
        let mut blocks: HashMap<UnicodeSet, HashMap<String, String>> = HashMap::new();

        let mut code_point_overrides: HashMap<UnicodeSet, HashMap<String, String>> = HashMap::new();

        let mut code_points: HashMap<u32, HashMap<String, String>> = HashMap::new();

        for line in parseable_lines {
            if is_property_line(&line) {
                update_aliases(&mut prop_aliases, &line);
            } else if is_defaults_line(&line) {
                defaults = get_defaults_prop_vals(&line);
            } else if is_block_line(&line) {
                let (range, prop_vals) = get_block_range_prop_vals(&line);
                blocks.insert(range, prop_vals);
            } else if is_code_point_line(&line) {
                // record code point override vals directly from line
                let (code_point_range, prop_vals) = get_code_point_overrides(&line);
                code_point_overrides.insert(code_point_range, prop_vals);

                // compute final code point property vals after applying all
                // levels of overrides
                // can't clone UnicodeSet, so recomputing code point range
                let (code_point_range, _) = get_code_point_overrides(&line);
                for code_point_char in code_point_range.iter() {
                    let code_point = code_point_char as u32;
                    let code_point_prop_vals = get_code_point_prop_vals(
                        code_point,
                        &code_point_overrides,
                        &blocks,
                        &defaults,
                    );
                    code_points.insert(code_point, code_point_prop_vals);
                }
            }
        }

        let binary_prop_unisets: HashMap<String, UnicodeSet> =
            get_binary_prop_unisets(&prop_aliases, &code_points);

        for (canonical_name, uniset) in binary_prop_unisets {
            println!("canonical name of binary property = [{}]", canonical_name);
            println!("{:?}", uniset); // pretty print UnicodeSet
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn skip_ppucd_line_test() {
        assert_eq!(true, is_skip_ppucd_line(&String::from("ucd;13.0.0")));
        assert_eq!(
            false,
            is_skip_ppucd_line(&String::from("value;InSC;Gemination_Mark;Gemination_Mark"))
        );
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
        &exp_prop_vals.insert(String::from("bc"), String::from("WS"));
        &exp_prop_vals.insert(String::from("gc"), String::from("Zs"));
        &exp_prop_vals.insert(String::from("lb"), String::from("SP"));
        &exp_prop_vals.insert(String::from("na"), String::from("SPACE"));
        &exp_prop_vals.insert(String::from("Name_Alias"), String::from("abbreviation=SP"));
        &exp_prop_vals.insert(String::from("Pat_WS"), String::from("Pat_WS"));
        &exp_prop_vals.insert(String::from("SB"), String::from("SP"));
        &exp_prop_vals.insert(String::from("WB"), String::from("WSegSpace"));
        &exp_prop_vals.insert(String::from("WSpace"), String::from("WSpace"));

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
        let line =
            String::from("block;0000..007F;age=1.1;blk=ASCII;ea=Na;gc=Cc;Gr_Base;lb=AL;sc=Zyyy");

        let mut exp_prop_vals: HashMap<String, String> = HashMap::new();
        &exp_prop_vals.insert(String::from("age"), String::from("1.1"));
        &exp_prop_vals.insert(String::from("blk"), String::from("ASCII"));
        &exp_prop_vals.insert(String::from("ea"), String::from("Na"));
        &exp_prop_vals.insert(String::from("gc"), String::from("Cc"));
        &exp_prop_vals.insert(String::from("Gr_Base"), String::from("Gr_Base"));
        &exp_prop_vals.insert(String::from("lb"), String::from("AL"));
        &exp_prop_vals.insert(String::from("sc"), String::from("Zyyy"));

        let exp_range_inv_list: Vec<u32> = vec![0, 128]; // PPUCD: end val is inclusive;
                                                         // inversion list: end val exclusive
        let exp_range = UnicodeSet::from_inversion_list(exp_range_inv_list).unwrap();

        let (range, prop_vals) = get_block_range_prop_vals(&line);

        assert_eq!(exp_range, range);
        assert_eq!(exp_prop_vals, prop_vals);
    }

    #[test]
    fn code_point_overrides_test() {
        let defaults_line = String::from("defaults;0000..10FFFF;age=NA;bc=L;blk=NB;bpt=n;cf=<code point>;dm=<code point>;dt=None;ea=N;FC_NFKC=<code point>;gc=Cn;GCB=XX;gcm=Cn;hst=NA;InPC=NA;InSC=Other;jg=No_Joining_Group;jt=U;lb=XX;lc=<code point>;NFC_QC=Y;NFD_QC=Y;NFKC_CF=<code point>;NFKC_QC=Y;NFKD_QC=Y;nt=None;SB=XX;sc=Zzzz;scf=<code point>;scx=<script>;slc=<code point>;stc=<code point>;suc=<code point>;tc=<code point>;uc=<code point>;vo=R;WB=XX");
        let block_line =
            String::from("block;0000..007F;age=1.1;blk=ASCII;ea=Na;gc=Cc;Gr_Base;lb=AL;sc=Zyyy");
        let code_point_line = String::from("cp;0020;bc=WS;gc=Zs;lb=SP;na=SPACE;Name_Alias=abbreviation=SP;Pat_WS;SB=SP;WB=WSegSpace;WSpace");

        let exp_code_point: u32 = 32;
        let mut exp_code_point_prop_vals: HashMap<String, String> = HashMap::new();
        &exp_code_point_prop_vals.insert(String::from("WB"), String::from("WSegSpace"));
        &exp_code_point_prop_vals.insert(String::from("FC_NFKC"), String::from("<code point>"));
        &exp_code_point_prop_vals.insert(String::from("GCB"), String::from("XX"));
        &exp_code_point_prop_vals.insert(String::from("gcm"), String::from("Cn"));
        &exp_code_point_prop_vals.insert(String::from("scx"), String::from("<script>"));
        &exp_code_point_prop_vals.insert(String::from("bc"), String::from("WS"));
        &exp_code_point_prop_vals.insert(String::from("lc"), String::from("<code point>"));
        &exp_code_point_prop_vals.insert(String::from("NFKC_CF"), String::from("<code point>"));
        &exp_code_point_prop_vals.insert(String::from("InPC"), String::from("NA"));
        &exp_code_point_prop_vals.insert(String::from("lb"), String::from("SP"));
        &exp_code_point_prop_vals.insert(String::from("NFC_QC"), String::from("Y"));
        &exp_code_point_prop_vals.insert(String::from("SB"), String::from("SP"));
        &exp_code_point_prop_vals
            .insert(String::from("Name_Alias"), String::from("abbreviation=SP"));
        &exp_code_point_prop_vals.insert(String::from("dt"), String::from("None"));
        &exp_code_point_prop_vals.insert(String::from("ea"), String::from("Na"));
        &exp_code_point_prop_vals.insert(String::from("nt"), String::from("None"));
        &exp_code_point_prop_vals.insert(String::from("InSC"), String::from("Other"));
        &exp_code_point_prop_vals.insert(String::from("NFD_QC"), String::from("Y"));
        &exp_code_point_prop_vals.insert(String::from("bpt"), String::from("n"));
        &exp_code_point_prop_vals.insert(String::from("jg"), String::from("No_Joining_Group"));
        &exp_code_point_prop_vals.insert(String::from("gc"), String::from("Zs"));
        &exp_code_point_prop_vals.insert(String::from("vo"), String::from("R"));
        &exp_code_point_prop_vals.insert(String::from("NFKD_QC"), String::from("Y"));
        &exp_code_point_prop_vals.insert(String::from("NFKC_QC"), String::from("Y"));
        &exp_code_point_prop_vals.insert(String::from("blk"), String::from("ASCII"));
        &exp_code_point_prop_vals.insert(String::from("uc"), String::from("<code point>"));
        &exp_code_point_prop_vals.insert(String::from("suc"), String::from("<code point>"));
        &exp_code_point_prop_vals.insert(String::from("scf"), String::from("<code point>"));
        &exp_code_point_prop_vals.insert(String::from("slc"), String::from("<code point>"));
        &exp_code_point_prop_vals.insert(String::from("age"), String::from("1.1"));
        &exp_code_point_prop_vals.insert(String::from("na"), String::from("SPACE"));
        &exp_code_point_prop_vals.insert(String::from("cf"), String::from("<code point>"));
        &exp_code_point_prop_vals.insert(String::from("hst"), String::from("NA"));
        &exp_code_point_prop_vals.insert(String::from("dm"), String::from("<code point>"));
        &exp_code_point_prop_vals.insert(String::from("sc"), String::from("Zyyy"));
        &exp_code_point_prop_vals.insert(String::from("stc"), String::from("<code point>"));
        &exp_code_point_prop_vals.insert(String::from("jt"), String::from("U"));
        &exp_code_point_prop_vals.insert(String::from("tc"), String::from("<code point>"));
        &exp_code_point_prop_vals.insert(String::from("Gr_Base"), String::from("Gr_Base"));
        &exp_code_point_prop_vals.insert(String::from("WSpace"), String::from("WSpace"));
        &exp_code_point_prop_vals.insert(String::from("Pat_WS"), String::from("Pat_WS"));
        let mut defaults: HashMap<String, String> = HashMap::new();
        defaults = get_defaults_prop_vals(&defaults_line);
        let mut exp_code_points: HashMap<u32, HashMap<String, String>> = HashMap::new();
        &exp_code_points.insert(exp_code_point, exp_code_point_prop_vals);

        let mut blocks: HashMap<UnicodeSet, HashMap<String, String>> = HashMap::new();
        let (range, prop_vals) = get_block_range_prop_vals(&block_line);
        &blocks.insert(range, prop_vals);

        let mut code_point_overrides: HashMap<UnicodeSet, HashMap<String, String>> = HashMap::new();
        let (code_point_range, prop_vals) = get_code_point_overrides(&code_point_line);
        &code_point_overrides.insert(code_point_range, prop_vals);

        let (code_point_range, _) = get_code_point_overrides(&code_point_line);
        let mut code_points: HashMap<u32, HashMap<String, String>> = HashMap::new();
        for code_point_char in code_point_range.iter() {
            let code_point = code_point_char as u32;
            let code_point_prop_vals =
                get_code_point_prop_vals(code_point, &code_point_overrides, &blocks, &defaults);
            &code_points.insert(code_point, code_point_prop_vals);
        }

        assert_eq!(&exp_code_points, &code_points);
    }
}
