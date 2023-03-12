// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::ops::Range;

use ndarray::{concatenate, Array1, Array2, ArrayBase, Axis, Dim, ViewRepr, Dimension, OwnedRepr};

#[derive(Debug, Clone)]
pub struct MatrixOwned<const D: usize> {
    data: Vec<f32>,
    dims: [usize; D],
}

impl<const D: usize> MatrixOwned<D> {
    pub fn as_borrowed(&self) -> MatrixBorrowed<D> {
        MatrixBorrowed {
            data: &self.data,
            dims: self.dims
        }
    }

    pub fn from_ndarray(nd: ArrayBase<OwnedRepr<f32>, Dim<[usize; D]>>) -> Self where Dim<[usize; D]>: Dimension {
        let dims = nd.shape().try_into().unwrap();
        Self {
            data: nd.into_raw_vec(),
            dims,
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
    pub fn submatrix<const M: usize>(&self, index: usize) -> MatrixBorrowed<M> {
        assert_eq!(M, D-1);
        let (range, sub_dims) = self.as_borrowed().submatrix_range(index);
        MatrixBorrowed { data: &self.data[range], dims: sub_dims }
    }

    pub fn as_mut(&mut self) -> MatrixBorrowedMut<D> {
        MatrixBorrowedMut {
            data: &mut self.data,
            dims: self.dims
        }
    }

    #[inline]
    pub fn submatrix_mut<const M: usize>(&mut self, index: usize) -> MatrixBorrowedMut<M> {
        assert_eq!(M, D-1);
        let (range, sub_dims) = self.as_borrowed().submatrix_range(index);
        MatrixBorrowedMut { data: &mut self.data[range], dims: sub_dims }
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

    pub fn to_owned(&self) -> MatrixOwned<D> {
        MatrixOwned {
            data: self.data.to_vec(),
            dims: self.dims
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

    pub fn from_ndarray(nd: &'a ArrayBase<ViewRepr<&f32>, Dim<[usize; D]>>) -> Self where Dim<[usize; D]>: Dimension {
        Self {
            data: nd.as_slice().unwrap(),
            dims: nd.shape().try_into().unwrap()
        }
    }

    #[inline]
    pub fn submatrix<const M: usize>(&self, index: usize) -> MatrixBorrowed<'a, M> {
        // This assertion should always fail and be elided at compile time
        assert_eq!(M, D-1);
        let (range, sub_dims) = self.submatrix_range(index);
        MatrixBorrowed { data: &self.data[range], dims: sub_dims }
    }

    #[inline]
    fn submatrix_range<const M: usize>(&self, index: usize) -> (Range<usize>, [usize; M]) {
        let sub_dims: [usize; M] = self.dims[1..].try_into().unwrap();
        let n = sub_dims.iter().product::<usize>();
        (n*index .. n*(index+1), sub_dims)
    }
}

impl<'a> MatrixBorrowed<'a, 1> {
    pub fn dim(&self) -> usize {
        // The type parameter guarantees that self.dims has 1 element
        #[allow(clippy::indexing_slicing)]
        self.dims[0]
    }

    pub fn read_4(&self) -> (f32, f32, f32, f32) {
        debug_assert_eq!(self.dims, [4]);
        debug_assert_eq!(self.data.len(), 4);
        if self.data.len() == 4 {
            // Safety: self.data has length 4
            unsafe {
                (*self.data.get_unchecked(0), *self.data.get_unchecked(1), *self.data.get_unchecked(2), *self.data.get_unchecked(3))
            }
        } else {
            (0.0, 0.0, 0.0, 0.0)
        }
    }
}

impl<'a> MatrixBorrowed<'a, 2> {
    pub fn dim(&self) -> (usize, usize) {
        (self.dims[0], self.dims[1])
    }
}

impl<'a> MatrixBorrowed<'a, 3> {
    pub fn dim(&self) -> (usize, usize, usize) {
        (self.dims[0], self.dims[1], self.dims[2])
    }
}

pub struct MatrixBorrowedMut<'a, const D: usize> {
    data: &'a mut [f32],
    dims: [usize; D],
}

impl<'a, const D: usize> MatrixBorrowedMut<'a, D> {
    pub fn as_borrowed(&self) -> MatrixBorrowed<D> {
        MatrixBorrowed {
            data: &self.data,
            dims: self.dims
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [f32] {
        self.data
    }

    pub fn submatrix_mut<const M: usize>(&mut self, index: usize) -> MatrixBorrowedMut<M> {
        assert_eq!(M, D-1);
        let (range, sub_dims) = self.as_borrowed().submatrix_range(index);
        MatrixBorrowedMut { data: &mut self.data[range], dims: sub_dims }
    }

    pub fn copy_submatrix<const M: usize>(&mut self, from: usize, to: usize) {
        let (range_from, _) = self.as_borrowed().submatrix_range::<M>(from);
        let (range_to, _) = self.as_borrowed().submatrix_range::<M>(to);
        self.data.copy_within(range_from, range_to.start)
    }

    pub fn set_to(&mut self, other: MatrixBorrowed<'_, D>) {
        assert_eq!(self.dims, other.dims);
        self.data.copy_from_slice(other.data);
    }

    pub fn add(&mut self, other: MatrixBorrowed<'_, D>) {
        assert_eq!(self.dims, other.dims);
        // TODO: Vectorize?
        for i in 0..self.data.len() {
            self.data[i] += other.data[i];
        }
    }

    pub fn to_softmax(&mut self) {
        let sm = self.data.iter().fold(0.0, |sm, v| sm + v.exp());
        self.data.iter_mut().for_each(|v| {
            *v = v.exp() / sm;
        });
    }
}

impl<'a> MatrixBorrowed<'a, 1> {
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
            self.as_mut_slice()[i] += unrolled_dot(a.data, b.submatrix::<1>(i).data);
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
            let rhs = &b.as_slice()[i*m..(i+1)*m];
            self.as_mut_slice()[i] += unrolled_dot(lhs, rhs);
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

/// `softmax` gets a 1d array of `f32` numbers, and compute the softmax probability for each element in the array.
pub fn softmax(arr: Array1<f32>) -> Array1<f32> {
    let sm = arr.fold(0.0, |sm, v| sm + v.exp());
    arr.map(|v| v.exp() / sm)
}

/// `max_arr1` returns the index of the maximum value in a 1d array.
pub fn max_arr1(arr: ArrayBase<ViewRepr<&f32>, Dim<[usize; 1]>>) -> usize {
    // No argmax in rust-ndarray (https://github.com/rust-ndarray/ndarray/issues/416)
    let mut mx: f32 = 0.0;
    let mut ind = 0;
    arr.indexed_iter().for_each(|(i, v)| {
        if mx < *v {
            mx = *v;
            ind = i
        }
    });
    ind
}

/// `change_row` gets one 2d array (`arr`), one 1d array (`arr1`), and an index (`row_id`), and replaces the `row_id`-th row of
/// `arr` with `arr1`
pub fn change_row(mut arr: Array2<f32>, row_id: usize, new_row: &Array1<f32>) -> Array2<f32> {
    arr.row_mut(row_id).assign(new_row);
    arr
}

/// `concatenate_arr1` concatenates two 1d arrays to form another 1d array.
pub fn concatenate_arr1(
    arr1: ArrayBase<ViewRepr<&f32>, Dim<[usize; 1]>>,
    arr2: ArrayBase<ViewRepr<&f32>, Dim<[usize; 1]>>,
) -> Array1<f32> {
    concatenate![Axis(0), arr1, arr2]
}

/// Compute the dot product.
///
/// `xs` and `ys` must be the same length
///
/// (From ndarray 0.15.6)
pub fn unrolled_dot(xs: &[f32], ys: &[f32]) -> f32
{
    debug_assert_eq!(xs.len(), ys.len());
    // eightfold unrolled so that floating point can be vectorized
    // (even with strict floating point accuracy semantics)
    let len = core::cmp::min(xs.len(), ys.len());
    let mut xs = &xs[..len];
    let mut ys = &ys[..len];
    let mut sum = 0.0;
    let (mut p0, mut p1, mut p2, mut p3, mut p4, mut p5, mut p6, mut p7) = (
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0,
    );
    while xs.len() >= 8 {
        p0 = p0 + xs[0] * ys[0];
        p1 = p1 + xs[1] * ys[1];
        p2 = p2 + xs[2] * ys[2];
        p3 = p3 + xs[3] * ys[3];
        p4 = p4 + xs[4] * ys[4];
        p5 = p5 + xs[5] * ys[5];
        p6 = p6 + xs[6] * ys[6];
        p7 = p7 + xs[7] * ys[7];

        xs = &xs[8..];
        ys = &ys[8..];
    }
    sum = sum + (p0 + p4);
    sum = sum + (p1 + p5);
    sum = sum + (p2 + p6);
    sum = sum + (p3 + p7);

    for (i, (&x, &y)) in xs.iter().zip(ys).enumerate() {
        if i >= 7 {
            break;
        }
        sum = sum + x * y;
    }
    sum
}
