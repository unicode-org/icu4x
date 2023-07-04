// @generated
/// Implement [`DataProvider<LocaleDisplayNamesV1Marker>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_displaynames_locales_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::DataProvider<icu_displaynames::provider::LocaleDisplayNamesV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu_displaynames::provider::LocaleDisplayNamesV1Marker>, icu_provider::DataError> {
                static FIL: <icu_displaynames::provider::LocaleDisplayNamesV1Marker as icu_provider::DataMarker>::Yokeable = icu_displaynames::provider::LocaleDisplayNamesV1 {
                    names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x11\0\0\0\0\0\x06\0\x0B\0\x10\0\x15\0\x1B\0 \0%\0*\0/\x005\0:\0?\0D\0I\0N\0U\0ar-001de-CHen-GBen-USes-419es-ESes-MXfa-AFfr-CHnds-NLnl-BEpt-BRpt-PTro-MDsw-CDzh-Hanszh-Hant") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x11\0\0\0\0\0\x1B\0,\0=\0O\0i\0y\0\x8C\0\x90\0\x9F\0\xA8\0\xAF\0\xC1\0\xD4\0\xDD\0\xEA\0\xFE\0Modernong Karaniwang ArabicSwiss High GermanIngles na BritishIngles na AmericanLatin American na EspanyolEuropean SpanishMexican na EspanyolDariSwiss na FrenchLow SaxonFlemishPortuges ng BrasilEuropean PortugueseMoldavianCongo SwahiliPinasimpleng ChineseTradisyonal na Chinese") })
                    },
                    short_names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x05\0en-GBen-US") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0C\0Ingles sa UKIngles sa US") })
                    },
                    long_names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x07\0zh-Hanszh-Hant") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x1D\0Pinasimpleng Mandarin ChineseTradisyonal na Mandarin Chinese") })
                    },
                    menu_names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                    },
                };
                static CCP: <icu_displaynames::provider::LocaleDisplayNamesV1Marker as icu_provider::DataMarker>::Yokeable = icu_displaynames::provider::LocaleDisplayNamesV1 {
                    names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x14\0\0\0\0\0\x06\0\x0B\0\x10\0\x15\0\x1A\0\x1F\0$\0*\0/\x004\09\0>\0D\0I\0N\0S\0X\0]\0d\0ar-001de-ATde-CHen-AUen-CAen-GBen-USes-419es-ESes-MXfr-CAfr-CHnds-NLnl-BEpt-BRpt-PTro-MDsw-CDzh-Hanszh-Hant") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x14\0\0\0\0\0.\0s\0\xAD\0\xFA\0/\x01l\x01\xA5\x01\x07\x02L\x02\x99\x02\xC6\x02\xF3\x02\x1F\x03?\x03\x8C\x03\xDD\x03\x01\x042\x04_\x04\xF0\x91\x84\x9A\xF0\x91\x84\xB1 \xF0\x91\x84\x89\xF0\x91\x84\xA7\xF0\x91\x84\x9F\xF0\x91\x84\xB4 \xF0\x91\x84\x83\xF0\x91\x84\xA2\xF0\x91\x84\xA7\xF0\x91\x84\x9D\xF0\x91\x84\xA9\xF0\x91\x84\x83\xF0\x91\x84\xA7\xF0\x91\x84\x8C\xF0\x91\x84\xB4\xF0\x91\x84\x91\xF0\x91\x84\xB3\xF0\x91\x84\xA2\xF0\x91\x84\xA8\xF0\x91\x84\xA0\xF0\x91\x84\x9A\xF0\x91\x84\xB4 \xF0\x91\x84\x8E\xF0\x91\x84\xA2\xF0\x91\x84\xB4\xF0\x91\x84\x9F\xF0\x91\x84\x9A\xF0\x91\x84\xB4\xF0\x91\x84\xA5\xF0\x91\x84\xAA\xF0\x91\x84\x83\xF0\x91\x84\xA8\xF0\x91\x84\x8C\xF0\x91\x84\xB4 \xF0\x91\x84\xA6\xF0\x91\x84\xAD \xF0\x91\x84\x8E\xF0\x91\x84\xA2\xF0\x91\x84\xB4\xF0\x91\x84\x9F\xF0\x91\x84\x9A\xF0\x91\x84\xB4\xF0\x91\x84\x83\xF0\x91\x84\xA7\xF0\x91\x84\x8C\xF0\x91\x84\xB4\xF0\x91\x84\x91\xF0\x91\x84\xB3\xF0\x91\x84\xA2\xF0\x91\x84\xAC\xF0\x91\x84\xA3\xF0\x91\x84\xA8\xF0\x91\x84\xA0\xF0\x91\x84\xA7 \xF0\x91\x84\x83\xF0\x91\x84\xA8\xF0\x91\x84\x81\xF0\x91\x84\xA2\xF0\x91\x84\xAC\xF0\x91\x84\x8E\xF0\x91\x84\xA8\xF0\x91\x84\x87\xF0\x91\x84\x9A\xF0\x91\x84\x93\xF0\x91\x84\xA9\xF0\x91\x84\xA0\xF0\x91\x84\xA7 \xF0\x91\x84\x83\xF0\x91\x84\xA8\xF0\x91\x84\x81\xF0\x91\x84\xA2\xF0\x91\x84\xAC\xF0\x91\x84\x8E\xF0\x91\x84\xA8\xF0\x91\x84\x9D\xF0\x91\x84\xB3\xF0\x91\x84\xA2\xF0\x91\x84\xA8\xF0\x91\x84\x91\xF0\x91\x84\xA8\xF0\x91\x84\x8C\xF0\x91\x84\xB4 \xF0\x91\x84\x83\xF0\x91\x84\xA8\xF0\x91\x84\x81\xF0\x91\x84\xA2\xF0\x91\x84\xAC\xF0\x91\x84\x8E\xF0\x91\x84\xA8\xF0\x91\x84\x83\xF0\x91\x84\x9F\xF0\x91\x84\xAC\xF0\x91\x84\xA2\xF0\x91\x84\xA8\xF0\x91\x84\x87\xF0\x91\x84\xA2\xF0\x91\x84\xB4 \xF0\x91\x84\x83\xF0\x91\x84\xA8\xF0\x91\x84\x81\xF0\x91\x84\xA2\xF0\x91\x84\x8E\xF0\x91\x84\xA8\xF0\x91\x84\xA3\xF0\x91\x84\xB3\xF0\x91\x84\xA0\xF0\x91\x84\x91\xF0\x91\x84\xA8\xF0\x91\x84\x9A\xF0\x91\x84\xB4 \xF0\x91\x84\x83\xF0\x91\x84\x9F\xF0\x91\x84\xAC\xF0\x91\x84\xA2\xF0\x91\x84\xA8\xF0\x91\x84\x87\xF0\x91\x84\x9A\xF0\x91\x84\xB4 \xF0\x91\x84\xA5\xF0\x91\x84\xB3\xF0\x91\x84\x9B\xF0\x91\x84\xB3\xF0\x91\x84\xA0\xF0\x91\x84\x9A\xF0\x91\x84\xA8\xF0\x91\x84\x8C\xF0\x91\x84\xB4\xF0\x91\x84\x84\xF0\x91\x84\x85\xF0\x91\x84\xA2\xF0\x91\x84\xAE\xF0\x91\x84\x9B\xF0\x91\x84\xA9\xF0\x91\x84\xA0\xF0\x91\x84\xA7 \xF0\x91\x84\xA5\xF0\x91\x84\xB3\xF0\x91\x84\x9B\xF0\x91\x84\xB3\xF0\x91\x84\xA0\xF0\x91\x84\x9A\xF0\x91\x84\xA8\xF0\x91\x84\x8C\xF0\x91\x84\xB4\xF0\x91\x84\x9F\xF0\x91\x84\xB3\xF0\x91\x84\xA0\xF0\x91\x84\x87\xF0\x91\x84\xB4\xF0\x91\x84\xA5\xF0\x91\x84\xA8\xF0\x91\x84\x87\xF0\x91\x84\x9A\xF0\x91\x84\xB4 \xF0\x91\x84\xA5\xF0\x91\x84\xB3\xF0\x91\x84\x9B\xF0\x91\x84\xB3\xF0\x91\x84\xA0\xF0\x91\x84\x9A\xF0\x91\x84\xA8\xF0\x91\x84\x8C\xF0\x91\x84\xB4\xF0\x91\x84\x87\xF0\x91\x84\x9A\xF0\x91\x84\x93\xF0\x91\x84\xA9\xF0\x91\x84\xA0\xF0\x91\x84\xA7 \xF0\x91\x84\x9C\xF0\x91\x84\xA7\xF0\x91\x84\xA2\xF0\x91\x84\xA5\xF0\x91\x84\xA8\xF0\x91\x84\xA5\xF0\x91\x84\xAA\xF0\x91\x84\x83\xF0\x91\x84\xA8\xF0\x91\x84\x8C\xF0\x91\x84\xB4 \xF0\x91\x84\x9C\xF0\x91\x84\xA7\xF0\x91\x84\xA2\xF0\x91\x84\xA5\xF0\x91\x84\xA8\xF0\x91\x84\xA3\xF0\x91\x84\xAE\xF0\x91\x84\xA5\xF0\x91\x84\xB3\xF0\x91\x84\xA0\xF0\x91\x84\x87\xF0\x91\x84\xB4\xF0\x91\x84\xA5\xF0\x91\x84\xA7\xF0\x91\x84\x9A\xF0\x91\x84\xB4\xF0\x91\x84\x9C\xF0\x91\x84\xB3\xF0\x91\x84\xA3\xF0\x91\x84\xAC\xF0\x91\x84\x9F\xF0\x91\x84\xA8\xF0\x91\x84\x8C\xF0\x91\x84\xB4\xF0\x91\x84\x9D\xF0\x91\x84\xB3\xF0\x91\x84\xA2\xF0\x91\x84\x8E\xF0\x91\x84\xA8\xF0\x91\x84\xA3\xF0\x91\x84\xAC\xF0\x91\x84\xA2\xF0\x91\x84\xB4 \xF0\x91\x84\x9B\xF0\x91\x84\xA7\xF0\x91\x84\xA2\xF0\x91\x84\xB4\xF0\x91\x84\x96\xF0\x91\x84\xAA\xF0\x91\x84\x89\xF0\x91\x84\xA8\xF0\x91\x84\x8E\xF0\x91\x84\xB4\xF0\x91\x84\x83\xF0\x91\x84\xA8\xF0\x91\x84\x83\xF0\x91\x84\xAA\xF0\x91\x84\xA2\xF0\x91\x84\xAE\xF0\x91\x84\x9B\xF0\x91\x84\xAC\xF0\x91\x84\xA2\xF0\x91\x84\xB4 \xF0\x91\x84\x9B\xF0\x91\x84\xA7\xF0\x91\x84\xA2\xF0\x91\x84\xB4\xF0\x91\x84\x96\xF0\x91\x84\xAA\xF0\x91\x84\x89\xF0\x91\x84\xA8\xF0\x91\x84\x8E\xF0\x91\x84\xB4\xF0\x91\x84\x9F\xF0\x91\x84\xA7\xF0\x91\x84\xA3\xF0\x91\x84\xB4\xF0\x91\x84\x98\xF0\x91\x84\x9E\xF0\x91\x84\xA8\xF0\x91\x84\xA0\xF0\x91\x84\xA7\xF0\x91\x84\x87\xF0\x91\x84\xA7\xF0\x91\x84\x8B\xF0\x91\x84\xB4\xF0\x91\x84\x89\xF0\x91\x84\xAE \xF0\x91\x84\xA5\xF0\x91\x84\xB1\xF0\x91\x84\xA6\xF0\x91\x84\xA8\xF0\x91\x84\xA3\xF0\x91\x84\xA8\xF0\x91\x84\x85\xF0\x91\x84\xAA\xF0\x91\x84\x8E\xF0\x91\x84\xAA\xF0\x91\x84\x85\xF0\x91\x84\xAA\xF0\x91\x84\x8F\xF0\x91\x84\xAB \xF0\x91\x84\x8C\xF0\x91\x84\xA9\xF0\x91\x84\x9A\xF0\x91\x84\xA2\xF0\x91\x84\xA8\xF0\x91\x84\x98\xF0\x91\x84\xA8\xF0\x91\x84\xA5\xF0\x91\x84\xAA\xF0\x91\x84\x98\xF0\x91\x84\xAE\xF0\x91\x84\x9F\xF0\x91\x84\xB4 \xF0\x91\x84\x8C\xF0\x91\x84\xA9\xF0\x91\x84\x9A") })
                    },
                    short_names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x05\0en-GBen-US") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0I\0\xF0\x91\x84\x8E\xF0\x91\x84\xA7\xF0\x91\x84\x99\xF0\x91\x84\xA2\xF0\x91\x84\xAC\xF0\x91\x84\x8C\xF0\x91\x84\xB4\xF0\x91\x84\x8E\xF0\x91\x84\xAE\xF0\x91\x84\xA2\xF0\x91\x84\xB4 \xF0\x91\x84\x83\xF0\x91\x84\xA8\xF0\x91\x84\x81\xF0\x91\x84\xA2\xF0\x91\x84\xAC\xF0\x91\x84\x8E\xF0\x91\x84\xA8\xF0\x91\x84\x83\xF0\x91\x84\xA8\xF0\x91\x84\x85\xF0\x91\x84\xAA\xF0\x91\x84\x83\xF0\x91\x84\xAC\xF0\x91\x84\x8C\xF0\x91\x84\xB4 \xF0\x91\x84\x83\xF0\x91\x84\xA8\xF0\x91\x84\x81\xF0\x91\x84\xA2\xF0\x91\x84\xAC\xF0\x91\x84\x8E\xF0\x91\x84\xA8") })
                    },
                    long_names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x07\0zh-Hanszh-Hant") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0-\0\xF0\x91\x84\x85\xF0\x91\x84\xAA\xF0\x91\x84\x8E\xF0\x91\x84\xAA\xF0\x91\x84\x85\xF0\x91\x84\xAA\xF0\x91\x84\x8F\xF0\x91\x84\xAB \xF0\x91\x84\x8C\xF0\x91\x84\xA9\xF0\x91\x84\x9A\xF0\x91\x84\xA2\xF0\x91\x84\xA8\xF0\x91\x84\x98\xF0\x91\x84\xA8\xF0\x91\x84\xA5\xF0\x91\x84\xAA\xF0\x91\x84\x98\xF0\x91\x84\xAE\xF0\x91\x84\x9F\xF0\x91\x84\xB4 \xF0\x91\x84\x8C\xF0\x91\x84\xA9\xF0\x91\x84\x9A") })
                    },
                    menu_names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                    },
                };
                static AR_EG: <icu_displaynames::provider::LocaleDisplayNamesV1Marker as icu_provider::DataMarker>::Yokeable = icu_displaynames::provider::LocaleDisplayNamesV1 {
                    names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x15\0\0\0\0\0\x06\0\x0B\0\x10\0\x15\0\x1A\0\x1F\0$\0*\0/\x004\09\0>\0C\0I\0N\0S\0X\0]\0b\0i\0ar-001de-ATde-CHen-AUen-CAen-GBen-USes-419es-ESes-MXfa-AFfr-CAfr-CHnds-NLnl-BEpt-BRpt-PTro-MDsw-CDzh-Hanszh-Hant") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x15\0\0\0\0\0*\0O\0\x81\0\xAA\0\xCD\0\xF6\0\x1D\x01O\x01t\x01\x99\x01\xA7\x01\xC6\x01\xE9\x01\x08\x02\x1A\x02C\x02j\x02~\x02\x9F\x02\xBC\x02\xD8\xA7\xD9\x84\xD8\xB9\xD8\xB1\xD8\xA8\xD9\x8A\xD8\xA9 \xD8\xA7\xD9\x84\xD9\x81\xD8\xB5\xD8\xAD\xD9\x89 \xD8\xA7\xD9\x84\xD8\xAD\xD8\xAF\xD9\x8A\xD8\xAB\xD8\xA9\xD8\xA7\xD9\x84\xD8\xA3\xD9\x84\xD9\x85\xD8\xA7\xD9\x86\xD9\x8A\xD8\xA9 \xD8\xA7\xD9\x84\xD9\x86\xD9\x85\xD8\xB3\xD8\xA7\xD9\x88\xD9\x8A\xD8\xA9\xD8\xA7\xD9\x84\xD8\xA3\xD9\x84\xD9\x85\xD8\xA7\xD9\x86\xD9\x8A\xD8\xA9 \xD8\xA7\xD9\x84\xD8\xB9\xD9\x84\xD9\x8A\xD8\xA7 \xD8\xA7\xD9\x84\xD8\xB3\xD9\x88\xD9\x8A\xD8\xB3\xD8\xB1\xD9\x8A\xD8\xA9\xD8\xA7\xD9\x84\xD8\xA5\xD9\x86\xD8\xAC\xD9\x84\xD9\x8A\xD8\xB2\xD9\x8A\xD8\xA9 \xD8\xA7\xD9\x84\xD8\xA3\xD8\xB3\xD8\xAA\xD8\xB1\xD8\xA7\xD9\x84\xD9\x8A\xD8\xA9\xD8\xA7\xD9\x84\xD8\xA5\xD9\x86\xD8\xAC\xD9\x84\xD9\x8A\xD8\xB2\xD9\x8A\xD8\xA9 \xD8\xA7\xD9\x84\xD9\x83\xD9\x86\xD8\xAF\xD9\x8A\xD8\xA9\xD8\xA7\xD9\x84\xD8\xA5\xD9\x86\xD8\xAC\xD9\x84\xD9\x8A\xD8\xB2\xD9\x8A\xD8\xA9 \xD8\xA7\xD9\x84\xD8\xA8\xD8\xB1\xD9\x8A\xD8\xB7\xD8\xA7\xD9\x86\xD9\x8A\xD8\xA9\xD8\xA7\xD9\x84\xD8\xA5\xD9\x86\xD8\xAC\xD9\x84\xD9\x8A\xD8\xB2\xD9\x8A\xD8\xA9 \xD8\xA7\xD9\x84\xD8\xA3\xD9\x85\xD8\xB1\xD9\x8A\xD9\x83\xD9\x8A\xD8\xA9\xD8\xA7\xD9\x84\xD8\xA5\xD8\xB3\xD8\xA8\xD8\xA7\xD9\x86\xD9\x8A\xD8\xA9 \xD8\xA3\xD9\x85\xD8\xB1\xD9\x8A\xD9\x83\xD8\xA7 \xD8\xA7\xD9\x84\xD9\x84\xD8\xA7\xD8\xAA\xD9\x8A\xD9\x86\xD9\x8A\xD8\xA9\xD8\xA7\xD9\x84\xD8\xA5\xD8\xB3\xD8\xA8\xD8\xA7\xD9\x86\xD9\x8A\xD8\xA9 \xD8\xA7\xD9\x84\xD8\xA3\xD9\x88\xD8\xB1\xD9\x88\xD8\xA8\xD9\x8A\xD8\xA9\xD8\xA7\xD9\x84\xD8\xA5\xD8\xB3\xD8\xA8\xD8\xA7\xD9\x86\xD9\x8A\xD8\xA9 \xD8\xA7\xD9\x84\xD9\x85\xD9\x83\xD8\xB3\xD9\x8A\xD9\x83\xD9\x8A\xD8\xA9\xD8\xA7\xD9\x84\xD8\xAF\xD8\xA7\xD8\xB1\xD9\x8A\xD8\xA9\xD8\xA7\xD9\x84\xD9\x81\xD8\xB1\xD9\x86\xD8\xB3\xD9\x8A\xD8\xA9 \xD8\xA7\xD9\x84\xD9\x83\xD9\x86\xD8\xAF\xD9\x8A\xD8\xA9\xD8\xA7\xD9\x84\xD9\x81\xD8\xB1\xD9\x86\xD8\xB3\xD9\x8A\xD8\xA9 \xD8\xA7\xD9\x84\xD8\xB3\xD9\x88\xD9\x8A\xD8\xB3\xD8\xB1\xD9\x8A\xD8\xA9\xD8\xA7\xD9\x84\xD8\xB3\xD9\x83\xD8\xB3\xD9\x88\xD9\x86\xD9\x8A\xD8\xA9 \xD8\xA7\xD9\x84\xD8\xB3\xD9\x81\xD9\x84\xD9\x89\xD8\xA7\xD9\x84\xD9\x81\xD9\x84\xD9\x85\xD9\x86\xD9\x83\xD9\x8A\xD8\xA9\xD8\xA7\xD9\x84\xD8\xA8\xD8\xB1\xD8\xAA\xD8\xBA\xD8\xA7\xD9\x84\xD9\x8A\xD8\xA9 \xD8\xA7\xD9\x84\xD8\xA8\xD8\xB1\xD8\xA7\xD8\xB2\xD9\x8A\xD9\x84\xD9\x8A\xD8\xA9\xD8\xA7\xD9\x84\xD8\xA8\xD8\xB1\xD8\xAA\xD8\xBA\xD8\xA7\xD9\x84\xD9\x8A\xD8\xA9 \xD8\xA7\xD9\x84\xD8\xA3\xD9\x88\xD8\xB1\xD9\x88\xD8\xA8\xD9\x8A\xD8\xA9\xD8\xA7\xD9\x84\xD9\x85\xD9\x88\xD9\x84\xD8\xAF\xD9\x88\xD9\x81\xD9\x8A\xD8\xA9\xD8\xA7\xD9\x84\xD9\x83\xD9\x88\xD9\x86\xD8\xBA\xD9\x88 \xD8\xA7\xD9\x84\xD8\xB3\xD9\x88\xD8\xA7\xD8\xAD\xD9\x84\xD9\x8A\xD8\xA9\xD8\xA7\xD9\x84\xD8\xB5\xD9\x8A\xD9\x86\xD9\x8A\xD8\xA9 \xD8\xA7\xD9\x84\xD9\x85\xD8\xA8\xD8\xB3\xD8\xB7\xD8\xA9\xD8\xA7\xD9\x84\xD8\xB5\xD9\x8A\xD9\x86\xD9\x8A\xD8\xA9 \xD8\xA7\xD9\x84\xD8\xAA\xD9\x82\xD9\x84\xD9\x8A\xD8\xAF\xD9\x8A\xD8\xA9") })
                    },
                    short_names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x05\0en-GBen-US") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\x002\0\xD8\xA7\xD9\x84\xD8\xA5\xD9\x86\xD8\xAC\xD9\x84\xD9\x8A\xD8\xB2\xD9\x8A\xD8\xA9 \xD8\xA7\xD9\x84\xD9\x85\xD9\x85\xD9\x84\xD9\x83\xD8\xA9 \xD8\xA7\xD9\x84\xD9\x85\xD8\xAA\xD8\xAD\xD8\xAF\xD8\xA9\xD8\xA7\xD9\x84\xD8\xA5\xD9\x86\xD8\xAC\xD9\x84\xD9\x8A\xD8\xB2\xD9\x8A\xD8\xA9 \xD8\xA7\xD9\x84\xD9\x88\xD9\x84\xD8\xA7\xD9\x8A\xD8\xA7\xD8\xAA \xD8\xA7\xD9\x84\xD9\x85\xD8\xAA\xD8\xAD\xD8\xAF\xD8\xA9") })
                    },
                    long_names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x07\0zh-Hanszh-Hant") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x1D\0\xD8\xA7\xD9\x84\xD8\xB5\xD9\x8A\xD9\x86\xD9\x8A\xD8\xA9 \xD8\xA7\xD9\x84\xD9\x85\xD8\xA8\xD8\xB3\xD8\xB7\xD8\xA9\xD8\xA7\xD9\x84\xD8\xB5\xD9\x8A\xD9\x86\xD9\x8A\xD8\xA9 \xD8\xA7\xD9\x84\xD8\xAA\xD9\x82\xD9\x84\xD9\x8A\xD8\xAF\xD9\x8A\xD8\xA9") })
                    },
                    menu_names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                    },
                };
                static ES_AR: <icu_displaynames::provider::LocaleDisplayNamesV1Marker as icu_provider::DataMarker>::Yokeable = icu_displaynames::provider::LocaleDisplayNamesV1 {
                    names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x15\0\0\0\0\0\x06\0\x0B\0\x10\0\x15\0\x1A\0\x1F\0$\0*\0/\x004\09\0>\0C\0I\0N\0S\0X\0]\0b\0i\0ar-001de-ATde-CHen-AUen-CAen-GBen-USes-419es-ESes-MXfa-AFfr-CAfr-CHnds-NLnl-BEpt-BRpt-PTro-MDsw-CDzh-Hanszh-Hant") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x15\0\0\0\0\0\x18\0*\0<\0O\0a\0s\0\x89\0\xA1\0\xB4\0\xC7\0\xCC\0\xDF\0\xED\0\xF8\0\0\x01\x14\x01*\x011\x01B\x01T\x01\xC3\xA1rabe est\xC3\xA1ndar modernoalem\xC3\xA1n austr\xC3\xADacoalto alem\xC3\xA1n suizoingl\xC3\xA9s australianoingl\xC3\xA9s canadienseingl\xC3\xA9s brit\xC3\xA1nicoingl\xC3\xA9s estadounidenseespa\xC3\xB1ol latinoamericanoespa\xC3\xB1ol de Espa\xC3\xB1aespa\xC3\xB1ol de M\xC3\xA9xicodar\xC3\xADfranc\xC3\xA9s canadiensefranc\xC3\xA9s suizobajo saj\xC3\xB3nflamencoportugu\xC3\xA9s de Brasilportugu\xC3\xA9s de Portugalmoldavosuajili del Congochino simplificadochino tradicional") })
                    },
                    short_names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x05\0en-GBen-US") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0C\0ingl\xC3\xA9s (RU)ingl\xC3\xA9s (EE. UU.)") })
                    },
                    long_names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x07\0zh-Hanszh-Hant") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x1C\0chino mandar\xC3\xADn simplificadochino mandar\xC3\xADn tradicional") })
                    },
                    menu_names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                    },
                };
                static HI_LATN: <icu_displaynames::provider::LocaleDisplayNamesV1Marker as icu_provider::DataMarker>::Yokeable = icu_displaynames::provider::LocaleDisplayNamesV1 {
                    names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x17\0\0\0\0\0\x06\0\x0B\0\x10\0\x15\0\x1A\0\x1F\0$\0*\0/\x004\09\0>\0C\0J\0P\0U\0Z\0_\0d\0i\0n\0u\0ar-001de-ATde-CHen-AUen-CAen-GBen-USes-419es-ESes-MXfa-AFfr-CAfr-CHhi-Latnnds-NLnl-BEpt-BRpt-PTro-MDsr-MEsw-CDzh-Hanszh-Hant") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x17\0\0\0\0\0\x16\0%\x006\0H\0X\0g\0w\0\x8D\0\x9D\0\xAC\0\xB0\0\xBF\0\xCB\0\xD8\0\xE1\0\xE8\0\xFC\0\x0F\x01\x18\x01#\x010\x01B\x01Modern Standard ArabicAustrian GermanSwiss High GermanAustralian EnglishCanadian EnglishBritish EnglishAmerican EnglishLatin American SpanishEuropean SpanishMexican SpanishDariCanadian FrenchSwiss FrenchHindi (Latin)Low SaxonFlemishBrazilian PortugueseEuropean PortugueseMoldavianMontenegrinCongo SwahiliSimplified ChineseTraditional Chinese") })
                    },
                    short_names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x05\0en-GBen-US") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\n\0UK EnglishUS English") })
                    },
                    long_names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x07\0zh-Hanszh-Hant") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x1B\0Simplified Mandarin ChineseTraditional Mandarin Chinese") })
                    },
                    menu_names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                    },
                };
                static EN_001: <icu_displaynames::provider::LocaleDisplayNamesV1Marker as icu_provider::DataMarker>::Yokeable = icu_displaynames::provider::LocaleDisplayNamesV1 {
                    names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x17\0\0\0\0\0\x06\0\x0B\0\x10\0\x15\0\x1A\0\x1F\0$\0*\0/\x004\09\0>\0C\0J\0P\0U\0Z\0_\0d\0i\0n\0u\0ar-001de-ATde-CHen-AUen-CAen-GBen-USes-419es-ESes-MXfa-AFfr-CAfr-CHhi-Latnnds-NLnl-BEpt-BRpt-PTro-MDsr-MEsw-CDzh-Hanszh-Hant") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x17\0\0\0\0\0\x16\0%\x006\0H\0X\0g\0w\0\x8D\0\x9D\0\xAC\0\xB0\0\xBF\0\xCB\0\xD8\0\xE7\0\xEE\0\x02\x01\x15\x01\x1E\x01)\x016\x01H\x01Modern Standard ArabicAustrian GermanSwiss High GermanAustralian EnglishCanadian EnglishBritish EnglishAmerican EnglishLatin American SpanishEuropean SpanishMexican SpanishDariCanadian FrenchSwiss FrenchHindi (Latin)West Low GermanFlemishBrazilian PortugueseEuropean PortugueseMoldavianMontenegrinCongo SwahiliSimplified ChineseTraditional Chinese") })
                    },
                    short_names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x05\0en-GBen-US") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\n\0UK EnglishUS English") })
                    },
                    long_names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x07\0zh-Hanszh-Hant") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x1B\0Simplified Mandarin ChineseTraditional Mandarin Chinese") })
                    },
                    menu_names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                    },
                };
                static EN_AU: <icu_displaynames::provider::LocaleDisplayNamesV1Marker as icu_provider::DataMarker>::Yokeable = icu_displaynames::provider::LocaleDisplayNamesV1 {
                    names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x17\0\0\0\0\0\x06\0\x0B\0\x10\0\x15\0\x1A\0\x1F\0$\0*\0/\x004\09\0>\0C\0J\0P\0U\0Z\0_\0d\0i\0n\0u\0ar-001de-ATde-CHen-AUen-CAen-GBen-USes-419es-ESes-MXfa-AFfr-CAfr-CHhi-Latnnds-NLnl-BEpt-BRpt-PTro-MDsr-MEsw-CDzh-Hanszh-Hant") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x17\0\0\0\0\0\x16\0%\x006\0H\0X\0g\0|\0\x92\0\xA2\0\xB1\0\xB5\0\xC4\0\xD0\0\xDD\0\xEC\0\xF3\0\x07\x01\x1A\x01\"\x01-\x01:\x01L\x01Modern Standard ArabicAustrian GermanSwiss High GermanAustralian EnglishCanadian EnglishBritish EnglishUnited States EnglishLatin American SpanishEuropean SpanishMexican SpanishDariCanadian FrenchSwiss FrenchHindi (Latin)West Low GermanFlemishBrazilian PortugueseEuropean PortugueseMoldovanMontenegrinCongo SwahiliSimplified ChineseTraditional Chinese") })
                    },
                    short_names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x05\0en-GBen-US") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\n\0UK EnglishUS English") })
                    },
                    long_names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x07\0zh-Hanszh-Hant") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x1B\0Simplified Mandarin ChineseTraditional Mandarin Chinese") })
                    },
                    menu_names: unsafe {
                        #[allow(unused_unsafe)]
                        zerovec::ZeroMap::from_parts_unchecked(zerovec::VarZeroVec::new(), zerovec::VarZeroVec::new())
                    },
                };
                static VALUES: [&<icu_displaynames::provider::LocaleDisplayNamesV1Marker as icu_provider::DataMarker>::Yokeable; 100usize] = [&AR_EG, &CCP, &CCP, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_AU, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &EN_001, &ES_AR, &FIL, &HI_LATN];
                static KEYS: [&str; 100usize] = ["ar-EG", "ccp", "ccp-IN", "en-001", "en-150", "en-AG", "en-AI", "en-AT", "en-AU", "en-BB", "en-BE", "en-BM", "en-BS", "en-BW", "en-BZ", "en-CC", "en-CH", "en-CK", "en-CM", "en-CX", "en-CY", "en-DE", "en-DG", "en-DK", "en-DM", "en-ER", "en-FI", "en-FJ", "en-FK", "en-FM", "en-GB", "en-GD", "en-GG", "en-GH", "en-GI", "en-GM", "en-GY", "en-HK", "en-IE", "en-IL", "en-IM", "en-IN", "en-IO", "en-JE", "en-JM", "en-KE", "en-KI", "en-KN", "en-KY", "en-LC", "en-LR", "en-LS", "en-MG", "en-MO", "en-MS", "en-MT", "en-MU", "en-MV", "en-MW", "en-MY", "en-NA", "en-NF", "en-NG", "en-NL", "en-NR", "en-NU", "en-NZ", "en-PG", "en-PK", "en-PN", "en-PW", "en-RW", "en-SB", "en-SC", "en-SD", "en-SE", "en-SG", "en-SH", "en-SI", "en-SL", "en-SS", "en-SX", "en-SZ", "en-TC", "en-TK", "en-TO", "en-TT", "en-TV", "en-TZ", "en-UG", "en-VC", "en-VG", "en-VU", "en-WS", "en-ZA", "en-ZM", "en-ZW", "es-AR", "fil", "hi-Latn"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    let mut fallback_iterator = icu_locid_transform::fallback::LocaleFallbacker::new().fallback_for(<icu_displaynames::provider::LocaleDisplayNamesV1Marker as icu_provider::KeyedDataMarker>::KEY.into(), req.locale.clone());
                    loop {
                        if fallback_iterator.get().is_empty() {
                            return Err(icu_provider::DataErrorKind::MissingLocale.with_req(<icu_displaynames::provider::LocaleDisplayNamesV1Marker as icu_provider::KeyedDataMarker>::KEY, req));
                        }
                        if let Ok(payload) = KEYS.binary_search_by(|k| fallback_iterator.get().strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                            metadata.locale = Some(fallback_iterator.take());
                            break payload;
                        }
                        fallback_iterator.step();
                    }
                };
                Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(payload)), metadata })
            }
        }
    };
}
