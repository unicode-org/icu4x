// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use ndarray::{Array1, Array2, ArrayBase, Dim, ViewRepr};

/// `sigmoid` computes the sigmoid function for a scalar value.
pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

/// `softmax` gets a 1d array of `f32` numbers, and compute the softmax probability for each element in the array.
pub fn softmax(arr: Array1<f32>) -> Array1<f32> {
    let mut sm: f32 = 0.0;
    for i in 0..arr.shape()[0] {
        sm += arr[[i]].exp();
    }
    let mut out = Array1::<f32>::zeros(arr.len());
    for i in 0..out.shape()[0] {
        out[[i]] = arr[[i]].exp() / sm;
    }
    out
}

/// `max_arr1` returns the index of the maximum value in a 1d array.
pub fn max_arr1(arr: ArrayBase<ViewRepr<&f32>, Dim<[usize; 1]>>) -> usize {
    let mut mx: f32 = 0.0;
    let mut ind = 0;
    for i in 0..arr.shape()[0] {
        if mx < arr[[i]] {
            mx = arr[[i]];
            ind = i;
        }
    }
    ind
}

/// `tanh_arr1` computes elementwise sigmoid funciton for elements of a 1d array
pub fn sigmoid_arr1(arr: ArrayBase<ViewRepr<&f32>, Dim<[usize; 1]>>) -> Array1<f32> {
    let mut out = Array1::<f32>::zeros(arr.shape()[0]);
    for i in 0..arr.shape()[0] {
        out[[i]] = sigmoid(arr[i]);
    }
    out
}

/// `tanh_arr1` computes elementwise tanh funciton for elements of a 1d array
pub fn tanh_arr1(arr: ArrayBase<ViewRepr<&f32>, Dim<[usize; 1]>>) -> Array1<f32> {
    let mut out = Array1::<f32>::zeros(arr.shape()[0]);
    for i in 0..arr.shape()[0] {
        out[i] = arr[i].tanh();
    }
    out
}

/// `change_row` gets one 2d array (`arr`), one 1d array (`arr1`), and an index (`row_id`), and replaces the `row_id`-th row of
/// `arr` with `arr1`
pub fn change_row(mut arr: Array2<f32>, row_id: usize, new_row: &Array1<f32>) -> Array2<f32> {
    for i in 0..new_row.shape()[0] {
        arr[[row_id, i]] = new_row[i];
    }
    arr
}

/// `concatenate_arr1` concatenates two 1d arrays to form another 1d array.
pub fn concatenate_arr1(
    arr1: ArrayBase<ViewRepr<&f32>, Dim<[usize; 1]>>,
    arr2: ArrayBase<ViewRepr<&f32>, Dim<[usize; 1]>>,
) -> Array1<f32> {
    let mut out = Array1::<f32>::zeros(arr1.len() + arr2.len());
    for i in 0..arr1.len() {
        out[i] = arr1[i]
    }
    for i in 0..arr2.len() {
        out[arr1.len() + i] = arr2[[i]]
    }
    out
}
