# Crate Design and Architectural Choices: `fused`

This document provides a detailed description of the crate structure, module layout, and key design choices for the `fused` crate. For the rigorous mathematical proofs and error bound derivations, please refer to [proofs.md](proofs.md).

---

## 1. Crate Structure & Module Layout

The crate is organized to maximize modularity, maintainability, and compilation efficiency under strict `#![no_std]` constraints.

### 1.1. Directory Structure

```
utils/fused/
├── Cargo.toml
├── DESIGN.md          # Crate structure and key architectural design choices (this file)
├── proofs.md          # Rigorous mathematical proofs and error bound derivations
└── src/
    ├── lib.rs         # Crate entry point, comparative research, and re-exports
    ├── mul_div.rs     # Multiply-Divide algorithm and tests
    ├── mul_div_add.rs # Multiply-Divide-Add algorithm and tests
    └── div_mul.rs     # Reciprocal Division algorithm and tests
```

### 1.2. Modularity Design Rationale
Rather than placing all algorithms in a single large file, each core FMA operation is isolated in its own dedicated Rust module (`src/mul_div.rs`, `src/mul_div_add.rs`, `src/div_mul.rs`).
*   **Encapsulation:** Each module encapsulates its own FMA compensation math, helper methods, and specialized unit test suites.
*   **Test Locality:** Sanity checks, edge cases (NaNs, infinities), subnormal handling, and intermediate overflow fallbacks are tested immediately next to the implementation.
*   **Documentation Locality:** The detailed documentation, mathematical models, and examples are placed directly on the corresponding function in its own file.

---

## 2. Key Architectural Design Choices

### 2.1. Native `f64` Interface (No Custom Numeric Types)
*   **The Choice:** The crate operates entirely on standard primitive `f64` types for both inputs and outputs.
*   **The Rationale:** Many high-precision libraries (such as `twofloat` or arbitrary-precision crates) require wrapping floating-point numbers in custom wrapper structs (e.g., `TwoFloat`). While this is excellent for general-purpose high-precision math, it introduces significant runtime overhead, extra branching, and severe API friction when integrating into an existing codebase like ICU4X. 
*   **The Solution:** `fused` uses double-word arithmetic and FMA *internally* to track and accumulate rounding errors, but the public API remains completely transparent. It is a zero-cost, drop-in replacement for standard native float operations.

### 2.2. Zero-Cost Fallback Semantics
*   **The Choice:** We implement a branchless FMA fast path, followed by a single check: `if corrected.is_finite() { corrected } else { fallback }`.
*   **The Rationale:** High-precision FMA compensation math relies on subtraction terms like `a.mul_add(b, -hi)` or division remainder terms. If any input is a special value (such as `NaN`, `Infinity`, or if a division by zero occurs), or if the intermediate product overflows the finite double range, the error-tracking FMA operations naturally produce `NaN` (due to indeterminate forms like \(\infty - \infty\)).
*   **The Solution:** Rather than adding expensive branch checks for every edge case at the start of the function, the algorithm runs branchless, allowing the CPU to execute at maximum pipeline speed. The final `is_finite()` check is a simple bitwise mask on the float's exponent field, which is extremely fast (zero-cost).
*   **Intermediate Overflow Handling:** If an intermediate product overflows, the fallback path evaluates the quotient first (e.g., `a * (num / den)`), which scales the factor to a moderate range, ensuring that the calculation completes successfully and returns the correct finite value.

### 2.3. Taylor Series Reciprocal Division
*   **The Choice:** We implement reciprocal division `a / (b * c)` using a first-order Taylor series expansion rather than performing a full double-word division.
*   **The Rationale:** Performing a full division of two double-word numbers (where both the numerator and denominator are double-words) is computationally expensive, requiring multiple divisions, multiplications, and additions.
*   **The Solution:** By representing the divisor product exactly as \(D = hi + err\) and using the first-order Taylor expansion \((1 + y)^{-1} \approx 1 - y\), we can express the quotient as:
    \[ Q \approx res + \frac{rem - res \cdot err}{hi} \]
    The term \(rem - res \cdot err\) is evaluated exactly in a single hardware FMA instruction. This achieves single-rounded double-precision accuracy at a fraction of the cost of full double-word division.

### 2.4. Knuth's 2Sum for Exact Offset Additions
*   **The Choice:** We use Knuth's 2Sum algorithm to add the offset in `f64_mul_div_add`.
*   **The Rationale:** In offset conversions (such as Celsius to Fahrenheit), adding the offset introduces a third potential rounding error.
*   **The Solution:** By representing the product-quotient as a double-word \(q_{\text{hi}} + q_{\text{lo}}\), we can add the offset \(d\) to \(q_{\text{hi}}\) exactly using Knuth's 2Sum algorithm. This yields a rounded sum \(s\) and an exact rounding error \(e\) with no precision lost. The small error terms \(e + q_{\text{lo}}\) are then accumulated and applied as a final single-rounded correction.

---

## 3. Comparative Ecosystem Analysis

In `src/lib.rs`, we compile a detailed comparative research section that contrasts `fused` with other crates in the Rust ecosystem:
*   **`twofloat` (Double-Double):** We avoid custom wrapper types and ongoing double-word execution overhead, keeping our scalar API transparent and lightweight.
*   **`accurate` (Compated Reductions):** While `accurate` focuses on vector-level compensated summation (Kahan/Ogita-Rump-Oishi) for slices, `fused` targets core three-term scalar-level fused equations essential for unit scaling.
*   **`metallic` (Multi-Precision):** We do not require heap allocation (`std::alloc` dependent) or heavy multi-precision logic, making `fused` completely `#![no_std]` and optimal for embedded systems.
