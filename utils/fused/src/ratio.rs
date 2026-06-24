// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Ratio types for high-precision scaling operations.

/// A representation of a 64-bit floating-point ratio (numerator / denominator).
///
/// Guarantees that both numerator and denominator are finite, and the denominator is non-zero.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "RatioF64Unchecked"))]
pub struct RatioF64 {
    numerator: f64,
    denominator: f64,
}

impl RatioF64 {
    /// Creates a new `RatioF64`.
    ///
    /// Returns `None` if either `numerator` or `denominator` is not finite,
    /// or if `denominator` is zero.
    #[inline]
    pub fn new(numerator: f64, denominator: f64) -> Option<Self> {
        if numerator.is_finite() && denominator.is_finite() && denominator != 0.0 {
            Some(Self {
                numerator,
                denominator,
            })
        } else {
            None
        }
    }

    /// Returns the numerator of the ratio.
    #[inline]
    pub const fn numerator(self) -> f64 {
        self.numerator
    }

    /// Returns the denominator of the ratio.
    #[inline]
    pub const fn denominator(self) -> f64 {
        self.denominator
    }

    /// Returns the reciprocal of the ratio (denominator / numerator).
    ///
    /// Returns `None` if the new denominator (the original numerator) is zero.
    #[inline]
    pub fn reciprocal(self) -> Option<Self> {
        Self::new(self.denominator, self.numerator)
    }
}

/// Unchecked helper struct for safe `serde` deserialization.
#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
struct RatioF64Unchecked {
    numerator: f64,
    denominator: f64,
}

#[cfg(feature = "serde")]
impl TryFrom<RatioF64Unchecked> for RatioF64 {
    type Error = &'static str;

    #[inline]
    fn try_from(unchecked: RatioF64Unchecked) -> Result<Self, Self::Error> {
        Self::new(unchecked.numerator, unchecked.denominator)
            .ok_or("invalid ratio: numerator and denominator must be finite, and denominator must be non-zero")
    }
}

/// A representation of a 32-bit floating-point ratio (numerator / denominator).
///
/// Guarantees that both numerator and denominator are finite, and the denominator is non-zero.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "RatioF32Unchecked"))]
pub struct RatioF32 {
    numerator: f32,
    denominator: f32,
}

impl RatioF32 {
    /// Creates a new `RatioF32`.
    ///
    /// Returns `None` if either `numerator` or `denominator` is not finite,
    /// or if `denominator` is zero.
    #[inline]
    pub fn new(numerator: f32, denominator: f32) -> Option<Self> {
        if numerator.is_finite() && denominator.is_finite() && denominator != 0.0 {
            Some(Self {
                numerator,
                denominator,
            })
        } else {
            None
        }
    }

    /// Returns the numerator of the ratio.
    #[inline]
    pub const fn numerator(self) -> f32 {
        self.numerator
    }

    /// Returns the denominator of the ratio.
    #[inline]
    pub const fn denominator(self) -> f32 {
        self.denominator
    }

    /// Returns the reciprocal of the ratio (denominator / numerator).
    ///
    /// Returns `None` if the new denominator (the original numerator) is zero.
    #[inline]
    pub fn reciprocal(self) -> Option<Self> {
        Self::new(self.denominator, self.numerator)
    }
}

/// Unchecked helper struct for safe `serde` deserialization.
#[cfg(feature = "serde")]
#[derive(serde::Deserialize)]
struct RatioF32Unchecked {
    numerator: f32,
    denominator: f32,
}

#[cfg(feature = "serde")]
impl TryFrom<RatioF32Unchecked> for RatioF32 {
    type Error = &'static str;

    #[inline]
    fn try_from(unchecked: RatioF32Unchecked) -> Result<Self, Self::Error> {
        Self::new(unchecked.numerator, unchecked.denominator)
            .ok_or("invalid ratio: numerator and denominator must be finite, and denominator must be non-zero")
    }
}
