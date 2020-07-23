use icu_locale::Locale;

fn locale_test() {
    let locale_strs = 
        vec![
            "en-u-hc-h12",
            "en-US-u-hc-h23",
            "en-US-u-foo",
            "en-US-u-hc-h23-ca-islamic-civil-ss-true",
            "en-US-t-pl-Latn-DE",
            "en-US-x-private-foobar",
            "en-US-t-h0-hybrid-k0-platform-s0-true",
            "en-US-t-es-AR-x-foo",
            "en-US-u-ca-buddhist-hc-h12-t-es-AR-h0-hybrid-x-private-foobar",
        ];
    for _i in 1..100000 {
        for locale_str in &locale_strs {
            let _locale: Locale = locale_str.parse().expect("Could not parse locale string");
        }
}
}

fn main() {
    println!("Hello, world!");
    locale_test();
}
