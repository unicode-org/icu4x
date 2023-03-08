// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use ndarray::{concatenate, Array1, Array2, ArrayBase, Axis, Dim, ViewRepr, OwnedRepr};

// Polyfill float operations with libm in case we're no_std.
#[allow(unused_imports)]
use num_traits::Float;

/// Approximation of 128*sigmoid(x/32)
fn pocket_sigmoid_128(x: i32) -> i32 {
    if x <= -128 {
        1
    } else if x <= -75 {
        x/8 + 20
    } else if x <= -32 {
        x/2 + 48
    } else if x <= 31 {
        x + 64
    } else if x <= 74 {
        x/2 + 80
    } else if x <= 127 {
        x/8 + 108
    } else {
        127
    }
}

/// Approximation of 128*tanh(x/64)
fn pocket_tanh_128(x: i32) -> i32 {
    if x <= -128 {
        -128
    } else if x <= -75 {
        x/4 - 88
    } else if x <= -32 {
        x - 32
    } else if x <= 31 {
        2*x
    } else if x <= 74 {
        x + 32
    } else if x <= 127 {
        x/4 + 88
    } else {
        127
    }
}

#[test]
fn test_pocket_sigmoid_tanh_128() {
    for x in -150..150 {
        let sigmoid_exact = 128.0 * sigmoid(x as f32 / 32.0);
        let sigmoid_appx = pocket_sigmoid_128(x) as f32;
        assert!((sigmoid_appx - sigmoid_exact).abs() < 10.0, "sigmoid: {x} {sigmoid_appx} {sigmoid_exact}");
        let tanh_exact = 128.0 * tanh(x as f32 / 64.0);
        let tanh_appx = pocket_tanh_128(x) as f32;
        assert!((tanh_appx - tanh_exact).abs() < 10.0, "tanh: {x} {tanh_appx} {tanh_exact}");
    }
}

/// `sigmoid` computes the sigmoid function for a scalar value.
#[inline]
fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + (-x).exp())
}

#[inline]
fn tanh(x: f32) -> f32 {
    x.tanh()
}

const FACTOR: f32 = 128.0;

/// `softmax` gets a 1d array of `f32` numbers, and compute the softmax probability for each element in the array.
pub fn softmax(arr: Array1<i32>) -> Array1<f32> {
    let sm = arr.fold(0.0, |sm, v| sm + (*v as f32 / FACTOR).exp());
    arr.map(|v| (*v as f32 / FACTOR).exp() / sm)
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

/// `max_arr1` returns the index of the maximum value in a 1d array.
pub fn max_arr1_owned(arr: &ArrayBase<OwnedRepr<f32>, Dim<[usize; 1]>>) -> usize {
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
pub fn sigmoid_arr1(arr: ArrayBase<ViewRepr<&i32>, Dim<[usize; 1]>>) -> Array1<i32> {
    arr.map(|x| pocket_sigmoid_128(x/4))
}

/// `tanh_arr1` computes elementwise tanh function for elements of a 1d array
pub fn tanh_arr1(arr: ArrayBase<ViewRepr<&i32>, Dim<[usize; 1]>>) -> Array1<i32> {
    arr.map(|x| pocket_tanh_128(x/2))
}

/// `change_row` gets one 2d array (`arr`), one 1d array (`arr1`), and an index (`row_id`), and replaces the `row_id`-th row of
/// `arr` with `arr1`
pub fn change_row(mut arr: Array2<i32>, row_id: usize, new_row: &Array1<i32>) -> Array2<i32> {
    arr.row_mut(row_id).assign(new_row);
    arr
}

/// `concatenate_arr1` concatenates two 1d arrays to form another 1d array.
pub fn concatenate_arr1(
    arr1: ArrayBase<ViewRepr<&i32>, Dim<[usize; 1]>>,
    arr2: ArrayBase<ViewRepr<&i32>, Dim<[usize; 1]>>,
) -> Array1<i32> {
    concatenate![Axis(0), arr1, arr2]
}
