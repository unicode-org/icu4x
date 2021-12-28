// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::prelude::*;
use alloc::format;
use alloc::string::String;
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
    #[displaydoc("Missing resource key: {0}")]
    MissingResourceKey(ResourceKey),

    /// There is data for the key, but not for this particular variant/locale.
    #[displaydoc("Missing resource options for key: {0}")]
    MissingResourceOptions(ResourceKey),

    /// The request should include a variant field.
    #[displaydoc("Request needs a variant field")]
    NeedsVariant,

    /// The request should include a locale.
    #[displaydoc("Request needs a locale")]
    NeedsLanguageIdentifier,

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
pub struct DataError {
    /// Broad category of the error.
    pub kind: DataErrorKind,

    /// Additional context, if available.
    pub str_context: Option<&'static str>,
}

impl core::fmt::Display for DataError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(str_context) = self.str_context {
            write!(f, "ICU4X data provider error: {}: {}", self.kind, str_context)
        } else {
            write!(f, "ICU4X data provider error: {}", self.kind)
        }
    }
}

impl DataError {
    /// Create a new DataError with the specified kind.
    /// 
    /// To add context to the error, follow this with a call to [`Self::with_str_context()`].
    #[inline]
    pub const fn new(kind: DataErrorKind) -> Self {
        Self {
            kind,
            str_context: None,
        }
    }

    /// Set the string context of a DataError, returning a modified error.
    #[inline]
    pub const fn with_str_context(self, context: &'static str) -> Self {
        Self {
            kind: self.kind,
            str_context: Some(context)
        }
    }

    /// Log the data error with the given context, then return self.
    /// 
    /// This does not modify the error, but if the "log_error_context" feature is enabled,
    /// it will print out the context.
    pub fn with_display_context(self, context: &impl core::fmt::Display) -> Self {
        #[cfg(feature = "log_error_context")]
        log::warn!("{}: {}", self, context);
        self
    }

    /// Log the data error with the given context, then return self.
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

/// Error enumeration for DataProvider.
#[non_exhaustive]
#[derive(Display, Debug)]
pub enum Error {
    /// The data provider does not support the resource key.
    #[displaydoc("Missing resource for key: {0}")]
    MissingResourceKey(ResourceKey),

    /// The data provider supports the key, but does not have data for the specific entry.
    #[displaydoc("Missing resource for request: {0}")]
    MissingResourceOptions(DataRequest),

    /// The data provider supports the key, but the key needs to be supplied with a variant
    #[displaydoc("Request {0} needs a variant")]
    NeedsVariant(DataRequest),

    /// Should not have variant or language id
    #[displaydoc("Requested key should not have a variant or language id: {0}")]
    ExtraneousVariantOrId(DataRequest),

    /// The resource was not returned due to a filter. The resource may or may not be available.
    #[displaydoc("Resource was filtered: {1}: {0}")]
    FilteredResource(DataRequest, String),

    /// The data provider supports the key, but it requires a language identifier, which was
    /// missing from the request.
    #[displaydoc("Requested key needs language identifier in request: {0}")]
    NeedsLanguageIdentifier(DataRequest),

    /// The operation cannot be completed without more type information. For example, data
    /// cannot be deserialized without the concrete type.
    #[displaydoc("Complete type information is required")]
    NeedsTypeInfo,

    /// The payload is missing. This error is usually unexpected.
    #[displaydoc("Payload is missing")]
    MissingPayload,

    /// The TypeID of the payload does not match the expected TypeID.
    #[displaydoc("Mismatched type: payload is {actual:?} (expected from generic type paramenter: {generic:?})")]
    MismatchedType {
        /// The actual TypeID of the payload, if available.
        actual: Option<TypeId>,

        /// The expected TypeID derived from the generic type parameter at the call site.
        generic: Option<TypeId>,
    },

    /// The requested operation failed to unwrap an Rc backing the data payload.
    #[displaydoc("Could not unwrap Rc due to multiple references")]
    MultipleReferences,

    /// An error occurred during serialization or deserialization.
    #[cfg(feature = "serde")]
    #[displaydoc("Serde error: {0}")]
    Serde(crate::serde::Error),

    /// The data provider encountered some other error when loading the resource, such as I/O.
    #[displaydoc("Failed to load resource: {0}")]
    Resource(String),
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[cfg(feature = "serde")]
impl From<crate::serde::Error> for Error {
    fn from(e: crate::serde::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::Resource(e)
    }
}

impl Error {
    pub fn new_resc_error<T>(err: T) -> Self
    where
        T: core::fmt::Display,
    {
        Self::Resource(format!("{}", err))
    }
}

impl From<&ResourceKey> for Error {
    fn from(resc_key: &ResourceKey) -> Self {
        Self::MissingResourceKey(*resc_key)
    }
}

impl From<DataRequest> for Error {
    fn from(req: DataRequest) -> Self {
        Self::MissingResourceOptions(req)
    }
}
