macro_rules! data_core_helloworld_v1_en {
    () => {
        ::icu_provider::hello_world::HelloWorldV1 {
            message: alloc::borrow::Cow::Borrowed("Hello World"),
        }
    }
}

pub(crate) use data_core_helloworld_v1_en;
