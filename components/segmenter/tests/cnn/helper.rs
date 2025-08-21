// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use ndarray::array;

pub fn get_shape() -> Vec<usize> {
    let a1 = array![1, 2, 3, 4];
    a1.shape().to_vec()
}
