// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_timezone::CustomTimeZone;
use icu_timezone::GmtOffset;
use icu_timezone::MetaZoneId;
use icu_timezone::TimeZoneBcp47Id;
use icu_timezone::ZoneVariant;

#[diplomat::bridge]
pub mod ffi {
    use crate::errors::ffi::ICU4XError;
    use crate::tinystr::ffi::TinyAsciiStr2;
    use crate::tinystr::ffi::TinyAsciiStr4;
    use crate::tinystr::ffi::TinyAsciiStr8;
    use alloc::boxed::Box;
    use core::str::FromStr;
    use diplomat_runtime::DiplomatResult;
    use icu_timezone::CustomTimeZone;

    pub struct ICU4XGmtOffset {
        pub(super) data: i32,
    }

    pub struct ICU4XTimeZoneBcp47Id {
        pub id: TinyAsciiStr8,
    }

    pub struct ICU4XMetaZoneId {
        pub id: TinyAsciiStr4,
    }

    pub struct ICU4XZoneVariant {
        pub id: TinyAsciiStr2,
    }

    #[diplomat::rust_link(icu::timezone::CustomTimeZone, Struct)]
    pub struct ICU4XCustomTimeZone {
        pub gmt_offset: Option<ICU4XGmtOffset>,
        pub time_zone_id: Option<ICU4XTimeZoneBcp47Id>,
        pub metazone_id: Option<ICU4XMetaZoneId>,
        pub zone_variant: Option<ICU4XZoneVariant>,
    }

    impl ICU4XCustomTimeZone {
        pub fn create_from_str(s: &str) -> DiplomatResult<ICU4XCustomTimeZone, ICU4XError> {
            CustomTimeZone::from_str(s)
                .map(Into::into)
                .map_err(Into::into)
                .into()
        }
    }
}

impl From<GmtOffset> for ffi::ICU4XGmtOffset {
    fn from(other: GmtOffset) -> Self {
        Self {
            data: other.raw_offset_seconds().into(),
        }
    }
}

impl From<ffi::ICU4XGmtOffset> for GmtOffset {
    fn from(other: ffi::ICU4XGmtOffset) -> Self {
        // Safety: The data field is private, and the only way to construct
        // one of these is the From impl above
        unsafe {
            Self::from_raw_unchecked(other.data)
        }
    }
}

crate::helpers::impl_newtype_convert!(TimeZoneBcp47Id, ffi::ICU4XTimeZoneBcp47Id, pub id);
crate::helpers::impl_newtype_convert!(MetaZoneId, ffi::ICU4XMetaZoneId, pub id);
crate::helpers::impl_newtype_convert!(ZoneVariant, ffi::ICU4XZoneVariant, pub id);

impl From<CustomTimeZone> for ffi::ICU4XCustomTimeZone {
    fn from(other: CustomTimeZone) -> Self {
        Self {
            gmt_offset: other.gmt_offset.map(Into::into),
            time_zone_id: other.time_zone_id.map(Into::into),
            metazone_id: other.metazone_id.map(Into::into),
            zone_variant: other.zone_variant.map(Into::into),
        }
    }
}

impl From<ffi::ICU4XCustomTimeZone> for CustomTimeZone {
    fn from(other: ffi::ICU4XCustomTimeZone) -> Self {
        Self {
            gmt_offset: other.gmt_offset.map(Into::into),
            time_zone_id: other.time_zone_id.map(Into::into),
            metazone_id: other.metazone_id.map(Into::into),
            zone_variant: other.zone_variant.map(Into::into),
        }
    }
}
