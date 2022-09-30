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
