include!("baked/macros.rs");

struct TestingProvider;

const _: () = { 
    use icu_normalizer_data::*;
    mod icu {
        pub(super) use super::icu_experimental as experimental;
        pub(super) use icu_normalizer as normalizer;
        pub(super) use icu_collections as collections;
    }
    self::make_provider!(TestingProvider);
    impl_normalizer_comp_v1!(TestingProvider);
    impl_normalizer_decomp_v1!(TestingProvider);
    impl_normalizer_nfd_v1!(TestingProvider);
    impl_normalizer_nfdex_v1!(TestingProvider);
    impl_normalizer_nfkd_v1!(TestingProvider);
    impl_normalizer_nfkdex_v1!(TestingProvider);
    impl_normalizer_uts46d_v1!(TestingProvider);
    impl_transliterator_rules_v1!(TestingProvider);
};