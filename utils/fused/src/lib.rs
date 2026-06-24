// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! # `fused`: High-Precision Fused Floating-Point Algorithms
//!
//! This crate provides robust, high-precision fused floating-point algorithms for three-term
//! mathematical operations, designed to achieve near-exact rounding ($\approx 0.5$ ULP) on
//! hardware supporting Fused Multiply-Add (FMA) with virtually zero performance overhead.
//!
//! ## Core Design & Philosophy
//!
//! Standard floating-point arithmetic introduces rounding errors at every step. For operations
//! like $(a \times \text{num}) / \text{den}$, the intermediate product is rounded, and then the
//! quotient is rounded again, potentially compounding the error (the table-maker's dilemma).
//!
//! The `fused` crate solves this by:
//! 1. **FMA-Based Error Extraction**: Using the Fused Multiply-Add instruction, we extract the
//!    *exact* mathematical error of intermediate products and division remainders.
//! 2. **Analytical Compensation**: Combining these exact error terms analytically to apply
//!    corrections to the final result, achieving double-word precision internally.
//! 3. **Zero-Cost Fallback**: Implementing a branchless hot path. If the compensated result is
//!    not finite (due to intermediate overflow, underflow, or special inputs like $\infty$/$\text{NaN}$),
//!    the algorithm immediately falls back to the standard IEEE 754 operation. This ensures
//!    identical behavior to standard Rust floats for all edge cases while maintaining peak speed.
//!
//! ---
//!
//! ## Comparative Research
//!
//! High-precision arithmetic in Rust is typically addressed by several existing crates. The table
//! below compares `fused` with other prominent solutions:
//!
//! | Crate | Primary Focus | Precision | Performance Profile | `#![no_std]` Support | Hardware FMA Optimization |
//! |---|---|---|---|---|---|
//! | **`twofloat`** | Double-double arithmetic | ~106 bits (double `f64`) | Moderate (5-10x slower; carries two floats for all ops) | Yes | Indirect (does not target fused 3-term ops) |
//! | **`accurate`** | Compensated sum/dot product | Accurate summation | Excellent for arrays, N/A for 3-term scaling | Yes | No |
//! | **`metallic`** | Faithfully rounded math functions | ~53 bits (faithful rounding) | Fast, focused on transcendentals (`sin`, `log`) | Yes | Yes (for polynomial approximation) |
//! | **`fused` (This)** | Targeted three-term fused ops | $\approx 53$ bits (near-exact rounding) | **Ultra-Fast** (near-native speed, zero-cost fallback) | **Yes** | **Natively Optimized** (designed around hardware FMA) |
//!
//! ### Deep Dive: Why `fused`?
//!
//! - **`twofloat`**: Represents all real numbers as a sum of two `f64` values. While extremely
//!   precise and useful for general-purpose high-precision math, it incurs a heavy performance
//!   tax because every addition, multiplication, and division must manipulate both words. `fused`
//!   only uses double-word representations *transiently* within the algorithm to compute a single,
//!   perfectly rounded `f64` output, avoiding the overhead of a persistent double-float wrapper.
//! - **`accurate`**: Specifically targets the accumulation of errors in large datasets (e.g.,
//!   Kahan/Neumaier summation, dot products). It is not designed to solve the rounding errors of
//!   individual three-term operations like scaling and division.
//! - **`metallic`**: A math library written from scratch to replace `libm`. It focuses on ensuring
//!   transcendental functions (like trigonometric or exponential functions) are faithfully rounded
//!   to less than 1 ULP. It does not provide the fused scaling operations implemented here.
//! - **`fused`**: Fills a critical niche in high-precision engineering (such as unit conversion,
//!   coordinate scaling, and time calculations). It targets specific, frequently used three-term
//!   patterns:
//!   - Fused Multiply-Divide: `(a * num) / den`
//!   - Fused Multiply-Divide-Add: `(a * num) / den + offset`
//!   - Fused Reciprocal Division: `a / (b * c)`
//!   By focusing strictly on these patterns, it achieves optimal performance on modern CPU pipelines.
//!
//! ---
//!
//! ## Hardware Requirements
//!
//! For maximum performance, this crate should be compiled with target features enabling hardware FMA:
//! ```bash
//! RUSTFLAGS="-C target-feature=+fma" cargo build
//! ```
//! If hardware FMA is not available, the crate automatically falls back to a software emulator
//! provided by `core_maths`, which remains accurate but will be slower.

#![no_std]

mod div_mul;
mod mul_div;
mod mul_div_add;

pub use div_mul::f64_div_mul;
pub use mul_div::f64_mul_div;
pub use mul_div_add::f64_mul_div_add;
