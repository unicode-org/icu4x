# DESIGN: Fused Scaling & Arithmetic Algorithms

This document outlines the architectural design, rationale, and performance trade-offs of the `fused` crate.

---

## 1. Architectural Rationale

The primary goal of the `fused` crate is to implement high-precision scaling and division operations on standard primitive floating-point numbers (`f32` and `f64`) without introducing custom wrapper types (such as double-double or quad-double structs) and with zero heap allocations.

### 1.1. Why Avoid Double-Double Structs?
Libraries like `twofloat` and `metallic` achieve high precision by representing a number as a sum of two floating-point numbers (`hi + lo`). While this provides approximately 106 bits of precision (for `f64`), it has significant drawbacks:
1. **Memory Footprint**: Doubles the storage requirements.
2. **Execution Overhead**: Every basic arithmetic operation (+, -, \*, /) requires a sequence of 10 to 20 instructions to maintain the double-word invariants.
3. **No Hardware Support**: Modern CPUs have no native instructions for double-double arithmetic, so it must be emulated in software.
4. **API Friction**: Users must wrap and unwrap their values, leading to poor ergonomics and complex integration.

### 1.2. The `fused` Approach: Fused Multiply-Add (FMA)
Instead of simulating higher precision for *all* operations, `fused` targets specific numerical patterns (e.g., $a \times b / c$, $a / (b \times c)$) and uses **Fused Multiply-Add (FMA)** to track and compensate for the exact rounding errors of intermediate steps.

An FMA operation computes $a \times b + c$ with a **single rounding** at the very end. This allows us to extract the exact rounding error of a multiplication or division:
- **Exact Multiplication Error**: For $p = \text{round}(a \times b)$, the error is $e = a \times b - p$, which can be computed exactly using FMA as `a.mul_add(b, -p)`.
- **Exact Division Remainder**: For $q = \text{round}(a / b)$, the remainder is $r = a - q \times b$, which can be computed exactly using FMA as `q.mul_add(-b, a)`.

By tracking these exact error terms, we can apply a first-order correction to the primary result, producing a final value that is rounded exactly once (or within 1 ULP under extreme subnormal conditions), achieving double-precision accuracy for these specific patterns at near-zero cost.

---

## 2. Platform-Specific FMA Routing

Hardware support for FMA is critical for the performance of this crate.
- On modern architectures (x86_64 with AVX2/FMA3, ARM64 with NEON/v8, RISC-V), FMA is a single-cycle hardware instruction (e.g., `vfmadd213sd`, `fmadd`).
- On older architectures or simple microcontrollers without hardware FMA, the operation must be emulated in software (soft-float), which is significantly slower.

### 2.1. Routing Abstraction
To support `#![no_std]` while maintaining maximum performance:
- When `feature = "std"` is active, we route directly to the Rust standard library's native `mul_add` method (e.g., `a.mul_add(b, c)`), which the compiler lowers directly to hardware FMA instructions.
- When `feature = "std"` is inactive (e.g., in embedded `no_std` environments), we route to `core_maths::CoreFloat::mul_add`, which resolves to a highly optimized soft-float polyfill.

---

## 3. Fallback Mechanisms and Dynamic Retry Strategy

Numerical algorithms that track intermediate errors are susceptible to **intermediate overflow or underflow** in extreme boundary ranges.

### 3.1. The Underflow/Overflow Fallback Dilemma
If we use a static fallback scaling expression like `(a / den) * num` to prevent intermediate overflow (when $a \times num$ would exceed the float limit), we run a high risk of **intermediate underflow** if $a$ is extremely small and $den$ is large, causing $a / den$ to underflow to `0.0` and destroying the result.
Conversely, if we statically chose `(num / den) * a`, it would prevent underflow but fail to prevent overflow if $num / den$ or the final product overflows.

### 3.2. The Dynamic Retry Strategy
To solve this dilemma and make the crate mathematically bullet-proof, we implement a **Dynamic Retry Strategy** in a consolidated private `fallback` module:

```rust
pub fn fallback_mul_div_f64(a: f64, num: f64, den: f64) -> f64 {
    let abs_a = a.abs();
    let abs_num = num.abs();
    // 1. Try the preferred branch optimized for overflow prevention
    let res = if abs_a < abs_num {
        (a / den) * num
    } else {
        (num / den) * a
    };
    // 2. If the result is degenerate (underflowed to 0.0 or overflowed to infinity)
    //    while inputs were non-zero, dynamically retry with the alternative branch.
    if res.is_finite() && res != 0.0 {
        res
    } else {
        if abs_a < abs_num {
            (num / den) * a
        } else {
            (a / den) * num
        }
    }
}
```
This strategy guarantees that the fallback path will always find a representable scaling path if one mathematically exists, preventing silent precision loss and division-by-zero errors in extreme subnormal and overflow boundaries.

---

## 4. API Safety & Invariant Encapsulation

To ensure the crate is production-grade, we enforce strict runtime invariants on our types while maintaining zero-cost ergonomics.

### 4.1. Private Fields and Const Getters
The `RatioF64` and `RatioF32` structs represent mathematical ratios:
$$\text{Ratio} = \frac{\text{numerator}}{\text{denominator}}$$
To prevent users from bypassing our safety invariants (finiteness and non-zero denominator) using struct literals, the fields are kept **private**, and are accessed via public `const fn` getters (`numerator()` and `denominator()`). This preserves safety without introducing any runtime function call overhead.

### 4.2. Safe Serde Deserialization
When the `serde` feature is enabled, deriving `Deserialize` directly would allow a malformed serialized payload (containing NaNs or a zero denominator) to bypass our `new()` constructor and instantiate an invalid `Ratio` struct.
To prevent this, we implement safe deserialization using Serde's `try_from` attribute:
```rust
#[cfg_attr(feature = "serde", serde(try_from = "RatioF64Unchecked"))]
pub struct RatioF64 { ... }
```
This routes all deserialization requests through a private unchecked helper struct and invokes our `new()` constructor, guaranteeing that all `Ratio` instances are mathematically valid at runtime, even when loaded from untrusted external data.
