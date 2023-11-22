// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).


struct SIPrefix {
    base: u32,
    power: i16,
    prefix: String,
}

impl SIPrefix {
pub  prefixes: Vec<SIPrefix> = vec![
    SIPrefix { base: 10, power: 24, prefix: "yotta".to_string() },
    SIPrefix { base: 10, power: 21, prefix: "zetta".to_string() },
    SIPrefix { base: 10, power: 18, prefix: "exa".to_string() },
    SIPrefix { base: 10, power: 15, prefix: "peta".to_string() },
    SIPrefix { base: 10, power: 12, prefix: "tera".to_string() },
    SIPrefix { base: 10, power: 9, prefix: "giga".to_string() },
    SIPrefix { base: 10, power: 6, prefix: "mega".to_string() },
    SIPrefix { base: 10, power: 3, prefix: "kilo".to_string() },
    SIPrefix { base: 10, power: 2, prefix: "hecto".to_string() },
    SIPrefix { base: 10, power: 1, prefix: "deca".to_string() },
    // TODO: do we need this ?
    SIPrefix { base: 10, power: 0, prefix: "".to_string() },
    SIPrefix { base: 10, power: -1, prefix: "deci".to_string() },
    SIPrefix { base: 10, power: -2, prefix: "centi".to_string() },
    SIPrefix { base: 10, power: -3, prefix: "milli".to_string() },
    SIPrefix { base: 10, power: -6, prefix: "micro".to_string() },
    SIPrefix { base: 10, power: -9, prefix: "nano".to_string() },
    SIPrefix { base: 10, power: -12, prefix: "pico".to_string() },
    SIPrefix { base: 10, power: -15, prefix: "femto".to_string() },
    SIPrefix { base: 10, power: -18, prefix: "atto".to_string() },
    SIPrefix { base: 10, power: -21, prefix: "zepto".to_string() },
    SIPrefix { base: 10, power: -24, prefix: "yocto".to_string() },
    SIPrefix { base: 2, power: 10, prefix: "kibi".to_string() },
    SIPrefix { base: 2, power: 20, prefix: "mebi".to_string() },
    SIPrefix { base: 2, power: 30, prefix: "gibi".to_string() },
    SIPrefix { base: 2, power: 40, prefix: "tebi".to_string() },
    SIPrefix { base: 2, power: 50, prefix: "pebi".to_string() },
    SIPrefix { base: 2, power: 60, prefix: "exbi".to_string() },
    SIPrefix { base: 2, power: 70, prefix: "zebi".to_string() },
    SIPrefix { base: 2, power: 80, prefix: "yobi".to_string() },
];

}