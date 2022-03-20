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
use core::str::FromStr;
use icu_locid::LanguageIdentifier;
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
/// use icu_locid_macros::langid;
///
/// let provider = HelloWorldProvider::new_with_placeholder_data();
///
/// let german_hello_world: DataPayload<HelloWorldV1Marker> = provider
///     .load_resource(&DataRequest {
///         options: langid!("de").into(),
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
    map: LiteMap<LanguageIdentifier, Cow<'static, str>>,
}

impl HelloWorldProvider {
    /// Creates a [`HelloWorldProvider`] pre-populated with hardcoded data from Wiktionary.
    pub fn new_with_placeholder_data() -> HelloWorldProvider {
        // Data from https://en.wiktionary.org/wiki/Hello_World#Translations
        // Note: we don't want to use langid!() because icu_langid_macros is heavy.
        HelloWorldProvider {
            map: [
                ("bn", "ওহে বিশ্ব"),
                ("cs", "Ahoj světe"),
                ("de", "Hallo Welt"),
                ("el", "Καλημέρα κόσμε"),
                ("en", "Hello World"),
                ("eo", "Saluton, Mondo"),
                ("fa", "سلام دنیا‎"),
                ("fi", "hei maailma"),
                ("is", "Halló, heimur"),
                ("ja", "こんにちは世界"),
                ("la", "Ave, munde"),
                ("pt", "Olá, mundo"),
                ("ro", "Salut, lume"),
                ("ru", "Привет, мир"),
                ("vi", "Xin chào thế giới"),
                ("zh", "你好世界"),
            ]
            .iter()
            .map(|(loc, value)| {
                (
                    LanguageIdentifier::from_str(loc).unwrap(),
                    Cow::Borrowed(*value),
                )
            })
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
        let langid = req
            .get_langid()
            .ok_or_else(|| DataErrorKind::NeedsLocale.with_req(HelloWorldV1Marker::KEY, req))?;
        let data = self
            .map
            .get(langid)
            .map(|s| HelloWorldV1 { message: s.clone() })
            .ok_or_else(|| DataErrorKind::MissingLocale.with_key(HelloWorldV1Marker::KEY))?;
        let metadata = DataResponseMetadata {
            data_langid: Some(langid.clone()),
            ..Default::default()
        };
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(data)),
        })
    }
}

impl_dyn_provider!(HelloWorldProvider, [HelloWorldV1Marker,], ANY);

#[cfg(all(feature = "serialize", not(feature = "datagen")))]
impl_dyn_provider!(HelloWorldProvider, [HelloWorldV1Marker,], SERDE_SE);

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
    let mut supported_langids: Vec<LanguageIdentifier> = provider
        .supported_options()
        .unwrap()
        .map(|resc_options| resc_options.langid.unwrap())
        .collect();
    supported_langids.sort();

    assert_eq!(
        supported_langids,
        vec![
            icu_locid_macros::langid!("bn"),
            icu_locid_macros::langid!("cs"),
            icu_locid_macros::langid!("de"),
            icu_locid_macros::langid!("el"),
            icu_locid_macros::langid!("en"),
            icu_locid_macros::langid!("eo"),
            icu_locid_macros::langid!("fa"),
            icu_locid_macros::langid!("fi"),
            icu_locid_macros::langid!("is"),
            icu_locid_macros::langid!("ja"),
            icu_locid_macros::langid!("la"),
            icu_locid_macros::langid!("pt"),
            icu_locid_macros::langid!("ro"),
            icu_locid_macros::langid!("ru"),
            icu_locid_macros::langid!("vi"),
            icu_locid_macros::langid!("zh")
        ]
    );
}
