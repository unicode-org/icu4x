// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider returning multilingual "Hello World" strings for testing.

#![allow(clippy::exhaustive_structs)] // data struct module

use crate as icu_provider;
use crate::fallback::LocaleFallbackConfig;

use crate::prelude::*;
use alloc::borrow::Cow;
use alloc::string::String;
use core::fmt::Debug;
#[cfg(feature = "datagen")]
use std::collections::HashSet;
use writeable::Writeable;
use yoke::*;
use zerofrom::*;

/// A struct containing "Hello World" in the requested language.
#[derive(Debug, PartialEq, Clone, Yokeable, ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    any(feature = "deserialize_json", feature = "datagen"),
    derive(serde::Serialize)
)]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_provider::hello_world))]
pub struct HelloWorldV1<'data> {
    /// The translation of "Hello World".
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
#[derive(Debug)]
pub struct HelloWorldV1Marker;

impl DynDataMarker for HelloWorldV1Marker {
    type Yokeable = HelloWorldV1<'static>;
}

impl DataMarker for HelloWorldV1Marker {
    const INFO: icu_provider::DataMarkerInfo = DataMarkerInfo {
        path: icu_provider::data_marker_path!("core/helloworld@1"),
        is_singleton: false,
        fallback_config: LocaleFallbackConfig::const_default(),
    };
}

/// A data provider returning Hello World strings in different languages.
///
/// Mostly useful for testing.
///
/// # Examples
///
/// ```
/// use icu_locale_core::langid;
/// use icu_provider::hello_world::*;
/// use icu_provider::prelude::*;
///
/// let german_hello_world: DataPayload<HelloWorldV1Marker> =
///     HelloWorldProvider
///         .load(DataRequest {
///             locale: &langid!("de").into(),
///             ..Default::default()
///         })
///         .expect("Loading should succeed")
///         .take_payload()
///         .expect("Data should be present");
///
/// assert_eq!("Hallo Welt", german_hello_world.get().message);
/// ```
///
/// Load the reverse string using an auxiliary key:
///
/// ```
/// use icu_provider::hello_world::*;
/// use icu_provider::prelude::*;
///
/// let reverse_hello_world: DataPayload<HelloWorldV1Marker> =
///     HelloWorldProvider
///         .load(DataRequest {
///             locale: &"en".parse().unwrap(),
///             marker_attributes: &"reverse".parse().unwrap(),
///             ..Default::default()
///         })
///         .expect("Loading should succeed")
///         .take_payload()
///         .expect("Data should be present");
///
/// assert_eq!("Olleh Dlrow", reverse_hello_world.get().message);
/// ```
#[derive(Debug, PartialEq, Default)]
pub struct HelloWorldProvider;

impl HelloWorldProvider {
    // Data from https://en.wiktionary.org/wiki/Hello_World#Translations
    // Keep this sorted!
    const DATA: &'static [(&'static str, &'static str, &'static str)] = &[
        ("bn", "", "ওহে বিশ্ব"),
        ("cs", "", "Ahoj světe"),
        ("de", "", "Hallo Welt"),
        ("de-AT", "", "Servus Welt"),
        ("el", "", "Καλημέρα κόσμε"),
        ("en", "", "Hello World"),
        ("en-001", "", "Hello from 🗺️"),            // WORLD
        ("en-002", "", "Hello from 🌍"),           // AFRICA
        ("en-019", "", "Hello from 🌎"),           // AMERICAS
        ("en-142", "", "Hello from 🌏"),           // ASIA
        ("en-GB", "", "Hello from 🇬🇧"),            // GREAT BRITAIN
        ("en-GB-u-sd-gbeng", "", "Hello from 🏴󠁧󠁢󠁥󠁮󠁧󠁿"), // ENGLAND
        ("en", "reverse", "Olleh Dlrow"),
        ("eo", "", "Saluton, Mondo"),
        ("fa", "", "سلام دنیا‎"),
        ("fi", "", "hei maailma"),
        ("is", "", "Halló, heimur"),
        ("ja", "", "こんにちは世界"),
        ("ja", "reverse", "界世はちにんこ"),
        ("la", "", "Ave, munde"),
        ("pt", "", "Olá, mundo"),
        ("ro", "", "Salut, lume"),
        ("ru", "", "Привет, мир"),
        ("sr", "", "Поздрав свете"),
        ("sr-Latn", "", "Pozdrav svete"),
        ("vi", "", "Xin chào thế giới"),
        ("zh", "", "你好世界"),
    ];

    /// Converts this provider into a [`BufferProvider`] that uses JSON serialization.
    #[cfg(feature = "deserialize_json")]
    pub fn into_json_provider(self) -> HelloWorldJsonProvider {
        HelloWorldJsonProvider
    }
}

impl DataProvider<HelloWorldV1Marker> for HelloWorldProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<HelloWorldV1Marker>, DataError> {
        #[allow(clippy::indexing_slicing)] // binary_search
        let data = Self::DATA
            .iter()
            .find(|(l, a, _)| {
                req.locale.strict_cmp(l.as_bytes()).is_eq() && **a == **req.marker_attributes
            })
            .map(|(_, _, v)| v)
            .ok_or_else(|| DataErrorKind::MissingLocale.with_req(HelloWorldV1Marker::INFO, req))?;
        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_static_str(data)),
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

// AnyProvider support.
#[cfg(not(feature = "datagen"))]
icu_provider::impl_dynamic_data_provider!(HelloWorldProvider, [HelloWorldV1Marker,], AnyMarker);

#[cfg(feature = "deserialize_json")]
/// A data provider returning Hello World strings in different languages as JSON blobs.
///
/// Mostly useful for testing.
///
/// # Examples
///
/// ```
/// use icu_locale_core::langid;
/// use icu_provider::hello_world::*;
/// use icu_provider::prelude::*;
///
/// let german_hello_world = HelloWorldProvider
///     .into_json_provider()
///     .load_data(HelloWorldV1Marker::INFO, DataRequest {
///         locale: &langid!("de").into(),
///         ..Default::default()
///     })
///     .expect("Loading should succeed")
///     .take_payload()
///     .expect("Data should be present");
///
/// assert_eq!(german_hello_world.get(), br#"{"message":"Hallo Welt"}"#);
#[derive(Debug)]
pub struct HelloWorldJsonProvider;

#[cfg(feature = "deserialize_json")]
impl DynamicDataProvider<BufferMarker> for HelloWorldJsonProvider {
    fn load_data(
        &self,
        marker: DataMarkerInfo,
        req: DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        marker.match_marker(HelloWorldV1Marker::INFO)?;
        let result = HelloWorldProvider.load(req)?;
        let (mut metadata, old_payload) =
            DataResponse::<HelloWorldV1Marker>::take_metadata_and_payload(result)?;
        metadata.buffer_format = Some(icu_provider::buf::BufferFormat::Json);
        #[allow(clippy::unwrap_used)] // HelloWorldV1::serialize is infallible
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned_buffer(
                serde_json::to_string(old_payload.get())
                    .unwrap()
                    .into_bytes()
                    .into_boxed_slice(),
            )),
        })
    }
}

#[cfg(feature = "datagen")]
impl icu_provider::datagen::IterableDataProvider<HelloWorldV1Marker> for HelloWorldProvider {
    fn supported_requests(&self) -> Result<HashSet<(DataLocale, DataMarkerAttributes)>, DataError> {
        #[allow(clippy::unwrap_used)] // datagen
        Ok(Self::DATA
            .iter()
            .map(|(l, a, _)| (l.parse().unwrap(), a.parse().unwrap()))
            .collect())
    }
}

#[cfg(feature = "datagen")]
icu_provider::make_exportable_provider!(HelloWorldProvider, [HelloWorldV1Marker,]);

/// A type that formats localized "hello world" strings.
///
/// This type is intended to take the shape of a typical ICU4X formatter API.
///
/// # Examples
///
/// ```
/// use icu_locale_core::locale;
/// use icu_provider::hello_world::{HelloWorldFormatter, HelloWorldProvider};
/// use writeable::assert_writeable_eq;
///
/// let fmt = HelloWorldFormatter::try_new_unstable(
///     &HelloWorldProvider,
///     &locale!("eo").into(),
/// )
/// .expect("locale exists");
///
/// assert_writeable_eq!(fmt.format(), "Saluton, Mondo");
/// ```
#[derive(Debug)]
pub struct HelloWorldFormatter {
    data: DataPayload<HelloWorldV1Marker>,
}

/// A formatted hello world message. Implements [`Writeable`].
///
/// For an example, see [`HelloWorldFormatter`].
#[derive(Debug)]
pub struct FormattedHelloWorld<'l> {
    data: &'l HelloWorldV1<'l>,
}

impl HelloWorldFormatter {
    /// Creates a new [`HelloWorldFormatter`] for the specified locale.
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    pub fn try_new(locale: &DataLocale) -> Result<Self, DataError> {
        Self::try_new_unstable(&HelloWorldProvider, locale)
    }

    icu_provider::gen_any_buffer_data_constructors!(locale: include, options: skip, error: DataError,
        #[cfg(skip)]
        functions: [
            try_new,
            try_new_with_any_provider,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self,
    ]);

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<P>(provider: &P, locale: &DataLocale) -> Result<Self, DataError>
    where
        P: DataProvider<HelloWorldV1Marker>,
    {
        let data = provider
            .load(DataRequest {
                locale,
                ..Default::default()
            })?
            .take_payload()?;
        Ok(Self { data })
    }

    /// Formats a hello world message, returning a [`FormattedHelloWorld`].
    #[allow(clippy::needless_lifetimes)] // documentary example
    pub fn format<'l>(&'l self) -> FormattedHelloWorld<'l> {
        FormattedHelloWorld {
            data: self.data.get(),
        }
    }

    /// Formats a hello world message, returning a [`String`].
    pub fn format_to_string(&self) -> String {
        self.format().write_to_string().into_owned()
    }
}

impl<'l> Writeable for FormattedHelloWorld<'l> {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        self.data.message.write_to(sink)
    }

    fn write_to_string(&self) -> Cow<str> {
        self.data.message.clone()
    }

    fn writeable_length_hint(&self) -> writeable::LengthHint {
        self.data.message.writeable_length_hint()
    }
}

writeable::impl_display_with_writeable!(FormattedHelloWorld<'_>);

#[cfg(feature = "datagen")]
#[test]
fn test_iter() {
    use crate::datagen::IterableDataProvider;
    use icu_locale_core::locale;

    assert_eq!(
        HelloWorldProvider.supported_requests().unwrap(),
        HashSet::from_iter([
            (locale!("bn").into(), Default::default()),
            (locale!("cs").into(), Default::default()),
            (locale!("de").into(), Default::default()),
            (locale!("de-AT").into(), Default::default()),
            (locale!("el").into(), Default::default()),
            (locale!("en").into(), Default::default()),
            (locale!("en-001").into(), Default::default()),
            (locale!("en-002").into(), Default::default()),
            (locale!("en-019").into(), Default::default()),
            (locale!("en-142").into(), Default::default()),
            (locale!("en-GB").into(), Default::default()),
            (locale!("en-GB-u-sd-gbeng").into(), Default::default()),
            (locale!("en").into(), "reverse".parse().unwrap()),
            (locale!("eo").into(), Default::default()),
            (locale!("fa").into(), Default::default()),
            (locale!("fi").into(), Default::default()),
            (locale!("is").into(), Default::default()),
            (locale!("ja").into(), Default::default()),
            (locale!("ja").into(), "reverse".parse().unwrap()),
            (locale!("la").into(), Default::default()),
            (locale!("pt").into(), Default::default()),
            (locale!("ro").into(), Default::default()),
            (locale!("ru").into(), Default::default()),
            (locale!("sr").into(), Default::default()),
            (locale!("sr-Latn").into(), Default::default()),
            (locale!("vi").into(), Default::default()),
            (locale!("zh").into(), Default::default()),
        ])
    );
}
