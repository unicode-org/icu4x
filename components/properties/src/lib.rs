// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Definitions of [Unicode Properties] and APIs for
//! retrieving property data in an appropriate data structure.
//!
//! This module is published as its own crate ([`icu_properties`](https://docs.rs/icu_properties/latest/icu_properties/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! APIs that return a [`CodePointSetData`] exist for binary properties and certain enumerated
//! properties. See the [`sets`] module for more details.
//!
//! APIs that return a [`CodePointMapData`] exist for certain enumerated properties. See the
//! [`maps`] module for more details.
//!
//! # Examples
//!
//! ## Property data as `CodePointSetData`s
//!
//! ```
//! use icu::properties::{maps, sets, GeneralCategory};
//!
//! // A binary property as a `CodePointSetData`
//!
//! let data = sets::load_emoji(&icu_testdata::unstable())
//!     .expect("The data should be valid");
//! let emoji = data.as_borrowed();
//!
//! assert!(emoji.contains('🎃')); // U+1F383 JACK-O-LANTERN
//! assert!(!emoji.contains('木')); // U+6728
//!
//! // An individual enumerated property value as a `CodePointSetData`
//!
//! let data = maps::load_general_category(&icu_testdata::unstable())
//!     .expect("The data should be valid");
//! let gc = data.as_borrowed();
//! let line_sep_data = gc.get_set_for_value(GeneralCategory::LineSeparator);
//! let line_sep = line_sep_data.as_borrowed();
//!
//! assert!(line_sep.contains32(0x2028));
//! assert!(!line_sep.contains32(0x2029));
//! ```
//!
//! ## Property data as `CodePointMapData`s
//!
//! ```
//! use icu::properties::{maps, Script};
//!
//! let map = maps::load_script(&icu_testdata::unstable())
//!     .expect("The data should be valid");
//! let script = map.as_borrowed();
//!
//! assert_eq!(script.get('🎃'), Script::Common); // U+1F383 JACK-O-LANTERN
//! assert_eq!(script.get('木'), Script::Han); // U+6728
//! ```
//!
//! [`ICU4X`]: ../icu/index.html
//! [Unicode Properties]: https://unicode-org.github.io/icu/userguide/strings/properties.html
//! [`CodePointSetData`]: crate::sets::CodePointSetData
//! [`CodePointMapData`]: crate::maps::CodePointMapData
//! [`sets`]: crate::sets

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

extern crate alloc;

#[cfg(feature = "bidi")]
pub mod bidi;

mod error;
pub mod maps;

// NOTE: The Pernosco debugger has special knowledge
// of the `CanonicalCombiningClass` struct inside the `props`
// module. Please do not change the crate-module-qualified
// name of that struct without coordination.
mod props;

pub mod bidi_data;
pub mod exemplar_chars;
pub mod provider;
pub(crate) mod runtime;
#[allow(clippy::exhaustive_structs)] // TODO
pub mod script;
pub mod sets;
mod trievalue;

pub use props::{
    BidiClass, CanonicalCombiningClass, EastAsianWidth, GeneralCategory, GeneralCategoryGroup,
    GraphemeClusterBreak, LineBreak, Script, SentenceBreak, WordBreak,
};

/// Module for working with the names of property values
pub mod names {
    pub use crate::props::{
        PropertyEnumToValueNameLinearMapper, PropertyEnumToValueNameLinearMapperBorrowed,
    };
    pub use crate::props::{
        PropertyEnumToValueNameLinearTiny4Mapper, PropertyEnumToValueNameLinearTiny4MapperBorrowed,
    };
    pub use crate::props::{
        PropertyEnumToValueNameSparseMapper, PropertyEnumToValueNameSparseMapperBorrowed,
    };
    pub use crate::props::{PropertyValueNameToEnumMapper, PropertyValueNameToEnumMapperBorrowed};
}

pub use error::PropertiesError;

#[doc(no_inline)]
pub use PropertiesError as Error;

#[cfg(feature = "data")]
#[doc(hidden)]
pub mod data {
    use icu_properties_data::*;

    use crate as icu_properties;
    pub(crate) struct Provider;
    impl_propnames_from_gcb_v1!(Provider);
    impl_propnames_from_bc_v1!(Provider);
    impl_propnames_from_ccc_v1!(Provider);
    impl_propnames_from_ea_v1!(Provider);
    impl_propnames_from_gc_v1!(Provider);
    impl_propnames_from_gcm_v1!(Provider);
    impl_propnames_from_lb_v1!(Provider);
    impl_propnames_from_sb_v1!(Provider);
    impl_propnames_from_sc_v1!(Provider);
    impl_propnames_from_wb_v1!(Provider);
    impl_propnames_to_long_linear_bc_v1!(Provider);
    impl_propnames_to_long_linear_ea_v1!(Provider);
    impl_propnames_to_long_linear_gc_v1!(Provider);
    impl_propnames_to_long_linear_gcb_v1!(Provider);
    impl_propnames_to_long_linear_lb_v1!(Provider);
    impl_propnames_to_long_linear_sb_v1!(Provider);
    impl_propnames_to_long_linear_sc_v1!(Provider);
    impl_propnames_to_long_linear_wb_v1!(Provider);
    impl_propnames_to_long_sparse_ccc_v1!(Provider);
    impl_propnames_to_short_linear_bc_v1!(Provider);
    impl_propnames_to_short_linear_ea_v1!(Provider);
    impl_propnames_to_short_linear_gc_v1!(Provider);
    impl_propnames_to_short_linear_gcb_v1!(Provider);
    impl_propnames_to_short_linear_lb_v1!(Provider);
    impl_propnames_to_short_linear_sb_v1!(Provider);
    impl_propnames_to_short_linear_wb_v1!(Provider);
    impl_propnames_to_short_linear4_sc_v1!(Provider);
    impl_propnames_to_short_sparse_ccc_v1!(Provider);
    impl_props_ahex_v1!(Provider);
    impl_props_alnum_v1!(Provider);
    impl_props_alpha_v1!(Provider);
    impl_props_basic_emoji_v1!(Provider);
    impl_props_bc_v1!(Provider);
    impl_props_bidi_c_v1!(Provider);
    impl_props_bidi_m_v1!(Provider);
    impl_props_bidiauxiliaryprops_v1!(Provider);
    impl_props_blank_v1!(Provider);
    impl_props_cased_v1!(Provider);
    impl_props_ccc_v1!(Provider);
    impl_props_ci_v1!(Provider);
    impl_props_comp_ex_v1!(Provider);
    impl_props_cwcf_v1!(Provider);
    impl_props_cwcm_v1!(Provider);
    impl_props_cwkcf_v1!(Provider);
    impl_props_cwl_v1!(Provider);
    impl_props_cwt_v1!(Provider);
    impl_props_cwu_v1!(Provider);
    impl_props_dash_v1!(Provider);
    impl_props_dep_v1!(Provider);
    impl_props_di_v1!(Provider);
    impl_props_dia_v1!(Provider);
    impl_props_ea_v1!(Provider);
    impl_props_ebase_v1!(Provider);
    impl_props_ecomp_v1!(Provider);
    impl_props_emod_v1!(Provider);
    impl_props_emoji_v1!(Provider);
    impl_props_epres_v1!(Provider);
    impl_props_exemplarchars_auxiliary_v1!(Provider);
    impl_props_exemplarchars_index_v1!(Provider);
    impl_props_exemplarchars_main_v1!(Provider);
    impl_props_exemplarchars_numbers_v1!(Provider);
    impl_props_exemplarchars_punctuation_v1!(Provider);
    impl_props_ext_v1!(Provider);
    impl_props_extpict_v1!(Provider);
    impl_props_gc_v1!(Provider);
    impl_props_gcb_v1!(Provider);
    impl_props_gr_base_v1!(Provider);
    impl_props_gr_ext_v1!(Provider);
    impl_props_gr_link_v1!(Provider);
    impl_props_graph_v1!(Provider);
    impl_props_hex_v1!(Provider);
    impl_props_hyphen_v1!(Provider);
    impl_props_idc_v1!(Provider);
    impl_props_ideo_v1!(Provider);
    impl_props_ids_v1!(Provider);
    impl_props_idsb_v1!(Provider);
    impl_props_idst_v1!(Provider);
    impl_props_join_c_v1!(Provider);
    impl_props_lb_v1!(Provider);
    impl_props_loe_v1!(Provider);
    impl_props_lower_v1!(Provider);
    impl_props_math_v1!(Provider);
    impl_props_nchar_v1!(Provider);
    impl_props_nfcinert_v1!(Provider);
    impl_props_nfdinert_v1!(Provider);
    impl_props_nfkcinert_v1!(Provider);
    impl_props_nfkdinert_v1!(Provider);
    impl_props_pat_syn_v1!(Provider);
    impl_props_pat_ws_v1!(Provider);
    impl_props_pcm_v1!(Provider);
    impl_props_print_v1!(Provider);
    impl_props_qmark_v1!(Provider);
    impl_props_radical_v1!(Provider);
    impl_props_ri_v1!(Provider);
    impl_props_sb_v1!(Provider);
    impl_props_sc_v1!(Provider);
    impl_props_scx_v1!(Provider);
    impl_props_sd_v1!(Provider);
    impl_props_segstart_v1!(Provider);
    impl_props_sensitive_v1!(Provider);
    impl_props_sterm_v1!(Provider);
    impl_props_term_v1!(Provider);
    impl_props_uideo_v1!(Provider);
    impl_props_upper_v1!(Provider);
    impl_props_vs_v1!(Provider);
    impl_props_wb_v1!(Provider);
    impl_props_wspace_v1!(Provider);
    impl_props_xdigit_v1!(Provider);
    impl_props_xidc_v1!(Provider);
    impl_props_xids_v1!(Provider);
}
