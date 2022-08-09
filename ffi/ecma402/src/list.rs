// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::fmt::{self, Display, Write};
use ecma402_traits::listformat::{
    options::{Style, Type},
    Options,
};
use ecma402_traits::Locale;
use writeable::Writeable;

pub struct ListFormat(icu::list::ListFormatter);

impl ecma402_traits::listformat::Format for ListFormat {
    type Error = icu_provider::DataError;

    fn try_new<L>(l: L, opts: Options) -> Result<Self, Self::Error>
    where
        L: Locale,
        Self: Sized,
    {
        Self::try_new_with_provider(l, opts, &crate::BakedDataProvider)
    }

    fn format<I, L, W>(&self, list: L, writer: &mut W) -> fmt::Result
    where
        I: Display,
        L: IntoIterator<Item = I>,
        W: Write,
    {
        struct WriteableWrap<J: Display>(J);

        impl<J: Display> Writeable for WriteableWrap<J> {
            fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
                write!(sink, "{}", self.0)
            }
        }

        let values = list.into_iter().map(WriteableWrap).collect::<Vec<_>>();
        self.0.format(values.iter()).write_to(writer)
    }
}

impl ListFormat {
    /// Creates a new [`ListFormat`], using the specified data provider.
    pub fn try_new_with_provider<P>(
        locale: impl Locale,
        opts: Options,
        provider: &P,
    ) -> Result<Self, icu_provider::DataError>
    where
        P: icu_provider::DataProvider<icu::list::provider::AndListV1Marker>
            + icu_provider::DataProvider<icu::list::provider::OrListV1Marker>,
    {
        let locale = crate::DataLocale::from_ecma_locale(locale);

        let style = match opts.style {
            Style::Long => icu::list::ListStyle::Wide,
            Style::Narrow => icu::list::ListStyle::Narrow,
            Style::Short => icu::list::ListStyle::Short,
        };

        Ok(Self(match opts.in_type {
            Type::Conjunction => icu::list::ListFormatter::try_new_and(&locale, provider, style),
            Type::Disjunction => icu::list::ListFormatter::try_new_or(&locale, provider, style),
        }?))
    }
}

#[test]
fn test() {
    let mut buf = String::new();

    ListFormat::try_new(
        crate::Locale::FromLangid(icu::locid::langid!("es")),
        Options {
            in_type: Type::Conjunction,
            style: Style::Long,
        },
    )
    .unwrap()
    .format(vec!["Mallorca", "Ibiza"], &mut buf)
    .unwrap();

    assert_eq!(buf, "Mallorca e Ibiza");
}
