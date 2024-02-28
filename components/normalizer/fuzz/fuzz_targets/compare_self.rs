// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![no_main]
use libfuzzer_sys::fuzz_target;
use icu_normalizer::ComposingNormalizer;
use icu_normalizer::DecomposingNormalizer;
use utf16_iter::Utf16CharsEx;
use utf8_iter::Utf8CharsEx;

fuzz_target!(|data: &[u8]| {
    let well_formed = String::from_utf8_lossy(data);
    let utf16: Vec<u16> = well_formed.encode_utf16().collect();

    let nfd = DecomposingNormalizer::new_nfd();
    let nfkd = DecomposingNormalizer::new_nfkd();
    let nfc = ComposingNormalizer::new_nfc();
    let nfkc = ComposingNormalizer::new_nfkc();

    // Not macroizing these to get nice line numbers by default.

    // NFD

    let is_normalized_nfd_str = nfd.is_normalized(&well_formed);
    let is_normalized_nfd_utf8 = nfd.is_normalized_utf8(data);
    assert_eq!(is_normalized_nfd_str, is_normalized_nfd_utf8);
    let is_normalized_nfd_utf16 = nfd.is_normalized_utf16(&utf16);
    assert_eq!(is_normalized_nfd_utf16, is_normalized_nfd_str);

    let nfd_str = nfd.normalize(&well_formed);
    let nfd_str_iter: String = nfd.normalize_iter(well_formed.chars()).collect();
    assert_eq!(nfd_str.as_str(), nfd_str_iter.as_str());

    let nfd_utf8 = nfd.normalize_utf8(data);
    let nfd_utf8_iter: String = nfd.normalize_iter(data.chars()).collect();
    assert_eq!(nfd_utf8.as_str(), nfd_utf8_iter.as_str());

    assert_eq!(nfd_str.as_str(), nfd_utf8.as_str());

    let nfd_utf16 = nfd.normalize_utf16(&utf16);
    let mut nfd_utf16_iter: Vec<u16> = Vec::new();
    for c in nfd.normalize_iter(utf16.chars()) {
        let mut buf = [0u16; 2];
        nfd_utf16_iter.extend_from_slice(c.encode_utf16(&mut buf));
    }
    assert_eq!(nfd_utf16.as_slice(), nfd_utf16_iter.as_slice());

    let nfd_from_utf16 = String::from_utf16(&nfd_utf16);
    assert!(nfd_from_utf16.is_ok());

    assert_eq!(nfd_str.as_str(), nfd_from_utf16.unwrap().as_str());

    if is_normalized_nfd_str {
        assert_eq!(nfd_str.as_str(), &well_formed);
        assert_eq!(nfd_utf8.as_str(), &well_formed); // not data!
        assert_eq!(nfd_utf16.as_slice(), utf16.as_slice());
    }

    // NFC

    let is_normalized_nfc_str = nfc.is_normalized(&well_formed);
    let is_normalized_nfc_utf8 = nfc.is_normalized_utf8(data);
    assert_eq!(is_normalized_nfc_str, is_normalized_nfc_utf8);
    let is_normalized_nfc_utf16 = nfc.is_normalized_utf16(&utf16);
    assert_eq!(is_normalized_nfc_utf16, is_normalized_nfc_str);

    let nfc_str = nfc.normalize(&well_formed);
    let nfc_str_iter: String = nfc.normalize_iter(well_formed.chars()).collect();
    assert_eq!(nfc_str.as_str(), nfc_str_iter.as_str());

    let nfc_utf8 = nfc.normalize_utf8(data);
    let nfc_utf8_iter: String = nfc.normalize_iter(data.chars()).collect();
    assert_eq!(nfc_utf8.as_str(), nfc_utf8_iter.as_str());

    assert_eq!(nfc_str.as_str(), nfc_utf8.as_str());

    let nfc_utf16 = nfc.normalize_utf16(&utf16);
    let mut nfc_utf16_iter: Vec<u16> = Vec::new();
    for c in nfc.normalize_iter(utf16.chars()) {
        let mut buf = [0u16; 2];
        nfc_utf16_iter.extend_from_slice(c.encode_utf16(&mut buf));
    }
    assert_eq!(nfc_utf16.as_slice(), nfc_utf16_iter.as_slice());

    let nfc_from_utf16 = String::from_utf16(&nfc_utf16);
    assert!(nfc_from_utf16.is_ok());

    assert_eq!(nfc_str.as_str(), nfc_from_utf16.unwrap().as_str());

    if is_normalized_nfc_str {
        assert_eq!(nfc_str.as_str(), &well_formed);
        assert_eq!(nfc_utf8.as_str(), &well_formed); // not data!
        assert_eq!(nfc_utf16.as_slice(), utf16.as_slice());
    }

    // NFKD

    let is_normalized_nfkd_str = nfkd.is_normalized(&well_formed);
    let is_normalized_nfkd_utf8 = nfkd.is_normalized_utf8(data);
    assert_eq!(is_normalized_nfkd_str, is_normalized_nfkd_utf8);
    let is_normalized_nfkd_utf16 = nfkd.is_normalized_utf16(&utf16);
    assert_eq!(is_normalized_nfkd_utf16, is_normalized_nfkd_str);

    let nfkd_str = nfkd.normalize(&well_formed);
    let nfkd_str_iter: String = nfkd.normalize_iter(well_formed.chars()).collect();
    assert_eq!(nfkd_str.as_str(), nfkd_str_iter.as_str());

    let nfkd_utf8 = nfkd.normalize_utf8(data);
    let nfkd_utf8_iter: String = nfkd.normalize_iter(data.chars()).collect();
    assert_eq!(nfkd_utf8.as_str(), nfkd_utf8_iter.as_str());

    assert_eq!(nfkd_str.as_str(), nfkd_utf8.as_str());

    let nfkd_utf16 = nfkd.normalize_utf16(&utf16);
    let mut nfkd_utf16_iter: Vec<u16> = Vec::new();
    for c in nfkd.normalize_iter(utf16.chars()) {
        let mut buf = [0u16; 2];
        nfkd_utf16_iter.extend_from_slice(c.encode_utf16(&mut buf));
    }
    assert_eq!(nfkd_utf16.as_slice(), nfkd_utf16_iter.as_slice());

    let nfkd_from_utf16 = String::from_utf16(&nfkd_utf16);
    assert!(nfkd_from_utf16.is_ok());

    assert_eq!(nfkd_str.as_str(), nfkd_from_utf16.unwrap().as_str());

    if is_normalized_nfkd_str {
        assert_eq!(nfkd_str.as_str(), &well_formed);
        assert_eq!(nfkd_utf8.as_str(), &well_formed); // not data!
        assert_eq!(nfkd_utf16.as_slice(), utf16.as_slice());
    }

    // NFKC

    let is_normalized_nfkc_str = nfkc.is_normalized(&well_formed);
    let is_normalized_nfkc_utf8 = nfkc.is_normalized_utf8(data);
    assert_eq!(is_normalized_nfkc_str, is_normalized_nfkc_utf8);
    let is_normalized_nfkc_utf16 = nfkc.is_normalized_utf16(&utf16);
    assert_eq!(is_normalized_nfkc_utf16, is_normalized_nfkc_str);

    let nfkc_str = nfkc.normalize(&well_formed);
    let nfkc_str_iter: String = nfkc.normalize_iter(well_formed.chars()).collect();
    assert_eq!(nfkc_str.as_str(), nfkc_str_iter.as_str());

    let nfkc_utf8 = nfkc.normalize_utf8(data);
    let nfkc_utf8_iter: String = nfkc.normalize_iter(data.chars()).collect();
    assert_eq!(nfkc_utf8.as_str(), nfkc_utf8_iter.as_str());

    assert_eq!(nfkc_str.as_str(), nfkc_utf8.as_str());

    let nfkc_utf16 = nfkc.normalize_utf16(&utf16);
    let mut nfkc_utf16_iter: Vec<u16> = Vec::new();
    for c in nfkc.normalize_iter(utf16.chars()) {
        let mut buf = [0u16; 2];
        nfkc_utf16_iter.extend_from_slice(c.encode_utf16(&mut buf));
    }
    assert_eq!(nfkc_utf16.as_slice(), nfkc_utf16_iter.as_slice());

    let nfkc_from_utf16 = String::from_utf16(&nfkc_utf16);
    assert!(nfkc_from_utf16.is_ok());

    assert_eq!(nfkc_str.as_str(), nfkc_from_utf16.unwrap().as_str());

    if is_normalized_nfkc_str {
        assert_eq!(nfkc_str.as_str(), &well_formed);
        assert_eq!(nfkc_utf8.as_str(), &well_formed); // not data!
        assert_eq!(nfkc_utf16.as_slice(), utf16.as_slice());
    }
});
