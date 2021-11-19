use crate::CaseMapping;
use icu_provider::yoke::{self, *};

pub mod key {
    use icu_provider::{resource_key, ResourceKey};

    // TODO: Rename ResourceCategory::UnicodeSet to UnicodeProperty?
    pub const CASE_MAPPING_V1: ResourceKey = resource_key!(UnicodeSet, "case_map", 1);
}

// #[icu_provider::data_struct]
// #[derive(Debug, PartialEq, Clone)]
// #[cfg_attr(
//     feature = "provider_serde",
//     derive(serde::Serialize, serde::Deserialize)
// )]
// pub struct CaseMappingV1<'data> {
//     #[cfg_attr(feature = "provider_serde", serde(borrow))]
//     pub inv_list: CaseMapping<'data>,
// }
