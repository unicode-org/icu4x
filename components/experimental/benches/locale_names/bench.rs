// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use criterion::{black_box, Criterion};
use criterion::{criterion_group, criterion_main};

use icu_experimental::displaynames::provider::{LocaleNamesRegionLongV1, RegionDisplayNamesV1};
use icu_experimental::provider::Baked;
use icu_locale::{locale, DataLocale};
use icu_provider::buf::{
    AsDeserializingBufferProvider, DeserializingBufferProvider, DeserializingOwnedBufferProvider,
};
use icu_provider::unstable::{
    BindLocaleDataProvider, BoundLocaleDataProvider, DataAttributesRequest,
};
use icu_provider::{
    DataIdentifierBorrowed, DataMarker, DataMarkerAttributes, DataProvider, DataRequest,
};
use icu_provider_blob::BlobDataProvider;
use tinystr::UnvalidatedTinyAsciiStr;

pub fn criterion_benchmark(criterion: &mut Criterion) {
    let group_name = "locale_names";

    let mut group = criterion.benchmark_group(group_name);

    let locales = &[
        DataLocale::from(locale!("af")),
        DataLocale::from(locale!("ak")),
        DataLocale::from(locale!("am")),
        DataLocale::from(locale!("ar")),
        DataLocale::from(locale!("as")),
        DataLocale::from(locale!("ast")),
        DataLocale::from(locale!("az")),
        DataLocale::from(locale!("ba")),
        DataLocale::from(locale!("bal-Latn")),
        DataLocale::from(locale!("be")),
        DataLocale::from(locale!("bg")),
        DataLocale::from(locale!("bgc")),
        DataLocale::from(locale!("bho")),
        DataLocale::from(locale!("blo")),
        DataLocale::from(locale!("bn")),
        DataLocale::from(locale!("br")),
        DataLocale::from(locale!("brx")),
        DataLocale::from(locale!("bs")),
        DataLocale::from(locale!("bs-Cyrl")),
        DataLocale::from(locale!("bua")),
        DataLocale::from(locale!("ca")),
        DataLocale::from(locale!("ceb")),
        DataLocale::from(locale!("chr")),
        DataLocale::from(locale!("cs")),
        DataLocale::from(locale!("csw")),
        DataLocale::from(locale!("cv")),
        DataLocale::from(locale!("cy")),
        DataLocale::from(locale!("da")),
        DataLocale::from(locale!("de")),
        DataLocale::from(locale!("doi")),
        DataLocale::from(locale!("dsb")),
        DataLocale::from(locale!("ee")),
        DataLocale::from(locale!("el")),
        DataLocale::from(locale!("en")),
        DataLocale::from(locale!("eo")),
        DataLocale::from(locale!("es")),
        DataLocale::from(locale!("et")),
        DataLocale::from(locale!("eu")),
        DataLocale::from(locale!("fa")),
        DataLocale::from(locale!("ff-Adlm")),
        DataLocale::from(locale!("fi")),
        DataLocale::from(locale!("fil")),
        DataLocale::from(locale!("fo")),
        DataLocale::from(locale!("fr")),
        DataLocale::from(locale!("fy")),
        DataLocale::from(locale!("ga")),
        DataLocale::from(locale!("gaa")),
        DataLocale::from(locale!("gd")),
        DataLocale::from(locale!("gl")),
        DataLocale::from(locale!("gu")),
        DataLocale::from(locale!("ha")),
        DataLocale::from(locale!("he")),
        DataLocale::from(locale!("hi")),
        DataLocale::from(locale!("hi-Latn")),
        DataLocale::from(locale!("hr")),
        DataLocale::from(locale!("hsb")),
        DataLocale::from(locale!("hu")),
        DataLocale::from(locale!("hy")),
        DataLocale::from(locale!("ia")),
        DataLocale::from(locale!("id")),
        DataLocale::from(locale!("ie")),
        DataLocale::from(locale!("ig")),
        DataLocale::from(locale!("ii")),
        DataLocale::from(locale!("is")),
        DataLocale::from(locale!("it")),
        DataLocale::from(locale!("ja")),
        DataLocale::from(locale!("jv")),
        DataLocale::from(locale!("ka")),
        DataLocale::from(locale!("kea")),
        DataLocale::from(locale!("kgp")),
        DataLocale::from(locale!("kk")),
        DataLocale::from(locale!("kk-Arab")),
        DataLocale::from(locale!("km")),
        DataLocale::from(locale!("kn")),
        DataLocale::from(locale!("ko")),
        DataLocale::from(locale!("kok")),
        DataLocale::from(locale!("kok-Latn")),
        DataLocale::from(locale!("ks")),
        DataLocale::from(locale!("ks-Deva")),
        DataLocale::from(locale!("ku")),
        DataLocale::from(locale!("kxv")),
        DataLocale::from(locale!("kxv-Deva")),
        DataLocale::from(locale!("kxv-Orya")),
        DataLocale::from(locale!("kxv-Telu")),
        DataLocale::from(locale!("ky")),
        DataLocale::from(locale!("lb")),
        DataLocale::from(locale!("lij")),
        DataLocale::from(locale!("lmo")),
        DataLocale::from(locale!("lo")),
        DataLocale::from(locale!("lt")),
        DataLocale::from(locale!("lv")),
        DataLocale::from(locale!("mai")),
        DataLocale::from(locale!("mi")),
        DataLocale::from(locale!("mk")),
        DataLocale::from(locale!("ml")),
        DataLocale::from(locale!("mn")),
        DataLocale::from(locale!("mni")),
        DataLocale::from(locale!("mr")),
        DataLocale::from(locale!("ms")),
        DataLocale::from(locale!("mt")),
        DataLocale::from(locale!("my")),
        DataLocale::from(locale!("nds")),
        DataLocale::from(locale!("ne")),
        DataLocale::from(locale!("nl")),
        DataLocale::from(locale!("nn")),
        DataLocale::from(locale!("no")),
        DataLocale::from(locale!("nqo")),
        DataLocale::from(locale!("nso")),
        DataLocale::from(locale!("oc")),
        DataLocale::from(locale!("om")),
        DataLocale::from(locale!("or")),
        DataLocale::from(locale!("pa")),
        DataLocale::from(locale!("pcm")),
        DataLocale::from(locale!("pl")),
        DataLocale::from(locale!("pms")),
        DataLocale::from(locale!("ps")),
        DataLocale::from(locale!("pt")),
        DataLocale::from(locale!("qu")),
        DataLocale::from(locale!("raj")),
        DataLocale::from(locale!("rm")),
        DataLocale::from(locale!("ro")),
        DataLocale::from(locale!("ru")),
        DataLocale::from(locale!("rw")),
        DataLocale::from(locale!("sa")),
        DataLocale::from(locale!("sah")),
        DataLocale::from(locale!("sat")),
        DataLocale::from(locale!("sc")),
        DataLocale::from(locale!("scn")),
        DataLocale::from(locale!("sd")),
        DataLocale::from(locale!("sd-Deva")),
        DataLocale::from(locale!("shn")),
        DataLocale::from(locale!("si")),
        DataLocale::from(locale!("sk")),
        DataLocale::from(locale!("sl")),
        DataLocale::from(locale!("so")),
        DataLocale::from(locale!("sq")),
        DataLocale::from(locale!("sr")),
        DataLocale::from(locale!("sr-Latn")),
        DataLocale::from(locale!("st")),
        DataLocale::from(locale!("su")),
        DataLocale::from(locale!("sv")),
        DataLocale::from(locale!("sw")),
        DataLocale::from(locale!("syr")),
        DataLocale::from(locale!("szl")),
        DataLocale::from(locale!("ta")),
        DataLocale::from(locale!("te")),
        DataLocale::from(locale!("tg")),
        DataLocale::from(locale!("th")),
        DataLocale::from(locale!("ti")),
        DataLocale::from(locale!("tk")),
        DataLocale::from(locale!("tn")),
        DataLocale::from(locale!("to")),
        DataLocale::from(locale!("tr")),
        DataLocale::from(locale!("tt")),
        DataLocale::from(locale!("tyv")),
        DataLocale::from(locale!("ug")),
        DataLocale::from(locale!("uk")),
        DataLocale::from(locale!("ur")),
        DataLocale::from(locale!("uz")),
        DataLocale::from(locale!("uz-Cyrl")),
        DataLocale::from(locale!("vec")),
        DataLocale::from(locale!("vi")),
        DataLocale::from(locale!("vmw")),
        DataLocale::from(locale!("wo")),
        DataLocale::from(locale!("xh")),
        DataLocale::from(locale!("xnr")),
        DataLocale::from(locale!("yo")),
        DataLocale::from(locale!("yrl")),
        DataLocale::from(locale!("yue")),
        DataLocale::from(locale!("yue-Hans")),
        DataLocale::from(locale!("za")),
        DataLocale::from(locale!("zh")),
        DataLocale::from(locale!("zh-Hant")),
        DataLocale::from(locale!("zu")),
    ];

    let attributeses = &[
        DataMarkerAttributes::try_from_str("001").unwrap(),
        DataMarkerAttributes::try_from_str("002").unwrap(),
        DataMarkerAttributes::try_from_str("003").unwrap(),
        DataMarkerAttributes::try_from_str("005").unwrap(),
        DataMarkerAttributes::try_from_str("009").unwrap(),
        DataMarkerAttributes::try_from_str("011").unwrap(),
        DataMarkerAttributes::try_from_str("013").unwrap(),
        DataMarkerAttributes::try_from_str("014").unwrap(),
        DataMarkerAttributes::try_from_str("015").unwrap(),
        DataMarkerAttributes::try_from_str("017").unwrap(),
        DataMarkerAttributes::try_from_str("018").unwrap(),
        DataMarkerAttributes::try_from_str("019").unwrap(),
        DataMarkerAttributes::try_from_str("021").unwrap(),
        DataMarkerAttributes::try_from_str("029").unwrap(),
        DataMarkerAttributes::try_from_str("030").unwrap(),
        DataMarkerAttributes::try_from_str("034").unwrap(),
        DataMarkerAttributes::try_from_str("035").unwrap(),
        DataMarkerAttributes::try_from_str("039").unwrap(),
        DataMarkerAttributes::try_from_str("053").unwrap(),
        DataMarkerAttributes::try_from_str("054").unwrap(),
        DataMarkerAttributes::try_from_str("057").unwrap(),
        DataMarkerAttributes::try_from_str("061").unwrap(),
        DataMarkerAttributes::try_from_str("142").unwrap(),
        DataMarkerAttributes::try_from_str("143").unwrap(),
        DataMarkerAttributes::try_from_str("145").unwrap(),
        DataMarkerAttributes::try_from_str("150").unwrap(),
        DataMarkerAttributes::try_from_str("151").unwrap(),
        DataMarkerAttributes::try_from_str("154").unwrap(),
        DataMarkerAttributes::try_from_str("155").unwrap(),
        DataMarkerAttributes::try_from_str("202").unwrap(),
        DataMarkerAttributes::try_from_str("419").unwrap(),
        DataMarkerAttributes::try_from_str("AC").unwrap(),
        DataMarkerAttributes::try_from_str("AD").unwrap(),
        DataMarkerAttributes::try_from_str("AE").unwrap(),
        DataMarkerAttributes::try_from_str("AF").unwrap(),
        DataMarkerAttributes::try_from_str("AG").unwrap(),
        DataMarkerAttributes::try_from_str("AI").unwrap(),
        DataMarkerAttributes::try_from_str("AL").unwrap(),
        DataMarkerAttributes::try_from_str("AM").unwrap(),
        DataMarkerAttributes::try_from_str("AO").unwrap(),
        DataMarkerAttributes::try_from_str("AQ").unwrap(),
        DataMarkerAttributes::try_from_str("AR").unwrap(),
        DataMarkerAttributes::try_from_str("AS").unwrap(),
        DataMarkerAttributes::try_from_str("AT").unwrap(),
        DataMarkerAttributes::try_from_str("AU").unwrap(),
        DataMarkerAttributes::try_from_str("AW").unwrap(),
        DataMarkerAttributes::try_from_str("AX").unwrap(),
        DataMarkerAttributes::try_from_str("AZ").unwrap(),
        DataMarkerAttributes::try_from_str("BA").unwrap(),
        DataMarkerAttributes::try_from_str("BA").unwrap(),
        DataMarkerAttributes::try_from_str("BB").unwrap(),
        DataMarkerAttributes::try_from_str("BD").unwrap(),
        DataMarkerAttributes::try_from_str("BE").unwrap(),
        DataMarkerAttributes::try_from_str("BF").unwrap(),
        DataMarkerAttributes::try_from_str("BG").unwrap(),
        DataMarkerAttributes::try_from_str("BH").unwrap(),
        DataMarkerAttributes::try_from_str("BI").unwrap(),
        DataMarkerAttributes::try_from_str("BJ").unwrap(),
        DataMarkerAttributes::try_from_str("BL").unwrap(),
        DataMarkerAttributes::try_from_str("BM").unwrap(),
        DataMarkerAttributes::try_from_str("BN").unwrap(),
        DataMarkerAttributes::try_from_str("BO").unwrap(),
        DataMarkerAttributes::try_from_str("BQ").unwrap(),
        DataMarkerAttributes::try_from_str("BR").unwrap(),
        DataMarkerAttributes::try_from_str("BS").unwrap(),
        DataMarkerAttributes::try_from_str("BT").unwrap(),
        DataMarkerAttributes::try_from_str("BV").unwrap(),
        DataMarkerAttributes::try_from_str("BW").unwrap(),
        DataMarkerAttributes::try_from_str("BY").unwrap(),
        DataMarkerAttributes::try_from_str("BZ").unwrap(),
        DataMarkerAttributes::try_from_str("CA").unwrap(),
        DataMarkerAttributes::try_from_str("CC").unwrap(),
        DataMarkerAttributes::try_from_str("CC").unwrap(),
        DataMarkerAttributes::try_from_str("CD").unwrap(),
        DataMarkerAttributes::try_from_str("CD").unwrap(),
        DataMarkerAttributes::try_from_str("CF").unwrap(),
        DataMarkerAttributes::try_from_str("CG").unwrap(),
        DataMarkerAttributes::try_from_str("CG").unwrap(),
        DataMarkerAttributes::try_from_str("CH").unwrap(),
        DataMarkerAttributes::try_from_str("CI").unwrap(),
        DataMarkerAttributes::try_from_str("CI").unwrap(),
        DataMarkerAttributes::try_from_str("CK").unwrap(),
        DataMarkerAttributes::try_from_str("CL").unwrap(),
        DataMarkerAttributes::try_from_str("CM").unwrap(),
        DataMarkerAttributes::try_from_str("CN").unwrap(),
        DataMarkerAttributes::try_from_str("CO").unwrap(),
        DataMarkerAttributes::try_from_str("CP").unwrap(),
        DataMarkerAttributes::try_from_str("CQ").unwrap(),
        DataMarkerAttributes::try_from_str("CR").unwrap(),
        DataMarkerAttributes::try_from_str("CU").unwrap(),
        DataMarkerAttributes::try_from_str("CV").unwrap(),
        DataMarkerAttributes::try_from_str("CV").unwrap(),
        DataMarkerAttributes::try_from_str("CW").unwrap(),
        DataMarkerAttributes::try_from_str("CX").unwrap(),
        DataMarkerAttributes::try_from_str("CY").unwrap(),
        DataMarkerAttributes::try_from_str("CZ").unwrap(),
        DataMarkerAttributes::try_from_str("CZ").unwrap(),
        DataMarkerAttributes::try_from_str("DE").unwrap(),
        DataMarkerAttributes::try_from_str("DG").unwrap(),
        DataMarkerAttributes::try_from_str("DJ").unwrap(),
        DataMarkerAttributes::try_from_str("DK").unwrap(),
        DataMarkerAttributes::try_from_str("DM").unwrap(),
        DataMarkerAttributes::try_from_str("DO").unwrap(),
        DataMarkerAttributes::try_from_str("DZ").unwrap(),
        DataMarkerAttributes::try_from_str("EA").unwrap(),
        DataMarkerAttributes::try_from_str("EC").unwrap(),
        DataMarkerAttributes::try_from_str("EE").unwrap(),
        DataMarkerAttributes::try_from_str("EG").unwrap(),
        DataMarkerAttributes::try_from_str("EH").unwrap(),
        DataMarkerAttributes::try_from_str("ER").unwrap(),
        DataMarkerAttributes::try_from_str("ES").unwrap(),
        DataMarkerAttributes::try_from_str("ET").unwrap(),
        DataMarkerAttributes::try_from_str("EU").unwrap(),
        DataMarkerAttributes::try_from_str("EZ").unwrap(),
        DataMarkerAttributes::try_from_str("FI").unwrap(),
        DataMarkerAttributes::try_from_str("FJ").unwrap(),
        DataMarkerAttributes::try_from_str("FK").unwrap(),
        DataMarkerAttributes::try_from_str("FK").unwrap(),
        DataMarkerAttributes::try_from_str("FM").unwrap(),
        DataMarkerAttributes::try_from_str("FO").unwrap(),
        DataMarkerAttributes::try_from_str("FR").unwrap(),
        DataMarkerAttributes::try_from_str("GA").unwrap(),
        DataMarkerAttributes::try_from_str("GB").unwrap(),
        DataMarkerAttributes::try_from_str("GB").unwrap(),
        DataMarkerAttributes::try_from_str("GD").unwrap(),
        DataMarkerAttributes::try_from_str("GE").unwrap(),
        DataMarkerAttributes::try_from_str("GF").unwrap(),
        DataMarkerAttributes::try_from_str("GG").unwrap(),
        DataMarkerAttributes::try_from_str("GH").unwrap(),
        DataMarkerAttributes::try_from_str("GI").unwrap(),
        DataMarkerAttributes::try_from_str("GL").unwrap(),
        DataMarkerAttributes::try_from_str("GM").unwrap(),
        DataMarkerAttributes::try_from_str("GN").unwrap(),
        DataMarkerAttributes::try_from_str("GP").unwrap(),
        DataMarkerAttributes::try_from_str("GQ").unwrap(),
        DataMarkerAttributes::try_from_str("GR").unwrap(),
        DataMarkerAttributes::try_from_str("GS").unwrap(),
        DataMarkerAttributes::try_from_str("GT").unwrap(),
        DataMarkerAttributes::try_from_str("GU").unwrap(),
        DataMarkerAttributes::try_from_str("GW").unwrap(),
        DataMarkerAttributes::try_from_str("GY").unwrap(),
        DataMarkerAttributes::try_from_str("HK").unwrap(),
        DataMarkerAttributes::try_from_str("HK").unwrap(),
        DataMarkerAttributes::try_from_str("HM").unwrap(),
        DataMarkerAttributes::try_from_str("HN").unwrap(),
        DataMarkerAttributes::try_from_str("HR").unwrap(),
        DataMarkerAttributes::try_from_str("HT").unwrap(),
        DataMarkerAttributes::try_from_str("HU").unwrap(),
        DataMarkerAttributes::try_from_str("IC").unwrap(),
        DataMarkerAttributes::try_from_str("ID").unwrap(),
        DataMarkerAttributes::try_from_str("IE").unwrap(),
        DataMarkerAttributes::try_from_str("IL").unwrap(),
        DataMarkerAttributes::try_from_str("IM").unwrap(),
        DataMarkerAttributes::try_from_str("IN").unwrap(),
        DataMarkerAttributes::try_from_str("IO").unwrap(),
        DataMarkerAttributes::try_from_str("IO").unwrap(),
        DataMarkerAttributes::try_from_str("IQ").unwrap(),
        DataMarkerAttributes::try_from_str("IR").unwrap(),
        DataMarkerAttributes::try_from_str("IS").unwrap(),
        DataMarkerAttributes::try_from_str("IT").unwrap(),
        DataMarkerAttributes::try_from_str("JE").unwrap(),
        DataMarkerAttributes::try_from_str("JM").unwrap(),
        DataMarkerAttributes::try_from_str("JO").unwrap(),
        DataMarkerAttributes::try_from_str("JP").unwrap(),
        DataMarkerAttributes::try_from_str("KE").unwrap(),
        DataMarkerAttributes::try_from_str("KG").unwrap(),
        DataMarkerAttributes::try_from_str("KH").unwrap(),
        DataMarkerAttributes::try_from_str("KI").unwrap(),
        DataMarkerAttributes::try_from_str("KM").unwrap(),
        DataMarkerAttributes::try_from_str("KN").unwrap(),
        DataMarkerAttributes::try_from_str("KP").unwrap(),
        DataMarkerAttributes::try_from_str("KR").unwrap(),
        DataMarkerAttributes::try_from_str("KW").unwrap(),
        DataMarkerAttributes::try_from_str("KY").unwrap(),
        DataMarkerAttributes::try_from_str("KZ").unwrap(),
        DataMarkerAttributes::try_from_str("LA").unwrap(),
        DataMarkerAttributes::try_from_str("LB").unwrap(),
        DataMarkerAttributes::try_from_str("LC").unwrap(),
        DataMarkerAttributes::try_from_str("LI").unwrap(),
        DataMarkerAttributes::try_from_str("LK").unwrap(),
        DataMarkerAttributes::try_from_str("LR").unwrap(),
        DataMarkerAttributes::try_from_str("LS").unwrap(),
        DataMarkerAttributes::try_from_str("LT").unwrap(),
        DataMarkerAttributes::try_from_str("LU").unwrap(),
        DataMarkerAttributes::try_from_str("LV").unwrap(),
        DataMarkerAttributes::try_from_str("LY").unwrap(),
        DataMarkerAttributes::try_from_str("MA").unwrap(),
        DataMarkerAttributes::try_from_str("MC").unwrap(),
        DataMarkerAttributes::try_from_str("MD").unwrap(),
        DataMarkerAttributes::try_from_str("ME").unwrap(),
        DataMarkerAttributes::try_from_str("MF").unwrap(),
        DataMarkerAttributes::try_from_str("MG").unwrap(),
        DataMarkerAttributes::try_from_str("MH").unwrap(),
        DataMarkerAttributes::try_from_str("MK").unwrap(),
        DataMarkerAttributes::try_from_str("ML").unwrap(),
        DataMarkerAttributes::try_from_str("MM").unwrap(),
        DataMarkerAttributes::try_from_str("MM").unwrap(),
        DataMarkerAttributes::try_from_str("MN").unwrap(),
        DataMarkerAttributes::try_from_str("MO").unwrap(),
        DataMarkerAttributes::try_from_str("MO").unwrap(),
        DataMarkerAttributes::try_from_str("MP").unwrap(),
        DataMarkerAttributes::try_from_str("MQ").unwrap(),
        DataMarkerAttributes::try_from_str("MR").unwrap(),
        DataMarkerAttributes::try_from_str("MS").unwrap(),
        DataMarkerAttributes::try_from_str("MT").unwrap(),
        DataMarkerAttributes::try_from_str("MU").unwrap(),
        DataMarkerAttributes::try_from_str("MV").unwrap(),
        DataMarkerAttributes::try_from_str("MW").unwrap(),
        DataMarkerAttributes::try_from_str("MX").unwrap(),
        DataMarkerAttributes::try_from_str("MY").unwrap(),
        DataMarkerAttributes::try_from_str("MZ").unwrap(),
        DataMarkerAttributes::try_from_str("NA").unwrap(),
        DataMarkerAttributes::try_from_str("NC").unwrap(),
        DataMarkerAttributes::try_from_str("NE").unwrap(),
        DataMarkerAttributes::try_from_str("NF").unwrap(),
        DataMarkerAttributes::try_from_str("NG").unwrap(),
        DataMarkerAttributes::try_from_str("NI").unwrap(),
        DataMarkerAttributes::try_from_str("NL").unwrap(),
        DataMarkerAttributes::try_from_str("NO").unwrap(),
        DataMarkerAttributes::try_from_str("NP").unwrap(),
        DataMarkerAttributes::try_from_str("NR").unwrap(),
        DataMarkerAttributes::try_from_str("NU").unwrap(),
        DataMarkerAttributes::try_from_str("NZ").unwrap(),
        DataMarkerAttributes::try_from_str("NZ").unwrap(),
        DataMarkerAttributes::try_from_str("OM").unwrap(),
        DataMarkerAttributes::try_from_str("PA").unwrap(),
        DataMarkerAttributes::try_from_str("PE").unwrap(),
        DataMarkerAttributes::try_from_str("PF").unwrap(),
        DataMarkerAttributes::try_from_str("PG").unwrap(),
        DataMarkerAttributes::try_from_str("PH").unwrap(),
        DataMarkerAttributes::try_from_str("PK").unwrap(),
        DataMarkerAttributes::try_from_str("PL").unwrap(),
        DataMarkerAttributes::try_from_str("PM").unwrap(),
        DataMarkerAttributes::try_from_str("PN").unwrap(),
        DataMarkerAttributes::try_from_str("PN").unwrap(),
        DataMarkerAttributes::try_from_str("PR").unwrap(),
        DataMarkerAttributes::try_from_str("PS").unwrap(),
        DataMarkerAttributes::try_from_str("PS").unwrap(),
        DataMarkerAttributes::try_from_str("PT").unwrap(),
        DataMarkerAttributes::try_from_str("PW").unwrap(),
        DataMarkerAttributes::try_from_str("PY").unwrap(),
        DataMarkerAttributes::try_from_str("QA").unwrap(),
        DataMarkerAttributes::try_from_str("QO").unwrap(),
        DataMarkerAttributes::try_from_str("RE").unwrap(),
        DataMarkerAttributes::try_from_str("RO").unwrap(),
        DataMarkerAttributes::try_from_str("RS").unwrap(),
        DataMarkerAttributes::try_from_str("RU").unwrap(),
        DataMarkerAttributes::try_from_str("RW").unwrap(),
        DataMarkerAttributes::try_from_str("SA").unwrap(),
        DataMarkerAttributes::try_from_str("SB").unwrap(),
        DataMarkerAttributes::try_from_str("SC").unwrap(),
        DataMarkerAttributes::try_from_str("SD").unwrap(),
        DataMarkerAttributes::try_from_str("SE").unwrap(),
        DataMarkerAttributes::try_from_str("SG").unwrap(),
        DataMarkerAttributes::try_from_str("SH").unwrap(),
        DataMarkerAttributes::try_from_str("SI").unwrap(),
        DataMarkerAttributes::try_from_str("SJ").unwrap(),
        DataMarkerAttributes::try_from_str("SK").unwrap(),
        DataMarkerAttributes::try_from_str("SL").unwrap(),
        DataMarkerAttributes::try_from_str("SM").unwrap(),
        DataMarkerAttributes::try_from_str("SN").unwrap(),
        DataMarkerAttributes::try_from_str("SO").unwrap(),
        DataMarkerAttributes::try_from_str("SR").unwrap(),
        DataMarkerAttributes::try_from_str("SS").unwrap(),
        DataMarkerAttributes::try_from_str("ST").unwrap(),
        DataMarkerAttributes::try_from_str("SV").unwrap(),
        DataMarkerAttributes::try_from_str("SX").unwrap(),
        DataMarkerAttributes::try_from_str("SY").unwrap(),
        DataMarkerAttributes::try_from_str("SZ").unwrap(),
        DataMarkerAttributes::try_from_str("SZ").unwrap(),
        DataMarkerAttributes::try_from_str("TA").unwrap(),
        DataMarkerAttributes::try_from_str("TC").unwrap(),
        DataMarkerAttributes::try_from_str("TD").unwrap(),
        DataMarkerAttributes::try_from_str("TF").unwrap(),
        DataMarkerAttributes::try_from_str("TG").unwrap(),
        DataMarkerAttributes::try_from_str("TH").unwrap(),
        DataMarkerAttributes::try_from_str("TJ").unwrap(),
        DataMarkerAttributes::try_from_str("TK").unwrap(),
        DataMarkerAttributes::try_from_str("TL").unwrap(),
        DataMarkerAttributes::try_from_str("TL").unwrap(),
        DataMarkerAttributes::try_from_str("TM").unwrap(),
        DataMarkerAttributes::try_from_str("TN").unwrap(),
        DataMarkerAttributes::try_from_str("TO").unwrap(),
        DataMarkerAttributes::try_from_str("TR").unwrap(),
        DataMarkerAttributes::try_from_str("TR").unwrap(),
        DataMarkerAttributes::try_from_str("TT").unwrap(),
        DataMarkerAttributes::try_from_str("TV").unwrap(),
        DataMarkerAttributes::try_from_str("TW").unwrap(),
        DataMarkerAttributes::try_from_str("TZ").unwrap(),
        DataMarkerAttributes::try_from_str("UA").unwrap(),
        DataMarkerAttributes::try_from_str("UG").unwrap(),
        DataMarkerAttributes::try_from_str("UM").unwrap(),
        DataMarkerAttributes::try_from_str("UN").unwrap(),
        DataMarkerAttributes::try_from_str("UN").unwrap(),
        DataMarkerAttributes::try_from_str("US").unwrap(),
        DataMarkerAttributes::try_from_str("US").unwrap(),
        DataMarkerAttributes::try_from_str("UY").unwrap(),
        DataMarkerAttributes::try_from_str("UZ").unwrap(),
        DataMarkerAttributes::try_from_str("VA").unwrap(),
        DataMarkerAttributes::try_from_str("VC").unwrap(),
        DataMarkerAttributes::try_from_str("VE").unwrap(),
        DataMarkerAttributes::try_from_str("VG").unwrap(),
        DataMarkerAttributes::try_from_str("VI").unwrap(),
        DataMarkerAttributes::try_from_str("VN").unwrap(),
        DataMarkerAttributes::try_from_str("VU").unwrap(),
        DataMarkerAttributes::try_from_str("WF").unwrap(),
        DataMarkerAttributes::try_from_str("WS").unwrap(),
        DataMarkerAttributes::try_from_str("XA").unwrap(),
        DataMarkerAttributes::try_from_str("XB").unwrap(),
        DataMarkerAttributes::try_from_str("XK").unwrap(),
        DataMarkerAttributes::try_from_str("YE").unwrap(),
        DataMarkerAttributes::try_from_str("YT").unwrap(),
        DataMarkerAttributes::try_from_str("ZA").unwrap(),
        DataMarkerAttributes::try_from_str("ZM").unwrap(),
        DataMarkerAttributes::try_from_str("ZW").unwrap(),
        DataMarkerAttributes::try_from_str("ZZ").unwrap(),
    ];

    group.bench_function("region/baked/all/1by1", |bencher| {
        bencher.iter(|| {
            let mut sum = 0;
            for locale in black_box(locales) {
                for attributes in black_box(attributeses) {
                    let req = DataRequest {
                        metadata: Default::default(),
                        id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                            *attributes,
                            locale,
                        ),
                    };
                    sum += DataProvider::<LocaleNamesRegionLongV1>::load(&Baked, req)
                        .map(|resp| resp.payload.get().name.len())
                        .unwrap_or_default();
                }
            }
            assert_eq!(sum, 801535);
            sum
        });
    });

    group.bench_function("region/baked/10x10pct/1by1", |bencher| {
        bencher.iter(|| {
            let mut sum = 0;
            let mut i = 0;
            for locale in black_box(locales) {
                i += 1;
                if i % 10 != 0 {
                    continue;
                }
                let mut j = 0;
                for attributes in black_box(attributeses) {
                    j += 1;
                    if j % 10 != 0 {
                        continue;
                    }
                    let req = DataRequest {
                        metadata: Default::default(),
                        id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                            *attributes,
                            locale,
                        ),
                    };
                    sum += DataProvider::<LocaleNamesRegionLongV1>::load(&Baked, req)
                        .map(|resp| resp.payload.get().name.len())
                        .unwrap_or_default();
                }
            }
            assert_eq!(sum, 6965);
            sum
        });
    });

    group.bench_function("region/baked/all/old_map_struct", |bencher| {
        bencher.iter(|| {
            let mut sum = 0;
            for locale in black_box(locales) {
                let req = DataRequest {
                    metadata: Default::default(),
                    id: DataIdentifierBorrowed::for_locale(locale),
                };
                let Ok(payload) = DataProvider::<RegionDisplayNamesV1>::load(&Baked, req) else {
                    continue;
                };
                let payload = payload.payload.get();
                for attributes in black_box(attributeses) {
                    let id_for_lookup = UnvalidatedTinyAsciiStr::try_from_utf8(attributes.as_bytes()).unwrap_or(UnvalidatedTinyAsciiStr::DEFAULT);
                    sum += payload.names.get(&id_for_lookup)
                        .map(|s| s.len())
                        .unwrap_or_default();
                }
            }
            assert_eq!(sum, 801535);
            sum
        });
    });

    group.bench_function("region/baked/10x10pct/old_map_struct", |bencher| {
        bencher.iter(|| {
            let mut sum = 0;
            let mut i = 0;
            for locale in black_box(locales) {
                i += 1;
                if i % 10 != 0 {
                    continue;
                }
                let req = DataRequest {
                    metadata: Default::default(),
                    id: DataIdentifierBorrowed::for_locale(locale),
                };
                let Ok(payload) = DataProvider::<RegionDisplayNamesV1>::load(&Baked, req) else {
                    continue;
                };
                let payload = payload.payload.get();
                let mut j = 0;
                for attributes in black_box(attributeses) {
                    j += 1;
                    if j % 10 != 0 {
                        continue;
                    }
                    let id_for_lookup = UnvalidatedTinyAsciiStr::try_from_utf8(attributes.as_bytes()).unwrap_or(UnvalidatedTinyAsciiStr::DEFAULT);
                    sum += payload.names.get(&id_for_lookup)
                        .map(|s| s.len())
                        .unwrap_or_default();
                }
            }
            assert_eq!(sum, 6965);
            sum
        });
    });

    if let Ok(path) = std::env::var("ICU4X_REGION_POSTCARD_PATH") {
        group.bench_function("region/postcard/all/1by1", |bencher| {
            let postcard = std::fs::read(&path).unwrap().into_boxed_slice();
            let provider = BlobDataProvider::try_new_from_blob(postcard).unwrap();
            let provider = provider.as_deserializing();
            bencher.iter(|| {
                let mut sum = 0;
                for locale in black_box(locales) {
                    for attributes in black_box(attributeses) {
                        let req = DataRequest {
                            metadata: Default::default(),
                            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                                *attributes,
                                locale,
                            ),
                        };
                        sum += DataProvider::<LocaleNamesRegionLongV1>::load(&provider, req)
                            .map(|resp| resp.payload.get().name.len())
                            .unwrap_or_default();
                    }
                }
                assert_eq!(sum, 801535);
                sum
            });
        });
    }

    if let Ok(path) = std::env::var("ICU4X_REGION_POSTCARD_PATH") {
        group.bench_function("region/postcard/10x10pct/1by1", |bencher| {
            let postcard = std::fs::read(&path).unwrap().into_boxed_slice();
            let provider = BlobDataProvider::try_new_from_blob(postcard).unwrap();
            let provider = provider.as_deserializing();
            bencher.iter(|| {
                let mut sum = 0;
                let mut i = 0;
                for locale in black_box(locales) {
                    i += 1;
                    if i % 10 != 0 {
                        continue;
                    }
                    let mut j = 0;
                    for attributes in black_box(attributeses) {
                        j += 1;
                        if j % 10 != 0 {
                            continue;
                        }
                        let req = DataRequest {
                            metadata: Default::default(),
                            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                                *attributes,
                                locale,
                            ),
                        };
                        sum += DataProvider::<LocaleNamesRegionLongV1>::load(&provider, req)
                            .map(|resp| resp.payload.get().name.len())
                            .unwrap_or_default();
                    }
                }
                assert_eq!(sum, 6965);
                sum
            });
        });
    }

    if let Ok(path) = std::env::var("ICU4X_REGION_POSTCARD_PATH") {
        group.bench_function("region/postcard/all/boundlocale", |bencher| {
            let postcard = std::fs::read(&path).unwrap().into_boxed_slice();
            let provider = BlobDataProvider::try_new_from_blob(postcard).unwrap();
            bencher.iter(|| {
                let mut sum = 0;
                for locale in black_box(locales) {
                    let req = DataRequest {
                        metadata: Default::default(),
                        id: DataIdentifierBorrowed::for_locale(locale),
                    };
                    let Ok(provider) = provider.bind_locale(LocaleNamesRegionLongV1::INFO, req)
                    else {
                        continue
                    };
                    let provider = DeserializingOwnedBufferProvider::new(provider.bound_provider);
                    for attributes in black_box(attributeses) {
                        sum += BoundLocaleDataProvider::<LocaleNamesRegionLongV1>::load_bound(
                            &provider,
                            DataAttributesRequest {
                                metadata: Default::default(),
                                marker_attributes: *attributes,
                            },
                        )
                        .map(|resp| resp.payload.name.len())
                        .unwrap_or_default();
                    }
                }
                assert_eq!(sum, 801535);
                sum
            });
        });
    }

    if let Ok(path) = std::env::var("ICU4X_REGION_POSTCARD_PATH") {
        group.bench_function("region/postcard/10x10pct/boundlocale", |bencher| {
            let postcard = std::fs::read(&path).unwrap().into_boxed_slice();
            let provider = BlobDataProvider::try_new_from_blob(postcard).unwrap();
            bencher.iter(|| {
                let mut sum = 0;
                let mut i = 0;
                for locale in black_box(locales) {
                    i += 1;
                    if i % 10 != 0 {
                        continue;
                    }
                    let req = DataRequest {
                        metadata: Default::default(),
                        id: DataIdentifierBorrowed::for_locale(locale),
                    };
                    let Ok(provider) = provider.bind_locale(LocaleNamesRegionLongV1::INFO, req)
                    else {
                        continue
                    };
                    let provider = DeserializingOwnedBufferProvider::new(provider.bound_provider);
                    let mut j = 0;
                    for attributes in black_box(attributeses) {
                        j += 1;
                        if j % 10 != 0 {
                            continue;
                        }
                        sum += BoundLocaleDataProvider::<LocaleNamesRegionLongV1>::load_bound(
                            &provider,
                            DataAttributesRequest {
                                metadata: Default::default(),
                                marker_attributes: *attributes,
                            },
                        )
                        .map(|resp| resp.payload.name.len())
                        .unwrap_or_default();
                    }
                }
                assert_eq!(sum, 6965);
                sum
            });
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark,);

criterion_main!(benches);
