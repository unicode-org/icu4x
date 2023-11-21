// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).



use alloc::borrow::Cow;
use zerovec;



#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(databake::Bake),
    databake(path = icu_unitsconversion::provider),
)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize),
    zerovec::derive(Serialize)
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    zerovec::derive(Deserialize)
)]
#[zerovec::derive(Debug)]
#[zerovec::make_varule(MeasurePrefixULE)]
pub struct MeasurePrefix <'data>{
    pub base : u8,
    pub power : i8,
    pub prefix_name: Cow<'data,  str>,
}


impl MeasurePrefix<'static> {
    pub const YOTTA : MeasurePrefix<'static> = MeasurePrefix{base: 10,  power: 24, prefix_name: Cow::Borrowed("yotta")};
    pub const ZETTA : MeasurePrefix<'static> = MeasurePrefix{base: 10,  power: 21, prefix_name: Cow::Borrowed("zetta")};
    pub const EXA   : MeasurePrefix<'static> = MeasurePrefix{base: 10,  power: 18, prefix_name: Cow::Borrowed("exa")};
    pub const PETA  : MeasurePrefix<'static> = MeasurePrefix{base: 10,  power: 15, prefix_name: Cow::Borrowed("peta")};
    pub const TERA  : MeasurePrefix<'static> = MeasurePrefix{base: 10,  power: 12, prefix_name: Cow::Borrowed("tera")};
    pub const GIGA  : MeasurePrefix<'static> = MeasurePrefix{base: 10,  power: 9,  prefix_name: Cow::Borrowed("giga")};
    pub const MEGA  : MeasurePrefix<'static> = MeasurePrefix{base: 10,  power: 6,  prefix_name: Cow::Borrowed("mega")};
    pub const KILO  : MeasurePrefix<'static> = MeasurePrefix{base: 10,  power: 3,  prefix_name: Cow::Borrowed("kilo")};
    pub const HECTO : MeasurePrefix<'static> = MeasurePrefix{base: 10,  power: 2,  prefix_name: Cow::Borrowed("hecto")};
    pub const DECA  : MeasurePrefix<'static> = MeasurePrefix{base: 10,  power: 1,  prefix_name: Cow::Borrowed("deca")};
    // TODO: shall we consider it ?
    /// absence of prefix
    pub const ONE : MeasurePrefix<'static> = MeasurePrefix{base: 10,  power: 0,  prefix_name: Cow::Borrowed("")};
    pub const DECI  : MeasurePrefix<'static> = MeasurePrefix{base: 10,  power: -1, prefix_name: Cow::Borrowed("deci")};
    pub const CENTI : MeasurePrefix<'static> = MeasurePrefix{base: 10,  power: -2, prefix_name: Cow::Borrowed("centi")};
    pub const MILLI : MeasurePrefix<'static> = MeasurePrefix{base: 10,  power: -3, prefix_name: Cow::Borrowed("milli")};
    pub const MICRO : MeasurePrefix<'static> = MeasurePrefix{base: 10,  power: -6, prefix_name: Cow::Borrowed("micro")};
    pub const NANO  : MeasurePrefix<'static> = MeasurePrefix{base: 10,  power: -9, prefix_name: Cow::Borrowed("nano")};
    pub const PICO  : MeasurePrefix<'static> = MeasurePrefix{base: 10,  power: -12, prefix_name: Cow::Borrowed("pico")};
    pub const FEMTO : MeasurePrefix<'static> = MeasurePrefix{base: 10,  power: -15, prefix_name: Cow::Borrowed("femto")};
    pub const ATTO  : MeasurePrefix<'static> = MeasurePrefix{base: 10,  power: -18, prefix_name: Cow::Borrowed("atto")};
    pub const ZEPTO : MeasurePrefix<'static> = MeasurePrefix{base: 10,  power: -21, prefix_name: Cow::Borrowed("zepto")};
    pub const YOCTO : MeasurePrefix<'static> = MeasurePrefix{base: 10,  power: -24, prefix_name: Cow::Borrowed("yocto")};
    pub const KIBI  : MeasurePrefix<'static> = MeasurePrefix{base: 2,   power: 10, prefix_name: Cow::Borrowed("kibi")};
    pub const MEBI  : MeasurePrefix<'static> = MeasurePrefix{base: 2,   power: 20, prefix_name: Cow::Borrowed("mebi")};
    pub const GIBI  : MeasurePrefix<'static> = MeasurePrefix{base: 2,   power: 30, prefix_name: Cow::Borrowed("gibi")};
    pub const TEBI  : MeasurePrefix<'static> = MeasurePrefix{base: 2,   power: 40, prefix_name: Cow::Borrowed("tebi")};
    pub const PEBI  : MeasurePrefix<'static> = MeasurePrefix{base: 2,   power: 50, prefix_name: Cow::Borrowed("pebi")};
    pub const EXBI  : MeasurePrefix<'static> = MeasurePrefix{base: 2,   power: 60, prefix_name: Cow::Borrowed("exbi")};
    pub const ZEBI  : MeasurePrefix<'static> = MeasurePrefix{base: 2,   power: 70, prefix_name: Cow::Borrowed("zebi")};
    pub const YOBI  : MeasurePrefix<'static> = MeasurePrefix{base: 2,   power: 80, prefix_name: Cow::Borrowed("yobi")};
    
}

