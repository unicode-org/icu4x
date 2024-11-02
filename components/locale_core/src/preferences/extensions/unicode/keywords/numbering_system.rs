// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::preferences::extensions::unicode::enum_keyword;

enum_keyword!(
    /// A Unicode Number System Identifier defines a type of number system.
    ///
    /// The valid values are listed in [LDML](https://unicode.org/reports/tr35/#UnicodeNumberSystemIdentifier).
    NumberingSystem {
        /// Adlam digits
        ("adlm" => Adlm),
        /// Ahom digits
        ("ahom" => Ahom),
        /// Arabic-Indic digits
        ("arab" => Arab),
        /// Extended Arabic-Indic digits
        ("arabext" => Arabext),
        /// Armenian upper case numerals — algorithmic
        ("armn" => Armn),
        /// Armenian lower case numerals — algorithmic
        ("armnlow" => Armnlow),
        /// Balinese digits
        ("bali" => Bali),
        /// Bengali digits
        ("beng" => Beng),
        /// Bhaiksuki digits
        ("bhks" => Bhks),
        /// Brahmi digits
        ("brah" => Brah),
        /// Chakma digits
        ("cakm" => Cakm),
        /// Cham digits
        ("cham" => Cham),
        /// Cyrillic numerals — algorithmic
        ("cyrl" => Cyrl),
        /// Devanagari digits
        ("deva" => Deva),
        /// Dives Akuru digits
        ("diak" => Diak),
        /// Ethiopic numerals — algorithmic
        ("ethi" => Ethi),
        /// Financial numerals — may be algorithmic
        ("finance" => Finance),
        /// Full width digits
        ("fullwide" => Fullwide),
        /// Garay digits
        ("gara" => Gara),
        /// Georgian numerals — algorithmic
        ("geor" => Geor),
        /// Gunjala Gondi digits
        ("gong" => Gong),
        /// Masaram Gondi digits
        ("gonm" => Gonm),
        /// Greek upper case numerals — algorithmic
        ("grek" => Grek),
        /// Greek lower case numerals — algorithmic
        ("greklow" => Greklow),
        /// Gujarati digits
        ("gujr" => Gujr),
        /// Gurung Khema digits
        ("gukh" => Gukh),
        /// Gurmukhi digits
        ("guru" => Guru),
        /// Han-character day-of-month numbering for lunar/other traditional calendars
        ("hanidays" => Hanidays),
        /// Positional decimal system using Chinese number ideographs as digits
        ("hanidec" => Hanidec),
        /// Simplified Chinese numerals — algorithmic
        ("hans" => Hans),
        /// Simplified Chinese financial numerals — algorithmic
        ("hansfin" => Hansfin),
        /// Traditional Chinese numerals — algorithmic
        ("hant" => Hant),
        /// Traditional Chinese financial numerals — algorithmic
        ("hantfin" => Hantfin),
        /// Hebrew numerals — algorithmic
        ("hebr" => Hebr),
        /// Pahawh Hmong digits
        ("hmng" => Hmng),
        /// Nyiakeng Puachue Hmong digits
        ("hmnp" => Hmnp),
        /// Javanese digits
        ("java" => Java),
        /// Japanese numerals — algorithmic
        ("jpan" => Jpan),
        /// Japanese financial numerals — algorithmic
        ("jpanfin" => Jpanfin),
        /// Japanese first-year Gannen numbering for Japanese calendar
        ("jpanyear" => Jpanyear),
        /// Kayah Li digits
        ("kali" => Kali),
        /// Kawi digits
        ("kawi" => Kawi),
        /// Khmer digits
        ("khmr" => Khmr),
        /// Kannada digits
        ("knda" => Knda),
        /// Kirat Rai digits
        ("krai" => Krai),
        /// Tai Tham Hora (secular) digits
        ("lana" => Lana),
        /// Tai Tham Tham (ecclesiastical) digits
        ("lanatham" => Lanatham),
        /// Lao digits
        ("laoo" => Laoo),
        /// Latin digits
        ("latn" => Latn),
        /// Lepcha digits
        ("lepc" => Lepc),
        /// Limbu digits
        ("limb" => Limb),
        /// Mathematical bold digits
        ("mathbold" => Mathbold),
        /// Mathematical double-struck digits
        ("mathdbl" => Mathdbl),
        /// Mathematical monospace digits
        ("mathmono" => Mathmono),
        /// Mathematical sans-serif bold digits
        ("mathsanb" => Mathsanb),
        /// Mathematical sans-serif digits
        ("mathsans" => Mathsans),
        /// Malayalam digits
        ("mlym" => Mlym),
        /// Modi digits
        ("modi" => Modi),
        /// Mongolian digits
        ("mong" => Mong),
        /// Mro digits
        ("mroo" => Mroo),
        /// Meetei Mayek digits
        ("mtei" => Mtei),
        /// Myanmar digits
        ("mymr" => Mymr),
        /// Myanmar Eastern Pwo Karen digits
        ("mymrepka" => Mymrepka),
        /// Myanmar Pao digits
        ("mymrpao" => Mymrpao),
        /// Myanmar Shan digits
        ("mymrshan" => Mymrshan),
        /// Myanmar Tai Laing digits
        ("mymrtlng" => Mymrtlng),
        /// Nag Mundari digits
        ("nagm" => Nagm),
        /// Native digits
        ("native" => Native),
        /// Newa digits
        ("newa" => Newa),
        /// N'Ko digits
        ("nkoo" => Nkoo),
        /// Ol Chiki digits
        ("olck" => Olck),
        /// Ol Onal digits
        ("onao" => Onao),
        /// Oriya digits
        ("orya" => Orya),
        /// Osmanya digits
        ("osma" => Osma),
        /// Legacy computing outlined digits
        ("outlined" => Outlined),
        /// Hanifi Rohingya digits
        ("rohg" => Rohg),
        /// Roman upper case numerals — algorithmic
        ("roman" => Roman),
        /// Roman lowercase numerals — algorithmic
        ("romanlow" => Romanlow),
        /// Saurashtra digits
        ("saur" => Saur),
        /// Legacy computing segmented digits
        ("segment" => Segment),
        /// Sharada digits
        ("shrd" => Shrd),
        /// Khudawadi digits
        ("sind" => Sind),
        /// Sinhala Lith digits
        ("sinh" => Sinh),
        /// Sora_Sompeng digits
        ("sora" => Sora),
        /// Sundanese digits
        ("sund" => Sund),
        /// Sunuwar digits
        ("sunu" => Sunu),
        /// Takri digits
        ("takr" => Takr),
        /// New Tai Lue digits
        ("talu" => Talu),
        /// Tamil numerals — algorithmic
        ("taml" => Taml),
        /// Modern Tamil decimal digits
        ("tamldec" => Tamldec),
        /// Tangsa digits
        ("tnsa" => Tnsa),
        /// Telugu digits
        ("telu" => Telu),
        /// Thai digits
        ("thai" => Thai),
        /// Tirhuta digits
        ("tirh" => Tirh),
        /// Tibetan digits
        ("tibt" => Tibt),
        /// Traditional numerals — may be algorithmic
        ("traditio" => Traditio),
        /// Vai digits
        ("vaii" => Vaii),
        /// Warang Citi digits
        ("wara" => Wara),
        /// Wancho digits
        ("wcho" => Wcho),
}, "nu");
