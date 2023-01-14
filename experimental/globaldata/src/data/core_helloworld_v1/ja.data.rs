macro_rules! data_core_helloworld_v1_ja {
    () => {
        ::icu_provider::hello_world::HelloWorldV1 {
            message: alloc::borrow::Cow::Borrowed("こんにちは世界"),
        }
    }
}

pub(crate) use data_core_helloworld_v1_ja;
