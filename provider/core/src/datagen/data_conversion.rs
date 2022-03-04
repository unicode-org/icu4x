// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::prelude::*;

pub trait ConvertData<MFrom: DataMarker, MTo: DataMarker> {
    fn convert(
        &self,
        key: crate::ResourceKey,
        from: DataPayload<MFrom>,
    ) -> Result<DataPayload<MTo>, ReturnedPayloadError<MFrom>>;
}

impl<MFrom, MTo, P> ConvertData<MFrom, MTo> for Box<P>
where
    MFrom: DataMarker,
    MTo: DataMarker,
    P: ConvertData<MFrom, MTo> + ?Sized,
{
    fn convert(
        &self,
        key: crate::ResourceKey,
        from: DataPayload<MFrom>,
    ) -> Result<DataPayload<MTo>, ReturnedPayloadError<MFrom>> {
        (**self).convert(key, from)
    }
}

pub struct ReturnedPayloadError<M: DataMarker>(pub DataPayload<M>, pub DataError);

impl<M: DataMarker> From<ReturnedPayloadError<M>> for DataError {
    fn from(other: ReturnedPayloadError<M>) -> Self {
        other.1
    }
}
