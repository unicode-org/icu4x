# Hooking up a data provider

`ResourceProvider` is a general mechanism for loading data required for ICU4X components to operate from a source.

At the moment, `ResourceProvider` is only synchronous, but the model of plugging it in is intended to extend to asynchronous `ResourceProviders` later.

## Data

The first step is to ensure that the provider has a structures to represent the data which will be collected. The structures live in a [`provider`] module in your crate and should represent the data efficiently (rather than 1-1 match to CLDR data model).

## Types of providers

Any component that needs to use `ResourceProvider` should only depend on `icu_provider` crate and use the `ResourceProvider` trait. The specific implementations such as `icu_provider_blob::BlobDataProvider` will be provided by the downstream consumer of the component.

## Hooking up data provider

Each component should use `ResourceProvider` only to construct the instance of each main struct that requires data. It means that all heavy data pulling should happen in the constructor, which, in result, must be fallible. Currently, since `ResourceProvider` is synchronous, the constructor may be synchronous as well, but in the future we expect to have both synchronous and asynchronous data providers and constructors.

## Example

```rust
pub struct AdditiveIdentity(char);

impl AdditiveIdentity {
    pub fn try_new<L: Into<Locale>, P: ResourceProvider<DecimalSymbolsV1Marker>>(
        locale: L,
        provider: &D,
    ) -> Result<Self, MyError> {
        let response = data_provider.load_resource(&DataRequest {
            options: locale.into().into(),
            metadata: Default::default(),
        })?.take_payload()?;

        let decimal_data: &DecimalSymbolsV1 = response.get();

        Ok(Self(decimal_data.digits[0]))
    }
}
```
