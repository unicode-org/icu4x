#[macro_export]
macro_rules! data_core_helloworld_v1_ru {
    () => {
        icu_provider::hello_world::HelloWorldV1 {
            message: alloc::borrow::Cow::Borrowed("Привет, мир"),
        }
    }
}

pub use data_core_helloworld_v1_ru;
