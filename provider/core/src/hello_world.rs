// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider returning multilingual "Hello World" strings for testing.

use crate::buf::BufferFormat;
use crate::helpers;
use crate::iter::IterableProvider;
use crate::prelude::*;
use crate::yoke::{self, *};
use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::fmt::Debug;
use core::str::FromStr;
use icu_locid::LanguageIdentifier;
use litemap::LiteMap;

pub mod key {
    use crate::resource::ResourceKey;
    pub const HELLO_WORLD_V1: ResourceKey = crate::resource_key!("core/helloworld@1");
}

/// A struct containing "Hello World" in the requested language.
#[derive(Debug, PartialEq, Clone, Yokeable, ZeroCopyFrom)]
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

/// A data provider returning Hello World strings in different languages.
///
/// Mostly useful for testing.
///
/// # Examples
///
/// ```
/// use icu_provider::hello_world::{key, HelloWorldProvider, HelloWorldV1Marker};
/// use icu_provider::prelude::*;
/// use icu_locid_macros::langid;
///
/// let provider = HelloWorldProvider::new_with_placeholder_data();
///
/// let german_hello_world: DataPayload<HelloWorldV1Marker> = provider
///     .load_payload(&DataRequest {
///         resource_path: ResourcePath {
///             key: key::HELLO_WORLD_V1,
///             options: ResourceOptions {
///                 variant: None,
///                 langid: Some(langid!("de")),
///             }
///         }
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

impl DataProvider<HelloWorldV1Marker> for HelloWorldProvider {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<HelloWorldV1Marker>, DataError> {
        key::HELLO_WORLD_V1.match_key(req.resource_path.key)?;
        let langid = req.try_langid()?;
        let data = self
            .map
            .get(langid)
            .map(|s| HelloWorldV1 { message: s.clone() })
            .ok_or_else(|| DataErrorKind::MissingLocale.with_req(req))?;
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

impl_dyn_provider!(HelloWorldProvider, {
    _ => HelloWorldV1Marker,
}, ANY);

#[cfg(feature = "serialize")]
impl_dyn_provider!(HelloWorldProvider, {
    _ => HelloWorldV1Marker,
}, SERDE_SE);

pub struct HelloWorldJsonProvider(HelloWorldProvider);

impl BufferProvider for HelloWorldJsonProvider {
    fn load_buffer(&self, req: &DataRequest) -> Result<DataResponse<BufferMarker>, DataError> {
        let result = self.0.load_payload(req)?;
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

impl IterableProvider for HelloWorldProvider {
    #[allow(clippy::needless_collect)] // https://github.com/rust-lang/rust-clippy/issues/7526
    fn supported_options_for_key(
        &self,
        resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        resc_key.match_key(key::HELLO_WORLD_V1)?;
        let list: Vec<ResourceOptions> = self
            .map
            .iter_keys()
            .map(|langid| ResourceOptions {
                variant: None,
                langid: Some(langid.clone()),
            })
            .collect();
        Ok(Box::new(list.into_iter()))
    }
}

/// Adds entries to a [`HelloWorldProvider`] from [`AnyMarker`](crate::any::AnyMarker)
impl crate::export::DataExporter<crate::any::AnyMarker> for HelloWorldProvider {
    fn put_payload(
        &mut self,
        req: DataRequest,
        payload: DataPayload<crate::any::AnyMarker>,
    ) -> Result<(), DataError> {
        req.resource_path.key.match_key(key::HELLO_WORLD_V1)?;
        let langid = req.try_langid()?;
        let downcast_payload: DataPayload<HelloWorldV1Marker> = payload.downcast()?;
        self.map.insert(
            langid.clone(),
            Cow::Owned(downcast_payload.get().message.to_string()),
        );
        Ok(())
    }
}
