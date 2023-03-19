// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::lstm_error::Error;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::Range;

#[derive(Debug, Clone)]
pub struct MatrixOwned<const D: usize> {
    data: Vec<f32>,
    dims: [usize; D],
}

impl<const D: usize> MatrixOwned<D> {
    pub fn as_borrowed(&self) -> MatrixBorrowed<D> {
        MatrixBorrowed {
            data: &self.data,
            dims: self.dims,
        }
    }

    pub fn try_from_parts(data: Vec<f32>, dims: [usize; D]) -> Result<Self, Error> {
        if dims.iter().product::<usize>() == data.len() {
            Ok(Self { data, dims })
        } else {
            Err(Error::DimensionMismatch)
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
fn unrolled_dot(xs: &[f32], ys: &[f32]) -> f32 {
    debug_assert_eq!(xs.len(), ys.len());
    // eightfold unrolled so that floating point can be vectorized
    // (even with strict floating point accuracy semantics)
    let mut p = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let xit = xs.chunks_exact(8);
    let yit = ys.chunks_exact(8);
    let sum = xit
        .remainder()
        .iter()
        .zip(yit.remainder().iter())
        .map(|(x, y)| x * y)
        .sum::<f32>();
    for (xx, yy) in xit.zip(yit) {
        // TODO: Use array_chunks once stable to avoid the unwrap.
        // <https://github.com/rust-lang/rust/issues/74985>
        #[allow(clippy::unwrap_used)]
        let [x0, x1, x2, x3, x4, x5, x6, x7] = *<&[f32; 8]>::try_from(xx).unwrap();
        #[allow(clippy::unwrap_used)]
        let [y0, y1, y2, y3, y4, y5, y6, y7] = *<&[f32; 8]>::try_from(yy).unwrap();
        p.0 += x0 * y0;
        p.1 += x1 * y1;
        p.2 += x2 * y2;
        p.3 += x3 * y3;
        p.4 += x4 * y4;
        p.5 += x5 * y5;
        p.6 += x6 * y6;
        p.7 += x7 * y7;
    }
    sum + (p.0 + p.4) + (p.1 + p.5) + (p.2 + p.6) + (p.3 + p.7)
}
