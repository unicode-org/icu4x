// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(clippy::unreadable_literal, dead_code)]

use icu_provider_ppucd::support::*;
use icu_uniset::enum_props::*;
use icu_uniset::UnicodeSet;

#[test]
fn test_wspace_getter() {
    let ppucd_property_files_root_path = "tests/testdata/ppucd-wspace-test.txt";
    let ppucd_property_file_str = std::fs::read_to_string(ppucd_property_files_root_path).unwrap();
    let ppucd_provider: PpucdDataProvider = PpucdDataProvider::new(&ppucd_property_file_str);
    let wspace_uniset: UnicodeSet =
        icu_uniset::props::get_white_space_property(&ppucd_provider).unwrap();
    let exp_uniset: UnicodeSet = UnicodeSet::from_inversion_list(vec![
        9, 14, 32, 33, 133, 134, 160, 161, 5760, 5761, 8192, 8203, 8232, 8234, 8239, 8240, 8287,
        8288, 12288, 12289,
    ])
    .unwrap();
    assert_eq!(wspace_uniset, exp_uniset);
}

#[test]
fn test_enum_props_getters() {
    let ppucd_property_files_root_path = "tests/testdata/ppucd-enum-props-test.txt";
    let ppucd_property_file_str = std::fs::read_to_string(ppucd_property_files_root_path).unwrap();
    let ppucd_provider: PpucdDataProvider = PpucdDataProvider::new(&ppucd_property_file_str);

    // lb=LF
    let lb_lf_uniset: UnicodeSet =
        icu_uniset::props::get_line_break_val_set(&ppucd_provider, LineBreak::LineFeed).unwrap();
    let exp_lb_lf_uniset: UnicodeSet = UnicodeSet::from_inversion_list(vec![10, 11]).unwrap();
    assert_eq!(lb_lf_uniset, exp_lb_lf_uniset);
}

#[test]
fn test_enum_props_getters_truncated_resc_key_subcategory() {
    let ppucd_property_files_root_path = "tests/testdata/ppucd-inpc-topbotleft-test.txt";
    let ppucd_property_file_str = std::fs::read_to_string(ppucd_property_files_root_path).unwrap();
    let ppucd_provider: PpucdDataProvider = PpucdDataProvider::new(&ppucd_property_file_str);

    // InPC=Top_And_Bottom_And_Left
    let inpc_topbotleft_uniset: UnicodeSet =
        icu_uniset::props::get_indic_positional_category_val_set(
            &ppucd_provider,
            IndicPositionalCategory::TopAndBottomAndLeft,
        )
        .unwrap();
    let exp_inpc_topbotleft_uniset: UnicodeSet =
        UnicodeSet::from_inversion_list(vec![4156, 4157, 71454, 71455]).unwrap();
    assert_eq!(inpc_topbotleft_uniset, exp_inpc_topbotleft_uniset);
}
