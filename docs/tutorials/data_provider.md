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

It is not generally required to implement a cache, especially if using the BakedDataProvider. However, if you wish to implement a cache in your data pipeline, you can do so like this:

```rust
use icu_provider::hello_world::HelloWorldFormatter;
use icu_provider::prelude::*;
use icu::locid::locale;
use lru::LruCache;
use std::convert::TryInto;
use std::sync::Mutex;
use yoke::trait_hack::YokeTraitHack;
use yoke::Yokeable;
use zerofrom::ZeroFrom;

pub struct MyLruDataCache<P> {
    cache: Mutex<LruCache<(DataKey, DataLocale), AnyResponse>>,
    provider: P,
}

impl<M, P> DataProvider<M> for MyLruDataCache<P>
where
    M: KeyedDataMarker + 'static,
    M::Yokeable: ZeroFrom<'static, M::Yokeable>,
    M::Yokeable: icu_provider::MaybeSendSync,
    for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: Clone,
    P: DataProvider<M>,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        let cache_key = (M::KEY, req.locale.clone());
        {
            let mut cache = self.cache.lock().unwrap();
            if let Some(any_resp) = cache.get(&cache_key) {
                return any_resp.clone_downcast();
            }
        }
        let computed_resp: DataResponse<M> = self.provider.load(req)?;
        let computed_any_resp: AnyResponse = computed_resp.wrap_into_any_response();
        {
            let mut cache = self.cache.lock().unwrap();
            let any_resp = cache.get_or_insert(cache_key, || computed_any_resp);
            return any_resp.clone_downcast();
        }
    }
}

// Usage example:
let provider = icu_testdata::buffer();
let lru_capacity = 100usize.try_into().unwrap();
let provider = MyLruDataCache {
    cache: Mutex::new(LruCache::new(lru_capacity)),
    provider: provider.as_deserializing(),
};

// The cache starts empty:
assert_eq!(provider.cache.lock().unwrap().len(), 0);

assert_eq!(
    "こんにちは世界",
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
