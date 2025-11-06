// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Testing locale parsing logic against various known datasets.

/// List of real-world POSIX locales, based on GNU libc's localization data
/// Data used:
/// - Filenames in https://sourceware.org/git/?p=glibc.git;a=tree;f=localedata/locales;hb=HEAD
/// - Legacy `@saaho` modifier: https://sourceware.org/git/?p=glibc.git;a=blob;f=ChangeLog.old/ChangeLog.28;hb=HEAD#l356
const POSIX_DATASET: &str = include_str!("posix.txt");
/// List of real-world Windows locales, based on Microsoft's LCID reference.
/// MS-LCID sections 2.2 ("LCID Structure") and 5 ("Appendix A: Product Behaviour")
/// https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-lcid/70feba9f-294e-491e-b6eb-56532684c37f
const WINDOWS_DATASET: &str = include_str!("windows.txt");

#[test]
fn posix() {
    use icu_host_info::locale::PosixLocale;
    use icu_locale_core::Locale;

    for locale in POSIX_DATASET.lines() {
        let posix_locale = PosixLocale::try_from_str(locale).expect(locale);

        Locale::try_from(posix_locale).expect(locale);
    }
}

#[test]
fn windows() {
    use icu_host_info::locale::WindowsLocale;
    use icu_locale_core::Locale;

    for locale in WINDOWS_DATASET.lines() {
        let windows_locale = WindowsLocale::try_from_str(locale).expect(locale);

        Locale::try_from(windows_locale).expect(locale);
    }
}
