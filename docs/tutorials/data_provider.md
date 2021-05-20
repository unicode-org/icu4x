# Hooking up a DataProvider

`DataProvider` is a general mechanism for loading data required for ICU4X components to operate from a source.

At the moment, `DataProvider` is only synchronous, but the model of plugging it in is intended to extend to asynchronous `DataProviders` later.

## Data

The first step is to ensure that the provider has a structures to represent the data which will be collected. The structures live in the [`provider/core/src/structs`](../../provider/core/src/structs) directory and should represent the data efficiently (rather than 1-1 match to CLDR data model).

## Types of providers

Any component that needs to use `DataProvider` should only depend on `icu_provider` crate and use the `DataProvider` trait. The specific implementations such as `icu_provider_cldr` and `icu_provider_fs` will be used by the downstream consumer of the component to provide the specific implementation of the provider later.

## Hooking up DataProvider

Each component should use `DataProvider` only to construct the instance of each main struct that requires data. It means that all heavy data pulling should happen in the constructor, which, in result, must be fallible. Currently, since `DataProvider` is synchronous, the constructor may be synchronous as well, but in the future we expect to have both synchronous and asynchronous data providers and constructors.

## Example

```rust
pub struct MyComponent {
    langid: LanguageIdentifier,
    zero_digit: char,
}

impl MyComponent {
    pub fn try_new<'d, D: DataProvider<'d>>(
        langid: LanguageIdentifier,
        data_provider: &D,
    ) -> Result<Self, MyError> {
        let response = data_provider.load(&DataRequest {
            resc_key: icu_resc_key!(decimals: symbols@1),
            resc_options: ResourceOptions {
                variant: None,
                langid: langid.clone(),
            },
        })?;

        let decimal_data: &structs::decimal::SymbolsV1 = response.borrow_payload()?;

        Ok(Self {
            langid,
            zero: decimal_data.zero_digit
        })
    }
}
```
