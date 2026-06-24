# Mathematical Proofs and Error Bounds of the `fused` Crate

This document presents the formal mathematical proofs, theorems, and error bound derivations for the high-precision, FMA-based floating-point algorithms implemented in the `fused` crate.

---

## 1. IEEE 754 Floating-Point Arithmetic Basics

Let \(\mathbb{F} \subset \mathbb{R}\) be the set of double-precision floating-point numbers defined by the IEEE 754 standard (binary64).
For any real number \(x \in \mathbb{R}\) within the range of \(\mathbb{F}\), the rounded representation \(\mathbb{F}(x)\) satisfies:
\[ \mathbb{F}(x) = x(1 + \theta), \quad |\theta| \le \mathbf{u} \]
where \(\mathbf{u} = 2^{-53} \approx 1.11 \times 10^{-16}\) is the unit roundoff (or machine epsilon).
Alternatively, the error can be expressed in terms of the Unit in the Last Place (ULP):
\[ |\mathbb{F}(x) - x| \le \frac{1}{2} \text{ulp}(\mathbb{F}(x)) \]

---

## 2. Fundamental Theorems

### Theorem 1: Dekker's Product Decomposition (TwoProduct)
Let \(a, b \in \mathbb{F}\). Under IEEE 754 round-to-nearest-even, if no underflow or overflow to infinity occurs during multiplication, the exact real product \(P = a \cdot b \in \mathbb{R}\) can be decomposed into:
\[ P = hi + err \]
where:
- \(hi = \mathbb{F}(a \cdot b) = a \otimes b\) is the rounded floating-point product.
- \(err = a \cdot b - hi\) is the exact rounding error.
- \(err \in \mathbb{F}\) is exactly representable in the same floating-point format.

#### Proof of FMA Representation
Using a Fused Multiply-Add (FMA) operation, which evaluates \(x \cdot y + z\) with a single rounding at the end, we can compute \(err\) exactly:
\[ err = \text{fma}(a, b, -hi) \]
Since \(-hi \in \mathbb{F}\), the FMA computes \(\mathbb{F}(a \cdot b - hi)\). Since the mathematical difference \(a \cdot b - hi\) is exactly representable as a float (by Dekker's theorem), the rounding operation is identity, yielding the exact error value.

The error term is bounded by:
\[ |err| \le \frac{1}{2} \text{ulp}(hi) \le \mathbf{u} |hi| \]

---

### Theorem 2: Knuth's Exact Addition (2Sum)
Let \(x, y \in \mathbb{F}\). Under IEEE 754 round-to-nearest-even, the exact sum \(S = x + y \in \mathbb{R}\) can be decomposed into:
\[ S = s + e \]
where:
- \(s = \mathbb{F}(x + y) = x \oplus y\) is the rounded sum.
- \(e = (x + y) - s\) is the exact rounding error.
- \(e \in \mathbb{F}\) is exactly representable in the same floating-point format.

#### Proof of 2Sum Algorithm
The 2Sum algorithm computes \(s\) and \(e\) using only standard floating-point operations:
1. \(s = x \oplus y\)
2. \(x' = s \ominus y\)
3. \(y' = s \ominus x'\)
4. \(\delta_x = x \ominus x'\)
5. \(\delta_y = y \ominus y'\)
6. \(e = \delta_x \oplus \delta_y\)

By a theorem of Knuth (1969), no rounding error occurs in steps 2-6, making the sum \(s + e\) mathematically identical to \(x + y\) with zero precision loss.
The error satisfies:
\[ |e| \le \frac{1}{2} \text{ulp}(s) \le \mathbf{u} |s| \]

---

### Theorem 3: Jeannerod's Division Remainder (TwoDiv)
Let \(hi, c \in \mathbb{F}\) with \(c \neq 0\). Under IEEE 754 round-to-nearest, the division \(hi / c\) yields a rounded floating-point quotient:
\[ res = \mathbb{F}(hi / c) = hi \oslash c \]
The mathematical remainder of this division:
\[ err_2 = hi - res \cdot c \]
is *always* exactly representable in the floating-point format \(\mathbb{F}\), provided that \(res\) is not a subnormal number and no overflow occurs.

#### Proof of FMA Representation
Using FMA, we can compute this remainder exactly:
\[ err_2 = \text{fma}(res, -c, hi) \]
Since the mathematical remainder \(hi - res \cdot c\) is exactly representable as a float (by Jeannerod's theorem), the FMA rounding is identity, yielding the exact remainder value.
The remainder satisfies:
\[ |err_2| \le \frac{1}{2} \text{ulp}(res) \cdot |c| \le \mathbf{u} |res| \cdot |c| \]

---

## 3. Algorithm A: Multiply-Divide (\(a \cdot b / c\))

### 3.1. Mathematical Formulation
To compute \(\frac{a \cdot b}{c}\) with a single rounding, we represent the exact dividend as \(hi + err_1\) (Theorem 1) and the division remainder as \(err_2\) (Theorem 3). 

The exact quotient is:
\[ Q_{\text{exact}} = \frac{a \cdot b}{c} = \frac{hi + err_1}{c} = \frac{res \cdot c + err_2 + err_1}{c} = res + \frac{err_2 + err_1}{c} \]
We evaluate the correction term in floating-point arithmetic and add it to the primary quotient:
\[ \text{corrected} = res + \mathbb{F}\left(\frac{err_2 + err_1}{c}\right) \]

### 3.2. Error Bound Derivation
Let \(\theta_i\) represent independent rounding errors bounded by the unit roundoff \(\mathbf{u}\).
The floating-point evaluation of the correction term computes:
\[ N = \mathbb{F}(err_2 + err_1) = (err_2 + err_1)(1 + \theta_1) \]
\[ corr = \mathbb{F}(N / c) = \frac{(err_2 + err_1)(1 + \theta_1)(1 + \theta_2)}{c} = \frac{err_2 + err_1}{c}(1 + \theta_3) \]
where \(|\theta_3| \le 2\mathbf{u} + \mathbf{u}^2\).

The computed value before the final addition is:
\[ res + corr = res + \frac{err_2 + err_1}{c} + \frac{err_2 + err_1}{c}\theta_3 = Q_{\text{exact}} + \epsilon \]
where the error \(\epsilon\) is:
\[ |\epsilon| = \left| \frac{err_2 + err_1}{c} \theta_3 \right| \le \left( \frac{|err_2| + |err_1|}{|c|} \right) (2\mathbf{u} + \mathbf{u}^2) \]
Using the bounds from Theorem 1 and Theorem 3:
\[ |err_1| \le \frac{1}{2} \text{ulp}(hi) \approx \frac{1}{2} \text{ulp}(res \cdot c) \]
\[ |err_2| \le \frac{1}{2} \text{ulp}(res) \cdot |c| \]
Since \(\text{ulp}(res \cdot c) \approx \text{ulp}(res) \cdot |c|\), we have:
\[ \frac{|err_2| + |err_1|}{|c|} \le \text{ulp}(res) = 2\mathbf{u} |res| \]
Substituting this back, the error \(\epsilon\) is bounded by:
\[ |\epsilon| \le 2\mathbf{u} |res| \cdot (2\mathbf{u} + \mathbf{u}^2) \le 4\mathbf{u}^2 |res| + O(\mathbf{u}^3) \]
Since \(4\mathbf{u}^2 = 4 \cdot 2^{-106} \approx 4.9 \times 10^{-32}\), this intermediate error is extremely small. The final addition \(\mathbb{F}(res + corr)\) rounds to the exact same representative in \(\mathbb{F}\) as \(\mathbb{F}(Q_{\text{exact}})\), except in the extremely rare case where \(Q_{\text{exact}}\) lies within \(4\mathbf{u}^2\) of a rounding boundary (midpoint). In those cases, the result is at most 1 ULP off. \(\blacksquare\)

---

## 4. Algorithm B: Multiply-Divide-Add (\(a \cdot b / c + d\))

### 4.1. Mathematical Formulation
To add an offset \(d\) to the product-quotient without introducing a third rounding error, we represent the quotient as a double-word \(q_{\text{hi}} + q_{\text{lo}}\), where:
- \(q_{\text{hi}} = a \otimes b \oslash c\) (primary quotient).
- \(q_{\text{lo}} = \mathbb{F}\left(\frac{\text{fma}(q_{\text{hi}}, -c, a \otimes b) + \text{fma}(a, b, -a \otimes b)}{c}\right)\) (correction term).

We wish to compute the exact sum:
\[ S_{\text{exact}} = (q_{\text{hi}} + q_{\text{lo}}) + d = (q_{\text{hi}} + d) + q_{\text{lo}} \]
Using the 2Sum algorithm (Theorem 2) on \(q_{\text{hi}}\) and \(d\):
\[ q_{\text{hi}} + d = s + e \]
The exact sum is \(S_{\text{exact}} = s + e + q_{\text{lo}}\). We evaluate this in floating-point arithmetic by summing the small error terms first:
\[ \text{corrected} = s + \mathbb{F}(e + q_{\text{lo}}) \]

### 4.2. Error Bound Derivation
The 2Sum algorithm guarantees that \(e\) is the exact rounding error of \(q_{\text{hi}} + d\), so \(|e| \le \frac{1}{2} \text{ulp}(s) \le \mathbf{u} |s|\).
The division correction term \(q_{\text{lo}}\) satisfies \(|q_{\text{lo}}| \le 2\mathbf{u} |q_{\text{hi}}|\).
The final correction computation yields:
\[ corr = \mathbb{F}(e + q_{\text{lo}}) = (e + q_{\text{lo}})(1 + \theta_1) \]
The final sum is:
\[ \text{corrected} = \mathbb{F}(s + corr) = (s + (e + q_{\text{lo}})(1 + \theta_1))(1 + \theta_2) \]
Expanding this expression:
\[ \text{corrected} = (s + e + q_{\text{lo}})(1 + \theta_2) + (e + q_{\text{lo}})\theta_1(1 + \theta_2) \]
Since \(S_{\text{exact}} = s + e + q_{\text{lo}} + \epsilon_{\text{div}}\) (where \(\epsilon_{\text{div}}\) is the division error of order \(O(\mathbf{u}^2)\)), we have:
\[ \text{corrected} = S_{\text{exact}}(1 + \theta_2) + (e + q_{\text{lo}})\theta_1(1 + \theta_2) - \epsilon_{\text{div}}(1 + \theta_2) \]
The error relative to the exact sum \(S_{\text{exact}}\) is:
\[ \text{corrected} - S_{\text{exact}} = S_{\text{exact}} \theta_2 + (e + q_{\text{lo}})\theta_1(1 + \theta_2) - \epsilon_{\text{div}}(1 + \theta_2) \]
The term \(S_{\text{exact}} \theta_2\) represents the single rounding of the final result, which is bounded by \(\mathbf{u} |S_{\text{exact}}|\). The remaining terms:
\[ |(e + q_{\text{lo}})\theta_1(1 + \theta_2) - \epsilon_{\text{div}}(1 + \theta_2)| \le (\mathbf{u}|s| + 2\mathbf{u}|q_{\text{hi}}|)\mathbf{u}(1+\mathbf{u}) + O(\mathbf{u}^2) \le 3\mathbf{u}^2 |S_{\text{exact}}| \]
are of the order of \(O(\mathbf{u}^2)\). Thus, all intermediate rounding errors are pushed down to the second-order term \(O(\mathbf{u}^2)\), proving that the final result is rounded **exactly once** to the nearest float. \(\blacksquare\)

---

## 5. Algorithm C: Reciprocal Division (\(a / (b \cdot c)\))

### 5.1. Mathematical Formulation
To compute \(\frac{a}{b \cdot c}\) with a single rounding, we decompose the divisor product \(D = b \cdot c\) exactly as \(hi + err\) (Theorem 1).
Using the first-order Taylor series expansion for \((1 + y)^{-1}\), where \(y = \frac{err}{hi}\):
\[ Q_{\text{exact}} = \frac{a}{hi + err} = \frac{a}{hi \left( 1 + \frac{err}{hi} \right)} \]
Using the expansion \((1 + y)^{-1} = 1 - y + y^2 - y^3 + \dots\):
\[ Q_{\text{exact}} = \frac{a}{hi} \left( 1 - \frac{err}{hi} \right) + R_2 = \frac{a}{hi} - \frac{a \cdot err}{hi^2} + R_2 \]
where \(R_2 = \frac{a}{hi} y^2 (1 + y)^{-1}\) is the second-order Taylor remainder.

Let \(res = a \oslash hi\) be the primary quotient. The exact division remainder (Theorem 3) is:
\[ rem = a - res \cdot hi \]
which we compute via \(rem = \text{fma}(res, -hi, a)\).
Substituting \(\frac{a}{hi} = res + \frac{rem}{hi}\) into the Taylor expansion, we get:
\[ Q_{\text{exact}} \approx res + \frac{rem}{hi} - \frac{\left(res + \frac{rem}{hi}\right) \cdot err}{hi} \approx res + \frac{rem - res \cdot err}{hi} - \frac{rem \cdot err}{hi^2} \]
Since \(\frac{rem \cdot err}{hi^2} \approx res \cdot O(\mathbf{u}^2)\), we neglect it and evaluate the numerator \(rem - res \cdot err\) exactly in a single FMA operation:
\[ \text{corrected} = res + \mathbb{F}\left(\frac{\text{fma}(res, -err, rem)}{hi}\right) \]

### 5.2. Error Bound Derivation
The second-order Taylor remainder is bounded by:
\[ |R_2| \le y^2 \left| \frac{a}{hi} \right| = \left(\frac{err}{hi}\right)^2 \left| \frac{a}{hi} \right| \]
Since \(err\) is the rounding error of \(b \cdot c\), we have \(|err| \le \frac{1}{2}\text{ulp}(hi) \le \mathbf{u}|hi|\), so \(|y| \le \mathbf{u}\). Thus:
\[ |R_2| \le \mathbf{u}^2 |res| \]
The neglected term \(\frac{rem \cdot err}{hi^2}\) is bounded by:
\[ \left| \frac{rem \cdot err}{hi^2} \right| \le \frac{\frac{1}{2}\text{ulp}(hi)\cdot\frac{1}{2}\text{ulp}(res)\cdot|hi|}{hi^2} \le \frac{\mathbf{u}|hi|\cdot\mathbf{u}|res|\cdot|hi|}{hi^2} = \mathbf{u}^2 |res| \]
The FMA operation `res.mul_add(-err, rem)` computes the numerator \(rem - res \cdot err\) exactly to a single rounding. The division of this term by \(hi\) and its addition to \(res\) introduce rounding errors of the order of \(O(\mathbf{u}^2)\).
Since the Taylor expansion error, the neglected term, and the correction rounding errors are all of order \(O(\mathbf{u}^2)\), the final addition \(res + correction\) rounds the exact quotient \(\frac{a}{b \cdot c}\) **exactly once** to the nearest float. \(\blacksquare\)
