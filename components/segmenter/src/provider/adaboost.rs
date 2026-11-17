// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for the `AdaBoost` segmenter.

use icu_provider::prelude::*;
use zerovec::ZeroMap;

/// The data powering an `AdaBoost` segmentation model.
///
/// Each variant holds the set of weak learner features used by a family of models.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(
    feature = "datagen",
    databake(path = icu_segmenter::provider::adaboost)
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
#[non_exhaustive]
#[allow(clippy::large_enum_variant)] // loaded once per model, not moved around in hot code
pub enum AdaboostData<'data> {
    /// Scalar and radical features, used by the Chinese model.
    ///
    /// The score of a candidate boundary is the bias plus the matching weights.
    Radical {
        /// The model bias.
        bias: i32,
        /// Weights for the scalar two positions to the left of a candidate boundary.
        #[cfg_attr(feature = "serde", serde(borrow))]
        uw2: ZeroMap<'data, char, i16>,
        /// Weights for the scalar immediately to the left of a candidate boundary.
        #[cfg_attr(feature = "serde", serde(borrow))]
        uw3: ZeroMap<'data, char, i16>,
        /// Weights for the scalar immediately to the right of a candidate boundary.
        #[cfg_attr(feature = "serde", serde(borrow))]
        uw4: ZeroMap<'data, char, i16>,
        /// Weights for the scalar two positions to the right of a candidate boundary.
        #[cfg_attr(feature = "serde", serde(borrow))]
        uw5: ZeroMap<'data, char, i16>,
        /// Weights for the scalar pair surrounding a candidate boundary.
        #[cfg_attr(feature = "serde", serde(borrow))]
        bw2: ZeroMap<'data, (char, char), i16>,
        /// Weights for the radical pair surrounding a candidate boundary.
        #[cfg_attr(feature = "serde", serde(borrow))]
        rad: ZeroMap<'data, (u8, u8), i16>,
        /// Weights for the left radical and right scalar pair.
        #[cfg_attr(feature = "serde", serde(borrow))]
        lsrid: ZeroMap<'data, (u8, char), i16>,
        /// Weights for the left scalar and right radical pair.
        #[cfg_attr(feature = "serde", serde(borrow))]
        rsrid: ZeroMap<'data, (char, u8), i16>,
    },
    /// Unigram, bigram, and trigram features, used by the Japanese and Thai models.
    ///
    /// The score of a candidate boundary is the bias plus twice the matching weights.
    Ngram {
        /// The doubled model bias.
        bias: i32,
        /// Unigram weights three positions to the left of a candidate boundary.
        #[cfg_attr(feature = "serde", serde(borrow))]
        uw1: ZeroMap<'data, char, i16>,
        /// Unigram weights two positions to the left of a candidate boundary.
        #[cfg_attr(feature = "serde", serde(borrow))]
        uw2: ZeroMap<'data, char, i16>,
        /// Unigram weights immediately to the left of a candidate boundary.
        #[cfg_attr(feature = "serde", serde(borrow))]
        uw3: ZeroMap<'data, char, i16>,
        /// Unigram weights immediately to the right of a candidate boundary.
        #[cfg_attr(feature = "serde", serde(borrow))]
        uw4: ZeroMap<'data, char, i16>,
        /// Unigram weights two positions to the right of a candidate boundary.
        #[cfg_attr(feature = "serde", serde(borrow))]
        uw5: ZeroMap<'data, char, i16>,
        /// Unigram weights three positions to the right of a candidate boundary.
        #[cfg_attr(feature = "serde", serde(borrow))]
        uw6: ZeroMap<'data, char, i16>,
        /// Bigram weights immediately to the left of a candidate boundary.
        #[cfg_attr(feature = "serde", serde(borrow))]
        bw1: ZeroMap<'data, (char, char), i16>,
        /// Bigram weights surrounding a candidate boundary.
        #[cfg_attr(feature = "serde", serde(borrow))]
        bw2: ZeroMap<'data, (char, char), i16>,
        /// Bigram weights immediately to the right of a candidate boundary.
        #[cfg_attr(feature = "serde", serde(borrow))]
        bw3: ZeroMap<'data, (char, char), i16>,
        /// Trigram weights to the left of a candidate boundary.
        ///
        /// These remain string-keyed to preserve the source model exactly.
        #[cfg_attr(feature = "serde", serde(borrow))]
        tw1: ZeroMap<'data, str, i16>,
        /// Trigram weights centered immediately to the left of a candidate boundary.
        #[cfg_attr(feature = "serde", serde(borrow))]
        tw2: ZeroMap<'data, str, i16>,
        /// Trigram weights centered immediately to the right of a candidate boundary.
        #[cfg_attr(feature = "serde", serde(borrow))]
        tw3: ZeroMap<'data, str, i16>,
        /// Trigram weights to the right of a candidate boundary.
        #[cfg_attr(feature = "serde", serde(borrow))]
        tw4: ZeroMap<'data, str, i16>,
    },
    // new variants should go BELOW existing ones
    // Serde serializes based on variant name and index in the enum
    // https://docs.rs/serde/latest/serde/trait.Serializer.html#tymethod.serialize_unit_variant
}

icu_provider::data_struct!(
    AdaboostData<'_>,
    #[cfg(feature = "datagen")]
);

icu_provider::data_marker!(
    /// `AdaBoost` segmentation model data, keyed by model name in the marker attributes.
    SegmenterAdaboostV1,
    "segmenter/adaboost/v1",
    AdaboostData<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "segmenter"
);

#[cfg(all(test, feature = "compiled_data"))]
mod tests {
    use super::*;
    use crate::provider::Baked;

    fn load(attributes: &str) -> Result<DataResponse<SegmenterAdaboostV1>, DataError> {
        Baked.load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes(
                DataMarkerAttributes::try_from_str(attributes).unwrap(),
            ),
            ..Default::default()
        })
    }

    #[test]
    fn baked_chinese_model() {
        let response = load("Chinese_adaboost").expect("the baked model should load");
        let AdaboostData::Radical {
            bias,
            uw2,
            uw3,
            uw4,
            uw5,
            bw2,
            rad,
            lsrid,
            rsrid,
        } = response.payload.get()
        else {
            panic!("the Chinese model should use radical features");
        };

        assert_eq!(*bias, 4);
        assert_eq!(
            uw2.len()
                + uw3.len()
                + uw4.len()
                + uw5.len()
                + bw2.len()
                + rad.len()
                + lsrid.len()
                + rsrid.len(),
            5291
        );
    }

    #[test]
    fn baked_ngram_models() {
        for (attributes, expected_bias, expected_len, tw3_entry, tw4_entry) in [
            ("Thai_adaboost", -3755, 3901, None, Some(("ละ", 870))),
            (
                "Japanese_adaboost",
                -3331,
                1710,
                Some(("という", 377)),
                None,
            ),
        ] {
            let response = load(attributes).expect("the baked model should load");
            let AdaboostData::Ngram {
                bias,
                uw1,
                uw2,
                uw3,
                uw4,
                uw5,
                uw6,
                bw1,
                bw2,
                bw3,
                tw1,
                tw2,
                tw3,
                tw4,
            } = response.payload.get()
            else {
                panic!("the {attributes} model should use n-gram features");
            };

            assert_eq!(*bias, expected_bias);
            assert_eq!(
                uw1.len()
                    + uw2.len()
                    + uw3.len()
                    + uw4.len()
                    + uw5.len()
                    + uw6.len()
                    + bw1.len()
                    + bw2.len()
                    + bw3.len()
                    + tw1.len()
                    + tw2.len()
                    + tw3.len()
                    + tw4.len(),
                expected_len
            );
            if let Some((trigram, weight)) = tw3_entry {
                assert_eq!(tw3.get_copied(trigram), Some(weight));
            }
            if let Some((trigram, weight)) = tw4_entry {
                assert_eq!(tw4.get_copied(trigram), Some(weight));
            }
        }
    }

    #[test]
    fn baked_model_rejects_unknown_attributes() {
        let error = load("unknown").expect_err("an unknown model attribute should not load");
        assert_eq!(error.kind, DataErrorKind::IdentifierNotFound);
    }
}
