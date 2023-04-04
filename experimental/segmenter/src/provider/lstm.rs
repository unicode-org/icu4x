// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for the lstm

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

use crate::math_helper::MatrixZero;
use icu_provider::prelude::*;
use zerovec::{ZeroMap, ZeroVec};

/// The struct that stores a LSTM's matrix.
#[derive(PartialEq, Debug, Clone)]
pub struct LstmMatrix<'data, const D: usize> {
    // Invariant: dims.product() == data.len()
    #[allow(missing_docs)]
    dims: [u16; D],
    #[allow(missing_docs)]
    data: ZeroVec<'data, f32>,
}

// TODO: make the yoke derive work with const generics
unsafe impl<'a, const D: usize> yoke::Yokeable<'a> for LstmMatrix<'static, D> {
    type Output = LstmMatrix<'a, D>;

    #[inline]
    fn transform(&'a self) -> &'a Self::Output {
        self
    }

    #[inline]
    fn transform_owned(self) -> Self::Output {
        self
    }

    #[inline]
    unsafe fn make(this: Self::Output) -> Self {
        use core::{mem, ptr};
        let ptr: *const Self = (&this as *const Self::Output).cast();
        mem::forget(this);
        ptr::read(ptr)
    }

    #[inline]
    fn transform_mut<F>(&'a mut self, f: F)
    where
        F: 'static + for<'b> FnOnce(&'b mut Self::Output),
    {
        unsafe {
            f(core::mem::transmute::<&'a mut Self, &'a mut Self::Output>(
                self,
            ))
        }
    }
}

// TODO: make the zerofrom derive work with const generics
impl<'zf, 'zf_inner, const D: usize> zerofrom::ZeroFrom<'zf, LstmMatrix<'zf_inner, D>>
    for LstmMatrix<'zf, D>
{
    fn zero_from(this: &'zf LstmMatrix<'zf_inner, D>) -> Self {
        LstmMatrix {
            dims: this.dims,
            data: zerofrom::ZeroFrom::zero_from(&this.data),
        }
    }
}

impl<'data, const D: usize> LstmMatrix<'data, D> {
    #[cfg(any(feature = "serde", feature = "datagen"))]
    /// Creates a LstmMatrix with the given dimensions. Fails if the dimensions don't match the data.
    pub fn from_parts(dims: [u16; D], data: ZeroVec<'data, f32>) -> Result<Self, DataError> {
        if dims.iter().map(|&i| i as usize).product::<usize>() != data.len() {
            Err(DataError::custom("Dimension mismatch"))
        } else {
            Ok(Self { dims, data })
        }
    }

    #[doc(hidden)] // databake
    pub const fn from_parts_unchecked(dims: [u16; D], data: ZeroVec<'data, f32>) -> Self {
        Self { dims, data }
    }

    pub(crate) fn as_matrix_zero(&self) -> MatrixZero<D> {
        MatrixZero::from_parts_unchecked(&self.data, self.dims.map(|x| x as usize))
    }
}

#[cfg(feature = "serde")]
impl<'de: 'data, 'data, const D: usize> serde::Deserialize<'de> for LstmMatrix<'data, D> {
    fn deserialize<S>(deserializer: S) -> Result<Self, S::Error>
    where
        S: serde::de::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Raw<'data> {
            // no const generic support in serde
            dims: alloc::borrow::Cow<'data, [u16]>,
            #[serde(borrow)]
            data: ZeroVec<'data, f32>,
        }

        let raw = Raw::deserialize(deserializer)?;

        use core::ops::Deref;
        use serde::de::Error;
        Self::from_parts(
            raw.dims
                .deref()
                .try_into()
                .map_err(|_| S::Error::custom("Dimension mismatch"))?,
            raw.data,
        )
        .map_err(|_| S::Error::custom("Dimension mismatch"))
    }
}

#[cfg(feature = "datagen")]
impl<const D: usize> serde::Serialize for LstmMatrix<'_, D> {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("LstmMatrix", 2)?;
        state.serialize_field("dims", self.dims.as_slice())?;
        state.serialize_field("data", &self.data)?;
        state.end()
    }
}

#[cfg(feature = "datagen")]
impl<const D: usize> databake::Bake for LstmMatrix<'_, D> {
    fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
        let dims = self.dims.bake(env);
        let data = self.data.bake(env);
        databake::quote! {
            icu_segmenter::provider::LstmMatrix::from_parts_unchecked(#dims, #data)
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize,databake::Bake),
    databake(path = icu_segmenter::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
/// The type of LSTM model
pub enum ModelType {
    /// A model working on code points
    Codepoints,
    /// A model working on grapheme clusters
    GraphemeClusters,
}

/// The struct that stores a LSTM model.
#[icu_provider::data_struct(LstmDataV1Marker = "segmenter/lstm@1")]
#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[yoke(prove_covariance_manually)]
pub struct LstmDataV1<'data> {
    /// Type of the model
    pub(crate) model: ModelType,
    /// The grapheme cluster dictionary used to train the model
    pub(crate) dic: ZeroMap<'data, str, u16>,
    /// The embedding layer. Shape (dic.len + 1, e)
    pub(crate) embedding: LstmMatrix<'data, 2>,
    /// The forward layer's first matrix. Shape (h, 4, e)
    pub(crate) fw_w: LstmMatrix<'data, 3>,
    /// The forward layer's second matrix. Shape (h, 4, h)
    pub(crate) fw_u: LstmMatrix<'data, 3>,
    /// The forward layer's bias. Shape (h, 4)
    pub(crate) fw_b: LstmMatrix<'data, 2>,
    /// The backward layer's first matrix. Shape (h, 4, e)
    pub(crate) bw_w: LstmMatrix<'data, 3>,
    /// The backward layer's second matrix. Shape (h, 4, h)
    pub(crate) bw_u: LstmMatrix<'data, 3>,
    /// The backward layer's bias. Shape (h, 4)
    pub(crate) bw_b: LstmMatrix<'data, 2>,
    /// The output layer's weights. Shape (2, 4, h)
    pub(crate) time_w: LstmMatrix<'data, 3>,
    /// The output layer's bias. Shape (4)
    pub(crate) time_b: LstmMatrix<'data, 1>,
}

impl<'data> LstmDataV1<'data> {
    #[doc(hidden)] // databake
    #[allow(clippy::too_many_arguments)] // constructor
    pub const fn from_parts_unchecked(
        model: ModelType,
        dic: ZeroMap<'data, str, u16>,
        embedding: LstmMatrix<'data, 2>,
        fw_w: LstmMatrix<'data, 3>,
        fw_u: LstmMatrix<'data, 3>,
        fw_b: LstmMatrix<'data, 2>,
        bw_w: LstmMatrix<'data, 3>,
        bw_u: LstmMatrix<'data, 3>,
        bw_b: LstmMatrix<'data, 2>,
        time_w: LstmMatrix<'data, 3>,
        time_b: LstmMatrix<'data, 1>,
    ) -> Self {
        Self {
            model,
            dic,
            embedding,
            fw_w,
            fw_u,
            fw_b,
            bw_w,
            bw_u,
            bw_b,
            time_w,
            time_b,
        }
    }

    #[cfg(any(feature = "serde", feature = "datagen"))]
    /// Creates a LstmDataV1 with the given data. Fails if the matrix dimensions are inconsisent.
    #[allow(clippy::too_many_arguments)] // constructor
    pub fn try_from_parts(
        model: ModelType,
        dic: ZeroMap<'data, str, u16>,
        embedding: LstmMatrix<'data, 2>,
        fw_w: LstmMatrix<'data, 3>,
        fw_u: LstmMatrix<'data, 3>,
        fw_b: LstmMatrix<'data, 2>,
        bw_w: LstmMatrix<'data, 3>,
        bw_u: LstmMatrix<'data, 3>,
        bw_b: LstmMatrix<'data, 2>,
        time_w: LstmMatrix<'data, 3>,
        time_b: LstmMatrix<'data, 1>,
    ) -> Result<Self, DataError> {
        let dic_len = u16::try_from(dic.len())
            .map_err(|_| DataError::custom("Dictionary does not fit in u16"))?;

        if embedding.dims[0] - 1 != dic_len
            || fw_w.dims != [fw_u.dims[0], 4, embedding.dims[1]]
            || fw_u.dims != [fw_u.dims[0], 4, fw_u.dims[0]]
            || fw_b.dims != [fw_u.dims[0], 4]
            || bw_w.dims != [fw_u.dims[0], 4, embedding.dims[1]]
            || bw_u.dims != [fw_u.dims[0], 4, fw_u.dims[0]]
            || bw_b.dims != [fw_u.dims[0], 4]
            || time_w.dims != [2, 4, fw_u.dims[0]]
            || time_b.dims != [4]
        {
            return Err(DataError::custom("LSTM dimension mismatch"));
        }

        #[cfg(debug_assertions)]
        if !dic.iter_copied_values().all(|(_, g)| g < dic_len) {
            return Err(DataError::custom("Invalid cluster id"));
        }

        Ok(Self {
            model,
            dic,
            embedding,
            fw_w,
            fw_u,
            fw_b,
            bw_w,
            bw_u,
            bw_b,
            time_w,
            time_b,
        })
    }
}

#[cfg(feature = "serde")]
impl<'de: 'data, 'data> serde::Deserialize<'de> for LstmDataV1<'data> {
    fn deserialize<S>(deserializer: S) -> Result<Self, S::Error>
    where
        S: serde::de::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct Raw<'data> {
            model: ModelType,
            #[cfg_attr(feature = "serde", serde(borrow))]
            dic: ZeroMap<'data, str, u16>,
            #[cfg_attr(feature = "serde", serde(borrow))]
            embedding: LstmMatrix<'data, 2>,
            #[cfg_attr(feature = "serde", serde(borrow))]
            fw_w: LstmMatrix<'data, 3>,
            #[cfg_attr(feature = "serde", serde(borrow))]
            fw_u: LstmMatrix<'data, 3>,
            #[cfg_attr(feature = "serde", serde(borrow))]
            fw_b: LstmMatrix<'data, 2>,
            #[cfg_attr(feature = "serde", serde(borrow))]
            bw_w: LstmMatrix<'data, 3>,
            #[cfg_attr(feature = "serde", serde(borrow))]
            bw_u: LstmMatrix<'data, 3>,
            #[cfg_attr(feature = "serde", serde(borrow))]
            bw_b: LstmMatrix<'data, 2>,
            #[cfg_attr(feature = "serde", serde(borrow))]
            time_w: LstmMatrix<'data, 3>,
            #[cfg_attr(feature = "serde", serde(borrow))]
            time_b: LstmMatrix<'data, 1>,
        }

        let raw = Raw::deserialize(deserializer)?;

        use serde::de::Error;
        Self::try_from_parts(
            raw.model,
            raw.dic,
            raw.embedding,
            raw.fw_w,
            raw.fw_u,
            raw.fw_b,
            raw.bw_w,
            raw.bw_u,
            raw.bw_b,
            raw.time_w,
            raw.time_b,
        )
        .map_err(|_| S::Error::custom("Invalid dimensions"))
    }
}

#[cfg(feature = "datagen")]
impl databake::Bake for LstmDataV1<'_> {
    fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
        let model = self.model.bake(env);
        let dic = self.dic.bake(env);
        let embedding = self.embedding.bake(env);
        let fw_w = self.fw_w.bake(env);
        let fw_u = self.fw_u.bake(env);
        let fw_b = self.fw_b.bake(env);
        let bw_w = self.bw_w.bake(env);
        let bw_u = self.bw_u.bake(env);
        let bw_b = self.bw_b.bake(env);
        let time_w = self.time_w.bake(env);
        let time_b = self.time_b.bake(env);
        databake::quote! {
            icu_segmenter::provider::LstmDataV1::from_parts_unchecked(
                #model,
                #dic,
                #embedding,
                #fw_w,
                #fw_u,
                #fw_b,
                #bw_w,
                #bw_u,
                #bw_b,
                #time_w,
                #time_b,
            )
        }
    }
}

#[cfg(feature = "datagen")]
impl databake::Bake for LstmDataV1Marker {
    fn bake(&self, _env: &databake::CrateEnv) -> databake::TokenStream {
        databake::quote! {
            ::icu_segmenter::provider::LstmDataV1Marker
        }
    }
}
