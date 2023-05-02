# Hooking up a data provider

`DataProvider` is a general mechanism for loading data required for ICU4X components to operate from a source.

At the moment, `DataProvider` is only synchronous, but the model of plugging it in is intended to extend to asynchronous `DataProviders` later.

## Data

The first step is to ensure that the provider has a structures to represent the data which will be collected. The structures live in a `provider` module in your crate and should represent the data efficiently (rather than 1-1 match to CLDR data model).

## Types of providers

Any component that needs to use `DataProvider` should only depend on `icu_provider` crate and use the `DataProvider` trait. The specific implementations such as `icu_provider_blob::BlobDataProvider` will be provided by the downstream consumer of the component.

## Hooking up data provider

Each component should use `DataProvider` only to construct the instance of each main struct that requires data. It means that all heavy data pulling should happen in the constructor, which, in result, must be fallible. Currently, since `DataProvider` is synchronous, the constructor may be synchronous as well, but in the future we expect to have both synchronous and asynchronous data providers and constructors.

## Example

```rust
use displaydoc::Display;
use icu_provider::prelude::*;
use icu::locid::Locale;
use icu::decimal::provider::{DecimalSymbolsV1Marker, DecimalSymbolsV1};

#[derive(Display, Debug, Copy, Clone)]
pub enum MyError {
     /// Some custom error
     SomeError,

     /// An error originating inside of the data provider.
     #[displaydoc("{0}")]
     DataProvider(DataError),
}

#[cfg(feature = "std")]
impl std::error::Error for MyError {}

impl From<DataError> for MyError {
     fn from(e: DataError) -> Self {
         MyError::DataProvider(e)
     }
}

pub struct AdditiveIdentity(char);

impl AdditiveIdentity {
    pub fn try_new<L: Into<Locale>, P: DataProvider<DecimalSymbolsV1Marker>>(
        locale: L,
        data_provider: &P,
    ) -> Result<Self, MyError> {
        let response = data_provider.load(DataRequest {
            locale: &locale.into().into(),
            metadata: Default::default(),
        })?.take_payload()?;

        let decimal_data: &DecimalSymbolsV1 = response.get();

        Ok(Self(decimal_data.digits[0]))
    }
}
```

## Loading Additional Data at Runtime

A key feature of ICU4X is the ability to download data dynamically, allowing clients to load additional locales at runtime.

Dynamic data loading can currently be performed in user code. A future core library API may provide this functionality; please submit feedback in [#2985](https://github.com/unicode-org/icu4x/issues/2985).

The following example loads additional locales bucketed by language. This means that different script and regional variants of the same language are assumed to be in the same dynamically loaded data file. However, clients should choose a dynamic loading strategy that works best for them.

```rust
use icu_provider_adapters::either::EitherProvider;
use icu_provider_adapters::fallback::LocaleFallbackProvider;
use icu_provider_adapters::fork::ForkByKeyProvider;
use icu_provider_adapters::fork::MultiForkByErrorProvider;
use icu_provider_adapters::fork::predicates::MissingLocalePredicate;
use icu_provider_blob::BlobDataProvider;
use icu_provider_fs::FsDataProvider;
use icu_provider::DataLocale;
use icu_provider::hello_world::HelloWorldFormatter;
use icu::locid::locale;
use icu::locid::subtags::Language;
use std::path::Path;
use writeable::Writeable;

// Create the empty MultiForkByErrorProvider:
let mut provider = MultiForkByErrorProvider::new_with_predicate(
    vec![],
    MissingLocalePredicate
);

// Helper function to add data into the growable provider on demand:
let mut get_hello_world_formatter = |loc: &DataLocale| {
    // Try to create the formatter a first time with data that has already been loaded.
    if let Ok(formatter) = HelloWorldFormatter::try_new_with_buffer_provider(&provider, loc) {
        return formatter;
    }

    // We failed to create the formatter. Load more data for the language.
    // Note: This assumes data is split by language subtag, which may or may not be the best
    // strategy for all use cases.
    let path_buf = 
        Path::new("../../provider/adapters/tests/data/langtest")
        .join(loc.language().as_str());
    let lang_provider = match FsDataProvider::try_new(&path_buf) {
        Ok(p) => p,
        Err(e) => panic!("Language not available? {:?}", e)
    };
    println!("Successfully loaded: {:?}", loc);

    // Add the data to the growable provider and try creating the formatter a second time.
    provider.push(lang_provider);
    HelloWorldFormatter::try_new_with_buffer_provider(&provider, loc)
        .expect("Language data should now be available")
};

// Test that it works:
assert_eq!(
    get_hello_world_formatter(&locale!("de").into()).format().write_to_string(),
    "Hallo Welt"
);
assert_eq!(
    get_hello_world_formatter(&locale!("ro").into()).format().write_to_string(),
    "Salut, lume"
);
```

## Caching Data Provider

ICU4X has no internal caches because there is no one-size-fits-all solution. It is easy for clients to implement their own cache for ICU4X, and although this is not generally required or recommended, it may be beneficial when latency is of utmost importance and, for example, a less-efficient data provider such as JSON is being used.

The following example illustrates an LRU cache on top of a data provider. A practical application would be a BufferProvider that saves deserialized data payloads as type-erased objects and then checks for a cache hit before calling the inner provider.

```rust
use icu_provider::hello_world::{HelloWorldFormatter, HelloWorldProvider};
use icu_provider::prelude::*;
use icu::locid::locale;
use lru::LruCache;
use std::borrow::{Borrow, Cow};
use std::convert::TryInto;
use std::sync::Mutex;
use yoke::trait_hack::YokeTraitHack;
use yoke::Yokeable;
use zerofrom::ZeroFrom;

/// A data provider that caches response payloads in an LRU cache.
pub struct LruDataCache<P> {
    cache: Mutex<LruCache<CacheKeyWrap, AnyResponse>>,
    provider: P,
}

/// Key for the cache: DataKey and DataLocale. The DataLocale is in a Cow
/// so that it can be borrowed during lookup.
#[derive(Debug, PartialEq, Eq, Hash)]
struct CacheKey<'a>(DataKey, Cow<'a, DataLocale>);

/// Wrapper over a fully owned CacheKey, required for key borrowing.
#[derive(Debug, PartialEq, Eq, Hash)]
struct CacheKeyWrap(CacheKey<'static>);

// This impl enables a borrowed DataLocale to be used during cache retrieval.
impl<'a> Borrow<CacheKey<'a>> for lru::KeyRef<CacheKeyWrap> {
    fn borrow(&self) -> &CacheKey<'a> {
        &Borrow::<CacheKeyWrap>::borrow(self).0
    }
}

impl<M, P> DataProvider<M> for LruDataCache<P>
where
    M: KeyedDataMarker,
    M::Yokeable: ZeroFrom<'static, M::Yokeable>,
    M::Yokeable: icu_provider::MaybeSendSync,
    for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: Clone,
    P: DataProvider<M>,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        {
            // First lock: cache retrieval
            let mut cache = self.cache.lock().unwrap();
            let borrowed_cache_key = CacheKey(M::KEY, Cow::Borrowed(req.locale));
            if let Some(any_res) = cache.get(&borrowed_cache_key) {
                // Note: Cloning a DataPayload is usually cheap, and it is necessary in order to
                // convert the short-lived cache object into one we can return.
                return any_res.downcast_cloned();
            }
        }
        // Release the lock to invoke the inner provider
        let response = self.provider.load(req)?;
        let owned_cache_key = CacheKeyWrap(CacheKey(M::KEY, Cow::Owned(req.locale.clone())));
        // Second lock: cache storage
        self.cache.lock()
            .unwrap()
            .get_or_insert(owned_cache_key, || response.wrap_into_any_response())
            .downcast_cloned()
    }
}

// Usage example:
// While HelloWorldProvider does not need to be cached, it may be useful to cache results from
// more expensive providers, like deserializing BufferProviders or providers doing I/O.
let provider = HelloWorldProvider;
let lru_capacity = 100usize.try_into().unwrap();
let provider = LruDataCache {
    cache: Mutex::new(LruCache::new(lru_capacity)),
    provider,
};

// The cache starts empty:
assert_eq!(provider.cache.lock().unwrap().len(), 0);

assert_eq!(
    "こんにちは世界",
    // Note: It is necessary to use `try_new_unstable` with LruDataCache.
    HelloWorldFormatter::try_new_unstable(
        &provider,
        &locale!("ja").into()
    )
    .unwrap()
    .format_to_string()
);

// One item in the cache:
assert_eq!(provider.cache.lock().unwrap().len(), 1);

assert_eq!(
    "ওহে বিশ্ব",
    HelloWorldFormatter::try_new_unstable(
        &provider,
        &locale!("bn").into()
    )
    .unwrap()
    .format_to_string()
);

// Two items in the cache:
assert_eq!(provider.cache.lock().unwrap().len(), 2);

assert_eq!(
    "こんにちは世界",
    HelloWorldFormatter::try_new_unstable(
        &provider,
        &locale!("ja").into()
    )
    .unwrap()
    .format_to_string()
);

// Still only two items in the cache, since we re-requested "ja" data:
assert_eq!(provider.cache.lock().unwrap().len(), 2);
```

## Overwriting Specific Data Items

ICU4X's explicit data pipeline allows for specific data entries to be overwritten in order to customize the output or comply with policy.

The following example illustrates how to overwrite the decimal separators for a region.

```rust
use icu::decimal::FixedDecimalFormatter;
use icu_provider::prelude::*;
use icu::locid::locale;
use icu::locid::subtags_region as region;
use std::borrow::Cow;
use tinystr::tinystr;

pub struct CustomDecimalSymbolsProvider<P>(P);

impl<P> AnyProvider for CustomDecimalSymbolsProvider<P>
where
    P: AnyProvider
{
    fn load_any(&self, key: DataKey, req: DataRequest) -> Result<AnyResponse, DataError> {
        use icu::decimal::provider::DecimalSymbolsV1Marker;
        let mut any_res = self.0.load_any(key, req)?;
        if key == DecimalSymbolsV1Marker::KEY && req.locale.region() == Some(region!("CH")) {
            let mut res: DataResponse<DecimalSymbolsV1Marker> = any_res.downcast()?;
            if let Some(payload) = &mut res.payload.as_mut() {
                payload.with_mut(|data| {
                    // Change the grouping separator for all Swiss locales to '🐮'
                    data.grouping_separator = Cow::Borrowed("🐮");
                });
            }
            any_res = res.wrap_into_any_response();
        }
        Ok(any_res)
    }
}

let provider = CustomDecimalSymbolsProvider(icu_testdata::any());
let formatter = FixedDecimalFormatter::try_new_with_any_provider(
    &provider,
    &locale!("de-CH").into(),
    Default::default(),
)
.unwrap();

assert_eq!(formatter.format_to_string(&100007i64.into()), "100🐮007");
```
