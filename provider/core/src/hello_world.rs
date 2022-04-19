// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider returning multilingual "Hello World" strings for testing.

use crate::buf::BufferFormat;
#[cfg(feature = "datagen")]
use crate::datagen::IterableResourceProvider;
use crate::helpers;
use crate::prelude::*;
use crate::yoke::{self, *};
use crate::zerofrom::{self, *};
use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use core::fmt::Debug;
use icu_locid::locale;
use litemap::LiteMap;

/// A struct containing "Hello World" in the requested language.
#[derive(Debug, PartialEq, Clone, Yokeable, ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HelloWorldV1<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub message: Cow<'data, str>,
}

impl Default for HelloWorldV1<'_> {
    fn default() -> Self {
        HelloWorldV1 {
            message: Cow::Borrowed("(und) Hello World"),
        }
    }
}

/// Marker type for [`HelloWorldV1`].
pub struct HelloWorldV1Marker;

impl DataMarker for HelloWorldV1Marker {
    type Yokeable = HelloWorldV1<'static>;
}

impl ResourceMarker for HelloWorldV1Marker {
    const KEY: ResourceKey = crate::resource_key!("core/helloworld@1");
}

/// A data provider returning Hello World strings in different languages.
///
/// Mostly useful for testing.
///
/// # Examples
///
/// ```
/// use icu_provider::hello_world::*;
/// use icu_provider::prelude::*;
/// use icu_locid::locale;
///
/// let provider = HelloWorldProvider::new_with_placeholder_data();
///
/// let german_hello_world: DataPayload<HelloWorldV1Marker> = provider
///     .load_resource(&DataRequest {
///         options: locale!("de").into(),
///         metadata: Default::default(),
///     })
///     .expect("Loading should succeed")
///     .take_payload()
///     .expect("Data should be present");
///
/// assert_eq!("Hallo Welt", german_hello_world.get().message);
/// ```
#[derive(Debug, PartialEq, Default)]
pub struct HelloWorldProvider {
    map: LiteMap<ResourceOptions, Cow<'static, str>>,
}

impl HelloWorldProvider {
    /// Creates a [`HelloWorldProvider`] pre-populated with hardcoded data from Wiktionary.
    pub fn new_with_placeholder_data() -> HelloWorldProvider {
        // Data from https://en.wiktionary.org/wiki/Hello_World#Translations
        HelloWorldProvider {
            map: [
                (locale!("bn"), "ওহে বিশ্ব"),
                (locale!("cs"), "Ahoj světe"),
                (locale!("de"), "Hallo Welt"),
                (locale!("el"), "Καλημέρα κόσμε"),
                (locale!("en"), "Hello World"),
                (locale!("eo"), "Saluton, Mondo"),
                (locale!("fa"), "سلام دنیا‎"),
                (locale!("fi"), "hei maailma"),
                (locale!("is"), "Halló, heimur"),
                (locale!("ja"), "こんにちは世界"),
                (locale!("la"), "Ave, munde"),
                (locale!("pt"), "Olá, mundo"),
                (locale!("ro"), "Salut, lume"),
                (locale!("ru"), "Привет, мир"),
                (locale!("vi"), "Xin chào thế giới"),
                (locale!("zh"), "你好世界"),
            ]
            .into_iter()
            .map(|(loc, value)| (loc.into(), value.into()))
            .collect(),
        }
    }

    /// Converts this provider into one that serves JSON blobs of the same data.
    pub fn into_json_provider(self) -> HelloWorldJsonProvider {
        HelloWorldJsonProvider(self)
    }
}

impl ResourceProvider<HelloWorldV1Marker> for HelloWorldProvider {
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<HelloWorldV1Marker>, DataError> {
        let data = self
            .map
            .get(&req.options)
            .map(|s| HelloWorldV1 { message: s.clone() })
            .ok_or_else(|| DataErrorKind::MissingLocale.with_key(HelloWorldV1Marker::KEY))?;
        let metadata = DataResponseMetadata {
            data_langid: Some(req.options.get_langid()),
            ..Default::default()
        };
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(data)),
        })
    }
}

impl_dyn_provider!(HelloWorldProvider, [HelloWorldV1Marker,], ANY);

#[cfg(feature = "datagen")]
impl_dyn_provider!(
    HelloWorldProvider,
    [HelloWorldV1Marker,],
    SERDE_SE,
    ITERABLE_SERDE_SE,
    DATA_CONVERTER
);

pub struct HelloWorldJsonProvider(HelloWorldProvider);

impl BufferProvider for HelloWorldJsonProvider {
    fn load_buffer(
        &self,
        key: ResourceKey,
        req: &DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        key.match_key(HelloWorldV1Marker::KEY)?;
        let result = self.0.load_resource(req)?;
        let (mut metadata, old_payload) =
            DataResponse::<HelloWorldV1Marker>::take_metadata_and_payload(result)?;
        metadata.buffer_format = Some(BufferFormat::Json);
        let mut buffer = String::new();
        buffer.push_str("{\"message\":\"");
        helpers::escape_for_json(&old_payload.get().message, &mut buffer);
        buffer.push_str("\"}");
        let boxed_u8: Box<[u8]> = buffer.into_boxed_str().into();
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_rc_buffer(Rc::from(boxed_u8))),
        })
    }
}

#[cfg(feature = "datagen")]
impl IterableResourceProvider<HelloWorldV1Marker> for HelloWorldProvider {
    fn supported_options(
        &self,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
        Ok(Box::new(
            self.map
                .iter_keys()
                .cloned()
                .map(Into::<ResourceOptions>::into),
        ))
    }
}

#[test]
fn test_iter() {
    let provider = HelloWorldProvider::new_with_placeholder_data();
    let mut supported_langids: Vec<ResourceOptions> =
        provider.supported_options().unwrap().collect();
    supported_langids.sort();

    assert_eq!(
        supported_langids,
        vec![
            locale!("bn").into(),
            locale!("cs").into(),
            locale!("de").into(),
            locale!("el").into(),
            locale!("en").into(),
            locale!("eo").into(),
            locale!("fa").into(),
            locale!("fi").into(),
            locale!("is").into(),
            locale!("ja").into(),
            locale!("la").into(),
            locale!("pt").into(),
            locale!("ro").into(),
            locale!("ru").into(),
            locale!("vi").into(),
            locale!("zh").into()
        ]
    );
}
