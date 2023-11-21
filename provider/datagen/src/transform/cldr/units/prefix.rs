// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

enum SIPrefix {
    YOTTA = (10, 24, "yotta"),
    ZETTA = (10, 21, "zetta"),
    EXA = (10, 18, "exa"),
    PETA = (10, 15, "peta"),
    TERA = (10, 12, "tera"),
    GIGA = (10, 9, "giga"),
    MEGA = (10, 6, "mega"),
    KILO = (10, 3, "kilo"),
    HECTO = (10, 2, "hecto"),
    DECA = (10, 1, "deca"),
    // TODO: do we need this ?
    ONE = (10, 0, ""),
    DECI = (10, -1, "deci"),
    CENTI = (10, -2, "centi"),
    MILLI = (10, -3, "milli"),
    MICRO = (10, -6, "micro"),
    NANO = (10, -9, "nano"),
    PICO = (10, -12, "pico"),
    FEMTO = (10, -15, "femto"),
    ATTO = (10, -18, "atto"),
    ZEPTO = (10, -21, "zepto"),
    YOCTO = (10, -24, "yocto"),
    KIBI = (2, 10, "kibi"),
    MEBI = (2, 20, "mebi"),
    GIBI = (2, 30, "gibi"),
    TEBI = (2, 40, "tebi"),
    PEBI = (2, 50, "pebi"),
    EXBI = (2, 60, "exbi"),
    ZEBI = (2, 70, "zebi"),
    YOBI = (2, 80, "yobi"),
}
