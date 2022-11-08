// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg(feature = "experimental")]

use std::borrow::Cow;

use crate::transform::cldr::cldr_serde;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use icu_relativetime::provider::*;
use zerovec::ZeroMap;

macro_rules! expand_key(
    ($key: literal, $datakey: expr) => {
        {
            if $datakey == NarrowRelativeTimeV1Marker::KEY {
                concat!($key, "-narrow")
            }
            else if $datakey == ShortRelativeTimeV1Marker::KEY {
                concat!($key, "-short")
            }
            else {
                $key
            }
        }
    }
);

macro_rules! get_entries(
    ($hm: expr, $datakey: expr, $($key: literal),+ $(,)?) => {
        [
            $(
                $hm.0.get(expand_key!($key, $datakey))
                    .ok_or(DataError::custom(concat!("key not found in relative time data ", $key)))
                    .map(From::from)?,
            )+
        ]
    }
);

macro_rules! make_data_provider {
    ($($marker: ident),+ $(,)?) => {
        $(
        impl DataProvider<$marker> for crate::DatagenProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                let langid = req.locale.get_langid();
                let resource: &cldr_serde::relativetime::Resource = self
                    .source
                    .cldr()?
                    .relativetime()
                    .read_and_parse(&langid, "dateFields.json")?;
                let fields = &resource
                    .main
                    .0
                    .get(&langid)
                    .ok_or(DataErrorKind::MissingLocale.into_error())?
                    .dates
                    .fields;

                let data = get_entries!(
                    fields,
                    $marker::KEY,
                    "second",
                    "minute",
                    "hour",
                    "day",
                    "week",
                    "month",
                    "quarter",
                    "year"
                );

                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: Some(DataPayload::from_owned(RelativeTimePatternsV1::new(data))),
                })
            }
        }

        impl IterableDataProvider<$marker> for crate::DatagenProvider {
            fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
                Ok(self
                    .source
                    .cldr()?
                    .relativetime()
                    .list_langs()?
                    .map(DataLocale::from)
                    .collect())
            }
        }
        )+
    };
}

impl From<&cldr_serde::relativetime::Field> for RelativeTimePattern<'_> {
    fn from(field: &cldr_serde::relativetime::Field) -> Self {
        let mut relatives = ZeroMap::new();
        for relative in &field.relatives {
            relatives.insert(&relative.count, relative.pattern.as_ref());
        }
        Self {
            display_name: field.display_name.as_ref().map(|s| Cow::Owned(s.clone())),
            relatives,
        }
    }
}

make_data_provider!(StandardRelativeTimeV1Marker);
make_data_provider!(NarrowRelativeTimeV1Marker);
make_data_provider!(ShortRelativeTimeV1Marker);

#[cfg(test)]
mod tests {
    use super::*;
    use icu_locid::locale;

    #[test]
    fn test_relativetime() {
        let provider = crate::DatagenProvider::for_test();
        let data: DataPayload<NarrowRelativeTimeV1Marker> = provider
            .load(DataRequest {
                locale: &locale!("en").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();
        assert_eq!(data.get().0[0].relatives.get(&0).unwrap(), "now");
    }
}
