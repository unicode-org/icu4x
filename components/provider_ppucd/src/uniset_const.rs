#![allow(clippy::unreadable_literal, dead_code)]

use crate::structs::*;
use crate::support::*;
use icu_uniset::UnicodeSet;
use icu_locid::LanguageIdentifier;
use icu_locid_macros::langid;
use icu_provider::prelude::*;
use std::borrow::Cow;
use std::convert::TryInto;


// helper fn
fn get_prop(ppucd_provider: &PpucdDataProvider, data_key: DataKey) -> UnicodeSet {
    const UND: LanguageIdentifier = langid!("und");
    let data_req = DataRequest {
        data_key: data_key,
        data_entry: DataEntry {
            variant: None,
            langid: UND,
        },
    };
    let resp: DataResponse = ppucd_provider.load(&data_req).unwrap();

    let ppucd_property_cow: Cow<UnicodeProperty> = resp.take_payload().unwrap();
    let ppucd_property: UnicodeProperty = ppucd_property_cow.into_owned();
    let uniset: UnicodeSet = ppucd_property.try_into().unwrap();
    uniset
}

/// Upper
pub fn upper(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::UPPER_V1)
}

/// Bidi_M
pub fn bidi_m(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::BIDI_M_V1)
}

/// PCM
pub fn pcm(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::PCM_V1)
}

/// Emoji
pub fn emoji(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::EMOJI_V1)
}

/// lower
pub fn lower(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::LOWER_V1)
}

/// EMod
pub fn emod(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::EMOD_V1)
}

/// XIDS
pub fn xids(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::XIDS_V1)
}

/// CWU
pub fn cwu(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::CWU_V1)
}

/// Pat_Syn
pub fn pat_syn(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::PAT_SYN_V1)
}

/// Gr_Base
pub fn gr_base(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::GR_BASE_V1)
}

/// CWCM
pub fn cwcm(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::CWCM_V1)
}

/// UIdeo
pub fn uideo(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::UIDEO_V1)
}

/// cwt
pub fn cwt(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::CWT_V1)
}

/// alpha
pub fn alpha(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::ALPHA_V1)
}

/// Term
pub fn term(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::TERM_V1)
}

/// VS
pub fn vs(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::VS_V1)
}

/// STerm
pub fn sterm(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::STERM_V1)
}

/// bidi_c
pub fn bidi_c(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::BIDI_C_V1)
}

/// Hex
pub fn hex(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::HEX_V1)
}

/// AHex
pub fn ahex(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::AHEX_V1)
}

/// math
pub fn math(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::MATH_V1)
}

/// IDST
pub fn idst(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::IDST_V1)
}

/// Pat_WS
pub fn pat_ws(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::PAT_WS_V1)
}

/// IDSB
pub fn idsb(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::IDSB_V1)
}

/// Gr_Ext
pub fn gr_ext(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::GR_EXT_V1)
}

/// Join_C
pub fn join_c(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::JOIN_C_V1)
}

/// EComp
pub fn ecomp(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::ECOMP_V1)
}

/// Dep
pub fn dep(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::DEP_V1)
}

/// ExtPict
pub fn ext_pict(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::EXT_PICT_V1)
}

/// IDS
pub fn ids(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::IDS_V1)
}

/// CWL
pub fn cwl(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::CWL_V1)
}

/// Ext
pub fn ext(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::EXT_V1)
}

/// WSpace
pub fn wspace(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::WSPACE_V1)
}

/// Gr_Link
pub fn gr_link(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::GR_LINK_V1)
}

/// EPres
pub fn epres(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::EPRES_V1)
}

/// EBase
pub fn ebase(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::EBASE_V1)
}

/// LOE
pub fn loe(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::LOE_V1)
}

/// Dash
pub fn dash(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::DASH_V1)
}

/// Comp_Ex
pub fn comp_ex(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::COMP_EX_V1)
}

/// CWCF
pub fn cwcf(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::CWCF_V1)
}

/// QMark
pub fn qmark(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::QMARK_V1)
}

/// Radical
pub fn radical(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::RADICAL_V1)
}

/// RI
pub fn ri(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::RI_V1)
}

/// IDC
pub fn idc(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::IDC_V1)
}

/// DI
pub fn di(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::DI_V1)
}

/// SD
pub fn sd(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::SD_V1)
}

/// Dia
pub fn dia(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::DIA_V1)
}

/// Hyphen
pub fn hyphen(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::HYPHEN_V1)
}

/// Ideo
pub fn ideo(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::IDEO_V1)
}

/// CWKCF
pub fn cwkcf(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::CWKCF_V1)
}

/// XIDC
pub fn xidc(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::XIDC_V1)
}

/// CI
pub fn ci(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::CI_V1)
}

/// Cased
pub fn cased(provider: &PpucdDataProvider) -> UnicodeSet {
    get_prop(provider, key::CASED_V1)
}

#[test]
fn test_wspace_getter() {
    let ppucd_property_files_root_path = "tests/testdata/ppucd-wspace-test.txt";
    let ppucd_property_file_str = std::fs::read_to_string(ppucd_property_files_root_path).unwrap();
    let ppucd_provider: PpucdDataProvider = PpucdDataProvider::new(&ppucd_property_file_str);
    let wspace_uniset: UnicodeSet = wspace(&ppucd_provider);
    let exp_uniset: UnicodeSet = UnicodeSet::from_inversion_list(
        vec![
            9, 14, 32, 33, 133, 134, 160, 161, 5760, 5761, 8192, 8203, 8232, 8234, 8239, 8240,
            8287, 8288, 12288, 12289,
        ]
    ).unwrap();
    assert_eq!(wspace_uniset, exp_uniset);

}
