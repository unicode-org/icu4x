// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::prelude::*;
use core::any::TypeId;
use displaydoc::Display;

/// A list specifying general categories of data provider error.
///
/// Errors may be caused either by a malformed request or by the data provider
/// not being able to fulfill a well-formed request.
#[derive(Clone, Copy, Eq, PartialEq, Display, Debug)]
#[non_exhaustive]
pub enum DataErrorKind {
    /// No data for the provided resource key.
    #[displaydoc("Missing resource key")]
    MissingResourceKey,

    /// There is data for the key, but not for this particular variant.
    #[displaydoc("Missing data for variant")]
    MissingVariant,

    /// There is data for the key, but not for this particular locale.
    #[displaydoc("Missing data for locale")]
    MissingLocale,

    /// There is data for the key, but not for this particular variant and/or locale.
    #[displaydoc("Missing data for variant or locale")]
    MissingResourceOptions,

    /// The request should include a variant field.
    #[displaydoc("Request needs a variant field")]
    NeedsVariant,

    /// The request should include a locale.
    #[displaydoc("Request needs a locale")]
    NeedsLocale,

    /// The request should not contain a variant and/or locale.
    #[displaydoc("Request has extraneous information")]
    ExtraneousResourceOptions,

    /// The resource was blocked by a filter. The resource may or may not be available.
    #[displaydoc("Resource blocked by filter")]
    FilteredResource,

    /// The generic type parameter does not match the TypeId. The actual TypeId is stored
    /// as context when this error is returned.
    #[displaydoc("Mismatched type information (expected {0:?})")]
    MismatchedType(TypeId),

    /// The payload is missing. This is usually caused by a previous error.
    #[displaydoc("Missing payload")]
    MissingPayload,

    /// An error occurred while serializing or deserializing data with Serde.
    ///
    /// Check debug logs for potentially more information.
    #[displaydoc("Serde error")]
    Serde,

    /// An error occurred while accessing a system resource.
    #[displaydoc("I/O error: {0:?}")]
    #[cfg(feature = "std")]
    Io(std::io::ErrorKind),
}

/// The error type for ICU4X data provider operations.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub struct DataError {
    /// Broad category of the error.
    pub kind: DataErrorKind,

    /// The data key of the request, if available.
    pub key: Option<ResourceKey>,

    /// Additional context, if available.
    pub str_context: Option<&'static str>,
}

impl core::fmt::Display for DataError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ICU4X data provider error: {}", self.kind)?;
        if let Some(key) = self.key {
            write!(f, " (key: {})", key)?;
        }
        if let Some(str_context) = self.str_context {
            write!(f, ": {}", str_context)?;
        }
        Ok(())
    }
}

impl DataErrorKind {
    /// Converts this DataErrorKind into a DataError.
    ///
    /// If possible, you should attach context to the DataErrorKind using a `with_` function.
    #[inline]
    pub const fn into_error(self) -> DataError {
        DataError {
            kind: self,
            key: None,
            str_context: None,
        }
    }

    /// Creates a DataError with a resource key context.
    #[inline]
    pub const fn with_key(self, key: ResourceKey) -> DataError {
        self.into_error().with_key(key)
    }

    /// Creates a DataError with a string context.
    #[inline]
    pub const fn with_str_context(self, context: &'static str) -> DataError {
        self.into_error().with_str_context(context)
    }

    /// Creates a DataError with a type name context.
    #[inline]
    pub fn with_type_context<T>(self) -> DataError {
        self.into_error().with_type_context::<T>()
    }

    /// Creates a DataError with a request context.
    #[inline]
    pub fn with_req(self, req: &DataRequest) -> DataError {
        self.into_error().with_req(req)
    }
}

impl DataError {
    /// Create sa new DataError with the specified kind.
    ///
    /// To add context to the error, follow this with a call to [`Self::with_str_context()`].
    #[inline]
    pub const fn new(kind: DataErrorKind) -> Self {
        Self {
            kind,
            key: None,
            str_context: None,
        }
    }

    /// Sets the resource key of a DataError, returning a modified error.
    #[inline]
    pub const fn with_key(self, key: ResourceKey) -> Self {
        Self {
            kind: self.kind,
            key: Some(key),
            str_context: self.str_context,
        }
    }

    /// Sets the string context of a DataError, returning a modified error.
    #[inline]
    pub const fn with_str_context(self, context: &'static str) -> Self {
        Self {
            kind: self.kind,
            key: self.key,
            str_context: Some(context),
        }
    }

    /// Sets the string context of a DataError to the given type name, returning a modified error.
    #[inline]
    pub fn with_type_context<T>(self) -> Self {
        self.with_str_context(core::any::type_name::<T>())
    }

    pub fn with_req(self, req: &DataRequest) -> Self {
        #[cfg(feature = "log_error_context")]
        log::warn!("{} (request: {})", self, req);
        self.with_key(req.resource_path.key)
    }

    /// Logs the data error with the given context, then return self.
    ///
    /// This does not modify the error, but if the "log_error_context" feature is enabled,
    /// it will print out the context.
    pub fn with_display_context(self, context: &impl core::fmt::Display) -> Self {
        #[cfg(feature = "log_error_context")]
        log::warn!("{}: {}", self, context);
        self
    }

    /// Logs the data error with the given context, then return self.
    ///
    /// This does not modify the error, but if the "log_error_context" feature is enabled,
    /// it will print out the context.
    pub fn with_debug_context(self, context: &impl core::fmt::Debug) -> Self {
        #[cfg(feature = "log_error_context")]
        log::warn!("{}: {:?}", self, context);
        self
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DataError {}

#[cfg(feature = "serde")]
impl From<crate::serde::Error> for DataError {
    fn from(e: crate::serde::Error) -> Self {
        #[cfg(feature = "log_error_context")]
        log::warn!("Serde error: {}", e);
        DataError::new(DataErrorKind::Serde)
    }
}
