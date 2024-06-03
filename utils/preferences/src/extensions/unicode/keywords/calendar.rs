// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(non_snake_case)]

use crate::enum_keyword;

// https://github.com/unicode-org/cldr/blob/main/common/bcp47/calendar.xml
enum_keyword!(IslamicCalendar {
    "umalqura" => Umalqura,
    "tbla" => Tbla,
    "civil" => Civil,
    "rgsa" => Rgsa
});

enum_keyword!(Calendar {
    "buddhist" => Buddhist,
    "chinese" => Chinese,
    "coptic" => Coptic,
    "dangi" => Dangi,
    "ethioaa" => Ethioaa,
    "ethiopic" => Ethiopic,
    "gregory" => Gregory,
    "hebrew" => Hebrew,
    "indian" => Indian,
    "islamic" => Islamic[IslamicCalendar] {
        "umalqura" => Umalqura,
        "tbla" => Tbla,
        "civil" => Civil,
        "rgsa" => Rgsa
    },
    "iso8601" => Iso8601,
    "japanese" => Japanese,
    "persian" => Persian,
    "roc" => Roc
}, "ca");
