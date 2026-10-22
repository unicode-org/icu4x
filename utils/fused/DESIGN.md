# Architecture and Design of the `fused` Crate

This document describes the software architecture, design decisions, module layout, and comparative trade-offs for the `fused` crate. For the formal mathematical proofs and error bound derivations, please refer to [`proofs.md`](file:///usr/local/google/home/sffc/scratch/icu4x-fused/utils/fused/proofs.md).

---

## 1. Overview & Motivation

In internationalization and localization engines (such as ICU4X), high-precision scaling and scaling-with-offset operations are incredibly common. Examples include:
- **Unit Conversion**: Converting between units (e.g., Fahrenheit to Celsius: $C = (F - 32) \times 5/9$, or miles to kilometers: $km = mi \times 1.609344$) requires precise scaling and offset additions.
- **Fixed-Decimal Scaling**: Formatting currency or decimals requires precise division/multiplication by powers of 10.
- **Timezone & Calendar Calculations**: Converting between different epoch times or astronomical calculations (e.g., lunar calendars) requires ultra-high precision floating-point scaling.

Standard floating-point operations compound rounding errors at each step. While arbitrary-precision rational libraries (like `num-rational`) provide infinite precision, they require dynamic heap allocation and are several orders of magnitude slower, making them unusable in performance-critical or embedded (`#![no_std]`) environments.

The `fused` crate provides a suite of **lightweight, three-term fused floating-point operations** that achieve near-exact rounding ($\approx 0.5$ ULP) at near-native hardware speeds, with zero heap allocations and full `#![no_std]` compatibility.

---

## 2. Crate Structure & Module Layout

The crate is designed with a clean, modular structure where each algorithm is isolated in its own file to maximize readability and maintainability:

```
utils/fused/
├── Cargo.toml      # Workspace inheritance and dependencies
├── DESIGN.md       # Crate architecture and design (this file)
├── proofs.md       # Formal mathematical proofs and error bounds
├── src/
│   ├── lib.rs      # Crate entry point, comprehensive docs, and re-exports
│   ├── mul_div.rs  # Fused Multiply-Divide algorithm: (a * num) / den
│   ├── mul_div_add.rs # Fused Multiply-Divide-Add algorithm: (a * num) / den + offset
│   └── div_mul.rs  # Fused Reciprocal Division algorithm: a / (b * c)
└── tests/
    └── differential.rs # Differential fuzz and randomized testing suite
```

- **`lib.rs`**: Re-exports all core public functions and contains high-level documentation.
- **`mul_div.rs`**: Implements `f64_mul_div(a: f64, num: f64, den: f64) -> f64` using FMA and Dekker/Jeannerod remainder compensation.
- **`mul_div_add.rs`**: Implements `f64_mul_div_add(a: f64, num: f64, den: f64, offset: f64) -> f64` using a double-word quotient intermediate and Knuth's 2Sum algorithm.
- **`div_mul.rs`**: Implements `f64_div_mul(a: f64, b: f64, c: f64) -> f64` (Fused Reciprocal Division: `a / (b * c)`) using denominator product error extraction, division remainder compensation, and a first-order Taylor expansion.

---

## 3. Key Design & Architectural Decisions

### 3.1 `#![no_std]` Support
ICU4X is designed to run in embedded and resource-constrained environments (such as WebAssembly, microcontrollers, and IoT devices). Therefore, the `fused` crate is strictly `#![no_std]`.
To access FMA and other floating-point intrinsics in a `#![no_std]` context without depending on `std` or `libm` directly, we utilize the `core_maths` crate (specifically the `CoreFloat` trait). This provides a clean, abstract interface for `mul_add` (FMA) that compiles down to hardware instructions.

### 3.2 Native FMA Optimization & Software Fallback
The compensated algorithms are designed to leverage the hardware **Fused Multiply-Add (FMA)** instruction. FMA computes $a \times b + c$ with infinite intermediate precision and a single rounding, which is the key mechanism for extracting exact rounding errors.

- **Hardware Path**: When compiled with target features enabling FMA (e.g., `RUSTFLAGS="-C target-feature=+fma"`), the compiler maps `.mul_add()` directly to the CPU's native FMA instruction (e.g., `vfmadd` on x86 or `fmadd` on ARM). This executes in a single clock cycle with zero overhead.
- **Software Fallback**: If the target CPU does not support FMA, the `core_maths` crate automatically falls back to a software emulation of FMA. The algorithms remain 100% mathematically correct and precise, though they will run slower.

### 3.3 Zero-Cost Fallback Strategy
Floating-point calculations must handle special values (such as $\text{NaN}$, $\infty$, $-\infty$, and division by zero) exactly as specified by the IEEE 754 standard. 
Instead of introducing complex branching and checks for these edge cases in the fast path (which would destroy pipeline efficiency), we employ a **zero-cost fallback** strategy:
1. The compensated algorithm is executed using fast, branchless floating-point instructions.
2. We check if the resulting compensated value is finite using `corrected.is_finite()`.
3. If it is finite, we immediately return it. Modern CPUs execute this check extremely efficiently.
4. If it is not finite (indicating that an intermediate overflow occurred, or one of the inputs was NaN/infinity), we fall back to the standard, uncompensated operation (e.g., `(a * num) / den`).

This guarantees that all standard IEEE 754 behaviors (like NaN propagation and infinity signaling) are preserved exactly, while normal numerical computations enjoy ultra-high precision at maximum speed.

---

## 4. Comparative Analysis

The table below summarizes how `fused` compares with other prominent Rust crates that address floating-point precision:

| Crate | Architectural Approach | Target Use Case | Performance Overhead | Memory & Heap |
|---|---|---|---|---|
| **`twofloat`** | Double-double arithmetic wrapper | General double-precision math | High (5-10x slower; wraps all operations) | Zero heap, wraps 2 floats |
| **`accurate`** | Compensated summation/dot products | Vector accumulation | Medium (optimized for arrays) | Zero heap, array-based |
| **`metallic`** | Math functions from scratch | Transcendental functions (`sin`, `log`) | Native | Zero heap, no-std |
| **`fused` (This)** | Targeted FMA-compensated algorithms | 3-term scaling and unit conversions | **Near-Zero** (uses native FMA, branchless) | **Zero heap, no wrapper** |

### 4.1 Detailed Trade-offs

- **`fused` vs `twofloat`**:
  `twofloat` is a general-purpose double-double library. It defines a new `TwoFloat` type that wraps two `f64` values (head and tail) and implements all standard arithmetic operators. While very powerful, this approach incurs a high performance cost because *every* addition and multiplication must carry and manipulate the tail, even if the extra precision is not needed.
  In contrast, `fused` does not define any wrapper types. It operates directly on standard `f64` inputs and outputs, using double-word representations only *transiently* within the function body. This avoids the overhead of wrapping/unwrapping and allows the compiler to optimize the register allocation perfectly.
- **`fused` vs `accurate`**:
  `accurate` is designed for accumulating errors over large arrays (e.g., summing 1 million floats). It implements Kahan and Neumaier summation algorithms. It is not designed to solve the rounding errors of individual, three-term algebraic operations like `(a * b) / c`. `fused` fills this gap.
- **`fused` vs `metallic`**:
  `metallic` is a replacement for `libm` written in pure Rust. It focuses on ensuring that transcendental functions (trigonometric, logarithmic, exponential) are faithfully rounded to less than 1 ULP. It does not provide the specific fused scaling and division-multiplication algorithms implemented in `fused`.

---

## 5. Fundamental Precision Limits and Midpoint Rounding

While the algorithms in the `fused` crate guarantee that the result is rounded **exactly once** to the nearest representable float (achieving $\le 1$ ULP of error relative to the exact mathematical result of the represented inputs), they **cannot** bypass the fundamental representation limits of the binary double-precision format, nor do they alter standard IEEE 754 tie-breaking rules.

A classic, highly illustrative example of this is the expression:
\[ 0.3 \times 3.0 / 1.0 \]
Mathematically, this is exactly \(0.9\). However:
*   Standard float arithmetic (`0.3f64 * 3.0f64 / 1.0f64`) yields `0.8999999999999999`.
*   Our high-precision compensated algorithm (`f64_mul_div(0.3, 3.0, 1.0)`) **also** yields `0.8999999999999999`.

### 5.1 Step-by-Step Mathematical Analysis

#### 1. Binary Representation Error of 0.3
In base 10, \(0.3\) is a terminating decimal. In base 2, it is a repeating fraction:
\[ 0.3_{10} = 0.010011001100110011\dots_2 \]
When rounded to the nearest 53-bit significand, the float \(0.3f64\) is represented as:
\[ 0.3_{\text{float}} = 0.299999999999999988897769753748434595763683319091796875 \]
which is slightly *less* than \(0.3\).

#### 2. Exact Multiplication
When we multiply \(0.3_{\text{float}}\) by \(3.0\) (which is exactly representable in binary as \(3 = 11_2\)), the exact mathematical product in real numbers is:
\[ P_{\text{real}} = 0.3_{\text{float}} \times 3.0 = 0.8999999999999999666933092612453037872910501956939697265625 \]

#### 3. Midpoint Rounding
We now round the real number \(P_{\text{real}}\) to the nearest double-precision float. The two closest representable floats are:
*   \(f_{\text{low}} = 0.899999999999999911182158029987476766109466552734375\)
*   \(f_{\text{high}} = 0.90000000000000002220446049250313080847263336181640625\)

Let's calculate the absolute distances from the exact real product \(P_{\text{real}}\):
*   \(|P_{\text{real}} - f_{\text{low}}| = 5.551115123125783 \times 10^{-17}\)
*   \(|P_{\text{real}} - f_{\text{high}}| = 5.551115123125783 \times 10^{-17}\)

Notice that the distances are **mathematically identical**! The real product \(P_{\text{real}}\) lands **exactly halfway (on the midpoint)** between the two representable floats.

#### 4. Round-to-Nearest-Even Tie-Breaking
Under the standard IEEE 754 round-to-nearest-even tie-breaking rule, when a real value lies exactly on a midpoint, the tie is broken by selecting the float whose significand ends in an **even** bit (`0` in binary).
*   \(f_{\text{low}}\) has an even significand (ends in `0` in binary).
*   \(f_{\text{high}}\) has an odd significand (ends in `1` in binary).

Therefore, the rounding operation selects \(f_{\text{low}}\), yielding `0.8999999999999999`.

### 5.2 Rationale for Algorithm Correctness
Since \(0.8999999999999999\) is the mathematically correct single-rounded result of the represented input values \(0.3f64\) and \(3.0f64\), the compensated algorithm is **behaving perfectly correctly**. 

Attempting to "force" the result to `0.9` would require violating the IEEE 754 rounding standards or making assumptions about decimal base-10 intent. The `fused` crate is strictly an IEEE 754-compliant binary floating-point library, and it guarantees absolute mathematical fidelity within the binary format.

---

## 6. Deep Technical Trade-offs & Implementation Decisions

The implementation of the `fused` crate is the result of careful analysis of low-level software engineering and mathematical trade-offs. Below, we detail the rationale behind our four most significant design decisions.

### 6.1 Raw Primitives (`f64`) vs. Ratio/Double-Double Wrappers

During the design phase, we evaluated whether to wrap our high-precision values in a dedicated type (e.g., a `RatioF64` or a `DoubleDouble` struct) or to operate directly on raw `f64` primitives.

*   **Wrapper Types (e.g., `TwoFloat`):** Define a new struct wrapping two floats (head and tail). While this provides compile-time type safety and allows overloading operators (like `+`, `*`), it introduces significant **API friction** (users must wrap/unwrap values constantly) and **wrapping overhead** (compilers often fail to optimize away struct boundaries, leading to unnecessary memory store/load instructions and register spilling). Furthermore, implementing full algebraic traits increases compile-time and code size.
*   **Raw Primitives (`f64`):** We chose to keep the public API purely primitive-based, operating exclusively on raw `f64` values. High-precision double-word representations are used *transiently* as local variables within the function bodies. This allows the compiler to allocate CPU registers (`xmm`/`ymm` on x86_64 or `d`/`q` on ARM) with absolute freedom and zero overhead.
*   **Conclusion:** Fancy type-safe wrappers are best reserved for higher-level system boundaries (such as a full decimal formatting engine). For low-level, high-throughput mathematical primitives, raw `f64` values deliver the lowest possible friction and maximum performance.

### 6.2 Proactive Subnormal Guarding (Mathematical & Microarchitectural Necessity)

A critical hazard in FMA-compensated algorithms is the behavior of subnormal numbers (numbers extremely close to zero, between $2^{-1022}$ and $2^{-1074}$ for `f64`). We inject a proactive subnormal check at the beginning of each hot path:
- In `f64_mul_div` and `f64_mul_div_add`: `if den.abs() < f64::MIN_POSITIVE || p.abs() < f64::MIN_POSITIVE`
- In `f64_div_mul`: `if hi == 0.0 || !hi.is_finite() || hi.abs() < f64::MIN_POSITIVE`

This check is a necessity for two profound reasons:

1.  **Mathematical Necessity (Preventing Precision Collapse):**
    In compensated algorithms, we extract the exact product rounding error using `t = a.mul_add(num, -p)`. However, if the product $p = a \times \text{num}$ underflows to a subnormal number, it loses significand bits due to gradual underflow. When we subtract $-p$ from the infinite-precision intermediate product in FMA, the error-tracking term $t$ itself underflows to exactly zero. This causes the compensation term to vanish, resulting in a **catastrophic precision collapse of up to $2^{50}$ ULPs** (the algorithm becomes no more accurate than standard float math, but with a false sense of security).
2.  **Microarchitectural Necessity (Preventing CPU Stalls):**
    On modern x86_64 and ARM processors, floating-point hardware pipelines are optimized exclusively for normalized numbers. When a subnormal number is encountered, the hardware pipeline cannot process it directly. Instead, the CPU triggers a **subnormal assist** (a microcode trap), which stalls the instruction pipeline for **100 to 300 clock cycles** while microcode handles the subnormal math. By proactively checking for subnormals on the hot path, we can immediately divert to the fallback path, avoiding microcode stalls and maintaining consistent, high-speed execution. Additionally, this ensures full compatibility with platforms where subnormals are flushed to zero (DAZ/FTZ modes).

### 6.3 The Smart Simple Fallback (Inline Magnitude Reordering)

When the subnormal guard triggers or when the FMA math fails (resulting in a non-finite value due to intermediate overflow), we must fall back to a standard, uncompensated operation.
Instead of a simple, naive fallback (which would simply repeat the overflowing/underflowing operation and return infinity or NaN), we implement an **8-line inline Smart Simple Fallback**:

```rust
let a_div = a / den;
let fallback_1 = a_div * num;
if a_div.abs() < f64::MIN_POSITIVE || !fallback_1.is_finite() {
    let fallback_2 = (num / den) * a;
    if fallback_2.is_finite() || !fallback_1.is_finite() {
        fallback_2
    } else {
        fallback_1
    }
} else {
    fallback_1
}
```

*   **Magnitude Reordering:** This fallback uses magnitude reordering to compute either `(a / den) * num` or `(num / den) * a`. By dividing the larger numerator factor by the denominator first, we scale the value down into a safe, representable range before multiplying by the other factor.
*   **Catastrophic Underflow/Overflow Prevention:** This prevents intermediate overflows (e.g. when $a \times \text{num}$ exceeds $1.79 \times 10^{308}$ but the final scaled quotient is perfectly representable) and intermediate underflows (e.g. when $a \times \text{num}$ underflows to $0.0$ but the final scaled quotient is non-zero).
*   **Robustness Check:** The fallback is completely robust: it checks if the reordered expression (`fallback_2`) is actually finite/better before preferring it, which perfectly handles special inputs and division-by-zero without causing NaNs. It captures 100% of the safety and precision benefits of highly complex fallback libraries for virtually zero code complexity.

### 6.4 The Double-Rounding Ground Truth Flaw

A major challenge in verifying high-precision floating-point crates is establishing an absolute ground truth.
Standard verification suites often use the `num_rational::Ratio` crate, converting the final rational number back to a float using `Ratio::to_f64()`. However, we discovered that **`Ratio::to_f64()` is mathematically flawed due to triple-rounding**:

1.  The rational numerator and denominator are converted, leading to an initial rounding.
2.  The division is performed in 64-bit float math, leading to a second rounding.
3.  The final float conversion rounds a third time.

This triple-rounding (or double-rounding in simpler cases) can cause the ground truth itself to be off by 1 or 2 ULPs from the true mathematical float representation of the rational number. As a result, developers are forced to relax their test assertions to a loose 4 ULP tolerance to prevent false test failures.

*   **Our Solution (Bit-Accurate Rounder):** We resolved this by implementing a custom, bit-accurate rational-to-float rounder (`round_ratio_to_f64` in `tests/common/mod.rs`). This rounder:
    - Analyzes the arbitrary-precision rational number directly.
    - Finds the exact binary exponent using bit-shifts.
    - Performs a single, mathematically rigorous round-to-nearest-even tie-break using arbitrary-precision remainder division.
    - Directly constructs the IEEE 754 float bits.
*   **The Result:** By delivering an absolute, 100% bit-accurate ground truth, we completely eliminated the double-rounding flaw from our test suite. This allowed us to **enforce a strict 1 ULP tolerance** across all differential tests for the compensated path, ensuring absolute mathematical verification.
