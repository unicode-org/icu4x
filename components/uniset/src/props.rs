// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
#![allow(clippy::unreadable_literal, dead_code)]

use crate::provider::*;
use crate::{UnicodeSet, UnicodeSetError};
use icu_provider::prelude::*;
use std::borrow::Cow;
use std::convert::TryInto;

// helper fn
fn get_prop<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    ppucd_provider: &D,
    resc_key: ResourceKey,
) -> Result<UnicodeSet, UnicodeSetError> {
    let data_req = DataRequest {
        resource_path: ResourcePath {
            key: resc_key,
            options: ResourceOptions {
                variant: None,
                langid: None,
            },
        },
    };
    let mut resp: DataResponse<UnicodeProperty> = ppucd_provider.load_payload(&data_req)?;

    let ppucd_property_cow: Cow<UnicodeProperty> = resp.take_payload()?;
    let ppucd_property: UnicodeProperty = ppucd_property_cow.into_owned();
    ppucd_property.try_into()
}

/// Upper
pub fn upper<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::UPPER_V1)
}

/// Bidi_M
pub fn bidi_m<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_M_V1)
}

/// PCM
pub fn pcm<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::PCM_V1)
}

/// Emoji
pub fn emoji<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::EMOJI_V1)
}

/// lower
pub fn lower<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LOWER_V1)
}

/// EMod
pub fn emod<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::EMOD_V1)
}

/// XIDS
pub fn xids<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::XIDS_V1)
}

/// CWU
pub fn cwu<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CWU_V1)
}

/// Pat_Syn
pub fn pat_syn<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::PAT_SYN_V1)
}

/// Gr_Base
pub fn gr_base<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GR_BASE_V1)
}

/// CWCM
pub fn cwcm<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CWCM_V1)
}

/// UIdeo
pub fn uideo<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::UIDEO_V1)
}

/// cwt
pub fn cwt<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CWT_V1)
}

/// alpha
pub fn alpha<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::ALPHA_V1)
}

/// Term
pub fn term<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TERM_V1)
}

/// VS
pub fn vs<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::VS_V1)
}

/// STerm
pub fn sterm<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::STERM_V1)
}

/// bidi_c
pub fn bidi_c<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_C_V1)
}

/// Hex
pub fn hex<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::HEX_V1)
}

/// AHex
pub fn ahex<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::AHEX_V1)
}

/// math
pub fn math<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::MATH_V1)
}

/// IDST
pub fn idst<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::IDST_V1)
}

/// Pat_WS
pub fn pat_ws<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::PAT_WS_V1)
}

/// IDSB
pub fn idsb<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::IDSB_V1)
}

/// Gr_Ext
pub fn gr_ext<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GR_EXT_V1)
}

/// Join_C
pub fn join_c<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOIN_C_V1)
}

/// EComp
pub fn ecomp<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::ECOMP_V1)
}

/// Dep
pub fn dep<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DEP_V1)
}

/// ExtPict
pub fn ext_pict<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::EXT_PICT_V1)
}

/// IDS
pub fn ids<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::IDS_V1)
}

/// CWL
pub fn cwl<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CWL_V1)
}

/// Ext
pub fn ext<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::EXT_V1)
}

/// WSpace
pub fn wspace<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WSPACE_V1)
}

/// Gr_Link
pub fn gr_link<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GR_LINK_V1)
}

/// EPres
pub fn epres<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::EPRES_V1)
}

/// EBase
pub fn ebase<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::EBASE_V1)
}

/// LOE
pub fn loe<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LOE_V1)
}

/// Dash
pub fn dash<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DASH_V1)
}

/// Comp_Ex
pub fn comp_ex<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::COMP_EX_V1)
}

/// CWCF
pub fn cwcf<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CWCF_V1)
}

/// QMark
pub fn qmark<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::QMARK_V1)
}

/// Radical
pub fn radical<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::RADICAL_V1)
}

/// RI
pub fn ri<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::RI_V1)
}

/// IDC
pub fn idc<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::IDC_V1)
}

/// DI
pub fn di<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DI_V1)
}

/// SD
pub fn sd<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::SD_V1)
}

/// Dia
pub fn dia<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DIA_V1)
}

/// Hyphen
pub fn hyphen<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::HYPHEN_V1)
}

/// Ideo
pub fn ideo<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::IDEO_V1)
}

/// CWKCF
pub fn cwkcf<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CWKCF_V1)
}

/// XIDC
pub fn xidc<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::XIDC_V1)
}

/// CI
pub fn ci<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CI_V1)
}

/// Cased
pub fn cased<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CASED_V1)
}
