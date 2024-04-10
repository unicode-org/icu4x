// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::any;
use core::fmt;

use crate::__zerovec_internal_reexport::boxed::Box;
use alloc::format;
use alloc::string::String;
use schemars::gen::SchemaGenerator;

use schemars::schema::InstanceType;
use schemars::schema::Schema;
use schemars::schema::SchemaObject;
use schemars::JsonSchema;

/// A generic error type to be used for decoding slices of ULE types
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ZeroVecError {
    /// Attempted to parse a buffer into a slice of the given ULE type but its
    /// length was not compatible
    InvalidLength { ty: &'static str, len: usize },
    /// The byte sequence provided for `ty` failed to parse correctly
    ParseError { ty: &'static str },
    /// The byte buffer was not in the appropriate format for VarZeroVec
    VarZeroVecFormatError,
}

impl fmt::Display for ZeroVecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match *self {
            ZeroVecError::InvalidLength { ty, len } => {
                write!(f, "Invalid length {len} for slice of type {ty}")
            }
            ZeroVecError::ParseError { ty } => {
                write!(f, "Could not parse bytes to slice of type {ty}")
            }
            ZeroVecError::VarZeroVecFormatError => {
                write!(f, "Invalid format for VarZeroVec buffer")
            }
        }
    }
}
impl JsonSchema for ZeroVecError {
    fn schema_name() -> String {
        format!("ZeroVecError")
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            metadata: Some(Box::new(schemars::schema::Metadata {
                description: Some(
                    "Represents variants: InvalidLength, ParseError, VarZeroVecFormatError.".into(),
                ),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}

impl ZeroVecError {
    /// Construct a parse error for the given type
    pub fn parse<T: ?Sized + 'static>() -> ZeroVecError {
        ZeroVecError::ParseError {
            ty: any::type_name::<T>(),
        }
    }

    /// Construct an "invalid length" error for the given type and length
    pub fn length<T: ?Sized + 'static>(len: usize) -> ZeroVecError {
        ZeroVecError::InvalidLength {
            ty: any::type_name::<T>(),
            len,
        }
    }
}

#[cfg(feature = "std")]
impl ::std::error::Error for ZeroVecError {}
