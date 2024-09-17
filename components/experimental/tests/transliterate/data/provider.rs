include!("baked/macros.rs");

pub struct TestingProvider;

const _: () = { 
    use icu_normalizer_data::*;
    mod icu {
        pub(super) use super::icu_experimental as experimental;
        pub(super) use icu_normalizer as normalizer;
        pub(super) use icu_collections as collections;
    }
    self::make_provider!(TestingProvider);
    impl_canonical_compositions_v1_marker!(TestingProvider);
    impl_non_recursive_decomposition_supplement_v1_marker!(TestingProvider);
    impl_canonical_decomposition_data_v1_marker!(TestingProvider);
    impl_canonical_decomposition_tables_v1_marker!(TestingProvider);
    impl_compatibility_decomposition_supplement_v1_marker!(TestingProvider);
    impl_compatibility_decomposition_tables_v1_marker!(TestingProvider);
    impl_uts46_decomposition_supplement_v1_marker!(TestingProvider);
    impl_transliterator_rules_v1!(TestingProvider);
};