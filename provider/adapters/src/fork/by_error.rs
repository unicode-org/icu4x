// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::marker::PhantomData;

use super::ForkByErrorPredicate;
use alloc::vec::Vec;
#[cfg(feature = "datagen")]
use icu_provider::datagen;
use icu_provider::prelude::*;

/// A provider that returns data from one of two child providers based on a predicate function.
///
/// This is an abstract forking provider that must be provided with a type implementing the
/// [`ForkByErrorPredicate`] trait.
#[derive(Debug, PartialEq, Eq)]
pub struct ForkByErrorProvider<P0, P1, F>(pub P0, pub P1, PhantomData<F>);

impl<P0, P1, F> ForkByErrorProvider<P0, P1, F> {
    /// Create a new provider that forks between the two children.
    pub fn new(p0: P0, p1: P1) -> Self {
        Self(p0, p1, PhantomData)
    }
}

impl<P0, P1, F> BufferProvider for ForkByErrorProvider<P0, P1, F>
where
    P0: BufferProvider,
    P1: BufferProvider,
    F: ForkByErrorPredicate,
{
    fn load_buffer(
        &self,
        key: DataKey,
        req: DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        let result = self.0.load_buffer(key, req);
        match result {
            Ok(ok) => return Ok(ok),
            Err(err) if !F::predicate(key, Some(req), err) => return Err(err),
            _ => (),
        };
        self.1.load_buffer(key, req)
    }
}

impl<P0, P1, F> AnyProvider for ForkByErrorProvider<P0, P1, F>
where
    P0: AnyProvider,
    P1: AnyProvider,
    F: ForkByErrorPredicate,
{
    fn load_any(&self, key: DataKey, req: DataRequest) -> Result<AnyResponse, DataError> {
        let result = self.0.load_any(key, req);
        match result {
            Ok(ok) => return Ok(ok),
            Err(err) if !F::predicate(key, Some(req), err) => return Err(err),
            _ => (),
        };
        self.1.load_any(key, req)
    }
}

impl<M, P0, P1, F> DynamicDataProvider<M> for ForkByErrorProvider<P0, P1, F>
where
    M: DataMarker,
    P0: DynamicDataProvider<M>,
    P1: DynamicDataProvider<M>,
    F: ForkByErrorPredicate,
{
    fn load_data(&self, key: DataKey, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        let result = self.0.load_data(key, req);
        match result {
            Ok(ok) => return Ok(ok),
            Err(err) if !F::predicate(key, Some(req), err) => return Err(err),
            _ => (),
        };
        self.1.load_data(key, req)
    }
}

#[cfg(feature = "datagen")]
impl<M, P0, P1, F> datagen::IterableDynamicDataProvider<M> for ForkByErrorProvider<P0, P1, F>
where
    M: DataMarker,
    P0: datagen::IterableDynamicDataProvider<M>,
    P1: datagen::IterableDynamicDataProvider<M>,
    F: ForkByErrorPredicate,
{
    fn supported_locales_for_key(&self, key: DataKey) -> Result<Vec<DataLocale>, DataError> {
        let result = self.0.supported_locales_for_key(key);
        match result {
            Ok(ok) => return Ok(ok),
            Err(err) if !F::predicate(key, None, err) => return Err(err),
            _ => (),
        };
        self.1.supported_locales_for_key(key)
    }
}

/// A provider that returns data from the first child provider passing a predicate function.
///
/// This is an abstract forking provider that must be provided with a type implementing the
/// [`ForkByErrorPredicate`] trait.
pub struct MultiForkByErrorProvider<P, F> {
    pub providers: Vec<P>,
    _predicate: PhantomData<F>,
}

impl<P, F> MultiForkByErrorProvider<P, F> {
    /// Create a new provider that forks between the vector of children.
    pub fn new(providers: Vec<P>) -> Self {
        Self {
            providers,
            _predicate: PhantomData,
        }
    }
}

impl<P, F> BufferProvider for MultiForkByErrorProvider<P, F>
where
    P: BufferProvider,
    F: ForkByErrorPredicate,
{
    fn load_buffer(
        &self,
        key: DataKey,
        req: DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        for provider in self.providers.iter() {
            let result = provider.load_buffer(key, req);
            match result {
                Ok(ok) => return Ok(ok),
                Err(err) if !F::predicate(key, Some(req), err) => return Err(err),
                _ => (),
            };
        }
        Err(DataErrorKind::MissingDataKey.with_key(key))
    }
}

impl<P, F> AnyProvider for MultiForkByErrorProvider<P, F>
where
    P: AnyProvider,
    F: ForkByErrorPredicate,
{
    fn load_any(&self, key: DataKey, req: DataRequest) -> Result<AnyResponse, DataError> {
        for provider in self.providers.iter() {
            let result = provider.load_any(key, req);
            match result {
                Ok(ok) => return Ok(ok),
                Err(err) if !F::predicate(key, Some(req), err) => return Err(err),
                _ => (),
            };
        }
        Err(DataErrorKind::MissingDataKey.with_key(key))
    }
}

impl<M, P, F> DynamicDataProvider<M> for MultiForkByErrorProvider<P, F>
where
    M: DataMarker,
    P: DynamicDataProvider<M>,
    F: ForkByErrorPredicate,
{
    fn load_data(&self, key: DataKey, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        for provider in self.providers.iter() {
            let result = provider.load_data(key, req);
            match result {
                Ok(ok) => return Ok(ok),
                Err(err) if !F::predicate(key, Some(req), err) => return Err(err),
                _ => (),
            };
        }
        Err(DataErrorKind::MissingDataKey.with_key(key))
    }
}

#[cfg(feature = "datagen")]
impl<M, P, F> datagen::IterableDynamicDataProvider<M> for MultiForkByErrorProvider<P, F>
where
    M: DataMarker,
    P: datagen::IterableDynamicDataProvider<M>,
    F: ForkByErrorPredicate,
{
    fn supported_locales_for_key(&self, key: DataKey) -> Result<Vec<DataLocale>, DataError> {
        for provider in self.providers.iter() {
            let result = provider.supported_locales_for_key(key);
            match result {
                Ok(ok) => return Ok(ok),
                Err(err) if !F::predicate(key, None, err) => return Err(err),
                _ => (),
            };
        }
        Err(DataErrorKind::MissingDataKey.with_key(key))
    }
}

#[cfg(feature = "datagen")]
impl<P, MFrom, MTo, F> datagen::DataConverter<MFrom, MTo> for MultiForkByErrorProvider<P, F>
where
    P: datagen::DataConverter<MFrom, MTo>,
    F: ForkByErrorPredicate,
    MFrom: DataMarker,
    MTo: DataMarker,
{
    fn convert(
        &self,
        key: DataKey,
        mut from: DataPayload<MFrom>,
    ) -> Result<DataPayload<MTo>, datagen::ReturnedPayloadError<MFrom>> {
        use datagen::ReturnedPayloadError;

        for provider in self.providers.iter() {
            let result = provider.convert(key, from);
            match result {
                Ok(ok) => return Ok(ok),
                Err(e) => {
                    let ReturnedPayloadError(returned, err) = e;
                    if !F::predicate(key, None, err) {
                        return Err(ReturnedPayloadError(returned, err));
                    }
                    from = returned;
                }
            };
        }
        Err(ReturnedPayloadError(
            from,
            DataErrorKind::MissingDataKey.with_key(key),
        ))
    }
}

#[cfg(feature = "datagen")]
impl<P0, P1, F, MFrom, MTo> datagen::DataConverter<MFrom, MTo> for ForkByErrorProvider<P0, P1, F>
where
    P0: datagen::DataConverter<MFrom, MTo>,
    P1: datagen::DataConverter<MFrom, MTo>,
    F: ForkByErrorPredicate,
    MFrom: DataMarker,
    MTo: DataMarker,
{
    fn convert(
        &self,
        key: DataKey,
        mut from: DataPayload<MFrom>,
    ) -> Result<DataPayload<MTo>, datagen::ReturnedPayloadError<MFrom>> {
        use datagen::ReturnedPayloadError;
        let result = self.0.convert(key, from);
        match result {
            Ok(ok) => return Ok(ok),
            Err(e) => {
                let ReturnedPayloadError(returned, err) = e;
                if !F::predicate(key, None, err) {
                    return Err(ReturnedPayloadError(returned, err));
                }
                from = returned;
            }
        };
        self.1.convert(key, from)
    }
}
