// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::vec;
use alloc::vec::Vec;
use core::ops::Range;

use ndarray::{ArrayBase, Dim, Dimension, OwnedRepr};

#[derive(Debug, Clone)]
pub struct MatrixOwned<const D: usize> {
    data: Vec<f32>,
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

    pub fn from_ndarray(nd: ArrayBase<OwnedRepr<f32>, Dim<[usize; D]>>) -> Self
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
            data: vec![0.0; total_len],
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
    data: &'a [f32],
    dims: [usize; D],
}

impl<'a, const D: usize> MatrixBorrowed<'a, D> {
    #[cfg(debug_assertions)]
    pub fn debug_assert_dims(&self, dims: [usize; D]) {
        debug_assert_eq!(dims, self.dims);
        let expected_len = dims.iter().product::<usize>();
        debug_assert_eq!(expected_len, self.data.len());
    }

    pub fn as_slice(&self) -> &'a [f32] {
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
        let mut mx: f32 = 0.0;
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
        // This assertion should always succeed and be elided at compile time
        assert_eq!(M, D - 1);
        let (range, dims) = self.submatrix_range(index);
        let data = &self.data.get(range)?;
        Some(MatrixBorrowed { data, dims })
    }

    #[inline]
    fn submatrix_range<const M: usize>(&self, index: usize) -> (Range<usize>, [usize; M]) {
        // This assertion should always succeed and be elided at compile time
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
        let [dim] = self.dims;
        dim
    }

    pub fn read_4(&self) -> Option<[f32; 4]> {
        <&[f32; 4]>::try_from(self.data).ok().copied()
    }
}

impl<'a> MatrixBorrowed<'a, 2> {
    pub fn dim(&self) -> (usize, usize) {
        let [d0, d1] = self.dims;
        (d0, d1)
    }
}

impl<'a> MatrixBorrowed<'a, 3> {
    pub fn dim(&self) -> (usize, usize, usize) {
        let [d0, d1, d2] = self.dims;
        (d0, d1, d2)
    }
}

pub struct MatrixBorrowedMut<'a, const D: usize> {
    data: &'a mut [f32],
    dims: [usize; D],
}

impl<'a, const D: usize> MatrixBorrowedMut<'a, D> {
    pub fn as_borrowed(&self) -> MatrixBorrowed<D> {
        MatrixBorrowed {
            data: self.data,
            dims: self.dims,
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [f32] {
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
        let sm = self.data.iter().fold(0.0, |sm, v| sm + v.exp());
        self.data.iter_mut().for_each(|v| {
            *v = v.exp() / sm;
        });
    }
}

impl<'a> MatrixBorrowed<'a, 1> {
    #[allow(dead_code)] // could be useful
    pub fn dot_1d(&self, other: MatrixBorrowed<1>) -> f32 {
        debug_assert_eq!(self.dims, other.dims);
        unrolled_dot(self.data, other.data)
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
                *dest += unrolled_dot(a.data, b_sub.data);
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
                *dest += unrolled_dot(lhs, rhs);
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
pub fn tanh(x: f32) -> f32 {
    x.tanh()
}

/// `sigmoid` computes the sigmoid function for a scalar value.
#[inline]
pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

/// Compute the dot product.
///
/// `xs` and `ys` must be the same length
///
/// (From ndarray 0.15.6)
#[allow(clippy::indexing_slicing)] // all indexing is protected by the entry assertion
fn unrolled_dot(xs: &[f32], ys: &[f32]) -> f32 {
    debug_assert_eq!(xs.len(), ys.len());
    if xs.len() != ys.len() {
        return 0.0;
    }
    // eightfold unrolled so that floating point can be vectorized
    // (even with strict floating point accuracy semantics)
    let len = core::cmp::min(xs.len(), ys.len());
    let mut xs = &xs[..len];
    let mut ys = &ys[..len];
    let mut sum = 0.0;
    let (mut p0, mut p1, mut p2, mut p3, mut p4, mut p5, mut p6, mut p7) =
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    while xs.len() >= 8 {
        p0 += xs[0] * ys[0];
        p1 += xs[1] * ys[1];
        p2 += xs[2] * ys[2];
        p3 += xs[3] * ys[3];
        p4 += xs[4] * ys[4];
        p5 += xs[5] * ys[5];
        p6 += xs[6] * ys[6];
        p7 += xs[7] * ys[7];

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
        sum += x * y;
    }
    sum
}
