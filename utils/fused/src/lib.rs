// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! # Fused Arithmetic Utilities
//!
//! This crate provides Fused Multiply-Add (FMA)-based high-precision floating-point algorithms
//! to eliminate double rounding and intermediate precision loss in scaling, unit conversions,
//! and range mappings.
//!
//! By using FMA instructions, these algorithms track and compensate for the rounding errors of
//! intermediate calculations (such as multiplication or division) in a single step, ensuring
//! that the final result is rounded exactly once to the nearest representable floating-point number.
//!
//! ## Features
//! - **`no_std` by default**: Fully compatible with embedded environments, with zero heap allocations.
//! - **High Performance**: Compiles to native CPU instructions (such as `vfmadd`/`fmadd`) when the `std` feature is enabled.
//! - **Robust Fallbacks**: Gracefully handles intermediate overflows and underflows using a dynamic retry strategy.
//! - **Ergonomic Invariants**: `Ratio` types that guarantee mathematical validity at runtime.
//!
//! ## Comparison with Other Crates
//!
//! | Feature | `fused` | `twofloat` | `metallic` | `accurate` |
//! | :--- | :--- | :--- | :--- | :--- |
//! | **Target Types** | Primitive `f32`/`f64` | Custom `TwoFloat` | Custom `DoubleDouble` | Primitive arrays |
//! | **Allocation Overhead** | Zero | Zero | Zero | Zero / Heap |
//! | **Execution Overhead** | Extremely Low (few FMAs) | Medium (emulated DD) | High (multi-word DD) | High (array passes) |
//! | **Primary Use Case** | Fused operations (`a*b/c`) | Double-precision math | Quad-precision math | Summation & Dot products |
//! | **`no_std` Support** | Yes | Yes | Yes | Yes |
//!
//! ### Summary of Differences:
//! - **`twofloat` & `metallic`**: These crates implement general-purpose double-double/quad-double arithmetic by wrapping values in custom structs. While they offer higher precision (e.g., 106 bits for double-double), they incur significant performance overhead for every basic operation. In contrast, `fused` operates directly on primitive `f32`/`f64` values and targets specific fused patterns to achieve single-rounded results with near-zero overhead.
//! - **`accurate`**: This crate focuses on compensated summation (like Kahan summation) and dot products across slices. It does not provide fused scaling/division algorithms like `a * b / c` or `a / (b * c)`.
//!
//! ## Preventing Double-Rounding: The `0.1 * 0.1 / 0.1` Case
//!
//! A classic double-rounding error occurs when computing $\frac{x \times x}{x}$ for $x = 0.1$.
//! Mathematically, $\frac{0.1 \times 0.1}{0.1} = 0.1$ exactly.
//!
//! However, using naive floating-point arithmetic:
//! 1. The intermediate product `0.1 * 0.1` cannot be represented exactly in binary and is rounded to the nearest `f64`, yielding:
//!    $$0.010000000000000001942890293094023945741355419158935546875 \quad (\text{hex: } \texttt{0x1.47ae147ae147cp-7})$$
//!    which is slightly larger than the true mathematical product $0.01$.
//! 2. When we divide this rounded intermediate value by `0.1` naive floating-point rounds it a second time, which rounds *up* to:
//!    $$0.10000000000000001942890293094023945741355419158935546875 \quad (\text{hex: } \texttt{0x1.999999999999bp-4})$$
//!    This is `0.10000000000000002`—one Unit in the Last Place (ULP) away from the correct value.
//!
//! Using `f64_mul_div(0.1, 0.1, 0.1)` tracks the exact rounding error of the intermediate multiplication and applies a correction to the division step, yielding the single-rounded mathematical result:
//!    $$0.1000000000000000055511151231257827021181583404541015625 \quad (\text{hex: } \texttt{0x1.999999999999ap-4})$$
//!    which is exactly `0.1`.
//!
//! ## The Counterintuitive `0.3 * 3` Case
//!
//! A common point of confusion is the expression `0.3 * 3`. Users often expect this to round to `0.9`, but naive floating-point arithmetic yields `0.8999999999999999`.
//!
//! 1. The representable float `0.3_f64` is slightly smaller than the real number $0.3$:
//!    $$0.299999999999999988897769753748434595763683319091796875$$
//! 2. The exact mathematical product of this representable float and $3$ is:
//!    $$0.899999999999999966693309261245303787291049957275390625$$
//! 3. This product is strictly closer to `0.8999999999999999` than it is to `0.9`.
//! 4. Therefore, under IEEE 754 rules, the single-rounded result of `0.3_f64 * 3.0` is indeed `0.8999999999999999`.
//!
//! Since division by $1.0$ is exact, naive `0.3 * 3.0 / 1.0` and `f64_mul_div(0.3, 3.0, 1.0)` both correctly yield `0.8999999999999999`. This case does not suffer from double-rounding, but it highlights the importance of understanding representational limits.
//!
//! ## Compilation Warning
//! > [!WARNING]
//! > Do **NOT** compile crates using `fused` with unsafe floating-point optimizations (such as fast-math reassociation or contraction flags: `-Cllvm-args=-enable-unsafe-fp-math` or `--ffast-math`). These compiler options allow the compiler to re-associate and contract floating-point expressions, which will violate the strict IEEE 754 rules required by the error-compensation algorithms and break the correctness of this crate.
//!
//! ## Examples
//!
//! ### Fused Multiply-Divide (Algorithm A)
//! ```rust
//! use fused::f64_mul_div;
//!
//! // Compute 0.1 * 0.1 / 0.1 with a single rounding
//! let result = f64_mul_div(0.1, 0.1, 0.1);
//! assert_eq!(result, 0.1);
//!
//! // Naive arithmetic yields 0.10000000000000002
//! let naive = 0.1 * 0.1 / 0.1;
//! assert_eq!(naive, 0.10000000000000002);
//! ```
//!
//! ### Fused Multiply-Divide-Add (Algorithm B)
//! ```rust
//! use fused::f64_mul_div_add;
//!
//! // Scaling with offset
//! let result = f64_mul_div_add(2.5, 4.0, 3.0, 1.0);
//! // Naive double rounded: (2.5 * 4) / 3 + 1 = 3.3333333333333335 + 1 = 4.333333333333334
//! // Fused single rounded: 13 / 3 rounded exactly once to nearest float = 4.333333333333333
//! assert_eq!(result, 4.333333333333333);
//! ```
//!
//! ### Reciprocal Division (Algorithm C)
//! ```rust
//! use fused::f64_div_mul;
//!
//! // Compute 1.0 / (3.0 * 7.0)
//! let result = f64_div_mul(1.0, 3.0, 7.0);
//! assert_eq!(result, 1.0 / 21.0);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

mod div_mul;
mod fallback;
mod mul_div;
mod mul_div_add;
mod ratio;

pub use div_mul::{f32_div_mul, f64_div_mul};
pub use mul_div::{f32_mul_div, f64_mul_div};
pub use mul_div_add::{f32_mul_div_add, f64_mul_div_add};
pub use ratio::{RatioF32, RatioF64};
