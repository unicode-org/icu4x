// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).


struct Prefix {
    base: u32,
    power: i16,
    prefix: &'static str,
}

enum SIPrefix {
    YOTTA = Prefix { base: 10, power: 24, prefix: "yotta" },
    ZETTA = Prefix { base: 10, power: 21, prefix: "zetta" },
    EXA = Prefix { base: 10, power: 18, prefix: "exa" },
    PETA = Prefix { base: 10, power: 15, prefix: "peta" },
    TERA = Prefix { base: 10, power: 12, prefix: "tera" },
    GIGA = Prefix { base: 10, power: 9, prefix: "giga" },
    MEGA = Prefix { base: 10, power: 6, prefix: "mega" },
    KILO = Prefix { base: 10, power: 3, prefix: "kilo" },
    HECTO = Prefix { base: 10, power: 2, prefix: "hecto" },
    DECA = Prefix { base: 10, power: 1, prefix: "deca" },
    // TODO: do we need this ?
    ONE = Prefix { base: 10, power: 0, prefix: "" },
    DECI = Prefix { base: 10, power: -1, prefix: "deci" },
    CENTI = Prefix { base: 10, power: -2, prefix: "centi" },
    MILLI = Prefix { base: 10, power: -3, prefix: "milli" },
    MICRO = Prefix { base: 10, power: -6, prefix: "micro" },
    NANO = Prefix { base: 10, power: -9, prefix: "nano" },
    PICO = Prefix { base: 10, power: -12, prefix: "pico" },
    FEMTO = Prefix { base: 10, power: -15, prefix: "femto" },
    ATTO = Prefix { base: 10, power: -18, prefix: "atto" },
    ZEPTO = Prefix { base: 10, power: -21, prefix: "zepto" },
    YOCTO = Prefix { base: 10, power: -24, prefix: "yocto" },
    KIBI = Prefix { base: 2, power: 10, prefix: "kibi" },
    MEBI = Prefix { base: 2, power: 20, prefix: "mebi" },
    GIBI = Prefix { base: 2, power: 30, prefix: "gibi" },
    TEBI = Prefix { base: 2, power: 40, prefix: "tebi" },
    PEBI = Prefix { base: 2, power: 50, prefix: "pebi" },
    EXBI = Prefix { base: 2, power: 60, prefix: "exbi" },
    ZEBI = Prefix { base: 2, power: 70, prefix: "zebi" },
    YOBI = Prefix { base: 2, power: 80, prefix: "yobi" },
}

