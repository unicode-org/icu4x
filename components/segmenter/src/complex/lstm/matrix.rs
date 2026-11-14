// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::ops::Range;
use zerovec::ZeroSlice;

/// A `D`-dimensional matrix borrowed from a [`ZeroSlice`].
#[derive(Debug, Clone, Copy)]
pub(super) struct MatrixZero<'a, const D: usize> {
    data: &'a ZeroSlice<f32>,
    dims: [usize; D],
}

impl<'a> From<&'a crate::provider::LstmMatrix1<'a>> for MatrixZero<'a, 1> {
    fn from(other: &'a crate::provider::LstmMatrix1<'a>) -> Self {
        Self {
            data: &other.data,
            dims: other.dims.map(|x| x as usize),
        }
    }
}

impl<'a> From<&'a crate::provider::LstmMatrix2<'a>> for MatrixZero<'a, 2> {
    fn from(other: &'a crate::provider::LstmMatrix2<'a>) -> Self {
        Self {
            data: &other.data,
            dims: other.dims.map(|x| x as usize),
        }
    }
}

impl<'a> From<&'a crate::provider::LstmMatrix3<'a>> for MatrixZero<'a, 3> {
    fn from(other: &'a crate::provider::LstmMatrix3<'a>) -> Self {
        Self {
            data: &other.data,
            dims: other.dims.map(|x| x as usize),
        }
    }
}

impl<'a, const D: usize> MatrixZero<'a, D> {
    pub(super) fn as_slice(&self) -> &ZeroSlice<f32> {
        self.data
    }

    /// Returns the `index`-th submatrix of one dimension less; for example,
    /// indexing into a matrix of size 5x4x3 returns a matrix of size 4x3.
    #[inline]
    pub(super) fn submatrix<const M: usize>(&self, index: usize) -> Option<MatrixZero<'a, M>> {
        // This assertion is based on const generics; it should always succeed and be elided.
        assert_eq!(M, D - 1);
        let (range, dims) = self.submatrix_range(index);
        let data = &self.data.get_subslice(range)?;
        Some(MatrixZero { data, dims })
    }

    #[inline]
    fn submatrix_range<const M: usize>(&self, index: usize) -> (Range<usize>, [usize; M]) {
        // This assertion is based on const generics; it should always succeed and be elided.
        assert_eq!(M, D - 1);
        // The above assertion guarantees that the following line will succeed
        #[expect(clippy::unwrap_used)]
        let sub_dims: [usize; M] = self.dims[1..].try_into().unwrap();
        let n = sub_dims.iter().product::<usize>();
        (n * index..n * (index + 1), sub_dims)
    }
}

impl MatrixZero<'_, 3> {
    pub(super) fn dim(&self) -> (usize, usize, usize) {
        let [d0, d1, d2] = self.dims;
        (d0, d1, d2)
    }
}
