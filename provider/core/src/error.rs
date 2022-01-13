// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::prelude::*;
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

    /// The generic type parameter does not match the TypeId. The expected type name is stored
    /// as context when this error is returned.
    #[displaydoc("Mismatched types: tried to downcast with {0}, but actual type is different")]
    MismatchedType(&'static str),

    /// The payload is missing. This is usually caused by a previous error.
    #[displaydoc("Missing payload")]
    MissingPayload,

    /// An error involving a lock or mutex occurred.
    ///
    /// Check debug logs for potentially more information.
    #[displaydoc("Mutex error")]
    Mutex,

    /// An error occurred while accessing a system resource.
    #[displaydoc("I/O error: {0:?}")]
    #[cfg(feature = "std")]
    Io(std::io::ErrorKind),

    /// An unspecified error occurred, such as a Serde error.
    ///
    /// Check debug logs for potentially more information.
    #[displaydoc("Custom")]
    Custom,
}

/// The error type for ICU4X data provider operations.
///
/// To create one of these, either start with a [`DataErrorKind`] or use [`DataError::custom()`].
///
/// # Example
///
/// Create a NeedsVariant error and attach a data request for context:
///
/// ```no_run
/// # use icu_provider::prelude::*;
/// let req: &DataRequest = unimplemented!();
/// DataErrorKind::NeedsVariant.with_req(req);
/// ```
///
/// Create a named custom error:
///
/// ```
/// # use icu_provider::prelude::*;
/// DataError::custom("This is an example error");
/// ```
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
        write!(f, "ICU4X data error")?;
        if self.kind != DataErrorKind::Custom {
            write!(f, ": {}", self.kind)?;
        }
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
    /// If possible, you should attach context using a `with_` function.
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
    /// Returns a new, empty DataError with kind Custom and a string error message.
    #[inline]
    pub const fn custom(str_context: &'static str) -> Self {
        Self {
            kind: DataErrorKind::Custom,
            key: None,
            str_context: Some(str_context),
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

    /// Logs the data error with the given request, returning an error containing the resource key.
    ///
    /// If the "log_error_context" feature is enabled, this logs the whole request. Either way,
    /// it returns an error with the resource key portion of the request as context.
    pub fn with_req(self, req: &DataRequest) -> Self {
        // Don't write out a log for MissingResourceKey since there is no context to add
        #[cfg(feature = "log_error_context")]
        if self.kind != DataErrorKind::MissingResourceKey {
            log::warn!("{} (request: {})", self, req);
        }
        self.with_key(req.resource_path.key)
    }

    /// Logs the data error with the given context, then return self.
    ///
    /// This does not modify the error, but if the "log_error_context" feature is enabled,
    /// it will print out the context.
    #[cfg(feature = "std")]
    #[cfg_attr(not(feature = "log_error_context"), allow(unused_variables))]
    pub fn with_path(self, path: &impl AsRef<std::path::Path>) -> Self {
        #[cfg(feature = "log_error_context")]
        log::warn!("{} (path: {:?})", self, path.as_ref());
        self
    }

    /// Logs the data error with the given context, then return self.
    ///
    /// This does not modify the error, but if the "log_error_context" feature is enabled,
    /// it will print out the context.
    #[cfg(feature = "std")]
    #[cfg_attr(not(feature = "log_error_context"), allow(unused_variables))]
    #[inline]
    pub fn with_error_context(self, err: &impl std::error::Error) -> Self {
        #[cfg(feature = "log_error_context")]
        log::warn!("{}: {}", self, err);
        self
    }

    /// Logs the data error with the given context, then return self.
    ///
    /// This does not modify the error, but if the "log_error_context" feature is enabled,
    /// it will print out the context.
    #[cfg_attr(not(feature = "log_error_context"), allow(unused_variables))]
    #[inline]
    pub fn with_display_context(self, context: &impl core::fmt::Display) -> Self {
        #[cfg(feature = "log_error_context")]
        log::warn!("{}: {}", self, context);
        self
    }

    /// Logs the data error with the given context, then return self.
    ///
    /// This does not modify the error, but if the "log_error_context" feature is enabled,
    /// it will print out the context.
    #[cfg_attr(not(feature = "log_error_context"), allow(unused_variables))]
    #[inline]
    pub fn with_debug_context(self, context: &impl core::fmt::Debug) -> Self {
        #[cfg(feature = "log_error_context")]
        log::warn!("{}: {:?}", self, context);
        self
    }

    pub(crate) fn for_type<T>() -> DataError {
        DataError {
            kind: DataErrorKind::MismatchedType(core::any::type_name::<T>()),
            key: None,
            str_context: None,
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DataError {}

#[cfg(feature = "serde")]
impl From<crate::serde::Error> for DataError {
    #[cfg_attr(not(feature = "log_error_context"), allow(unused_variables))]
    fn from(e: crate::serde::Error) -> Self {
        #[cfg(feature = "log_error_context")]
        log::warn!("Serde error: {}", e);
        DataError::custom("Serde error")
    }
}

#[cfg(feature = "postcard")]
impl From<postcard::Error> for DataError {
    #[cfg_attr(not(feature = "log_error_context"), allow(unused_variables))]
    fn from(e: postcard::Error) -> Self {
        #[cfg(feature = "log_error_context")]
        log::warn!("Postcard error: {}", e);
        DataError::custom("Postcard error")
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for DataError {
    fn from(e: std::io::Error) -> Self {
        #[cfg(feature = "log_error_context")]
        log::warn!("I/O error: {}", e);
        DataErrorKind::Io(e.kind()).into_error()
    }
}

#[cfg(feature = "std")]
impl<T> From<std::sync::PoisonError<T>> for DataError {
    #[cfg_attr(not(feature = "log_error_context"), allow(unused_variables))]
    fn from(e: std::sync::PoisonError<T>) -> Self {
        #[cfg(feature = "log_error_context")]
        log::warn!("Poison error: {}", e);
        DataErrorKind::Mutex.into_error()
    }
}
