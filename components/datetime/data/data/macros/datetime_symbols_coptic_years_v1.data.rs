// @generated
/// Implement `DataProvider<CopticYearSymbolsV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_datetime_symbols_coptic_years_v1 {
    ($ provider : ty) => {
        #[clippy::msrv = "1.67"]
        const _: () = <$provider>::MUST_USE_MAKE_PROVIDER_MACRO;
        #[clippy::msrv = "1.67"]
        impl icu_provider::DataProvider<icu::datetime::provider::neo::CopticYearSymbolsV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::datetime::provider::neo::CopticYearSymbolsV1Marker>, icu_provider::DataError> {
                static FF_ADLM_X_5: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0I\0\xF0\x9E\xA4\xA9\xF0\x9E\xA4\xA2\xF0\x9E\xA5\x84\xF0\x9E\xA4\xB1\xF0\x9E\xA4\xAE \xF0\x9E\xA4\x81\xF0\x9E\xA4\xA2\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xA2\xF0\x9E\xA4\xB3\xF0\x9E\xA4\xAD\xF0\x9E\xA4\xA4\xF0\x9E\xA4\xBC\xF0\x9E\xA4\xAD\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xA2\xF0\x9E\xA5\x84\xF0\x9E\xA4\xB2\xF0\x9E\xA4\xA2\xF0\x9E\xA4\xA3\xF0\x9E\xA4\xAE \xF0\x9E\xA4\x81\xF0\x9E\xA4\xA2\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xA2\xF0\x9E\xA4\xB3\xF0\x9E\xA4\xAD\xF0\x9E\xA4\xA4\xF0\x9E\xA4\xBC\xF0\x9E\xA4\xAD\xF0\x9E\xA4\xB4\xF0\x9E\xA4\xA2\xF0\x9E\xA5\x84\xF0\x9E\xA4\xB2") })
                });
                static HE_X_5: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\n\0\xD7\xA2\xD7\x99\xD7\x93\xD7\x9F 1\xD7\xA2\xD7\x99\xD7\x93\xD7\x9F 0") })
                });
                static MR_X_3: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\n\0\xE0\xA4\xAF\xE0\xA5\x81\xE0\xA4\x971\xE0\xA4\xAF\xE0\xA5\x81\xE0\xA4\x970") })
                });
                static PA_X_3: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\n\0\xE0\xA8\x95\xE0\xA8\xBE\xE0\xA8\xB21\xE0\xA8\x95\xE0\xA8\xBE\xE0\xA8\xB20") })
                });
                static GU_X_3: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\n\0\xE0\xAA\x8F\xE0\xAA\xB0\xE0\xAA\xBE1\xE0\xAA\x8F\xE0\xAA\xB0\xE0\xAA\xBE0") })
                });
                static BN_X_3: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\r\0\xE0\xA6\xAF\xE0\xA7\x81\xE0\xA6\x97 \xE0\xA7\xA7\xE0\xA6\xAF\xE0\xA7\x81\xE0\xA6\x97 \xE0\xA7\xA6") })
                });
                static DA_X_3: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\t\x001. tidsr.0. tidsr.") })
                });
                static NB_X_4: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x03\0TA1TA0") })
                });
                static RO_X_3: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0A.M.\xC3\xAE.A.M.") })
                });
                static UND_X_3: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0ERA1ERA0") })
                });
                static SC_X_3: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x04\0a.M.a.D.") })
                });
                static DA_X_4: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x05\x001. t.0. t.") })
                });
                static FA_X_4: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0\xD8\xA8.\xD9\x85.\xD9\x82.\xD9\x85.") })
                });
                static FR_X_3: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x06\0ap. D.av. D.") })
                });
                static MK_X_3: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x07\0\xD0\x95\xD0\xA0\xD0\x901\xD0\x95\xD0\xA0\xD0\x900") })
                });
                static UR_X_3: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x07\0\xD8\xAF\xD9\x88\xD8\xB11\xD8\xAF\xD9\x88\xD8\xB10") })
                });
                static NB_X_3: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x07\x001. t.a.0. t.a.") })
                });
                static FF_ADLM_X_3: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x08\0\xF0\x9E\xA4\x87\xF0\x9E\xA4\x81\xF0\x9E\xA4\x80\xF0\x9E\xA4\x81") })
                });
                static LV_X_3: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0B\0p\xC4\x93c Diokl.pirms Diokl.") })
                });
                static ZH_X_3: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0C\0\xE7\xA7\x91\xE6\x99\xAE\xE7\x89\xB9\xE5\x8E\x86\xE7\xA7\x91\xE6\x99\xAE\xE7\x89\xB9\xE5\x8E\x86\xE5\x89\x8D") })
                });
                static NB_X_5: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0C\x001. tidsalder0. tidsalder") })
                });
                static DA_X_5: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x0E\x001. tidsregning0. tidsregning") })
                });
                static RU_X_3: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x10\0\xD0\xBE\xD1\x82 \xD0\x94\xD0\xB8\xD0\xBE\xD0\xBA\xD0\xBB.\xD0\xB4\xD0\xBE \xD0\x94\xD0\xB8\xD0\xBE\xD0\xBA\xD0\xBB.") })
                });
                static LV_X_5: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x11\0p\xC4\x93c Diokleti\xC4\x81napirms Diokleti\xC4\x81na") })
                });
                static FA_X_3: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x12\0\xD9\xBE\xD8\xB3 \xD8\xA7\xD8\xB2 \xD9\x85\xD8\xB3\xDB\x8C\xD8\xAD\xD9\x82\xD8\xA8\xD9\x84 \xD8\xA7\xD8\xB2 \xD9\x85\xD8\xB3\xDB\x8C\xD8\xAD") })
                });
                static FR_X_5: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x12\0apr\xC3\xA8s Diocl\xC3\xA9tienavant Diocl\xC3\xA9tien") })
                });
                static RO_X_5: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x13\0dup\xC4\x83 Anno Martyrum\xC3\xAEnainte de Anno Martyrum") })
                });
                static SC_X_5: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x15\0annu de sos m\xC3\xA0rtiresin antis de Diocletzianu") })
                });
                static ML_X_3: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x19\0\xE0\xB4\x95\xE0\xB4\xBE\xE0\xB4\xB2\xE0\xB4\x98\xE0\xB4\x9F\xE0\xB5\x8D\xE0\xB4\x9F\xE0\xB4\x821\xE0\xB4\x95\xE0\xB4\xBE\xE0\xB4\xB2\xE0\xB4\x98\xE0\xB4\x9F\xE0\xB5\x8D\xE0\xB4\x9F\xE0\xB4\x820") })
                });
                static RU_X_5: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x1B\0\xD0\xBE\xD1\x82 \xD0\x94\xD0\xB8\xD0\xBE\xD0\xBA\xD0\xBB\xD0\xB5\xD1\x82\xD0\xB8\xD0\xB0\xD0\xBD\xD0\xB0\xD0\xB4\xD0\xBE \xD0\x94\xD0\xB8\xD0\xBE\xD0\xBA\xD0\xBB\xD0\xB5\xD1\x82\xD0\xB8\xD0\xB0\xD0\xBD\xD0\xB0") })
                });
                static FA_X_5: <icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable = icu::datetime::provider::neo::YearSymbolsV1::Eras(unsafe {
                    #[allow(unused_unsafe)]
                    zerovec::ZeroMap::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"ad\0\0\0\0\0\0\0\0\0\0\0\0\0\0bd\0\0\0\0\0\0\0\0\0\0\0\0\0\0") }, unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x02\0\0\0\0\0\x1D\0\xD8\xA8\xD8\xB9\xD8\xAF \xD8\xA7\xD8\xB2 \xD8\xAD\xD9\x84\xD9\x88\xD9\x84 \xD9\x85\xD8\xB3\xDB\x8C\xD8\xAD\xD9\x82\xD8\xA8\xD9\x84 \xD8\xA7\xD8\xB2 \xD8\xAD\xD9\x84\xD9\x88\xD9\x84 \xD9\x85\xD8\xB3\xDB\x8C\xD8\xAD") })
                });
                static VALUES: [&<icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::DataMarker>::Yokeable; 61usize] = [&BN_X_3, &BN_X_3, &BN_X_3, &DA_X_3, &DA_X_4, &DA_X_5, &FA_X_3, &FA_X_4, &FA_X_5, &FF_ADLM_X_3, &FF_ADLM_X_3, &FF_ADLM_X_5, &FR_X_3, &FR_X_3, &FR_X_5, &GU_X_3, &GU_X_3, &GU_X_3, &HE_X_5, &LV_X_3, &LV_X_3, &LV_X_5, &MK_X_3, &MK_X_3, &MK_X_3, &ML_X_3, &ML_X_3, &ML_X_3, &MR_X_3, &MR_X_3, &MR_X_3, &NB_X_3, &NB_X_4, &NB_X_5, &NB_X_3, &NB_X_4, &NB_X_5, &NB_X_3, &NB_X_4, &NB_X_5, &PA_X_3, &PA_X_3, &PA_X_3, &RO_X_3, &RO_X_3, &RO_X_5, &RU_X_3, &RU_X_3, &RU_X_5, &SC_X_3, &SC_X_3, &SC_X_5, &UND_X_3, &UND_X_3, &UND_X_3, &UR_X_3, &UR_X_3, &UR_X_3, &ZH_X_3, &ZH_X_3, &ZH_X_3];
                static KEYS: [&str; 61usize] = ["bn-x-3", "bn-x-4", "bn-x-5", "da-x-3", "da-x-4", "da-x-5", "fa-x-3", "fa-x-4", "fa-x-5", "ff-Adlm-x-3", "ff-Adlm-x-4", "ff-Adlm-x-5", "fr-x-3", "fr-x-4", "fr-x-5", "gu-x-3", "gu-x-4", "gu-x-5", "he-x-5", "lv-x-3", "lv-x-4", "lv-x-5", "mk-x-3", "mk-x-4", "mk-x-5", "ml-x-3", "ml-x-4", "ml-x-5", "mr-x-3", "mr-x-4", "mr-x-5", "nb-x-3", "nb-x-4", "nb-x-5", "nn-x-3", "nn-x-4", "nn-x-5", "no-x-3", "no-x-4", "no-x-5", "pa-x-3", "pa-x-4", "pa-x-5", "ro-x-3", "ro-x-4", "ro-x-5", "ru-x-3", "ru-x-4", "ru-x-5", "sc-x-3", "sc-x-4", "sc-x-5", "und-x-3", "und-x-4", "und-x-5", "ur-x-3", "ur-x-4", "ur-x-5", "zh-x-3", "zh-x-4", "zh-x-5"];
                let mut metadata = icu_provider::DataResponseMetadata::default();
                let payload = if let Ok(payload) = KEYS.binary_search_by(|k| req.locale.strict_cmp(k.as_bytes()).reverse()).map(|i| *unsafe { VALUES.get_unchecked(i) }) {
                    payload
                } else {
                    const FALLBACKER: icu::locid_transform::fallback::LocaleFallbackerWithConfig<'static> = icu::locid_transform::fallback::LocaleFallbacker::new().for_config(<icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.fallback_config());
                    let mut fallback_iterator = FALLBACKER.fallback_for(req.locale.clone());
                    loop {
                        if fallback_iterator.get().is_und() {
                            return Err(icu_provider::DataErrorKind::MissingLocale.with_req(<icu::datetime::provider::neo::CopticYearSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY, req));
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
