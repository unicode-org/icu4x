// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use ndarray::{concatenate, Array1, Array2, ArrayBase, Axis, Dim, ViewRepr, Dimension};

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

    pub fn from_ndarray(nd: &'a ArrayBase<ViewRepr<&f32>, Dim<[usize; D]>>) -> Self where Dim<[usize; D]>: Dimension {
        Self {
            data: nd.as_slice().unwrap(),
            dims: nd.shape().try_into().unwrap()
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
