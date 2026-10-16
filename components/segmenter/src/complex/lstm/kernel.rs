// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! The LSTM recurrence, with the weights rearranged so that every inner loop
//! runs over contiguous `f32` slices of length `4 * hunits` and can be
//! vectorized by the compiler.
//!
//! The data stores `W` as `(4, hunits, embedd_dim)` and `U` as `(4, hunits, hunits)`,
//! so `W·x` and `U·h` are many short dot products. Here both are transposed once per
//! segmentation call, which turns them into `axpy` operations over the `4 * hunits`
//! gate values. `W·x` only depends on the input id, so it is computed once for every
//! distinct id of the input instead of once per position.

use super::matrix::MatrixZero;
use alloc::vec;
use alloc::vec::Vec;
#[allow(unused_imports)]
use core_maths::*;
use zerovec::ZeroSlice;
use zerovec::ule::AsULE;

/// Gate values are processed in blocks of this many lanes, which stay in registers
/// while the block accumulates `U·h`.
const LANES: usize = 16;

/// The weights of one direction of the LSTM, prepared for one input.
///
/// Rows of `xw` and `ut` hold the `4 * hunits` gate values padded with zeros to
/// `stride`, a multiple of [`LANES`].
#[derive(Debug)]
pub(super) struct Direction {
    hunits: usize,
    stride: usize,
    /// `b + W·x` for each distinct input id (in the order of `ids`).
    xw: Vec<f32>,
    /// `U` transposed: row `k` holds the gate weights that multiply `h[k]`.
    ut: Vec<f32>,
}

/// Transposes a row-major `rows × cols` matrix into `cols` rows of `stride >= rows` values.
fn transpose(m: &ZeroSlice<f32>, rows: usize, cols: usize, stride: usize) -> Vec<f32> {
    let mut out = vec![0.0; stride * cols];
    if cols == 0 {
        return out;
    }
    for (r, row) in m.as_ule_slice().chunks_exact(cols).take(rows).enumerate() {
        for (c, v) in row.iter().enumerate() {
            if let Some(dest) = out.get_mut(c * stride + r) {
                *dest = f32::from_unaligned(*v);
            }
        }
    }
    out
}

impl Direction {
    /// `ids` are the sorted, distinct embedding ids of the input.
    pub(super) fn new(
        embedding: MatrixZero<'_, 2>,
        w: MatrixZero<'_, 3>,
        u: MatrixZero<'_, 3>,
        b: MatrixZero<'_, 2>,
        ids: &[u16],
    ) -> Self {
        let (_, hunits, embedd_dim) = w.dim();
        let gates = 4 * hunits;
        let stride = gates.div_ceil(LANES) * LANES;
        let ut = transpose(u.as_slice(), gates, hunits, stride);
        let mut bias: Vec<f32> = b.as_slice().iter().collect();
        bias.resize(stride, 0.0);

        // W is transposed one block of LANES rows at a time instead of as a whole,
        // which keeps the scratch space small.
        let mut xw = vec![0.0; ids.len() * stride];
        let mut tile = vec![0.0; embedd_dim * LANES];
        let w_rows = w.as_slice().as_ule_slice().chunks_exact(embedd_dim.max(1));
        for (bi, bias_block) in bias.as_chunks::<LANES>().0.iter().enumerate() {
            tile.fill(0.0);
            for (l, row) in w_rows.clone().skip(bi * LANES).take(LANES).enumerate() {
                for (k, v) in row.iter().enumerate() {
                    if let Some(dest) = tile.get_mut(k * LANES + l) {
                        *dest = f32::from_unaligned(*v);
                    }
                }
            }
            for (&id, out) in ids.iter().zip(xw.chunks_exact_mut(stride)) {
                let Some(e) = embedding.submatrix::<1>(id as usize) else {
                    debug_assert!(false, "id is at most dict.len()");
                    continue;
                };
                let mut acc = *bias_block;
                for (xk, t) in e.as_slice().iter().zip(tile.as_chunks::<LANES>().0) {
                    for (a, t) in acc.iter_mut().zip(t) {
                        *a += xk * t;
                    }
                }
                if let Some(out) = out.as_chunks_mut::<LANES>().0.get_mut(bi) {
                    *out = acc;
                }
            }
        }

        Self {
            hunits,
            stride,
            xw,
            ut,
        }
    }

    /// The length of the scratch space that [`Self::step`] needs.
    pub(super) fn scratch_len(&self) -> usize {
        self.stride
    }

    /// One LSTM step for the input id at index `slot` of `ids`. Updates `h` and `c`;
    /// `s` is scratch space of length [`Self::scratch_len`].
    pub(super) fn step(&self, slot: usize, h: &mut [f32], c: &mut [f32], s: &mut [f32]) {
        let hunits = self.hunits;
        let stride = self.stride;
        let Some(xw) = self.xw.get(slot * stride..(slot + 1) * stride) else {
            debug_assert!(false, "slot is an index of ids");
            return;
        };
        // s = xw + U·h
        add_mul_transposed(xw, h, &self.ut, s);

        // Gates in the order input, forget, cell, output. Each activation runs over a
        // contiguous slice so that the loop can be vectorized.
        let Some((ifg, o)) = s.get_mut(..4 * hunits).map(|s| s.split_at_mut(3 * hunits)) else {
            debug_assert!(false, "s has 4 * hunits values");
            return;
        };
        let (i_f, g) = ifg.split_at_mut(2 * hunits);
        i_f.iter_mut().for_each(|x| *x = sigmoid(*x));
        g.iter_mut().for_each(|x| *x = tanh(*x));
        o.iter_mut().for_each(|x| *x = sigmoid(*x));
        let (i, f) = i_f.split_at(hunits);
        for ((((c, i), f), g), o) in c.iter_mut().zip(i).zip(f).zip(&*g).zip(o.iter_mut()) {
            *c = i * g + *c * f;
            *o *= tanh(*c);
        }
        h.copy_from_slice(o);
    }
}

/// `out = init + Mᵀ·x`, where `mt` holds `x.len()` rows of `out.len()` values.
///
/// `out` is computed one block of [`LANES`] values at a time, so the block stays in
/// registers while the rows of `mt` are accumulated into it. All lengths are multiples
/// of [`LANES`] except `x.len()`.
#[inline]
fn add_mul_transposed(init: &[f32], x: &[f32], mt: &[f32], out: &mut [f32]) {
    let stride = out.len();
    for (bi, (out_block, init_block)) in out
        .as_chunks_mut::<LANES>()
        .0
        .iter_mut()
        .zip(init.as_chunks::<LANES>().0)
        .enumerate()
    {
        let mut acc = *init_block;
        for (xk, row) in x.iter().zip(mt.chunks_exact(stride)) {
            if let Some(m) = row.as_chunks::<LANES>().0.get(bi) {
                for (a, m) in acc.iter_mut().zip(m) {
                    *a += xk * m;
                }
            }
        }
        *out_block = acc;
    }
}

#[inline]
fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + exp(-x))
}

#[inline]
fn tanh(x: f32) -> f32 {
    // tanh(x) = 1 - 2 / (e^2x + 1); the absolute error near 0 is about 1e-7,
    // which is below what the model can resolve.
    1.0 - 2.0 / (exp(2.0 * x) + 1.0)
}

/// `e^x` without branches or table lookups, so that loops calling it can be vectorized.
///
/// Splits `x = n ln 2 + r` with `|r| <= ln 2 / 2` and evaluates `e^r` with a degree-6
/// polynomial (Cephes `expf`), relative error about 2e-7. The input is clamped so that
/// `2^n` stays a normal `f32`.
#[inline]
fn exp(x: f32) -> f32 {
    const LOG2_E: f32 = core::f32::consts::LOG2_E;
    // ln 2 split into a high part with few mantissa bits and a correction
    const LN2_HI: f32 = 0.693_359_4;
    const LN2_LO: f32 = -2.121_944_4e-4;
    let x = x.clamp(-87.0, 88.0);
    // Round to nearest by adding and subtracting 1.5 * 2^23 (|x * LOG2_E| < 2^22);
    // `f32::round` is a libm call in no_std builds.
    const ROUND: f32 = 12_582_912.0;
    let t = x * LOG2_E + ROUND;
    let n = t - ROUND;
    let r = x - n * LN2_HI - n * LN2_LO;
    let p = 1.987_569_1e-4_f32;
    let p = p * r + 1.398_199_9e-3;
    let p = p * r + 8.333_452e-3;
    let p = p * r + 4.166_579_6e-2;
    let p = p * r + 1.666_666_5e-1;
    let p = p * r + 0.5;
    let er = p * r * r + r + 1.0;
    // 2^n, built from the exponent bits; n is in [-126, 127] after the clamp. The low
    // mantissa bits of `t` hold n, which avoids a float-to-int cast that would stop
    // the loop from being vectorized.
    let n_bits = t.to_bits().wrapping_sub(ROUND.to_bits());
    let scale = f32::from_bits(n_bits.wrapping_add(127) << 23);
    er * scale
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::*;
    use icu_provider::prelude::*;

    fn inputs() -> impl Iterator<Item = f32> {
        (-20_000..=20_000).map(|i| i as f32 / 100.0)
    }

    #[test]
    fn exp_is_accurate() {
        for x in inputs().filter(|x| (-87.0..=88.0).contains(x)) {
            let expected = f64::from(x).exp();
            let rel = ((f64::from(exp(x)) - expected) / expected).abs();
            assert!(rel < 1e-6, "exp({x}): relative error {rel}");
        }
        // Clamped instead of overflowing or becoming subnormal.
        for x in [-1e30, -1000.0, -88.0, 89.0, 1000.0, 1e30, f32::INFINITY] {
            assert!(exp(x).is_normal(), "exp({x}) = {}", exp(x));
        }
        assert!(exp(f32::NEG_INFINITY).is_normal());
    }

    #[test]
    fn activations_are_accurate() {
        for x in inputs().chain([-1e30, 1e30, f32::INFINITY, f32::NEG_INFINITY]) {
            let x64 = f64::from(x);
            let s = sigmoid(x);
            let t = tanh(x);
            assert!((0.0..=1.0).contains(&s), "sigmoid({x}) = {s}");
            assert!((-1.0..=1.0).contains(&t), "tanh({x}) = {t}");
            assert!(
                (f64::from(s) - 1.0 / (1.0 + (-x64).exp())).abs() < 1e-6,
                "sigmoid({x})"
            );
            assert!((f64::from(t) - x64.tanh()).abs() < 1e-6, "tanh({x})");
        }
    }

    /// One LSTM step computed directly from the data layout, in `f64`.
    fn reference_step(
        x: &[f64],
        h: &mut [f64],
        c: &mut [f64],
        w: MatrixZero<'_, 3>,
        u: MatrixZero<'_, 3>,
        b: MatrixZero<'_, 2>,
    ) {
        let (_, hunits, embedd_dim) = w.dim();
        let w: Vec<f64> = w.as_slice().iter().map(f64::from).collect();
        let u: Vec<f64> = u.as_slice().iter().map(f64::from).collect();
        let b: Vec<f64> = b.as_slice().iter().map(f64::from).collect();
        let s: Vec<f64> = (0..4 * hunits)
            .map(|r| {
                let wx: f64 = (0..embedd_dim).map(|k| w[r * embedd_dim + k] * x[k]).sum();
                let uh: f64 = (0..hunits).map(|k| u[r * hunits + k] * h[k]).sum();
                b[r] + wx + uh
            })
            .collect();
        let sig = |v: f64| 1.0 / (1.0 + (-v).exp());
        for j in 0..hunits {
            let (i, f, g, o) = (
                sig(s[j]),
                sig(s[hunits + j]),
                s[2 * hunits + j].tanh(),
                sig(s[3 * hunits + j]),
            );
            c[j] = i * g + f * c[j];
            h[j] = o * c[j].tanh();
        }
    }

    #[test]
    fn step_matches_reference() {
        let lstm: DataResponse<SegmenterLstmAutoV1> = Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes(
                    DataMarkerAttributes::from_str_or_panic(
                        "Thai_codepoints_exclusive_model4_heavy",
                    ),
                ),
                ..Default::default()
            })
            .unwrap();
        let LstmData::Float32(lstm) = lstm.payload.get();
        let embedding = MatrixZero::from(&lstm.embedding);
        let num_ids = lstm.dic.len() + 1;

        // A pseudo-random input that uses every id, including the one for unknown
        // characters, and repeats some of them.
        let input: Vec<u16> = (0..3 * num_ids)
            .map(|i| ((i * 37 + i / 5) % num_ids) as u16)
            .collect();
        let mut ids = input.clone();
        ids.sort_unstable();
        ids.dedup();

        for (w, u, b) in [
            (&lstm.fw_w, &lstm.fw_u, &lstm.fw_b),
            (&lstm.bw_w, &lstm.bw_u, &lstm.bw_b),
        ] {
            let (w, u, b) = (
                MatrixZero::from(w),
                MatrixZero::from(u),
                MatrixZero::from(b),
            );
            let hunits = u.dim().1;
            let direction = Direction::new(embedding, w, u, b, &ids);

            let mut h = vec![0.0; hunits];
            let mut c = vec![0.0; hunits];
            let mut s = vec![0.0; direction.scratch_len()];
            let mut h_ref = vec![0.0; hunits];
            let mut c_ref = vec![0.0; hunits];
            for &id in &input {
                let slot = ids.binary_search(&id).unwrap();
                direction.step(slot, &mut h, &mut c, &mut s);

                let x: Vec<f64> = embedding
                    .submatrix::<1>(id as usize)
                    .unwrap()
                    .as_slice()
                    .iter()
                    .map(f64::from)
                    .collect();
                reference_step(&x, &mut h_ref, &mut c_ref, w, u, b);

                for (a, e) in h.iter().zip(&h_ref).chain(c.iter().zip(&c_ref)) {
                    assert!((f64::from(*a) - e).abs() < 1e-4, "{a} != {e}");
                }
            }
        }
    }
}
