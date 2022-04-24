#[macro_export]
macro_rules! preferences {
    ($name:ident, $resolved_name:ident, {$($key:ident => $pref:ty, $resolved:ty, $ue:expr),*}) => (
        #[derive(Default)]
        pub struct $name {
            $(
                $key: $pref,
            )*
        }

        pub struct $resolved_name {
            $(
                $key: $resolved,
            )*
        }

        impl TryFrom<(&$name, &Locale, &$resolved_name)> for $resolved_name {
            type Error = ();

            fn try_from(input: (&$name, &Locale, &$resolved_name)) -> Result<Self, Self::Error> {
                let (prefs, locale, defaults) = input;
                let keywords = &locale.extensions.unicode.keywords;

                $(
                    let mut $key = prefs.$key;
                    if $key.is_none() {
                        $key = $ue.and_then(
                            |ue_key| keywords.get(&ue_key).map(TryInto::try_into)
                        ).transpose()?;
                    }
                )*
                Ok($resolved_name {
                    $(
                        $key: $key.unwrap_or(defaults.$key),
                    )*
                })
            }
        }
    )
}
