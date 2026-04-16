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

#[derive(Debug)]
pub struct ListFormat(icu::list::ListFormatter);

impl ecma402_traits::listformat::Format for ListFormat {
    type Error = icu_provider::DataError;

    fn try_new<L>(locale: L, opts: Options) -> Result<Self, Self::Error>
    where
        L: Locale,
        Self: Sized,
    {
        #[expect(clippy::unwrap_used)] // ecma402_traits::Locale::to_string is a valid locale
        let locale = icu::locale::Locale::try_from_str(&locale.to_string()).unwrap();

        let prefs = icu::list::ListFormatterPreferences::from(&locale);

        let length = match opts.style {
            Style::Long => icu::list::options::ListLength::Wide,
            Style::Narrow => icu::list::options::ListLength::Narrow,
            Style::Short => icu::list::options::ListLength::Short,
        };
        let options = icu::list::options::ListFormatterOptions::default().with_length(length);

        Ok(Self(match opts.in_type {
            Type::Conjunction => icu::list::ListFormatter::try_new_and(prefs, options),
            Type::Disjunction => icu::list::ListFormatter::try_new_or(prefs, options),
        }?))
    }

    fn format<I, L, W>(&self, list: L, writer: &mut W) -> fmt::Result
    where
        I: Display,
        L: IntoIterator<Item = I>,
        W: Write,
    {
        struct WriteableWrap<J: Display>(J);

        impl<J: Display> Writeable for WriteableWrap<J> {
            fn write_to<W: Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
                write!(sink, "{}", self.0)
            }
        }

        let values = list.into_iter().map(WriteableWrap).collect::<Vec<_>>();
        self.0.format(values.iter()).write_to(writer)
    }
}

#[test]
fn test() {
    use ecma402_traits::listformat::Format;

    let mut buf = String::new();

    ListFormat::try_new(
        crate::testing::TestLocale("es"),
        Options {
            in_type: Type::Conjunction,
            style: Style::Long,
        },
    )
    .unwrap()
    .format(["Mallorca", "Ibiza"], &mut buf)
    .unwrap();

    assert_eq!(buf, "Mallorca e Ibiza");
}
