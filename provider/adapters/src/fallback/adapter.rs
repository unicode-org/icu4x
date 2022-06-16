// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;

pub trait LocaleFallbackProvider {
    fn load_fallback_metadata_for_key(
        &self,
        key: ResourceKey,
    ) -> Result<LocaleFallbackKeyMetadata, DataError>;
}

pub struct LocaleFallbackAdapter<P>(pub P);

impl<P> BufferProvider for LocaleFallbackAdapter<P>
where
    P: LocaleFallbackProvider + BufferProvider,
{
    fn load_buffer(
        &self,
        _key: ResourceKey,
        _req: &DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        todo!()
    }
}
