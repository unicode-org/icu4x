// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use ndarray::{Array, Array1, Array2, Array3, ArrayBase, Dim, Dimension, OwnedRepr};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use zerovec::ZeroVec;

mod helper;
use helper::*;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ModelType {
    Codepoints,
    GraphemeClusters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CnnDataFloat32<'data> {
    model: ModelType,
    dic: HashMap<String, u16>,
    embedding: CnnMatrix2<'data>,
    cnn_w1: CnnMatrix3<'data>,
    cnn_b1: CnnMatrix1<'data>,
    cnn_w2: CnnMatrix3<'data>,
    cnn_b2: CnnMatrix1<'data>,
    softmax_w: CnnMatrix2<'data>,
    softmax_b: CnnMatrix1<'data>,
}

impl<'data> CnnDataFloat32<'data> {
    #[allow(clippy::too_many_arguments)]
    pub const fn from_parts_unchecked(
        model: ModelType,
        dic: HashMap<String, u16>,
        embedding: CnnMatrix2<'data>,
        cnn_w1: CnnMatrix3<'data>,
        cnn_b1: CnnMatrix1<'data>,
        cnn_w2: CnnMatrix3<'data>,
        cnn_b2: CnnMatrix1<'data>,
        softmax_w: CnnMatrix2<'data>,
        softmax_b: CnnMatrix1<'data>,
    ) -> Self {
        Self {
            model,
            dic,
            embedding,
            cnn_w1,
            cnn_b1,
            cnn_w2,
            cnn_b2,
            softmax_w,
            softmax_b,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn try_from_parts(
        model: ModelType,
        dic: HashMap<String, u16>,
        embedding: CnnMatrix2<'data>,
        cnn_w1: CnnMatrix3<'data>,
        cnn_b1: CnnMatrix1<'data>,
        cnn_w2: CnnMatrix3<'data>,
        cnn_b2: CnnMatrix1<'data>,
        softmax_w: CnnMatrix2<'data>,
        softmax_b: CnnMatrix1<'data>,
    ) -> Result<Self, &'static str> {
        let dic_len = u16::try_from(dic.len()).map_err(|_| "dictionary too big for u16")?;

        let num_classes = embedding.dims[0];
        let embed_dim = embedding.dims[1];

        if !(num_classes == dic_len || num_classes == dic_len + 1) {
            return Err("embedding rows must equal dic.len() or dic.len()+1");
        }

        // let c1   = cnn_w1.dims[0];
        // let k1   = cnn_w1.dims[2];
        let c1 = cnn_w1.dims[2];
        let k1 = cnn_w1.dims[1];
        if cnn_w1.dims[1] != embed_dim {
            return Err("cnn_w1: embed_dim mismatch");
        }
        if cnn_b1.dims != [c1] {
            return Err("cnn_b1: bias size != out-channels");
        }
        if k1 == 0 {
            return Err("cnn_w1: kernel size must be > 0");
        }

        let c2 = cnn_w2.dims[2];
        let k2 = cnn_w2.dims[0];
        if cnn_w2.dims[1] != embed_dim {
            return Err("cnn_w2: embed_dim mismatch");
        }
        if cnn_b2.dims != [c2] {
            return Err("cnn_b2: bias size != out-channels");
        }
        if k2 == 0 {
            return Err("cnn_w2: kernel size must be > 0");
        }

        let classes = softmax_b.dims[0];
        if softmax_w.dims != [c2, classes] {
            return Err("softmax_w must be [classes, conv2_out]");
        }

        Ok(Self {
            model,
            dic,
            embedding,
            cnn_w1,
            cnn_b1,
            cnn_w2,
            cnn_b2,
            softmax_w,
            softmax_b,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CnnData<'data> {
    Float32(CnnDataFloat32<'data>),
}

#[derive(Deserialize, Debug)]
struct RawCnnMatrix {
    dim: Vec<usize>,
    data: Vec<f32>,
}

const DIMENSION_MISMATCH: &str = "dimension mismatch";

impl RawCnnMatrix {
    fn to_ndarray1(&self) -> Result<Array1<f32>, &'static str> {
        if self.dim.len() == 1 {
            Ok(Array::from_vec(self.data.clone()))
        } else {
            Err(DIMENSION_MISMATCH)
        }
    }

    fn to_ndarray2(&self) -> Result<Array2<f32>, &'static str> {
        let [d0, d1] =
            *<&[usize; 2]>::try_from(self.dim.as_slice()).map_err(|_| DIMENSION_MISMATCH)?;
        Array::from_shape_vec((d0, d1), self.data.clone()).map_err(|_| DIMENSION_MISMATCH)
    }

    fn to_ndarray3(&self) -> Result<Array3<f32>, &'static str> {
        let [d0, d1, d2] =
            *<&[usize; 3]>::try_from(self.dim.as_slice()).map_err(|_| DIMENSION_MISMATCH)?;
        Array::from_shape_vec((d0, d1, d2), self.data.clone()).map_err(|_| DIMENSION_MISMATCH)
    }
}

#[derive(Deserialize, Debug)]
struct RawCnnData {
    model: String,
    dic: HashMap<String, u16>,
    #[serde(rename = "mat1")]
    embedding: RawCnnMatrix,
    #[serde(rename = "mat2")]
    cnn_w1: RawCnnMatrix,
    #[serde(rename = "mat3")]
    cnn_b1: RawCnnMatrix,
    #[serde(rename = "mat4")]
    cnn_w2: RawCnnMatrix,
    #[serde(rename = "mat5")]
    cnn_b2: RawCnnMatrix,
    #[serde(rename = "mat6")]
    softmax_w: RawCnnMatrix,
    #[serde(rename = "mat7")]
    softmax_b: RawCnnMatrix,
}

impl RawCnnData {
    fn try_convert(&self) -> Result<CnnData<'static>, String> {
        let embedding = self.embedding.to_ndarray2()?;
        // let mut cnn_w1 = self.cnn_w1.to_ndarray3()?;
        let cnn_w1 = self.cnn_w1.to_ndarray3()?;
        let cnn_b1 = self.cnn_b1.to_ndarray1()?;
        // let mut cnn_w2 = self.cnn_w2.to_ndarray3()?;
        let cnn_w2 = self.cnn_w2.to_ndarray3()?;
        let cnn_b2 = self.cnn_b2.to_ndarray1()?;
        // let mut softmax_w = self.softmax_w.to_ndarray2()?;
        let softmax_w = self.softmax_w.to_ndarray2()?;
        let softmax_b = self.softmax_b.to_ndarray1()?;
        let model = if self.model.contains("_codepoints") {
            ModelType::Codepoints
        } else {
            ModelType::GraphemeClusters
        };

        let cnn_data_float32 = CnnDataFloat32::try_from_parts(
            model,
            self.dic.iter().map(|(k, &v)| (k.clone(), v)).collect(),
            ndarray_to_cnn_matrix2(embedding)?,
            ndarray_to_cnn_matrix3(cnn_w1)?,
            ndarray_to_cnn_matrix1(cnn_b1)?,
            ndarray_to_cnn_matrix3(cnn_w2)?,
            ndarray_to_cnn_matrix1(cnn_b2)?,
            ndarray_to_cnn_matrix2(softmax_w)?,
            ndarray_to_cnn_matrix1(softmax_b)?,
        )
        .map_err(|_| "Just checked the shapes")?;
        Ok(CnnData::Float32(cnn_data_float32))
    }
}

macro_rules! cnn_matrix {
    ($name:ident, $generic:literal) => {
        #[derive(PartialEq, Debug, Clone)]
        pub struct $name<'data> {
            pub dims: [u16; $generic],
            pub data: ZeroVec<'data, f32>,
        }

        impl<'data> $name<'data> {
            pub fn from_parts(
                dims: [u16; $generic],
                data: ZeroVec<'data, f32>,
            ) -> Result<Self, &'static str> {
                let expected = dims.iter().map(|&i| i as usize).product::<usize>();
                if expected != data.len() {
                    Err("Dimension Mismatch")
                } else {
                    Ok(Self { dims, data })
                }
            }

            pub const fn from_parts_unchecked(
                dims: [u16; $generic],
                data: ZeroVec<'data, f32>,
            ) -> Self {
                Self { dims, data }
            }
        }
    };
}

cnn_matrix!(CnnMatrix1, 1);
cnn_matrix!(CnnMatrix2, 2);
cnn_matrix!(CnnMatrix3, 3);

macro_rules! convert {
    ($fn_name:ident, $matrix_name:ident, $generic:literal) => {
        fn $fn_name(
            nd: ArrayBase<OwnedRepr<f32>, Dim<[usize; $generic]>>,
        ) -> Result<$matrix_name<'static>, &'static str>
        where
            Dim<[usize; $generic]>: Dimension,
        {
            let dims = <[u16; $generic]>::try_from(
                nd.shape()
                    .iter()
                    .copied()
                    .map(u16::try_from)
                    .collect::<Result<Vec<u16>, _>>()
                    .map_err(|_| "Bounds too big for u16")?,
            )
            .map_err(|_| "Dimensions mismatch")?;
            let data = nd
                .as_slice_memory_order()
                .ok_or_else(|| "ndarray matrix not in memory order")?;
            $matrix_name::from_parts(dims, ZeroVec::alloc_from_slice(data))
                .map_err(|_| "Dimensions mismatch")
        }
    };
}

convert!(ndarray_to_cnn_matrix1, CnnMatrix1, 1);
convert!(ndarray_to_cnn_matrix2, CnnMatrix2, 2);
convert!(ndarray_to_cnn_matrix3, CnnMatrix3, 3);

fn conv1d(
    x: MatrixBorrowed<'_, 2>,
    mut y: MatrixBorrowedMut<'_, 2>,
    w: MatrixZero<'_, 3>,
    b: MatrixZero<'_, 1>,
    dilation: usize,
) {
    let (l, cin) = x.dim();
    let (k, _, cout) = w.dim();

    let pad = ((k - 1) * dilation) / 2;
    let weights: Vec<f32> = w.as_slice().iter().collect();
    let bias: Vec<f32> = b.as_slice().iter().collect();
    let mut acc = vec![0.0f32; cout];

    for t in 0..l {
        // acc = np.zeros((Cout,), dtype=x.dtype)
        acc.fill(0.0f32);
        for ki in 0..k {
            let s = {
                let s = t as isize + (ki * dilation) as isize - pad as isize;
                if s < 0 || s >= l as isize {
                    continue;
                }
                s as usize
            }; // can optimise if minibatch guarantees no padding needed
            let x_src = x.submatrix::<1>(s).unwrap().as_slice(); // x_pad[idx]
            let base = ki * cin * cout; // kernel[k]
            for (o, acc_o) in acc.iter_mut().enumerate() {
                // equivalent to np.matmul()
                let mut sum = 0.0f32;
                for (c, &xi) in x_src.iter().enumerate() {
                    sum += xi * weights[base + c * cout + o];
                }
                *acc_o += sum;
            }
        }
        let start = t * cout;
        let end = start + cout;
        let dst_row: &mut [f32] = &mut y.as_mut_slice()[start..end];
        for (o, dst) in dst_row.iter_mut().enumerate() {
            let v = acc[o] + bias[o];
            *dst = if v > 0.0 { v } else { 0.0 };
        }
    }
}

fn elementwise_max(
    a: MatrixBorrowed<'_, 2>,
    b: MatrixBorrowed<'_, 2>,
    mut y: MatrixBorrowedMut<'_, 2>,
) {
    let (l, c) = y.as_borrowed().dim();
    let aslice = a.as_slice();
    let bslice = b.as_slice();
    let dst = y.as_mut_slice();

    for t in 0..l {
        let start = t * c;
        let end = start + c;

        let ar = &aslice[start..end];
        let br = &bslice[start..end];
        let dr = &mut dst[start..end];

        for o in 0..c {
            let av = ar[o];
            let bv = br[o];
            dr[o] = if av > bv { av } else { bv };
        }
    }
}

fn dense_softmax(
    x: MatrixBorrowed<'_, 2>,
    mut y: MatrixBorrowedMut<'_, 2>,
    w: MatrixZero<'_, 2>,
    b: MatrixZero<'_, 1>,
) {
    let (l, _cin) = x.dim();
    let (_, classes) = w.dim();
    let classes_usize = classes;

    let w_zeroslice = w.as_slice();
    let b_zeroslice = b.as_slice();

    let mut acc = vec![0.0f32; classes_usize];

    for t in 0..l {
        // logits = x_row * w + b
        acc.fill(0.0);
        let x_row = x.submatrix::<1>(t).unwrap().as_slice();

        // matmul
        for (o, acc_o) in acc.iter_mut().enumerate() {
            let mut sum = 0.0f32;
            for (c, &xi) in x_row.iter().enumerate() {
                let w_val = w_zeroslice.get(c * classes_usize + o).unwrap();
                sum += xi * w_val;
            }
            sum += b_zeroslice.get(o).unwrap();
            *acc_o = sum;
        }

        // logits -= max(logits)
        let mut m = acc[0];
        for &v in &acc[1..] {
            if v > m {
                m = v;
            }
        }
        for v in acc.iter_mut() {
            *v -= m;
        }

        // probs = exp(logits); probs /= sum(probs)
        let mut s = 0.0f32;
        for v in acc.iter_mut() {
            *v = v.exp();
            s += *v;
        }
        for v in acc.iter_mut() {
            *v *= 1.0 / s;
        }

        let start = t * classes_usize;
        let end = start + classes_usize;
        y.as_mut_slice()[start..end].copy_from_slice(&acc);
    }
}

fn class_idx_to_bies(i: usize) -> char {
    match i {
        0 => 'b',
        1 => 'i',
        2 => 'e',
        3 => 's',
        _ => '?',
    }
}

fn argmax(slice: &[f32]) -> usize {
    let mut bi = 0usize;
    let mut bv = slice[0];
    for (i, &v) in slice.iter().enumerate().skip(1) {
        if v > bv {
            bv = v;
            bi = i;
        }
    }
    bi
}

pub struct CnnSegmenterIterator<'s, 'data> {
    _input: &'s str,
    _pos_utf8: usize,
    bies: BiesIterator<'s, 'data>,
}

struct BiesIterator<'l, 'data> {
    _segmenter: &'l CnnSegmenter<'data>,
    _input_seq: core::iter::Enumerate<std::vec::IntoIter<u16>>,
    pub probs: MatrixOwned<2>,
}

impl<'l, 'data> BiesIterator<'l, 'data> {
    fn new(segmenter: &'l CnnSegmenter<'data>, input_seq: Vec<u16>) -> Self {
        let l = input_seq.len();
        let embed_zero: MatrixZero<'_, 2> = segmenter.embedding;
        let embed = embed_zero.to_owned();
        let (vocab, edim) = embed.dim();
        let mut x = MatrixOwned::<2>::new_zero([l, edim]);
        for (i, &id) in input_seq.iter().enumerate() {
            let row = (id as usize).min(vocab - 1);
            {
                let row_view = embed.submatrix::<1>(row).unwrap();
                let src = row_view.as_slice();
                let mut row_mut = x.submatrix_mut::<1>(i).unwrap();
                let dst = row_mut.as_mut_slice();
                dst.copy_from_slice(src);
            }
        }
        let x_t = x.as_borrowed();
        let cout = segmenter.cnn_b1.dim();
        let mut y1 = MatrixOwned::<2>::new_zero([l, cout]); // parallel with y2
        conv1d(x_t, y1.as_mut(), segmenter.cnn_w1, segmenter.cnn_b1, 1);

        let mut y2 = MatrixOwned::<2>::new_zero([l, cout]);
        conv1d(x_t, y2.as_mut(), segmenter.cnn_w2, segmenter.cnn_b2, 2);

        let mut maximum = MatrixOwned::<2>::new_zero([l, cout]);
        elementwise_max(y1.as_borrowed(), y2.as_borrowed(), maximum.as_mut());

        let mut probs = MatrixOwned::<2>::new_zero([l, 4]);
        dense_softmax(
            maximum.as_borrowed(),
            probs.as_mut(),
            segmenter.softmax_w,
            segmenter.softmax_b,
        );

        Self {
            _segmenter: segmenter,
            _input_seq: input_seq.into_iter().enumerate(),
            probs,
        }
    }
}

pub struct CnnSegmenter<'data> {
    dic: &'data HashMap<String, u16>,
    embedding: MatrixZero<'data, 2>,
    cnn_w1: MatrixZero<'data, 3>,
    cnn_b1: MatrixZero<'data, 1>,
    cnn_w2: MatrixZero<'data, 3>,
    cnn_b2: MatrixZero<'data, 1>,
    softmax_w: MatrixZero<'data, 2>,
    softmax_b: MatrixZero<'data, 1>,
}

impl<'data> CnnSegmenter<'data> {
    pub fn new(lstm: &'data CnnData<'data>) -> Self {
        let CnnData::Float32(lstm) = lstm;
        Self {
            dic: &lstm.dic,
            embedding: MatrixZero::from(&lstm.embedding),
            cnn_w1: MatrixZero::from(&lstm.cnn_w1),
            cnn_b1: MatrixZero::from(&lstm.cnn_b1),
            cnn_w2: MatrixZero::from(&lstm.cnn_w2),
            cnn_b2: MatrixZero::from(&lstm.cnn_b2),
            softmax_w: MatrixZero::from(&lstm.softmax_w),
            softmax_b: MatrixZero::from(&lstm.softmax_b),
        }
    }

    pub fn segment_str<'a>(&'a self, input: &'a str) -> CnnSegmenterIterator<'a, 'data> {
        let input_seq = input
            .chars()
            .map(|c| {
                self.dic
                    .get(&c.to_string())
                    .copied()
                    .unwrap_or(self.dic.len() as u16)
            })
            .collect();
        CnnSegmenterIterator {
            _input: input,
            _pos_utf8: 0,
            bies: BiesIterator::new(self, input_seq),
        }
    }
}

#[test]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "tests/cnn/sample.json".to_string();
    let json = fs::read_to_string(&path)?;

    let rawcnndata: RawCnnData = serde_json::from_str(&json)?;
    let cnndata = rawcnndata
        .try_convert()
        .map_err(|_| "validation/conversion failed".to_string())?;
    let segmenter = CnnSegmenter::new(&cnndata);

    let thai = "ปัญหาความแตกต่างที่เกิดขึ้นระหว่างความเป็นธรรมในทางสังคมกับความเป็นธรรมทางกฎหมาย".to_string();
    println!("Input: {}", thai);
    let out = segmenter.segment_str(&thai);

    let (l, c) = out.bies.probs.as_borrowed().dim();
    let flat = out.bies.probs.as_borrowed().as_slice();

    let mut tags = String::with_capacity(l);
    for t in 0..l {
        let row = &flat[t * c..(t + 1) * c];
        let idx = argmax(row);
        tags.push(class_idx_to_bies(idx));
    }

    println!("BIES: {}", tags);
    Ok(())
}
