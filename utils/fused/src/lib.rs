// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! # `fused`
//!
//! High-precision, FMA-based floating-point arithmetic algorithms designed for unit conversion,
//! scaling, and offset adjustments in ICU4X.
//!
//! This crate provides specialized, academically rigorous, and highly optimized algorithms
//! that leverage **Fused Multiply-Add (FMA)** hardware instructions to eliminate intermediate
//! rounding errors, achieving near-infinite precision with single-rounded results.
//!
//! ## Core Algorithms
//!
//! The crate implements three fundamental operations:
//! 1. [`f64_mul_div`]: Computes \( \frac{a \cdot num}{den} \) with a single rounding.
//! 2. [`f64_mul_div_add`]: Computes \( \frac{a \cdot num}{den} + offset \) with a single rounding.
//! 3. [`f64_div_mul`]: Computes \( \frac{a}{b \cdot c} \) with a single rounding.
//!
//! All algorithms are `#![no_std]` compatible, perform zero allocations, and feature
//! zero-cost fallbacks that safely handle non-finite inputs, divisions by zero, and
//! intermediate overflows.
//!
//! ## Comparative Analysis with Existing Crates
//!
//! To understand the design decisions of the `fused` crate, it is helpful to compare it
//! with other popular high-precision or compensated floating-point libraries in the Rust ecosystem:
//!
//! ### 1. `twofloat`
//! * **Concept:** Implements double-double arithmetic (representing numbers as a pair of `f64` values: a head and a tail, yielding ~106 bits of precision).
//! * **Comparison:** `twofloat` is excellent for general-purpose high-precision calculations. However, it requires wrapping all floats in a custom `TwoFloat` type, and executing operations (+, -, *, /) requires maintaining the double-word state continuously. This incurs significant runtime overhead, extra branching, and code complexity.
//! * **The `fused` Advantage:** `fused` does *not* introduce a new numeric type. It operates entirely on native, standard `f64` types. It uses double-word arithmetic and FMA internally to compute and apply error corrections, but the inputs and final outputs remain standard `f64` values. It is a lightweight, zero-overhead solution for specific fused algebraic expressions.
//!
//! ### 2. `accurate`
//! * **Concept:** Implements compensated algorithms specifically for summation and dot products of collections (e.g., Kahan summation, Ogita-Rump-Oishi compensated summation).
//! * **Comparison:** `accurate` is highly optimized for reducing accumulation errors across large arrays or vectors. However, it does not address core, three-term fused algebraic operations on individual scalar values (such as multiply-divide or offset addition).
//! * **The `fused` Advantage:** `fused` targets scalar-level three-term fused equations. It is designed specifically for unit conversion engines where single scalar values must be scaled precisely without double rounding.
//!
//! ### 3. `metallic`
//! * **Concept:** A multi-precision or arbitrary-precision floating-point library (often wrapping MPFR or implementing multi-precision in pure Rust).
//! * **Comparison:** `metallic` offers arbitrary precision (e.g., hundreds or thousands of bits) but is heavy, relies on heap allocation (`std::alloc` dependent), and is unsuitable for performance-critical or `#![no_std]` embedded environments.
//! * **The `fused` Advantage:** `fused` is `#![no_std]`, dependency-free (except for `core_maths` in no-std), performs no allocations, and compiles down directly to a few hardware-accelerated FMA instructions, running at CPU-native speeds.
//!
//! ## Hardware Acceleration & MSRV
//!
//! This crate is fully `#![no_std]` compatible. On CPUs with native hardware support for FMA
//! (such as x86_64 with AVX2, and AArch64 with NEON), the FMA operations are compiled to single,
//! high-performance instructions. On platforms lacking hardware FMA, the crate automatically
//! falls back to software polyfills provided by the `core_maths` crate, ensuring correctness
//! across all platforms.

#![no_std]

mod div_mul;
mod mul_div;
mod mul_div_add;

pub use div_mul::f64_div_mul;
pub use mul_div::f64_mul_div;
pub use mul_div_add::f64_mul_div_add;
