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
use icu_provider::{DataPayload, DataProvider, DataRequest, DataError};
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

## Caching Data Provider

ICU4X has no internal caches because there is no one-size-fits-all solution. It is easy for clients to implement their own cache for ICU4X, and although this is not generally required or recommended, it may be beneficial when latency is of utmost importance and, for example, a less-efficient data provider such as JSON is being used.

The following example illustrates an LRU cache on top of a BufferProvider that saves deserialized data payloads as type-erased objects and then checks for a cache hit before calling the inner provider.

```rust
use icu_provider::hello_world::HelloWorldFormatter;
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
    M: KeyedDataMarker + 'static,
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
let provider = icu_testdata::buffer();
let lru_capacity = 100usize.try_into().unwrap();
let provider = LruDataCache {
    cache: Mutex::new(LruCache::new(lru_capacity)),
    provider: provider.as_deserializing(),
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
