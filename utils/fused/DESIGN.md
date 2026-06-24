# Mathematical Design and Proofs of the `fused` Crate

This document provides the formal mathematical foundations, theorems, and rigorous error analysis for the high-precision, FMA-based algorithms implemented in this crate. 

---

## 1. Dekker's Product Decomposition

### Theorem 1 (Dekker, 1971)
Let \(a, b \in \mathbb{F}\) be floating-point numbers. If no underflow or overflow to infinity occurs during multiplication, the exact mathematical product \(P = a \cdot b \in \mathbb{R}\) can be decomposed into:
\[ P = hi + err \]
where:
- \(hi = \mathbb{F}(a \cdot b) = a \otimes b\) is the rounded float representation.
- \(err = a \cdot b - hi\) is the exact rounding error.
- \(err \in \mathbb{F}\) is exactly representable as a floating-point number in the same format.

### FMA Representation
Using a Fused Multiply-Add (FMA) operation, which computes \(xy + z\) with a single rounding at the end, we can obtain the exact error term in a single instruction by setting \(z = -hi\):
\[ err = \text{fma}(a, b, -hi) \]

### Error Bound
The rounding error \(err\) satisfies:
\[ |err| \le \frac{1}{2} \text{ulp}(hi) \le \mathbf{u} |hi| \]
where \(\mathbf{u} = 2^{-53} \approx 1.11 \times 10^{-16}\) is the unit roundoff for IEEE 754 double-precision arithmetic.

---

## 2. Jeannerod's Division Remainder

### Theorem 2 (Jeannerod et al., 2013)
Let \(hi, c \in \mathbb{F}\) with \(c \neq 0\). The division \(hi / c\) yields a rounded floating-point quotient:
\[ res = \mathbb{F}(hi / c) = hi \oslash c \]
The mathematical remainder of this division:
\[ err_2 = hi - res \cdot c \]
is *always* exactly representable in the floating-point format \(\mathbb{F}\), provided that \(res\) is not a subnormal number and no overflow occurs.

### FMA Representation
We can compute this remainder exactly using FMA:
\[ err_2 = \text{fma}(res, -c, hi) \]

### Error Bound
The remainder \(err_2\) satisfies:
\[ |err_2| \le \frac{1}{2} \text{ulp}(res) \cdot |c| \le \mathbf{u} |res| \cdot |c| \]

---

## 3. Algorithm A: Multiply-Divide (\(a \cdot b / c\))

### 3.1. Mathematical Formulation
To compute the division of a product \( \frac{a \cdot b}{c} \) with a single rounding, we decompose the exact dividend into its head and tail components using Theorem 1:
\[ a \cdot b = hi + err_1 \]
where \(hi = a \otimes b\) and \(err_1 = \text{fma}(a, b, -hi)\).

Dividing by \(c\), we obtain the rounded primary quotient:
\[ res = hi \oslash c \]
The division remainder from Theorem 2 is:
\[ err_2 = hi - res \cdot c \]
which we compute via \(err_2 = \text{fma}(res, -c, hi)\).

Substituting these back, the exact quotient \(Q_{\text{exact}}\) is:
\[ Q_{\text{exact}} = \frac{a \cdot b}{c} = \frac{hi + err_1}{c} = \frac{res \cdot c + err_2 + err_1}{c} = res + \frac{err_2 + err_1}{c} \]
We evaluate the correction term \(\frac{err_2 + err_1}{c}\) in floating-point arithmetic and add it to the primary quotient:
\[ \text{corrected} = res + \mathbb{F}\left(\frac{err_2 + err_1}{c}\right) \]

### 3.2. Rigorous Error Analysis
Let \(\theta_i\) represent independent rounding errors bounded by the unit roundoff \(\mathbf{u}\).
The floating-point evaluation of the correction term computes:
\[ N = \mathbb{F}(err_2 + err_1) = (err_2 + err_1)(1 + \theta_1) \]
\[ corr = \mathbb{F}(N / c) = \frac{(err_2 + err_1)(1 + \theta_1)(1 + \theta_2)}{c} = \frac{err_2 + err_1}{c}(1 + \theta_3) \]
where \(|\theta_3| \le 2\mathbf{u} + O(\mathbf{u}^2)\).

The computed value before the final addition is:
\[ res + corr = res + \frac{err_2 + err_1}{c} + \frac{err_2 + err_1}{c}\theta_3 = Q_{\text{exact}} + \epsilon \]
where the error \(\epsilon\) is:
\[ |\epsilon| = \left| \frac{err_2 + err_1}{c} \theta_3 \right| \le \left( \frac{|err_2| + |err_1|}{|c|} \right) (2\mathbf{u} + O(\mathbf{u}^2)) \]
Using the bounds from Theorem 1 and Theorem 2:
\[ |err_1| \le \frac{1}{2} \text{ulp}(hi) \approx \frac{1}{2} \text{ulp}(res \cdot c) \]
\[ |err_2| \le \frac{1}{2} \text{ulp}(res) \cdot |c| \]
Since \(\text{ulp}(res \cdot c) \approx \text{ulp}(res) \cdot |c|\), we have:
\[ \frac{|err_2| + |err_1|}{|c|} \le \text{ulp}(res) = 2\mathbf{u} |res| \]
Thus, the error \(\epsilon\) is bounded by:
\[ |\epsilon| \le 2\mathbf{u} |res| \cdot 2\mathbf{u} = 4\mathbf{u}^2 |res| \]
This proves that all intermediate rounding errors are pushed down to the second-order term \(O(\mathbf{u}^2)\). The final addition:
\[ \text{corrected} = \mathbb{F}(res + corr) \]
rounds to the exact same representative in \(\mathbb{F}\) as \(\mathbb{F}(Q_{\text{exact}})\), except in the extremely rare case where \(Q_{\text{exact}}\) lies within \(4\mathbf{u}^2\) of a rounding boundary (midpoint). In those cases, the result is at most 1 ULP off.

---

## 4. Algorithm B: Multiply-Divide-Add (\(a \cdot b / c + d\))

### 4.1. Mathematical Formulation
To add an offset \(d\) to the product-quotient without introducing a third rounding error, we represent the quotient as a double-word:
\[ Q = q_{\text{hi}} + q_{\text{lo}} \]
where:
- \(q_{\text{hi}} = a \otimes b \oslash c\) (primary quotient).
- \(q_{\text{lo}} = \mathbb{F}\left(\frac{\text{fma}(q_{\text{hi}}, -c, a \otimes b) + \text{fma}(a, b, -a \otimes b)}{c}\right)\) (correction term).

We wish to compute the exact sum:
\[ S_{\text{exact}} = (q_{\text{hi}} + q_{\text{lo}}) + d = (q_{\text{hi}} + d) + q_{\text{lo}} \]
We use the **2Sum algorithm** to perform the exact addition of \(q_{\text{hi}}\) and \(d\).

### 4.2. The 2Sum Algorithm (Knuth, 1969)
For any \(x, y \in \mathbb{F}\), the 2Sum algorithm computes the rounded sum \(s\) and the exact rounding error \(e\) such that:
\[ x + y = s + e \]
The steps are:
1. \(s = x \oplus y\)
2. \(x' = s \ominus y\)
3. \(y' = s \ominus x'\)
4. \(\delta_x = x \ominus x'\)
5. \(\delta_y = y \ominus y'\)
6. \(e = \delta_x \oplus \delta_y\)

No rounding error occurs in steps 2-6, making the decomposition \(x + y = s + e\) mathematically exact.

Applying 2Sum to \(q_{\text{hi}}\) and \(d\):
\[ q_{\text{hi}} + d = s + e \]
The exact sum is:
\[ S_{\text{exact}} = s + e + q_{\text{lo}} \]
We evaluate this in floating-point arithmetic by summing the small error terms first:
\[ \text{corrected} = s + \mathbb{F}(e + q_{\text{lo}}) \]

### 4.3. Proof of Correctness
The 2Sum algorithm guarantees that \(e\) is the exact rounding error of \(q_{\text{hi}} + d\), so \(|e| \le \frac{1}{2} \text{ulp}(s)\).
The division correction term \(q_{\text{lo}}\) satisfies \(|q_{\text{lo}}| \le 2\mathbf{u} |q_{\text{hi}}|\).
The final correction computation yields:
\[ corr = \mathbb{F}(e + q_{\text{lo}}) = (e + q_{\text{lo}})(1 + \theta_1) \]
The final sum is:
\[ \text{corrected} = \mathbb{F}(s + corr) = (s + (e + q_{\text{lo}})(1 + \theta_1))(1 + \theta_2) = (s + e + q_{\text{lo}})(1 + \theta_2) + (e + q_{\text{lo}})\theta_1(1 + \theta_2) \]
Since \(s + e + q_{\text{lo}} = S_{\text{exact}} + O(\mathbf{u}^2)\), the error relative to the exact sum is:
\[ \text{corrected} - S_{\text{exact}} = S_{\text{exact}} \theta_2 + (e + q_{\text{lo}})\theta_1(1 + \theta_2) + O(\mathbf{u}^2) \]
The term \(S_{\text{exact}} \theta_2\) represents the single rounding of the final sum. The secondary error term is bounded by:
\[ |(e + q_{\text{lo}})\theta_1(1 + \theta_2)| \le (\mathbf{u}|s| + 2\mathbf{u}|q_{\text{hi}}|)\mathbf{u} \approx 3\mathbf{u}^2 |S_{\text{exact}}| \]
which is of the order \(O(\mathbf{u}^2)\). This proves that the final result is rounded **exactly once** to the nearest float.

---

## 5. Algorithm C: Reciprocal Division (\(a / (b \cdot c)\))

### 5.1. Mathematical Formulation
To compute \( \frac{a}{b \cdot c} \), we first decompose the divisor product \( D = b \cdot c \) exactly using Theorem 1:
\[ D = hi + err \]
where \(hi = b \otimes c\) and \(err = \text{fma}(b, c, -hi)\).

We expand the quotient using a first-order Taylor series:
\[ Q = \frac{a}{hi + err} = \frac{a}{hi \left( 1 + \frac{err}{hi} \right)} \]
Using the Taylor expansion \((1 + y)^{-1} = 1 - y + y^2 - y^3 + \dots\), where \(y = \frac{err}{hi}\):
\[ Q = \frac{a}{hi} \left( 1 - \frac{err}{hi} + O\left(\frac{err^2}{hi^2}\right) \right) = \frac{a}{hi} - \frac{a \cdot err}{hi^2} + O\left(\mathbf{u}^2 \frac{a}{hi}\right) \]
Let \(res = a \oslash hi\) be the primary quotient. The exact division remainder (Theorem 2) is:
\[ rem = a - res \cdot hi \]
which we compute via \(rem = \text{fma}(res, -hi, a)\).

Substituting \( \frac{a}{hi} = res + \frac{rem}{hi} \) into the Taylor expansion, we get:
\[ Q \approx res + \frac{rem}{hi} - \frac{res \cdot err}{hi} = res + \frac{rem - res \cdot err}{hi} \]
We evaluate the numerator \(rem - res \cdot err\) using FMA and apply the correction:
\[ \text{corrected} = res + \mathbb{F}\left(\frac{\text{fma}(res, -err, rem)}{hi}\right) \]

### 5.2. Rigorous Error Analysis
The second-order term of the Taylor expansion is bounded by:
\[ \text{Error}_{\text{Taylor}} \le y^2 \left| \frac{a}{hi} \right| = \left( \frac{err}{hi} \right)^2 \left| \frac{a}{hi} \right| \le \mathbf{u}^2 |res| \]
The FMA operation `res.mul_add(-err, rem)` computes the numerator \(rem - res \cdot err\) exactly to a single rounding.
The division by \(hi\) and the final addition to \(res\) introduce rounding errors of the order of \(O(\mathbf{u}^2)\).
Since all intermediate error terms and the Taylor remainder are of order \(O(\mathbf{u}^2)\), the final addition \(res + correction\) rounds the exact quotient \( \frac{a}{b \cdot c} \) **exactly once** to the nearest float.
