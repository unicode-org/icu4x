use std::fs::File;
use std::io::{self, BufRead, Error};
use std::iter::Iterator;
use std::path::Path;
use std::collections::{HashMap, HashSet};


fn main() {
    let filename = "ppucd-test.txt";

    // println!("In file {}", filename);

    if let Ok(lines) = read_lines(filename) {

        let line_opts = lines.map(get_line_str_opt);
        let line_strs = line_opts.flatten();
        let parseable_lines = line_strs.filter(|line| !is_skip_ppucd_line(line));

        let mut prop_aliases: HashMap<String, HashSet<String>> = HashMap::new();

        for line in parseable_lines {
            if is_property_line(&line) {
                update_aliases(&mut prop_aliases, &line);
            } else {
                println!("{}", &line);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
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

fn split_line(line: &String) -> Vec<&str> {
    line.split(";").collect::<Vec<_>>()
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
    let mut line_parts = split_line(&line);
    assert_eq!(&"property", &line_parts[0]);
    let prop_type = &line_parts[1];
    if prop_type == &"Binary" {
        let canonical_name = String::from(*&line_parts[2]);
        &line_parts.drain(0..2);
        let all_names: Vec<String> = line_parts.iter().map(|line| String::from(*line)).collect();
        let all_names_set: HashSet<String> = all_names.into_iter().collect();
        &prop_aliases.insert(canonical_name, all_names_set);
    }
}