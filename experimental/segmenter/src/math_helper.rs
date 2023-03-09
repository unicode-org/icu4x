// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use ndarray::{concatenate, Array1, Array2, ArrayBase, Axis, Dim, OwnedRepr, ViewRepr};

// Polyfill float operations with libm in case we're no_std.
#[allow(unused_imports)]
use num_traits::Float;

/// Approximation of 128*sigmoid(x/32)
fn pocket_sigmoid_128(x: i32) -> i32 {
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
fn pocket_tanh_128(x: i32) -> i32 {
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

#[test]
fn test_pocket_sigmoid_tanh_128() {
    for x in -150..150 {
        let sigmoid_exact = 128.0 * sigmoid(x as f32 / 32.0);
        let sigmoid_appx = pocket_sigmoid_128(x) as f32;
        assert!(
            (sigmoid_appx - sigmoid_exact).abs() < 10.0,
            "sigmoid: {x} {sigmoid_appx} {sigmoid_exact}"
        );
        let tanh_exact = 128.0 * tanh(x as f32 / 64.0);
        let tanh_appx = pocket_tanh_128(x) as f32;
        assert!(
            (tanh_appx - tanh_exact).abs() < 10.0,
            "tanh: {x} {tanh_appx} {tanh_exact}"
        );
    }
}

fn tanh_1024(x: i32) -> i32 {
    let y = match x.abs() {
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
        -y
    } else {
        y
    }
}

#[test]
fn test_tanh_1024() {
    for x in -4000..4000 {
        let tanh_exact = 1024.0 * tanh(x as f32 / 1024.0);
        let tanh_appx = tanh_1024(x);
        assert!(
            (tanh_exact - tanh_appx as f32).abs() < 10.0,
            "tanh: {x} {tanh_appx} {tanh_exact}"
        );
    }
}

fn sigmoid_1024(x: i32) -> i32 {
    let y = match x.abs() {
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
        1024 - y
    } else {
        y
    }
}

#[test]
fn test_sigmoid_1024() {
    for x in -4000..4000 {
        let sigmoid_exact = 1024.0 * sigmoid(x as f32 / 1024.0);
        let sigmoid_appx = sigmoid_1024(x);
        assert!(
            (sigmoid_exact - sigmoid_appx as f32).abs() < 10.0,
            "sigmoid: {x} {sigmoid_appx} {sigmoid_exact}"
        );
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

const FACTOR: f32 = 1024.0;

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
    arr.map(|x| sigmoid_1024(*x))
    // arr.map(|x| (sigmoid(*x as f32 / 1024.0) * 1024.0) as i32)
}

/// `tanh_arr1` computes elementwise tanh function for elements of a 1d array
pub fn tanh_arr1(arr: ArrayBase<ViewRepr<&i32>, Dim<[usize; 1]>>) -> Array1<i32> {
    arr.map(|x| tanh_1024(*x))
    // arr.map(|x| (tanh(*x as f32 / 1024.0) * 1024.0) as i32)
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
