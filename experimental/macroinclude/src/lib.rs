#![allow(unused_imports)]
#![allow(unused_macros)]

extern crate alloc;

mod baked {
    #[cfg(feature = "custom_data")]
    include!(env!("ICU4X_MACROINCLUDE_PATH"));
    #[cfg(not(feature = "custom_data"))]
    include!("data/mod.rs");
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_provider::prelude::*;

    #[test]
    fn test() {
        use baked::core_helloworld_v1::*;
        struct BakedProvider;
        impl_core_helloworld_v1!(BakedProvider);

        assert_eq!(
            "こんにちは世界",
            icu_provider::hello_world::HelloWorldFormatter::try_new_unstable(
                &BakedProvider,
                &icu_locid::locale!("ja").into(),
            )
            .unwrap()
            .format_to_string()
        );
    }
}
