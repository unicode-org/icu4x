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
//! ## Mathematical Precision: Opportunities & Limits
//!
//! High-precision fused operations are designed to eliminate intermediate rounding errors, but they
//! cannot bypass the fundamental representation limits of the IEEE 754 binary format. Understanding
//! this distinction is key to using this crate effectively.
//!
//! Below are two classic examples that illustrate the power and the limits of FMA-compensated arithmetic.
//!
//! ### 1. Correcting Double-Rounding: `0.1 * 0.1 / 0.1`
//!
//! Mathematically, $\frac{0.1 \times 0.1}{0.1} = 0.1$.
//!
//! - **Standard Float Arithmetic (`(0.1 * 0.1) / 0.1`):** Yields `0.09999999999999999`.
//! - **Compensated Arithmetic (`f64_mul_div(0.1, 0.1, 0.1)`):** Yields `0.1` (exactly representable $0.1$ float).
//!
//! **Why standard arithmetic fails:**
//! 1. The input `0.1` is not exactly representable in binary. It is rounded to the nearest float:
//!    $$0.1_{\text{float}} = 0.1000000000000000055511151231257827021181583404541015625$$
//! 2. In standard arithmetic, the intermediate product $0.1_{\text{float}} \times 0.1_{\text{float}}$ is computed and immediately rounded to a 53-bit float:
//!    $$P_{\text{rounded}} = 0.01000000000000000020816681711721685132943093776702880859375$$
//!    Notice that $P_{\text{rounded}} is slightly *smaller* than the true mathematical product $0.1_{\text{float}}^2$. This is the first rounding.
//! 3. We then divide $P_{\text{rounded}}$ by $0.1_{\text{float}}$. Because the numerator was rounded down, the quotient is slightly less than $0.1_{\text{float}}$.
//! 4. Finally, this quotient is rounded to the nearest float, which drops down to `0.09999999999999999`. This is the second rounding (double-rounding flaw).
//!
//! **How `fused` corrects this:**
//! The `f64_mul_div` function uses FMA to extract the exact mathematical error of the multiplication. It retains the full double-width representation of the intermediate product ($0.1_{\text{float}}^2$) and applies it during the division. This ensures the result is rounded **exactly once** at the very end, yielding the correct float `0.1`.
//!
//! ---
//!
//! ### 2. The Inescapable Midpoint: `0.3 * 3.0 / 1.0`
//!
//! Mathematically, $\frac{0.3 \times 3.0}{1.0} = 0.9$.
//!
//! - **Standard Float Arithmetic:** Yields `0.8999999999999999`.
//! - **Compensated Arithmetic:** **Also** yields `0.8999999999999999`.
//!
//! **Why even high-precision math cannot "fix" this:**
//! 1. The input `0.3` is rounded to:
//!    $$0.3_{\text{float}} = 0.299999999999999988897769753748434595763683319091796875$$
//!    which is slightly less than $0.3$.
//! 2. When we multiply $0.3_{\text{float}}$ by $3.0$ (which is exactly representable), the exact mathematical product in real numbers is:
//!    $$P_{\text{exact}} = 0.8999999999999999666933092612453037872910501956939697265625$$
//! 3. We must round $P_{\text{exact}}$ to the nearest 53-bit float. The two adjacent representable floats are:
//!    - $f_{\text{low}} = 0.899999999999999911182158029987476766109466552734375$
//!    - $f_{\text{high}} = 0.90000000000000002220446049250313080847263336181640625$
//! 4. The absolute distances are mathematically identical:
//!    $$|P_{\text{exact}} - f_{\text{low}}| = |P_{\text{exact}} - f_{\text{high}}| = 5.551115123125783 \times 10^{-17}$$
//!    The exact product lands **exactly on the midpoint** between the two representable floats.
//! 5. Under the IEEE 754 round-to-nearest-even rule, the tie is broken by selecting the float with an even significand (ending in `0` in binary), which is $f_{\text{low}}$.
//!
//! Thus, even with infinite intermediate precision, the mathematically correct rounded result of the represented inputs is `0.8999999999999999`. The algorithm is behaving with perfect fidelity to the IEEE 754 standard.
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
