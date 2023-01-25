// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[path = "tries/mod.rs"]
mod tries;

// The string has 41 chars.
#[cfg(ICU4X_EXTENDED_BENCHING)]
const SAMPLE_STRING_LATIN1: &str = "Declaration loremips umdolo loremipsompi";
#[cfg(ICU4X_EXTENDED_BENCHING)]
const SAMPLE_STRING_MIXED: &str = "Dèclaråcion ЗАГАЛЬНА 世界人权宣言 𑄟𑄚𑄬𑄭𑄃𑄇𑄴𑄇𑄥𑄧𑄁𑄢𑄴";

#[cfg(ICU4X_EXTENDED_BENCHING)]
fn bench_iai_cpt_overview(fast: bool, mixed: bool) {
    // Tests the instructions required to get CPT for 100,000 chars.

    let cpt = if fast {
        tries::gc_fast::get()
    } else {
        tries::gc_small::get()
    };
    let sample = if mixed {
        SAMPLE_STRING_MIXED
    } else {
        SAMPLE_STRING_LATIN1
    };

    let mut i: u8 = 0;
    for c in sample.chars() {
        i = i.wrapping_add(cpt.get32(c as u32))
        //i = i.wrapping_add(1);
    }

    // Ensure the loop is not DCEd
    assert!(i < 255);
}

#[cfg(ICU4X_EXTENDED_BENCHING)]
fn bench_iai_cpt_latin_fast() {
    bench_iai_cpt_overview(true, false);
}

#[cfg(ICU4X_EXTENDED_BENCHING)]
fn bench_iai_cpt_latin_small() {
    bench_iai_cpt_overview(false, false);
}

#[cfg(ICU4X_EXTENDED_BENCHING)]
fn bench_iai_cpt_mixed_fast() {
    bench_iai_cpt_overview(true, true);
}

#[cfg(ICU4X_EXTENDED_BENCHING)]
fn bench_iai_cpt_mixed_small() {
    bench_iai_cpt_overview(false, true);
}

#[cfg(ICU4X_EXTENDED_BENCHING)]
iai::main!(
    bench_iai_cpt_latin_fast,
    bench_iai_cpt_latin_small,
    bench_iai_cpt_mixed_fast,
    bench_iai_cpt_mixed_small,
);

#[cfg(not(ICU4X_EXTENDED_BENCHING))]
fn main() {}
