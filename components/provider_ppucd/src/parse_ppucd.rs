// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use std::collections::{HashMap, HashSet};
use std::iter::Iterator;
use std::panic;
use std::u32;

use crate::structs::{UnicodeProperties, UnicodeProperty};
use icu_uniset::{UnicodeSet, UnicodeSetBuilder};

fn split_line(line: &str) -> Vec<&str> {
    line.split(';').collect::<Vec<_>>()
}

/// For lines in PPUCD that begin with "defaults", "blk", and "cp" -- the lines
/// that convey property values for code points (or ranges thereof) -- return
/// the property information as a map. Enumerated properties' names and values
/// become keys and values in the map, while the binary property strings are
/// stored as the key and value in the map. Binary property strings are stored
/// as-is, so no parsing occurs here when the "false" state of a binary
/// property is represented with a minus ("-") prefix before the name.
fn get_data_line_prop_vals<'s>(data_line_parts: &[&'s str]) -> HashMap<&'s str, &'s str> {
    let mut m = HashMap::new();
    // idx 0 = line type identifier (property, value, defaults, blk, cp)
    // idx 1 = data line code point value or code point range
    let line_parts = &data_line_parts[2..];
    for prop_str in line_parts {
        let first_equals_idx = prop_str.find('=');
        if let Some(idx) = first_equals_idx {
            // Properties with an "=" in their string have values associated
            // (ex: enumerated properties).
            m.insert(&prop_str[0..idx], &prop_str[idx + 1..]);
        } else {
            // For properties that don't take values, let their value in the map be the prop name itself
            // This applies to binary properties.
            m.insert(prop_str, prop_str);
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
fn update_aliases<'s>(prop_aliases: &mut HashMap<&'s str, HashSet<&'s str>>, line: &'s str) {
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
        if line_parts[0].is_empty() {
            line_parts.drain(0..1);
        }

        let canonical_name = line_parts[0];
        let all_names: Vec<&'s str> = line_parts.iter().copied().collect();
        let all_names_set: HashSet<&'s str> = all_names.into_iter().collect();
        prop_aliases.insert(canonical_name, all_names_set);
    }
}

fn is_defaults_line(line: &str) -> bool {
    line.starts_with("defaults;")
}

/// Return the property key-value information represented in the "defaults"
/// line as a map. "defaults" is like the base level of overrides of property
/// values for all code points in PPUCD.
fn get_defaults_prop_vals<'s>(line: &'s str) -> HashMap<&'s str, &'s str> {
    let line_parts = split_line(&line);
    assert_eq!(&"defaults", &line_parts[0]);
    get_data_line_prop_vals(&line_parts)
}

fn is_block_line(line: &str) -> bool {
    line.starts_with("block;")
}

/// Return the property key-value information represented in a "blk"
/// line as a map. "blocks" represent overrides of property values for code
/// points in PPUCD within a Unicode block above the "defaults" values.
fn get_block_range_prop_vals(line: &str) -> (UnicodeSet, HashMap<&str, &str>) {
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

/// Return the property key-value information represented in a "cp"
/// line as a map. "cp" represents overrides of property values for a code
/// point (or range of code points) that are layered above "blk" and "defaults".
fn get_code_point_overrides(line: &str) -> (UnicodeSet, HashMap<&str, &str>) {
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

/// For a single code point, apply the overrides from the PPUCD lines
/// "defaults", "blk", and "cp".
fn get_code_point_prop_vals<'s>(
    code_point: u32,
    code_point_overrides: &HashMap<UnicodeSet, HashMap<&'s str, &'s str>>,
    blocks: &HashMap<UnicodeSet, HashMap<&'s str, &'s str>>,
    defaults: &HashMap<&'s str, &'s str>,
) -> HashMap<&'s str, &'s str> {
    // create the map of applicable property values, and initialize it to
    // defaults for the whole Unicode range
    let mut prop_vals: HashMap<&'s str, &'s str> = HashMap::new();
    prop_vals.clone_from(defaults);

    // determine if this code point matches any of the block-wide defaults
    // ("overrides")
    for (range, block_prop_vals) in blocks {
        if range.contains_u32(code_point) {
            prop_vals.extend(block_prop_vals);
        }
    }

    // finally, apply the overrides for this code point
    for (range, code_point_prop_vals) in code_point_overrides {
        if range.contains_u32(code_point) {
            prop_vals.extend(code_point_prop_vals);
        }
    }

    prop_vals
}

/// Return a map of property names to UnicodeSets when given a map of known
/// binary property names and the assigned property key-value information for
/// each code point in PPUCD. This function effectively changes the "grouping"
/// dimension of code point <-> property key-value information from the
/// grouping by code point (as given by `code_points`) to a grouping by
/// property name.
fn get_binary_prop_unisets<'s>(
    prop_aliases: &HashMap<&'s str, HashSet<&'s str>>,
    code_points: &HashMap<u32, HashMap<&'s str, &'s str>>,
) -> HashMap<&'s str, UnicodeSet> {
    let mut m: HashMap<&'s str, UnicodeSet> = HashMap::new();

    for (canonical_name, all_names) in prop_aliases {
        let mut uniset_builder = UnicodeSetBuilder::new();
        'outer: for (code_point, code_point_prop_vals) in code_points {
            for prop_key in code_point_prop_vals.keys() {
                if all_names.contains(prop_key) {
                    uniset_builder.add_u32(*code_point);
                    continue 'outer;
                }
            }
        }
        if !&uniset_builder.is_empty() {
            let uniset = uniset_builder.build();
            m.insert(canonical_name, uniset);
        }
    }

    m
}

/// Parse a whole PPUCD file that was loaded into a `String` and return a
/// struct of the binary and enumerated property inversion lists.
/// Note: even though `UnicodeProperties` stores a sequential data structure of
/// the `UnicodeProperty` struct, there is no inherent ordering of the entries.
pub fn parse<'s>(s: &'s str) -> UnicodeProperties<'s> {
    let lines: std::str::Lines = s.lines();

    let parseable_lines = lines.filter(|line| !is_skip_ppucd_line(line));

    let mut prop_aliases: HashMap<&'s str, HashSet<&'s str>> = HashMap::new();

    let mut defaults: HashMap<&'s str, &'s str> = HashMap::new();

    // UnicodeSet is used to store the code point range described in a PPUCD
    // `block` line
    let mut blocks: HashMap<UnicodeSet, HashMap<&'s str, &'s str>> = HashMap::new();

    // UnicodeSet is used to store the code point or code point range described
    // in a PPUCD `cp` line, according to the PPUCD file format spec.
    let mut code_point_overrides: HashMap<UnicodeSet, HashMap<&'s str, &'s str>> = HashMap::new();

    let mut code_points: HashMap<u32, HashMap<&'s str, &'s str>> = HashMap::new();

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
            // TODO: can we allow UnicodeSet to derive Clone ?
            let (code_point_range, _) = get_code_point_overrides(&line);
            for code_point_char in code_point_range.iter() {
                let code_point = code_point_char as u32;
                let code_point_prop_vals =
                    get_code_point_prop_vals(code_point, &code_point_overrides, &blocks, &defaults);
                code_points.insert(code_point, code_point_prop_vals);
            }
        }
    }

    let binary_prop_unisets: HashMap<&'s str, UnicodeSet> =
        get_binary_prop_unisets(&prop_aliases, &code_points);

    let mut props: Vec<UnicodeProperty> = vec![];

    for (canonical_name, uniset) in binary_prop_unisets {
        let ppucd_prop: UnicodeProperty = UnicodeProperty::from_uniset(&uniset, canonical_name);
        props.push(ppucd_prop);
    }

    UnicodeProperties { props }
}

#[cfg(test)]
mod gen_properties_test {
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
        let line = "cp;0020;bc=WS;gc=Zs;lb=SP;na=SPACE;Name_Alias=abbreviation=SP;Pat_WS;SB=SP;WB=WSegSpace;WSpace";

        let mut exp_prop_vals: HashMap<&str, &str> = HashMap::new();
        exp_prop_vals.insert("bc", "WS");
        exp_prop_vals.insert("gc", "Zs");
        exp_prop_vals.insert("lb", "SP");
        exp_prop_vals.insert("na", "SPACE");
        exp_prop_vals.insert("Name_Alias", "abbreviation=SP");
        exp_prop_vals.insert("Pat_WS", "Pat_WS");
        exp_prop_vals.insert("SB", "SP");
        exp_prop_vals.insert("WB", "WSegSpace");
        exp_prop_vals.insert("WSpace", "WSpace");

        let line_parts = split_line(&line);
        let act_props_vals = get_data_line_prop_vals(&line_parts);

        assert_eq!(exp_prop_vals, act_props_vals);
    }

    #[test]
    fn update_aliases_test() {
        let line = "property;Binary;Upper;Uppercase";
        let mut prop_aliases: HashMap<&str, HashSet<&str>> = HashMap::new();

        update_aliases(&mut prop_aliases, &line);

        let mut exp_prop_aliases: HashMap<&str, HashSet<&str>> = HashMap::new();
        let exp_aliases = vec!["Uppercase", "Upper"]; // order won't matter
        let exp_aliases: HashSet<&str> = exp_aliases.into_iter().collect();
        exp_prop_aliases.insert("Upper", exp_aliases);

        assert_eq!(&exp_prop_aliases, &prop_aliases);
    }

    #[test]
    fn block_range_prop_vals_test() {
        let line = "block;0000..007F;age=1.1;blk=ASCII;ea=Na;gc=Cc;Gr_Base;lb=AL;sc=Zyyy";

        let mut exp_prop_vals: HashMap<&str, &str> = HashMap::new();
        exp_prop_vals.insert("age", "1.1");
        exp_prop_vals.insert("blk", "ASCII");
        exp_prop_vals.insert("ea", "Na");
        exp_prop_vals.insert("gc", "Cc");
        exp_prop_vals.insert("Gr_Base", "Gr_Base");
        exp_prop_vals.insert("lb", "AL");
        exp_prop_vals.insert("sc", "Zyyy");

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
        let mut exp_code_point_prop_vals: HashMap<&str, &str> = HashMap::new();
        exp_code_point_prop_vals.insert("WB", "WSegSpace");
        exp_code_point_prop_vals.insert("FC_NFKC", "<code point>");
        exp_code_point_prop_vals.insert("GCB", "XX");
        exp_code_point_prop_vals.insert("gcm", "Cn");
        exp_code_point_prop_vals.insert("scx", "<script>");
        exp_code_point_prop_vals.insert("bc", "WS");
        exp_code_point_prop_vals.insert("lc", "<code point>");
        exp_code_point_prop_vals.insert("NFKC_CF", "<code point>");
        exp_code_point_prop_vals.insert("InPC", "NA");
        exp_code_point_prop_vals.insert("lb", "SP");
        exp_code_point_prop_vals.insert("NFC_QC", "Y");
        exp_code_point_prop_vals.insert("SB", "SP");
        exp_code_point_prop_vals.insert("Name_Alias", "abbreviation=SP");
        exp_code_point_prop_vals.insert("dt", "None");
        exp_code_point_prop_vals.insert("ea", "Na");
        exp_code_point_prop_vals.insert("nt", "None");
        exp_code_point_prop_vals.insert("InSC", "Other");
        exp_code_point_prop_vals.insert("NFD_QC", "Y");
        exp_code_point_prop_vals.insert("bpt", "n");
        exp_code_point_prop_vals.insert("jg", "No_Joining_Group");
        exp_code_point_prop_vals.insert("gc", "Zs");
        exp_code_point_prop_vals.insert("vo", "R");
        exp_code_point_prop_vals.insert("NFKD_QC", "Y");
        exp_code_point_prop_vals.insert("NFKC_QC", "Y");
        exp_code_point_prop_vals.insert("blk", "ASCII");
        exp_code_point_prop_vals.insert("uc", "<code point>");
        exp_code_point_prop_vals.insert("suc", "<code point>");
        exp_code_point_prop_vals.insert("scf", "<code point>");
        exp_code_point_prop_vals.insert("slc", "<code point>");
        exp_code_point_prop_vals.insert("age", "1.1");
        exp_code_point_prop_vals.insert("na", "SPACE");
        exp_code_point_prop_vals.insert("cf", "<code point>");
        exp_code_point_prop_vals.insert("hst", "NA");
        exp_code_point_prop_vals.insert("dm", "<code point>");
        exp_code_point_prop_vals.insert("sc", "Zyyy");
        exp_code_point_prop_vals.insert("stc", "<code point>");
        exp_code_point_prop_vals.insert("jt", "U");
        exp_code_point_prop_vals.insert("tc", "<code point>");
        exp_code_point_prop_vals.insert("Gr_Base", "Gr_Base");
        exp_code_point_prop_vals.insert("WSpace", "WSpace");
        exp_code_point_prop_vals.insert("Pat_WS", "Pat_WS");
        let defaults: HashMap<&str, &str> = get_defaults_prop_vals(&defaults_line);
        let mut exp_code_points: HashMap<u32, HashMap<&str, &str>> = HashMap::new();
        exp_code_points.insert(exp_code_point, exp_code_point_prop_vals);

        let mut blocks: HashMap<UnicodeSet, HashMap<&str, &str>> = HashMap::new();
        let (range, prop_vals) = get_block_range_prop_vals(&block_line);
        blocks.insert(range, prop_vals);

        let mut code_point_overrides: HashMap<UnicodeSet, HashMap<&str, &str>> = HashMap::new();
        let (code_point_range, prop_vals) = get_code_point_overrides(&code_point_line);
        code_point_overrides.insert(code_point_range, prop_vals);

        let (code_point_range, _) = get_code_point_overrides(&code_point_line);
        let mut code_points: HashMap<u32, HashMap<&str, &str>> = HashMap::new();
        for code_point_char in code_point_range.iter() {
            let code_point = code_point_char as u32;
            let code_point_prop_vals =
                get_code_point_prop_vals(code_point, &code_point_overrides, &blocks, &defaults);
            code_points.insert(code_point, code_point_prop_vals);
        }

        assert_eq!(&exp_code_points, &code_points);
    }
}
