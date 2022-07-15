// @generated
type DataStruct = & 'static < :: icu_provider :: hello_world :: HelloWorldV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: &[(&str, DataStruct)] = &[("bn", BN), ("en", EN), ("ja", JA), ("ru", RU)];
static BN: DataStruct = &::icu_provider::hello_world::HelloWorldV1 {
    message: alloc::borrow::Cow::Borrowed("ওহে বিশ\u{9cd}ব"),
};
static EN: DataStruct = &::icu_provider::hello_world::HelloWorldV1 {
    message: alloc::borrow::Cow::Borrowed("Hello World"),
};
static JA: DataStruct = &::icu_provider::hello_world::HelloWorldV1 {
    message: alloc::borrow::Cow::Borrowed("こんにちは世界"),
};
static RU: DataStruct = &::icu_provider::hello_world::HelloWorldV1 {
    message: alloc::borrow::Cow::Borrowed("Привет, мир"),
};
