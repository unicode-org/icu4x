// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider returning multilingual "Hello World" strings for testing.

#![allow(clippy::exhaustive_structs)] // data struct module

use crate::buf::BufferFormat;
#[cfg(feature = "datagen")]
use crate::datagen::IterableDataProvider;
use crate::helpers;
use crate::prelude::*;
use crate::yoke::{self, *};
use crate::zerofrom::{self, *};
use alloc::borrow::Cow;
use alloc::string::String;
use core::fmt::Debug;

/// A struct containing "Hello World" in the requested language.
#[derive(Debug, PartialEq, Clone, Yokeable, ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_provider::hello_world))]
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
#[cfg_attr(feature = "datagen", derive(Default, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_provider::hello_world))]
pub struct HelloWorldV1Marker;

impl DataMarker for HelloWorldV1Marker {
    type Yokeable = HelloWorldV1<'static>;
}

impl KeyedDataMarker for HelloWorldV1Marker {
    const KEY: DataKey = crate::data_key!("core/helloworld@1");
}

/// A data provider returning Hello World strings in different languages.
///
/// Mostly useful for testing.
///
/// # Examples
///
/// ```
/// use icu_locid::locale;
/// use icu_provider::hello_world::*;
/// use icu_provider::prelude::*;
///
/// let german_hello_world: DataPayload<HelloWorldV1Marker> = HelloWorldProvider
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
pub struct HelloWorldProvider;

impl HelloWorldProvider {
    // Data from https://en.wiktionary.org/wiki/Hello_World#Translations
    // Keep this sorted!
    const DATA: &'static [(&'static str, &'static str)] = &[
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
    ];

    /// Converts this provider into one that serves JSON blobs of the same data.
    pub fn into_json_provider(self) -> HelloWorldJsonProvider {
        HelloWorldJsonProvider(self)
    }
}

impl DataProvider<HelloWorldV1Marker> for HelloWorldProvider {
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<HelloWorldV1Marker>, DataError> {
        #[allow(clippy::indexing_slicing)] // binary_search
        let data = Self::DATA
            .binary_search_by(|(k, _)| req.options.strict_cmp(k.as_bytes()).reverse())
            .map(|i| Self::DATA[i].1)
            .map(|s| HelloWorldV1 {
                message: Cow::Borrowed(s),
            })
            .map_err(|_| DataErrorKind::MissingLocale.with_req(HelloWorldV1Marker::KEY, req))?;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(data)),
        })
    }
}

impl DataPayload<HelloWorldV1Marker> {
    /// Make a [`DataPayload`]`<`[`HelloWorldV1Marker`]`>` from a static string slice.
    pub fn from_static_str(s: &'static str) -> DataPayload<HelloWorldV1Marker> {
        DataPayload::from_owned(HelloWorldV1 {
            message: Cow::Borrowed(s),
        })
    }
}

impl_dynamic_data_provider!(HelloWorldProvider, [HelloWorldV1Marker,], AnyMarker);

#[cfg(feature = "datagen")]
make_exportable_provider!(HelloWorldProvider, [HelloWorldV1Marker,]);

pub struct HelloWorldJsonProvider(HelloWorldProvider);

impl BufferProvider for HelloWorldJsonProvider {
    fn load_buffer(
        &self,
        key: DataKey,
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
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_rc_buffer(buffer.as_bytes().into())),
        })
    }
}

#[cfg(feature = "datagen")]
impl IterableDataProvider<HelloWorldV1Marker> for HelloWorldProvider {
    fn supported_options(&self) -> Result<Vec<DataOptions>, DataError> {
        #[allow(clippy::unwrap_used)] // datagen
        Ok(Self::DATA
            .iter()
            .map(|(s, _)| s.parse::<icu_locid::LanguageIdentifier>().unwrap())
            .map(DataOptions::from)
            .collect())
    }
}

#[test]
fn test_iter() {
    use icu_locid::locale;
    let supported_langids: Vec<DataOptions> = HelloWorldProvider.supported_options().unwrap();

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
