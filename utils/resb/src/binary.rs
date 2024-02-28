// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The `binary` module provides a `serde` deserializer for the binary resource
//! bundle format as well as a means of writing resource bundles to the binary
//! file format.

mod deserializer;
pub use self::deserializer::from_bytes;

#[cfg(feature = "serialize")]
mod serializer;
#[cfg(feature = "serialize")]
pub use self::serializer::Serializer;

use core::fmt;

/// The `BinHeader` struct represents the in-memory layout of a binary resource
/// bundle header.
#[repr(C)]
struct BinHeader {
    /// The size of the header in bytes, padded such that it is divisible by 16.
    size: u16,

    /// A magic word. See [`MAGIC_WORD`].
    magic: [u8; 2],

    /// Information on the representation of data in the binary bundle.
    repr_info: BinReprInfo,
}

/// The `BinReprInfo` struct represents the in-memory layout of a binary
/// resource bundle's representation specifiers. These data describe the
/// parameters necessary for correctly reading a bundle file.
#[repr(C)]
struct BinReprInfo {
    /// The size of the representation info in bytes.
    size: u16,

    /// Reserved. Always 0.
    reserved_word: u16,

    /// The endianness of values in the file. `0` for little endian and `1` for
    /// big endian.
    endianness: Endianness,

    /// The character set family in which key strings are represented.
    charset_family: CharsetFamily,

    /// The size of a character in a string resource in bytes. May be 1, 2, or
    /// 4.
    size_of_char: u8,

    /// Reserved. Always 0.
    reserved_byte: u8,

    /// The data format identifier. Always `b"ResB"`.
    data_format: [u8; 4],

    /// The format version used for laying out the bundle.
    format_version: FormatVersion,

    /// The version of the data in the file. This is `[1, 4, 0, 0]` in all known
    /// versions of ICU4C's `genrb`.
    data_version: [u8; 4],
}

/// The `BinIndex` struct represents details of the written bundle.
///
/// The index is present from [`FormatVersion::V1_1`] on.
#[allow(dead_code)]
struct BinIndex {
    /// The number of 32-bit fields written in the index, including the field
    /// count.
    field_count: u32,

    /// The offset of the end of the key block in 32-bit values from the
    /// beginning of the body.
    keys_end: u32,

    /// The offset of the end of the resources block in 32-bit values from the
    /// beginning of the body.
    resources_end: u32,

    /// The offset of the end of the bundle in 32-bit values from the beginning
    /// of the body.
    ///
    /// In all versions through [`FormatVersion::V3_0`], this is always the same
    /// as `resources_end`.
    bundle_end: u32,

    /// The number of entries in the largest table in the bundle.
    largest_table_entry_count: u32,

    /// Attributes describing resolution of external resources.
    ///
    /// Present from [`FormatVersion::V1_2`] on.
    bundle_attributes: Option<u32>,

    /// The offset of the end of the 16-bit data block in 32-bit values from the
    /// beginning of the body.
    ///
    /// Present from [`FormatVersion::V2_0`] on.
    data_16_bit_end: Option<u32>,

    /// The resource pool bundle checksum.
    ///
    /// Present from [`FormatVersion::V2_0`] on when the bundle either is a pool
    /// bundle or uses a pool bundle for sharing resources.
    pool_checksum: Option<u32>,
}

/// Adds convenience properties to an `enum` represented as a primitive type,
/// including conversions to and from the primitive type.
macro_rules! primitive_enum {
    ($type:ty, $(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$variant_meta:meta])* $variant:ident = $value:expr,)*
    }) => {
        // At a minimum, `$meta` is needed to allow for doc comments on enums
        // created via this macro.
        $(#[$meta])*
        #[repr($type)]
        $vis enum $name {
            // At a minimum, `$variant_meta` is needed to allow for doc comments
            // on variants created via this macro.
            $($(#[$variant_meta])* $variant = $value,)*
        }

        impl From<$name> for $type {
            fn from(v: $name) -> Self {
                v as $type
            }
        }

        impl TryFrom<$type> for $name {
            type Error = BinaryDeserializerError;

            fn try_from(value: $type) -> Result<Self, Self::Error> {
                match value {
                    $(x if x == $name::$variant as $type => Ok($name::$variant),)*
                    _ => Err(BinaryDeserializerError::invalid_data(
                        concat!("unrecognized value for ", stringify!($name))
                    )),
                }
            }
        }
    }
}

primitive_enum!(
    u8,
    /// The endianness used to write a resource bundle.
    #[derive(Clone, Copy, Debug, PartialEq)]
    enum Endianness {
        Little = 0,
        Big = 1,
    }
);

primitive_enum!(
    u8,
    /// A family of character sets used to represent the characters of key strings.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum CharsetFamily {
        /// The ASCII family of character sets, such as ASCII, latin1, and
        /// UTF-8.
        Ascii = 0,

        /// The EBCDIC family of character sets, such as EBCDIC and UTF-EBCDIC.
        ///
        /// The EBCDIC family is currently unsupported by this crate both for
        /// serialization and deserialization of binary bundles.
        Ebcdic = 1,
    }
);

primitive_enum!(
    u16,
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    /// The type of a resource representation within a binary resource bundle.
    ///
    /// The representation is distinct from the resource type as presented to
    /// consumers. Some resource types may have multiple possible
    /// representations, depending on the [`FormatVersion`] and—in the case of
    /// collections—number and type of their constituent resources.
    enum ResourceReprType {
        /// A string resource. Not yet supported.
        _String = 0,

        /// A raw binary resource.
        ///
        /// Consists of a 32-bit length value `n` followed by `n` arbitrary
        /// bytes.
        Binary = 1,

        /// A table resource for bundles with fewer than `0x1_0000` keys.
        ///
        /// Consists of a 16-bit length value `n` followed by `n` 16-bit key
        /// offsets from the beginning of the key block, 0 or 16 bits of padding
        /// (in order to align the table representation so far to a 32-bit
        /// boundary), and `n` 32-bit resource descriptors. For details on the
        /// representation of resource descriptors, see [`ResDescriptor`].
        ///
        /// The `i`th entry in the resulting table is a a pair of the `i`th key
        /// and the `i`th resource.
        Table = 2,

        /// An alias resource. Not yet supported.
        _Alias = 3,

        /// A table resource for bundles with `0x1_0000` or more keys. Not yet
        /// supported.
        _Table32 = 4,

        /// A 16-bit table resource. Not yet supported.
        Table16 = 5,

        /// A 16-bit string resource for [`FormatVersion::V2_0`] and later.
        ///
        /// Consists of a UTF-16 string with length marked in one of the
        /// following ways:
        ///
        /// - For strings of length `[1..40]` characters, there is no length
        ///   marker and the string must be null-terminated (i.e., by two `0`
        ///   bytes in a row).
        ///
        /// - For strings of length `n` in the range `(40..0x3ef)`, the string
        ///   is preceded by a single UTF-16 low surrogate composed as
        ///   `0xdc00 & n`.
        ///
        /// - For strings of length `n` in the range `[0x3ef..0x10_0000)`, the
        ///   string is preceded by a length marker consisting of a UTF-16 low
        ///   surrogate followed by a 16-bit value, composed as
        ///   `[0xdfef + (n >> 0x10), n & 0xffff]`.
        ///
        /// - For strings of length `n` in the range `[0x10_0000,
        ///   0x1_0000_0000)`, the string is preceded by a length marker
        ///   consisting of a UTF-16 low surrogate followed by two 16-bit length
        ///   values, composed as `[0xdfff, n >> 0x10, n & 0xffff]`.
        ///
        /// Strings of greater length than those described above may not be
        /// stored in binary bundles.
        ///
        /// These length markers can be reliably detected, as UTF-16 low
        /// surrogates may not legally appear without a preceding high surrogate
        /// in a UTF-16 string.
        StringV2 = 6,

        /// A 28-bit integer resource.
        ///
        /// Consists solely of the resource descriptor with the 28 bits of the
        /// integer in place of an offset.
        ///
        /// The signedness of 28-bit integers is not indicated in the resource
        /// bundle itself. Consumers are expected to explicitly request a signed
        /// or unsigned integer. In order to
        Int = 7,

        /// A general array resource.
        ///
        /// Consists of a 32-bit length value `n` followed by `n` 32-bit
        /// resource descriptors. For more details on the representation of
        /// resource descriptors, see [`ResDescriptor`].
        Array = 8,

        /// A 16-bit array resource.
        ///
        /// Consists of a 16-bit length value `n` followed by `n` 16-bit offsets
        /// from the beginning of the 16-bit data block.
        ///
        /// As of [`FormatVersion::V3_0`], only `StringV2` representations can
        /// be fully stored in the 16-bit data block. As such, only `StringV2`
        /// resources can appear in an `Array16`.
        Array16 = 9,

        /// An integer array resource.
        ///
        /// Consists of a 32-bit length value `n` followed by `n` 32-bit integer
        /// values.
        ///
        /// Note that these are not integer _resources_, but rather full 32-bit
        /// integers.
        IntVector = 14,
    }
);

/// A `FormatVersion` represents a specific binary file format used for
/// representing resource bundles.
///
/// A partial [specification] of each format version is present in the ICU4C
/// source code.
///
/// [specification]: https://github.com/unicode-org/icu/blob/main/icu4c/source/common/uresdata.h
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
// We use `u32` representation to enforce correct sizing for structs containing
// a `FormatVersion`.
//
// Note that order of declaration is important for partial ordering.
#[repr(u32)]
enum FormatVersion {
    V1_0,
    V1_1,
    V1_2,
    V1_3,
    V2_0,
    V3_0,
}

/// The `ResDescriptor` struct represents a typed pointer to a resource body
/// within a binary resource bundle.
///
/// It is represented within the binary bundle as a 4-bit resource type in the
/// most significant nibble of a 32-bit integer with a 28-bit unsigned offset
/// in the remaining bits. The offset is interpreted as a count of 32-bit
/// values from the start of the body.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ResDescriptor {
    resource_type: ResourceReprType,
    value: u32,
}

impl ResDescriptor {
    /// Makes a new resource descriptor with the given type and 28-bit value.
    pub const fn new(resource_type: ResourceReprType, value: u32) -> Self {
        Self {
            resource_type,
            value,
        }
    }

    /// Makes a new resource descriptor with the given type and no body.
    pub const fn _new_empty(resource_type: ResourceReprType) -> Self {
        Self::new(resource_type, 0)
    }

    /// Returns `true` if the described resource is empty.
    pub fn is_empty(&self) -> bool {
        self.value == 0
    }

    /// Gets the offset to the described 16-bit resource in bytes.
    ///
    /// The type of the resource representation is not verified. Consumers are
    /// expected to call the function appropriate to the resource type they are
    /// querying.
    fn value_as_16_bit_offset(&self) -> usize {
        // When the value of a resource descriptor is an offset, it is counted
        // in units dependent on the resource type (16-bit values for 16-bit
        // resources, 32-bit values for 32-bit resources). Translate that into
        // bytes for consumers.
        (self.value as usize) * core::mem::size_of::<u16>()
    }

    /// Gets the offset to the described 32-bit resource in bytes.
    ///
    /// The type of the resource representation is not verified. Consumers are
    /// expected to call the function appropriate to the resource type they are
    /// querying.
    fn value_as_32_bit_offset(&self) -> usize {
        // When the value of a resource descriptor is an offset, it is counted
        // in units dependent on the resource type (16-bit values for 16-bit
        // resources, 32-bit values for 32-bit resources). Translate that into
        // bytes for consumers.
        (self.value as usize) * core::mem::size_of::<u32>()
    }

    /// Gets the value of the resource descriptor as a signed integer.
    ///
    /// The type of the resource representation is not verified. Consumers are
    /// expected to call the function appropriate to the resource type they are
    /// querying.
    fn value_as_signed_int(&self) -> i32 {
        ((self.value as i32) << 4) >> 4
    }

    /// Gets the value of the resource descriptor as an unsigned integer.
    ///
    /// The type of the resource representation is not verified. Consumers are
    /// expected to call the function appropriate to the resource type they are
    /// querying.
    fn value_as_unsigned_int(&self) -> u32 {
        self.value
    }

    /// Gets the resource type of the described resource.
    pub fn resource_type(&self) -> ResourceReprType {
        self.resource_type
    }
}

/// The `Error` type provides basic error handling for deserialization of binary
/// resource bundles.
#[derive(Clone, Copy, Debug)]
pub struct BinaryDeserializerError {
    kind: ErrorKind,
    message: &'static str,
}

impl BinaryDeserializerError {
    fn invalid_data(message: &'static str) -> Self {
        Self {
            kind: ErrorKind::InvalidData,
            message,
        }
    }

    fn resource_type_mismatch(message: &'static str) -> Self {
        Self {
            kind: ErrorKind::ResourceTypeMismatch,
            message,
        }
    }

    fn unsupported_format(message: &'static str) -> Self {
        Self {
            kind: ErrorKind::UnsupportedFormat,
            message,
        }
    }

    fn unknown(message: &'static str) -> Self {
        Self {
            kind: ErrorKind::Unknown,
            message,
        }
    }
}

impl fmt::Display for BinaryDeserializerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = match self.kind {
            ErrorKind::InvalidData => "Invalid resource bundle data",
            ErrorKind::ResourceTypeMismatch => "Resource did not match expected data type",
            ErrorKind::UnsupportedFormat => "Unsupported resource bundle format",
            ErrorKind::Unknown => "Unknown error",
        };

        write!(f, "{description}: {}", self.message)
    }
}

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
enum ErrorKind {
    ResourceTypeMismatch,
    InvalidData,
    UnsupportedFormat,

    Unknown,
}
