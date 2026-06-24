# Mathematical Proofs for Fused Floating-Point Algorithms

This document provides rigorous mathematical proofs for the error-compensation algorithms implemented in the `fused` crate.

---

## 1. Mathematical Preliminaries

Let $\mathbb{F}$ be a set of IEEE 754 floating-point numbers.
Let $\text{fl}(x)$ denote the floating-point representation of a real number $x \in \mathbb{R}$ rounded to the nearest even floating-point number in $\mathbb{F}$.

### 1.1. Rounding and Machine Epsilon
Under the round-to-nearest-even mode, the relative rounding error is bounded by the machine epsilon $\mathbf{u}$ (where $\mathbf{u} = 2^{-24}$ for `f32` and $\mathbf{u} = 2^{-53}$ for `f64`):
$$\text{fl}(x) = x(1 + \delta), \quad |\delta| \le \mathbf{u}$$

Alternatively, we can express this bound in terms of the Unit in the Last Place (ULP):
$$| \text{fl}(x) - x | \le \frac{1}{2} \text{ulp}(\text{fl}(x))$$

### 1.2. Fused Multiply-Add (FMA) and Error-Free Transforms (EFT)
The Fused Multiply-Add operation is defined as:
$$\text{fma}(a, b, c) = \text{fl}(a \times b + c)$$
Importantly, the intermediate product $a \times b$ is computed to infinite precision, and only a single rounding is performed at the end.

This property allows FMA to implement **Error-Free Transforms (EFTs)**, which extract the exact rounding errors of basic operations:
1. **Exact Multiplication Error**: For $p = \text{fl}(a \times b)$, the error $e = a \times b - p$ is exactly representable in $\mathbb{F}$ (provided no underflow occurs). Thus, FMA computes it exactly:
   $$\text{fma}(a, b, -p) = a \times b - p \quad (\text{exactly})$$
2. **Exact Division Remainder**: For $q = \text{fl}(A / B)$, the remainder $r = A - q \times B$ is always exactly representable in $\mathbb{F}$ (Jeannerod, Louvet, and Muller, 2012). Thus, FMA computes it exactly:
   $$\text{fma}(q, -B, A) = A - q \times B \quad (\text{exactly})$$

---

## 2. Proof of Algorithm A: Fused Multiply-Divide

Algorithm A computes $\text{fl}(a \times x / y)$ with a single rounding.

### 2.1. The Algorithm Steps
1. Compute the primary quotient:
   $$q = \text{fl}\left(\frac{\text{fl}(a \times x)}{y}\right)$$
2. Compute the exact multiplication error using FMA (EFT):
   $$e_{\text{mul}} = \text{fma}(a, x, -\text{fl}(a \times x)) = a \times x - \text{fl}(a \times x)$$
3. Compute the division remainder using FMA (EFT) and add the multiplication error:
   $$e_{\text{div}} = \text{fma}(q, -y, \text{fl}(a \times x)) = \text{fl}(a \times x) - q \times y$$
4. Compute the corrected quotient:
   $$\hat{q} = \text{fl}\left(q + \frac{e_{\text{div}} + e_{\text{mul}}}{y}\right)$$

### 2.2. Rigorous Proof of Correctness
We want to prove that the error term $\Delta = \frac{a \times x}{y} - \left(q + \frac{e_{\text{div}} + e_{\text{mul}}}{y}\right)$ is bounded by $O(\mathbf{u}^2)$.

Substituting the definitions of $e_{\text{mul}}$ and $e_{\text{div}}$ into the correction term:
$$e_{\text{div}} + e_{\text{mul}} = \left(\text{fl}(a \times x) - q \times y\right) + \left(a \times x - \text{fl}(a \times x)\right)$$
$$e_{\text{div}} + e_{\text{mul}} = a \times x - q \times y$$

Thus, the exact correction we wish to apply is:
$$\frac{e_{\text{div}} + e_{\text{mul}}}{y} = \frac{a \times x - q \times y}{y} = \frac{a \times x}{y} - q$$

Now, let's analyze the rounding errors in the floating-point calculation of the correction.
Because $q$ is double-rounded (rounded first in multiplication, then in division), the distance between $q$ and the true quotient $\frac{a \times x}{y}$ accumulates both rounding errors. By the triangle inequality:
$$\left| \frac{a \times x}{y} - q \right| \le \frac{|e_{\text{mul}}|}{y} + \left| \frac{\text{fl}(a \times x)}{y} - q \right| \le \frac{\frac{1}{2} \text{ulp}(\text{fl}(a \times x))}{y} + \frac{1}{2} \text{ulp}(q) \le 1.5 \text{ulp}(q) \le 3 q \mathbf{u}$$

Let $s = \text{fl}(e_{\text{div}} + e_{\text{mul}})$. Since $e_{\text{div}} + e_{\text{mul}}$ is computed using floating-point addition:
$$s = (e_{\text{div}} + e_{\text{mul}})(1 + \delta_1), \quad |\delta_1| \le \mathbf{u}$$

Let the division of the error term be:
$$d = \text{fl}(s / y) = \frac{s}{y}(1 + \delta_2) = \frac{e_{\text{div}} + e_{\text{mul}}}{y}(1 + \delta_1)(1 + \delta_2), \quad |\delta_2| \le \mathbf{u}$$

Therefore, the absolute error in the computed correction $d$ is:
$$\left|d - \left(\frac{a \times x}{y} - q\right)\right| = \left|\frac{e_{\text{div}} + e_{\text{mul}}}{y}\right| |(1 + \delta_1)(1 + \delta_2) - 1|$$
$$\le (3 q \mathbf{u}) (2\mathbf{u} + \mathbf{u}^2) = 6 q \mathbf{u}^2 + O(\mathbf{u}^3)$$

Finally, the corrected quotient $\hat{q}$ is:
$$\hat{q} = \text{fl}(q + d) = (q + d)(1 + \delta_3), \quad |\delta_3| \le \mathbf{u}$$

Since $q + d \approx \frac{a \times x}{y}$, the rounding error $\delta_3$ corresponds to the final rounding of the result to the nearest float (which is mathematically required). The intermediate error term is strictly bounded by $6 q \mathbf{u}^2$, which is well below the threshold of $0.5 \text{ulp}(q)$ (which is $q \mathbf{u}$).

Thus, the algorithm successfully eliminates the double-rounding error, and the final result is rounded exactly once.

---

## 3. Proof of Algorithm B: Fused Multiply-Divide-Add

Algorithm B computes $\text{fl}(a \times x / y + z)$ with a single rounding.

### 3.1. The Algorithm Steps
1. Compute the high and low parts of the quotient using Algorithm A:
   $$q_{\text{hi}} = q, \quad q_{\text{lo}} = \text{fl}\left(\frac{e_{\text{div}} + e_{\text{mul}}}{y}\right)$$
   where $q_{\text{hi}} + q_{\text{lo}} \approx \frac{a \times x}{y}$ with error bounded by $O(\mathbf{u}^2)$.
2. Sum $q_{\text{hi}}$ and $z$ exactly using the **2Sum algorithm**:
   - $s = \text{fl}(q_{\text{hi}} + z)$
   - $q_{\text{hi}}' = \text{fl}(s - z)$
   - $z' = \text{fl}(s - q_{\text{hi}}')$
   - $e = \text{fl}\left((q_{\text{hi}} - q_{\text{hi}}') + (z - z')\right)$
   Under IEEE 754 arithmetic, the 2Sum algorithm guarantees that $s + e = q_{\text{hi}} + z$ exactly, without any rounding error.
3. Compute the final corrected sum:
   $$\hat{s} = \text{fl}\left(s + (e + q_{\text{lo}})\right)$$

### 3.2. Rigorous Proof of Correctness
Since $s + e = q_{\text{hi}} + z$ exactly, the exact mathematical sum is:
$$\frac{a \times x}{y} + z = (q_{\text{hi}} + q_{\text{lo}} + \epsilon_q) + z = (q_{\text{hi}} + z) + q_{\text{lo}} + \epsilon_q = s + e + q_{\text{lo}} + \epsilon_q$$
where $|\epsilon_q| \le 6 q \mathbf{u}^2$ is the error of the double-word quotient.

The correction term computed by the algorithm is $\text{fl}(e + q_{\text{lo}})$. Let's analyze its rounding error:
$$\text{fl}(e + q_{\text{lo}}) = (e + q_{\text{lo}})(1 + \delta_4), \quad |\delta_4| \le \mathbf{u}$$

Since both $e$ and $q_{\text{lo}}$ are first-order error terms bounded by $O(\mathbf{u})$, their sum $e + q_{\text{lo}}$ is also bounded by $O(\mathbf{u})$.
Thus, the rounding error of their addition is:
$$|(e + q_{\text{lo}})\delta_4| \le O(\mathbf{u}^2)$$

Finally, the corrected sum is:
$$\hat{s} = \text{fl}\left(s + \text{fl}(e + q_{\text{lo}})\right) = \left(s + (e + q_{\text{lo}})(1 + \delta_4)\right)(1 + \delta_5)$$
$$= \left(s + e + q_{\text{lo}} + O(\mathbf{u}^2)\right)(1 + \delta_5)$$
$$= \left(\frac{a \times x}{y} + z + O(\mathbf{u}^2)\right)(1 + \delta_5)$$

This proves that the total error before the final rounding $\delta_5$ is strictly bounded by $O(\mathbf{u}^2)$, ensuring a single-rounded result.

---

## 4. Proof of Algorithm C: Reciprocal Division

Algorithm C computes $\text{fl}\left(\frac{a}{b \times c}\right)$ with a single rounding.

### 4.1. The Algorithm Steps
1. Compute the exact product of the divisor:
   $$h = \text{fl}(b \times c), \quad e = \text{fma}(b, c, -h) = b \times c - h \quad (\text{EFT})$$
   Note that $h + e = b \times c$ exactly.
2. Compute the primary quotient:
   $$r = \text{fl}(a / h)$$
3. Compute the exact division remainder:
   $$\text{rem} = \text{fma}(r, -h, a) = a - r \times h \quad (\text{EFT})$$
4. Apply the first-order Taylor correction:
   $$\hat{r} = \text{fl}\left(r + \frac{\text{fma}(r, -e, \text{rem})}{h}\right)$$

### 4.2. Rigorous Proof of Correctness
We want to approximate $f = \frac{a}{b \times c} = \frac{a}{h + e}$.
Using the Taylor expansion of $\frac{1}{1 + x}$ around $x = 0$:
$$\frac{a}{h + e} = \frac{a}{h \left(1 + \frac{e}{h}\right)} = \frac{a}{h} \left(1 - \frac{e}{h} + \left(\frac{e}{h}\right)^2 - \dots\right)$$
$$\frac{a}{h + e} = \frac{a}{h} - \frac{a \times e}{h^2} + O\left(\frac{a \times e^2}{h^3}\right)$$

Since $r \approx \frac{a}{h}$, we can substitute $r$ into the first-order error term:
$$\frac{a}{h + e} \approx r + \frac{a - r \times h}{h} - \frac{r \times e}{h} = r + \frac{\text{rem} - r \times e}{h}$$

Let us rigorously analyze the two components of the error in this approximation:
1. **Taylor Remainder**: The second-order term of the expansion of $\frac{a}{h(1 + e/h)}$ is:
   $$\text{Err}_{\text{Taylor}} \approx \frac{a \times e^2}{h^3} \approx r \left(\frac{e}{h}\right)^2$$
   Since $e$ is the multiplication error of $b \times c$, $|e| \le \frac{1}{2} \text{ulp}(h) \le h \mathbf{u}$.
   Thus, $\left|\frac{e}{h}\right| \le \mathbf{u}$, which bounds the Taylor remainder by:
   $$|\text{Err}_{\text{Taylor}}| \le r \mathbf{u}^2$$
2. **Substitution Error**: Substituting $r$ for $a/h$ in the first-order term $- \frac{(a/h) \times e}{h}$ introduces the error:
   $$\text{Err}_{\text{Sub}} = \left(\frac{a}{h} - r\right)\frac{e}{h} = \frac{\text{rem}}{h} \frac{e}{h}$$
   Since $r = \text{fl}(a/h)$, the remainder $|\text{rem}| \le \frac{1}{2} \text{ulp}(a/h) \le h \mathbf{u}$.
   Thus, $\left|\frac{\text{rem}}{h}\right| \le r \mathbf{u}$, which bounds the substitution error by:
   $$|\text{Err}_{\text{Sub}}| \le (r \mathbf{u}) \mathbf{u} = r \mathbf{u}^2$$

Summing both components, the total mathematical error of our first-order approximation is strictly bounded by:
$$|\text{Err}_{\text{Taylor}}| + |\text{Err}_{\text{Sub}}| \le 2 r \mathbf{u}^2 = O(r \mathbf{u}^2)$$

By computing the numerator of this correction using FMA:
$$\text{fl}(\text{rem} - r \times e) = \text{fma}(r, -e, \text{rem}) + O(\mathbf{u}^2)$$
The relative error of the entire computed correction divided by $h$ remains strictly bounded by $O(\mathbf{u}^2)$.

Thus, the final corrected quotient $\hat{r} = \text{fl}\left(r + \frac{\text{num}}{h}\right)$ has a total intermediate error bounded by $O(\mathbf{u}^2)$, ensuring the final result is rounded exactly once to the nearest float.
