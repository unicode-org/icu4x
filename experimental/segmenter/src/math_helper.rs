// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use ndarray::{concatenate, Array1, Array2, ArrayBase, Axis, Dim, ViewRepr};

// Polyfill float operations with libm in case we're no_std.
#[allow(unused_imports)]
use num_traits::Float;

/// `sigmoid` computes the sigmoid function for a scalar value.
#[inline]
fn sigmoid(x: f32) -> f32 {
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

/// `tanh_arr1` computes elementwise sigmoid function for elements of a 1d array
pub fn sigmoid_arr1(arr: ArrayBase<ViewRepr<&f32>, Dim<[usize; 1]>>) -> Array1<f32> {
    arr.map(|v| sigmoid(*v))
}

/// `tanh_arr1` computes elementwise tanh function for elements of a 1d array
pub fn tanh_arr1(arr: ArrayBase<ViewRepr<&f32>, Dim<[usize; 1]>>) -> Array1<f32> {
    arr.map(|v| v.tanh())
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

pub fn mul_mul_sum(a: &[f32], b: &[f32], c: &[f32], d: &[f32], e: &[f32]) -> Vec<f32> {
    let m = a.len();
    let n = b.len() / m;
    debug_assert_eq!(n / 4, c.len());
    debug_assert_eq!(n*n/4, d.len());
    debug_assert_eq!(n, e.len());
    let mut result = Vec::from(e);
    for i in 0..n {
        let x = result.get_mut(i).unwrap();
        *x += unrolled_dot(a, &b[i*m..(i+1)*m]);
    }
    for i in 0..n {
        let x = result.get_mut(i).unwrap();
        *x += unrolled_dot(c, &d[i*n/4..(i+1)*n/4]);
    }
    result
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
