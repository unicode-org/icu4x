#[macro_export]
macro_rules! data_core_helloworld_v1_bn {
    () => {
        icu_provider::hello_world::HelloWorldV1 {
            message: alloc::borrow::Cow::Borrowed("ওহে বিশ\u{9cd}ব"),
        }
    };
}

pub(crate) use data_core_helloworld_v1_bn;
