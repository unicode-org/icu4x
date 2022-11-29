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

/// `tanh_arr1` computes elementwise sigmoid funciton for elements of a 1d array
pub fn sigmoid_arr1(arr: ArrayBase<ViewRepr<&f32>, Dim<[usize; 1]>>) -> Array1<f32> {
    arr.map(|v| sigmoid(*v))
}

/// `tanh_arr1` computes elementwise tanh funciton for elements of a 1d array
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
