// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::ops::Range;

use ndarray::{ArrayBase, Dim, Dimension, OwnedRepr};
use intmath::saturating_i16_mul_div_1024;

pub type MatType = i16;
pub const FACTOR: MatType = 1024;
pub const ZERO: MatType = 0;
pub use intmath::saturating_i16_mul_div_1024 as muldiv;

#[inline]
pub fn to_mat_type(input: f32) -> MatType {
    (input * FACTOR as f32).round() as i16
}

#[derive(Debug, Clone)]
pub struct MatrixOwned<const D: usize> {
    data: Vec<MatType>,
    dims: [usize; D],
}

impl<const D: usize> MatrixOwned<D> {
    pub fn empty() -> Self {
        Self {
            data: vec![],
            dims: [0; D],
        }
    }

    pub fn as_borrowed(&self) -> MatrixBorrowed<D> {
        MatrixBorrowed {
            data: &self.data,
            dims: self.dims,
        }
    }

    pub fn from_ndarray(nd: ArrayBase<OwnedRepr<MatType>, Dim<[usize; D]>>) -> Self
    where
        Dim<[usize; D]>: Dimension,
    {
        let maybe_dims: Result<[usize; D], _> = nd.shape().try_into();
        let data = nd.into_raw_vec();
        match maybe_dims {
            Ok(dims) if dims.iter().product::<usize>() == data.len() => Self { data, dims },
            _ => {
                debug_assert!(false);
                Self::empty()
            }
        }
    }

    pub fn new_zero(dims: [usize; D]) -> Self {
        let total_len = dims.iter().product::<usize>();
        MatrixOwned {
            data: vec![ZERO; total_len],
            dims,
        }
    }

    #[inline]
    pub fn submatrix<const M: usize>(&self, index: usize) -> Option<MatrixBorrowed<M>> {
        assert_eq!(M, D - 1);
        let (range, dims) = self.as_borrowed().submatrix_range(index);
        let data = &self.data.get(range)?;
        Some(MatrixBorrowed { data, dims })
    }

    pub fn as_mut(&mut self) -> MatrixBorrowedMut<D> {
        MatrixBorrowedMut {
            data: &mut self.data,
            dims: self.dims,
        }
    }

    #[inline]
    pub fn submatrix_mut<const M: usize>(&mut self, index: usize) -> Option<MatrixBorrowedMut<M>> {
        assert_eq!(M, D - 1);
        let (range, dims) = self.as_borrowed().submatrix_range(index);
        let data = self.data.get_mut(range)?;
        Some(MatrixBorrowedMut { data, dims })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MatrixBorrowed<'a, const D: usize> {
    data: &'a [MatType],
    dims: [usize; D],
}

impl<'a, const D: usize> MatrixBorrowed<'a, D> {
    #[cfg(debug_assertions)]
    pub fn debug_assert_dims(&self, dims: [usize; D]) {
        debug_assert_eq!(dims, self.dims);
        let expected_len = dims.iter().product::<usize>();
        debug_assert_eq!(expected_len, self.data.len());
    }

    pub fn as_slice(&self) -> &'a [MatType] {
        self.data
    }

    #[allow(clippy::wrong_self_convention)] // same convention as slice::to_vec
    pub fn to_owned(&self) -> MatrixOwned<D> {
        MatrixOwned {
            data: self.data.to_vec(),
            dims: self.dims,
        }
    }

    pub fn argmax(&self) -> usize {
        let mut mx = ZERO;
        let mut ind = 0;
        self.data.iter().enumerate().for_each(|(i, v)| {
            if mx < *v {
                mx = *v;
                ind = i
            }
        });
        ind
    }

    #[inline]
    pub fn submatrix<const M: usize>(&self, index: usize) -> Option<MatrixBorrowed<'a, M>> {
        // This assertion should always fail and be elided at compile time
        assert_eq!(M, D - 1);
        let (range, dims) = self.submatrix_range(index);
        let data = &self.data.get(range)?;
        Some(MatrixBorrowed { data, dims })
    }

    #[inline]
    fn submatrix_range<const M: usize>(&self, index: usize) -> (Range<usize>, [usize; M]) {
        // This assertion should always fail and be elided at compile time
        assert_eq!(M, D - 1);
        // The above assertion guarantees that the following line will succeed
        #[allow(clippy::indexing_slicing, clippy::unwrap_used)]
        let sub_dims: [usize; M] = self.dims[1..].try_into().unwrap();
        let n = sub_dims.iter().product::<usize>();
        (n * index..n * (index + 1), sub_dims)
    }
}

impl<'a> MatrixBorrowed<'a, 1> {
    pub fn dim(&self) -> usize {
        // The type parameter guarantees that self.dims has 1 element
        #[allow(clippy::indexing_slicing)]
        self.dims[0]
    }

    pub fn read_4(&self) -> Option<(MatType, MatType, MatType, MatType)> {
        debug_assert_eq!(self.dims, [4]);
        debug_assert_eq!(self.data.len(), 4);
        if self.data.len() == 4 {
            // Safety: self.data has length 4
            unsafe {
                Some((
                    *self.data.get_unchecked(0),
                    *self.data.get_unchecked(1),
                    *self.data.get_unchecked(2),
                    *self.data.get_unchecked(3),
                ))
            }
        } else {
            None
        }
    }
}

impl<'a> MatrixBorrowed<'a, 2> {
    pub fn dim(&self) -> (usize, usize) {
        // The type parameter guarantees that self.dims has 2 elements
        #[allow(clippy::indexing_slicing)]
        (self.dims[0], self.dims[1])
    }
}

impl<'a> MatrixBorrowed<'a, 3> {
    pub fn dim(&self) -> (usize, usize, usize) {
        // The type parameter guarantees that self.dims has 3 elements
        #[allow(clippy::indexing_slicing)]
        (self.dims[0], self.dims[1], self.dims[2])
    }
}

pub struct MatrixBorrowedMut<'a, const D: usize> {
    data: &'a mut [MatType],
    dims: [usize; D],
}

impl<'a, const D: usize> MatrixBorrowedMut<'a, D> {
    pub fn as_borrowed(&self) -> MatrixBorrowed<D> {
        MatrixBorrowed {
            data: self.data,
            dims: self.dims,
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [MatType] {
        self.data
    }

    pub fn copy_submatrix<const M: usize>(&mut self, from: usize, to: usize) {
        let (range_from, _) = self.as_borrowed().submatrix_range::<M>(from);
        let (range_to, _) = self.as_borrowed().submatrix_range::<M>(to);
        // TODO: The following function call is panicky
        self.data.copy_within(range_from, range_to.start);
    }

    #[must_use]
    pub fn add(&mut self, other: MatrixBorrowed<'_, D>) -> Option<()> {
        debug_assert_eq!(self.dims, other.dims);
        // TODO: Vectorize?
        for i in 0..self.data.len() {
            *self.data.get_mut(i)? += other.data.get(i)?;
        }
        Some(())
    }

    pub fn softmax_transform(&mut self) {
        let sm = self
            .data
            .iter()
            .fold(0.0, |sm, v| sm + (*v as f32 / FACTOR as f32).exp());
        self.data.iter_mut().for_each(|v| {
            *v = to_mat_type((*v as f32 / FACTOR as f32).exp() / sm);
        });
    }
}

impl<'a> MatrixBorrowed<'a, 1> {
    #[allow(dead_code)] // could be useful
    pub fn dot_1d(&self, other: MatrixBorrowed<1>) -> MatType {
        debug_assert_eq!(self.dims, other.dims);
        unrolled_dot_1024(self.data, other.data) / FACTOR
    }
}

impl<'a> MatrixBorrowedMut<'a, 1> {
    /// Calculate the dot product of a and b, adding the result to self.
    ///
    /// Note: For better dot product efficiency, if `b` is MxN, then `a` should be N;
    /// this is the opposite of standard practice.
    pub fn add_dot_2d(&mut self, a: MatrixBorrowed<1>, b: MatrixBorrowed<2>) {
        let m = a.dim();
        let n = self.as_borrowed().dim();
        debug_assert_eq!(m, b.dim().1);
        debug_assert_eq!(n, b.dim().0);
        for i in 0..n {
            if let (Some(dest), Some(b_sub)) = (self.as_mut_slice().get_mut(i), b.submatrix::<1>(i))
            {
                *dest += unrolled_dot_1024(a.data, b_sub.data);
            } else {
                debug_assert!(false);
            }
        }
    }
}

impl<'a> MatrixBorrowedMut<'a, 2> {
    /// Calculate the dot product of a and b, adding the result to self.
    ///
    /// Self should be _MxN_; `a`, _O_; and `b`, _MxNxO_.
    pub fn add_dot_3d(&mut self, a: MatrixBorrowed<1>, b: MatrixBorrowed<3>) {
        let m = a.dim();
        let n = self.as_borrowed().dim().0 * self.as_borrowed().dim().1;
        debug_assert_eq!(m, b.dim().2);
        debug_assert_eq!(n, b.dim().0 * b.dim().1);
        // Note: The following two loops are equivalent, but the second has more opportunity for
        // vectorization since it allows the vectorization to span submatrices.
        // for i in 0..b.dim().0 {
        //     self.submatrix_mut::<1>(i).add_dot_2d(a, b.submatrix(i));
        // }
        let lhs = a.as_slice();
        for i in 0..n {
            if let (Some(dest), Some(rhs)) = (
                self.as_mut_slice().get_mut(i),
                b.as_slice().get(i * m..(i + 1) * m),
            ) {
                *dest = dest.saturating_add(unrolled_dot_1024(lhs, rhs));
            } else {
                debug_assert!(false);
            }
        }
    }
}

// Polyfill float operations with libm in case we're no_std.
#[allow(unused_imports)]
use num_traits::Float;

/// `tanh` computes the tanh function for a scalar value.
#[inline]
pub fn tanh(x: MatType) -> MatType {
    // to_mat_type((x as f32 / FACTOR as f32).tanh())
    // pocket_tanh_128(x / 4)
    tanh_1024(x)
}

/// `sigmoid` computes the sigmoid function for a scalar value.
#[inline]
pub fn sigmoid(x: MatType) -> MatType {
    // to_mat_type(1.0 / (1.0 + (-(x as f32) / FACTOR as f32).exp()))
    // pocket_sigmoid_128(x / 2)
    sigmoid_1024(x)
}

/// Approximation of 128*sigmoid(x/32)
fn pocket_sigmoid_128(x: i16) -> i16 {
    if x <= -128 {
        1
    } else if x <= -75 {
        x / 8 + 20
    } else if x <= -32 {
        x / 2 + 48
    } else if x <= 31 {
        x + 64
    } else if x <= 74 {
        x / 2 + 80
    } else if x <= 127 {
        x / 8 + 108
    } else {
        127
    }
}

/// Approximation of 128*tanh(x/64)
fn pocket_tanh_128(x: i16) -> i16 {
    if x <= -128 {
        -128
    } else if x <= -75 {
        x / 4 - 88
    } else if x <= -32 {
        x - 32
    } else if x <= 31 {
        2 * x
    } else if x <= 74 {
        x + 32
    } else if x <= 127 {
        x / 4 + 88
    } else {
        127
    }
}

fn tanh_1024(x: MatType) -> MatType {
    let y = match x.unsigned_abs() {
        x if x <= 273 => x,
        x if x <= 567 => x * 5 / 6 + 46,
        x if x <= 788 => x * 2 / 3 + 140,
        x if x <= 999 => x * 1 / 2 + 272,
        x if x <= 1210 => x * 3 / 8 + 396,
        x if x <= 1527 => x * 1 / 4 + 548,
        x if x <= 1920 => x * 1 / 8 + 738,
        x if x <= 2286 => x * 1 / 16 + 859,
        x if x <= 2604 => x * 1 / 32 + 930,
        x if x <= 3335 => x * 1 / 64 + 971,
        _ => 1023,
    };
    if x < 0 {
        -(y as i16)
    } else {
        y as i16
    }
}

#[test]
fn test_tanh_1024() {
    for x in -4000..4000 {
        let tanh_exact = 1024.0 * ((x as f32).tanh() / 1024.0);
        let tanh_appx = tanh_1024(x);
        assert!(
            (tanh_exact - tanh_appx as f32).abs() < 10.0,
            "tanh: {x} {tanh_appx} {tanh_exact}"
        );
    }
}

fn sigmoid_1024(x: MatType) -> MatType {
    let y = match x.unsigned_abs() {
        x if x <= 470 => x * 1 / 4 + 512,
        x if x <= 958 => x * 7 / 32 + 527,
        x if x <= 1293 => x * 3 / 16 + 557,
        x if x <= 1629 => x * 5 / 32 + 597,
        x if x <= 1992 => x * 1 / 8 + 648,
        x if x <= 2423 => x * 3 / 32 + 710,
        x if x <= 3054 => x * 1 / 16 + 786,
        x if x <= 3843 => x * 1 / 32 + 881,
        x if x <= 4487 => x * 1 / 64 + 941,
        x if x <= 5956 => x * 1 / 128 + 976,
        _ => 1023,
    };
    if x < 0 {
        1024 - y as i16
    } else {
        y as i16
    }
}

#[test]
fn test_sigmoid_1024() {
    for x in -4000..4000 {
        let sigmoid_exact = 1024.0 / (1.0 + (-x as f32 / 1024.0));
        let sigmoid_appx = sigmoid_1024(x);
        assert!(
            (sigmoid_exact - sigmoid_appx as f32).abs() < 10.0,
            "sigmoid: {x} {sigmoid_appx} {sigmoid_exact}"
        );
    }
}

/// Compute the dot product.
///
/// `xs` and `ys` must be the same length
///
/// (From ndarray 0.15.6)
#[allow(clippy::indexing_slicing)] // all indexing is protected by the entry assertion
fn unrolled_dot(xs: &[MatType], ys: &[MatType]) -> MatType {
    debug_assert_eq!(xs.len(), ys.len());
    if xs.len() != ys.len() {
        return ZERO;
    }
    // eightfold unrolled so that floating point can be vectorized
    // (even with strict floating point accuracy semantics)
    let len = core::cmp::min(xs.len(), ys.len());
    let mut xs = &xs[..len];
    let mut ys = &ys[..len];
    let mut sum: MatType = ZERO;
    let (mut p0, mut p1, mut p2, mut p3, mut p4, mut p5, mut p6, mut p7) =
        (ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO);
    while xs.len() >= 8 {
        p0 += xs[0] * ys[0] / FACTOR;
        p1 += xs[1] * ys[1] / FACTOR;
        p2 += xs[2] * ys[2] / FACTOR;
        p3 += xs[3] * ys[3] / FACTOR;
        p4 += xs[4] * ys[4] / FACTOR;
        p5 += xs[5] * ys[5] / FACTOR;
        p6 += xs[6] * ys[6] / FACTOR;
        p7 += xs[7] * ys[7] / FACTOR;

        xs = &xs[8..];
        ys = &ys[8..];
    }
    sum += p0 + p4;
    sum += p1 + p5;
    sum += p2 + p6;
    sum += p3 + p7;

    for (i, (&x, &y)) in xs.iter().zip(ys).enumerate() {
        if i >= 7 {
            break;
        }
        sum += x * y / FACTOR;
    }
    sum
}

#[allow(clippy::indexing_slicing)] // all indexing is protected by the entry assertion
fn unrolled_dot_1024(xs: &[i16], ys: &[i16]) -> i16 {
    debug_assert_eq!(FACTOR, 1024);
    debug_assert_eq!(xs.len(), ys.len());
    if xs.len() != ys.len() {
        return 0;
    }
    // eightfold unrolled so that floating point can be vectorized
    // (even with strict floating point accuracy semantics)
    let len = core::cmp::min(xs.len(), ys.len());
    let mut xs = &xs[..len];
    let mut ys = &ys[..len];
    let mut sum = 0i16;
    let (mut p0, mut p1, mut p2, mut p3, mut p4, mut p5, mut p6, mut p7) =
        (0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16, 0i16);
    while xs.len() >= 8 {
        p0 = p0.saturating_add(saturating_i16_mul_div_1024(xs[0], ys[0]));
        p1 = p1.saturating_add(saturating_i16_mul_div_1024(xs[1], ys[1]));
        p2 = p2.saturating_add(saturating_i16_mul_div_1024(xs[2], ys[2]));
        p3 = p3.saturating_add(saturating_i16_mul_div_1024(xs[3], ys[3]));
        p4 = p4.saturating_add(saturating_i16_mul_div_1024(xs[4], ys[4]));
        p5 = p5.saturating_add(saturating_i16_mul_div_1024(xs[5], ys[5]));
        p6 = p6.saturating_add(saturating_i16_mul_div_1024(xs[6], ys[6]));
        p7 = p7.saturating_add(saturating_i16_mul_div_1024(xs[7], ys[7]));

        xs = &xs[8..];
        ys = &ys[8..];
    }
    sum = sum.saturating_add(p0.saturating_add(p4));
    sum = sum.saturating_add(p1.saturating_add(p5));
    sum = sum.saturating_add(p2.saturating_add(p6));
    sum = sum.saturating_add(p3.saturating_add(p7));

    for (i, (&x, &y)) in xs.iter().zip(ys).enumerate() {
        if i >= 7 {
            break;
        }
        sum = sum.saturating_add(saturating_i16_mul_div_1024(x, y));
    }
    sum
}
