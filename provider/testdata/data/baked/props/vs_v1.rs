// @generated
type DataStruct = & 'static < :: icu_properties :: provider :: VariationSelectorV1Marker as :: icu_provider :: DataMarker > :: Yokeable ;
pub static DATA: &[(&str, DataStruct)] = &[("und", UND)];
static UND: DataStruct =
    &::icu_properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
        #[allow(unused_unsafe)]
        ::icu_uniset::CodePointInversionList::from_parts_unchecked(
            unsafe {
                ::zerovec::ZeroVec::from_bytes_unchecked(&[
                    11u8, 24u8, 0u8, 0u8, 14u8, 24u8, 0u8, 0u8, 15u8, 24u8, 0u8, 0u8, 16u8, 24u8,
                    0u8, 0u8, 0u8, 254u8, 0u8, 0u8, 16u8, 254u8, 0u8, 0u8, 0u8, 1u8, 14u8, 0u8,
                    240u8, 1u8, 14u8, 0u8,
                ])
            },
            260usize,
        )
    });
