// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transliterate::provider::TransliteratorRulesV1Marker;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::RefCell;
use icu_locale_core::Locale;
use icu_normalizer::provider::*;
use icu_properties::{provider::*, sets};
use icu_provider::prelude::*;
#[cfg(feature = "datagen")]
use std::collections::HashSet;

mod parse;
mod pass1;
mod pass2;
mod rule_group_agg;

/// The direction of a rule-based transliterator in respect to its source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Direction {
    /// Forwards, i.e., left-to-right in the source.
    Forward,
    /// Reverse, i.e., right-to-left in the source.
    Reverse,
    /// Both forwards and reverse.
    Both,
}

impl Direction {
    /// Whether `self` is a superset of `other` or not.
    pub(crate) fn permits(&self, other: Direction) -> bool {
        match self {
            Direction::Forward => other == Direction::Forward,
            Direction::Reverse => other == Direction::Reverse,
            Direction::Both => true,
        }
    }
}

#[derive(Debug, Default)]
/// A collection of transliteration rules.
///
/// ✨ *Enabled with the `compile` Cargo feature.*
///
/// # Example
/// ```
/// use icu::experimental::transliterate::{RuleCollection, Transliterator};
///
/// let mut collection = RuleCollection::default();
/// collection.register_source(
///     &"de-t-de-d0-ascii".parse().unwrap(),
///     r#"
///       $AE = [Ä {A \u0308}];
///       $OE = [Ö {O \u0308}];
///       $UE = [Ü {U \u0308}];
///  
///       [ä {a \u0308}] → ae;
///       [ö {o \u0308}] → oe;
///       [ü {u \u0308}] → ue;
///  
///       {$AE} [:Lowercase:] → Ae;
///       {$OE} [:Lowercase:] → Oe;
///       {$UE} [:Lowercase:] → Ue;
///  
///       $AE → AE;
///       $OE → OE;
///       $UE → UE;
///  
///       ::Any-ASCII;
///     "#.to_string(),
///     // legacy alias, doesn't actually do anything in this case
///     ["de-ASCII"],
///     false,
///     true,
/// );
/// collection.register_source(
///     &"und-t-und-latn-d0-ascii".parse().unwrap(),
///     "# ... ".to_string(),
///     // the previous transliterator refers to this one by the `Any-ASCII` ID,
///     // so it's important that the alias is listed here.
///     ["Latin-Ascii", "Any-ASCII"],
///     false,
///     true,
/// );
///
/// let t = Transliterator::try_new_unstable("de-t-de-d0-ascii".parse().unwrap(), &collection.as_provider()).unwrap();
/// assert_eq!(t.transliterate("Käse".into()), "Kaese");
#[allow(clippy::type_complexity)] // well
pub struct RuleCollection {
    id_mapping: BTreeMap<String, Locale>, // alias -> bcp id
    // these two maps need to lock together
    data: RefCell<(
        BTreeMap<String, (String, bool, bool)>, // marker-attributes -> source/reverse/visible
        BTreeMap<String, Result<DataResponse<TransliteratorRulesV1Marker>, DataError>>, // cache
    )>,
}

impl RuleCollection {
    /// Add a new transliteration source to the collection.
    pub fn register_source<'a>(
        &mut self,
        id: &icu_locale_core::Locale,
        source: String,
        aliases: impl IntoIterator<Item = &'a str>,
        reverse: bool,
        visible: bool,
    ) {
        self.data.borrow_mut().0.insert(
            id.to_string().to_ascii_lowercase(),
            (source, reverse, visible),
        );

        for alias in aliases.into_iter() {
            self.id_mapping
                .insert(alias.to_ascii_lowercase(), id.clone());
        }
    }

    /// Returns a provider that is usable by [`Transliterator::try_new_unstable`](crate::transliterate::Transliterator::try_new_unstable).
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    #[cfg(feature = "compiled_data")]
    pub fn as_provider(
        &self,
    ) -> RuleCollectionProvider<'_, icu_properties::provider::Baked, icu_normalizer::provider::Baked>
    {
        RuleCollectionProvider {
            collection: self,
            properties_provider: &icu_properties::provider::Baked,
            normalizer_provider: &icu_normalizer::provider::Baked,
            xid_start: sets::xid_start().static_to_owned(),
            xid_continue: sets::xid_continue().static_to_owned(),
            pat_ws: sets::pattern_white_space().static_to_owned(),
        }
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::as_provider)]
    pub fn as_provider_unstable<'a, PP, NP>(
        &'a self,
        properties_provider: &'a PP,
        normalizer_provider: &'a NP,
    ) -> Result<RuleCollectionProvider<'a, PP, NP>, DataError>
    where
        PP: ?Sized
            + DataProvider<AsciiHexDigitV1Marker>
            + DataProvider<AlphabeticV1Marker>
            + DataProvider<BidiControlV1Marker>
            + DataProvider<BidiMirroredV1Marker>
            + DataProvider<CaseIgnorableV1Marker>
            + DataProvider<CasedV1Marker>
            + DataProvider<ChangesWhenCasefoldedV1Marker>
            + DataProvider<ChangesWhenCasemappedV1Marker>
            + DataProvider<ChangesWhenLowercasedV1Marker>
            + DataProvider<ChangesWhenNfkcCasefoldedV1Marker>
            + DataProvider<ChangesWhenTitlecasedV1Marker>
            + DataProvider<ChangesWhenUppercasedV1Marker>
            + DataProvider<DashV1Marker>
            + DataProvider<DefaultIgnorableCodePointV1Marker>
            + DataProvider<DeprecatedV1Marker>
            + DataProvider<DiacriticV1Marker>
            + DataProvider<EmojiV1Marker>
            + DataProvider<EmojiComponentV1Marker>
            + DataProvider<EmojiModifierV1Marker>
            + DataProvider<EmojiModifierBaseV1Marker>
            + DataProvider<EmojiPresentationV1Marker>
            + DataProvider<ExtendedPictographicV1Marker>
            + DataProvider<ExtenderV1Marker>
            + DataProvider<GraphemeBaseV1Marker>
            + DataProvider<GraphemeClusterBreakV1Marker>
            + DataProvider<GraphemeClusterBreakNameToValueV1Marker>
            + DataProvider<GraphemeExtendV1Marker>
            + DataProvider<HexDigitV1Marker>
            + DataProvider<IdsBinaryOperatorV1Marker>
            + DataProvider<IdsTrinaryOperatorV1Marker>
            + DataProvider<IdContinueV1Marker>
            + DataProvider<IdStartV1Marker>
            + DataProvider<IdeographicV1Marker>
            + DataProvider<JoinControlV1Marker>
            + DataProvider<LogicalOrderExceptionV1Marker>
            + DataProvider<LowercaseV1Marker>
            + DataProvider<MathV1Marker>
            + DataProvider<NoncharacterCodePointV1Marker>
            + DataProvider<PatternSyntaxV1Marker>
            + DataProvider<PatternWhiteSpaceV1Marker>
            + DataProvider<QuotationMarkV1Marker>
            + DataProvider<RadicalV1Marker>
            + DataProvider<RegionalIndicatorV1Marker>
            + DataProvider<SentenceBreakV1Marker>
            + DataProvider<SentenceBreakNameToValueV1Marker>
            + DataProvider<SentenceTerminalV1Marker>
            + DataProvider<SoftDottedV1Marker>
            + DataProvider<TerminalPunctuationV1Marker>
            + DataProvider<UnifiedIdeographV1Marker>
            + DataProvider<UppercaseV1Marker>
            + DataProvider<VariationSelectorV1Marker>
            + DataProvider<WhiteSpaceV1Marker>
            + DataProvider<WordBreakV1Marker>
            + DataProvider<WordBreakNameToValueV1Marker>
            + DataProvider<XidContinueV1Marker>
            + DataProvider<GeneralCategoryMaskNameToValueV1Marker>
            + DataProvider<GeneralCategoryV1Marker>
            + DataProvider<ScriptNameToValueV1Marker>
            + DataProvider<ScriptV1Marker>
            + DataProvider<ScriptWithExtensionsPropertyV1Marker>
            + DataProvider<XidStartV1Marker>,
        NP: ?Sized
            + DataProvider<CanonicalDecompositionDataV1Marker>
            + DataProvider<CompatibilityDecompositionSupplementV1Marker>
            + DataProvider<CanonicalDecompositionTablesV1Marker>
            + DataProvider<CompatibilityDecompositionTablesV1Marker>
            + DataProvider<CanonicalCompositionsV1Marker>,
    {
        Ok(RuleCollectionProvider {
            collection: self,
            properties_provider,
            normalizer_provider,
            xid_start: sets::load_xid_start(properties_provider)?,
            xid_continue: sets::load_xid_continue(properties_provider)?,
            pat_ws: sets::load_pattern_white_space(properties_provider)?,
        })
    }
}

/// A provider that is usable by [`Transliterator::try_new_unstable`](crate::Transliterator::try_new_unstable).
#[derive(Debug)]
pub struct RuleCollectionProvider<'a, PP: ?Sized, NP: ?Sized> {
    collection: &'a RuleCollection,
    properties_provider: &'a PP,
    normalizer_provider: &'a NP,
    xid_start: sets::CodePointSetData,
    xid_continue: sets::CodePointSetData,
    pat_ws: sets::CodePointSetData,
}

impl<PP, NP> DataProvider<TransliteratorRulesV1Marker> for RuleCollectionProvider<'_, PP, NP>
where
    PP: ?Sized
        + DataProvider<AsciiHexDigitV1Marker>
        + DataProvider<AlphabeticV1Marker>
        + DataProvider<BidiControlV1Marker>
        + DataProvider<BidiMirroredV1Marker>
        + DataProvider<CaseIgnorableV1Marker>
        + DataProvider<CasedV1Marker>
        + DataProvider<ChangesWhenCasefoldedV1Marker>
        + DataProvider<ChangesWhenCasemappedV1Marker>
        + DataProvider<ChangesWhenLowercasedV1Marker>
        + DataProvider<ChangesWhenNfkcCasefoldedV1Marker>
        + DataProvider<ChangesWhenTitlecasedV1Marker>
        + DataProvider<ChangesWhenUppercasedV1Marker>
        + DataProvider<DashV1Marker>
        + DataProvider<DefaultIgnorableCodePointV1Marker>
        + DataProvider<DeprecatedV1Marker>
        + DataProvider<DiacriticV1Marker>
        + DataProvider<EmojiV1Marker>
        + DataProvider<EmojiComponentV1Marker>
        + DataProvider<EmojiModifierV1Marker>
        + DataProvider<EmojiModifierBaseV1Marker>
        + DataProvider<EmojiPresentationV1Marker>
        + DataProvider<ExtendedPictographicV1Marker>
        + DataProvider<ExtenderV1Marker>
        + DataProvider<GraphemeBaseV1Marker>
        + DataProvider<GraphemeClusterBreakV1Marker>
        + DataProvider<GraphemeClusterBreakNameToValueV1Marker>
        + DataProvider<GraphemeExtendV1Marker>
        + DataProvider<HexDigitV1Marker>
        + DataProvider<IdsBinaryOperatorV1Marker>
        + DataProvider<IdsTrinaryOperatorV1Marker>
        + DataProvider<IdContinueV1Marker>
        + DataProvider<IdStartV1Marker>
        + DataProvider<IdeographicV1Marker>
        + DataProvider<JoinControlV1Marker>
        + DataProvider<LogicalOrderExceptionV1Marker>
        + DataProvider<LowercaseV1Marker>
        + DataProvider<MathV1Marker>
        + DataProvider<NoncharacterCodePointV1Marker>
        + DataProvider<PatternSyntaxV1Marker>
        + DataProvider<PatternWhiteSpaceV1Marker>
        + DataProvider<QuotationMarkV1Marker>
        + DataProvider<RadicalV1Marker>
        + DataProvider<RegionalIndicatorV1Marker>
        + DataProvider<SentenceBreakV1Marker>
        + DataProvider<SentenceBreakNameToValueV1Marker>
        + DataProvider<SentenceTerminalV1Marker>
        + DataProvider<SoftDottedV1Marker>
        + DataProvider<TerminalPunctuationV1Marker>
        + DataProvider<UnifiedIdeographV1Marker>
        + DataProvider<UppercaseV1Marker>
        + DataProvider<VariationSelectorV1Marker>
        + DataProvider<WhiteSpaceV1Marker>
        + DataProvider<WordBreakV1Marker>
        + DataProvider<WordBreakNameToValueV1Marker>
        + DataProvider<XidContinueV1Marker>
        + DataProvider<GeneralCategoryMaskNameToValueV1Marker>
        + DataProvider<GeneralCategoryV1Marker>
        + DataProvider<ScriptNameToValueV1Marker>
        + DataProvider<ScriptV1Marker>
        + DataProvider<ScriptWithExtensionsPropertyV1Marker>
        + DataProvider<XidStartV1Marker>,
    NP: ?Sized,
{
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<TransliteratorRulesV1Marker>, DataError> {
        let mut exclusive_data = self.collection.data.borrow_mut();

        if let Some(response) = exclusive_data.1.get(req.id.marker_attributes.as_str()) {
            return response.clone();
        };

        let result = || -> Result<DataResponse<TransliteratorRulesV1Marker>, DataError> {
            let Some((source, reverse, visible)) =
                exclusive_data.0.remove(req.id.marker_attributes as &str)
            else {
                return Err(
                    DataErrorKind::MissingLocale.with_req(TransliteratorRulesV1Marker::INFO, req)
                );
            };

            let rules = parse::Parser::run(
                &source,
                &self.xid_start.to_code_point_inversion_list(),
                &self.xid_continue.to_code_point_inversion_list(),
                &self.pat_ws.to_code_point_inversion_list(),
                self.properties_provider,
            )
            .map_err(|e| {
                e.explain(&source)
                    .with_req(TransliteratorRulesV1Marker::INFO, req)
            })?;

            let pass1 = pass1::Pass1::run(
                if reverse {
                    Direction::Reverse
                } else {
                    Direction::Forward
                },
                &rules,
            )
            .map_err(|e| {
                e.explain(&source)
                    .with_req(TransliteratorRulesV1Marker::INFO, req)
            })?;

            let mut transliterator = pass2::Pass2::run(
                if reverse {
                    pass1.reverse_result
                } else {
                    pass1.forward_result
                },
                &pass1.variable_definitions,
                &self.collection.id_mapping,
            )
            .map_err(|e| {
                e.explain(&source)
                    .with_req(TransliteratorRulesV1Marker::INFO, req)
            })?;

            transliterator.visibility = visible;

            Ok(DataResponse {
                metadata: Default::default(),
                payload: DataPayload::from_owned(transliterator),
            })
        }();

        exclusive_data
            .1
            .insert(req.id.marker_attributes.to_string(), result.clone());

        result
    }
}

macro_rules! redirect {
    ($($marker:ty),*) => {
        $(
            impl<PP: ?Sized, NP: ?Sized + DataProvider<$marker>> DataProvider<$marker> for RuleCollectionProvider<'_, PP, NP> {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    self.normalizer_provider.load(req)
                }
            }
        )*
    }
}

redirect!(
    CanonicalDecompositionDataV1Marker,
    CompatibilityDecompositionSupplementV1Marker,
    CanonicalDecompositionTablesV1Marker,
    CompatibilityDecompositionTablesV1Marker,
    CanonicalCompositionsV1Marker
);

#[cfg(feature = "datagen")]
impl<PP, NP> IterableDataProvider<TransliteratorRulesV1Marker>
    for RuleCollectionProvider<'_, PP, NP>
where
    PP: ?Sized
        + DataProvider<AsciiHexDigitV1Marker>
        + DataProvider<AlphabeticV1Marker>
        + DataProvider<BidiControlV1Marker>
        + DataProvider<BidiMirroredV1Marker>
        + DataProvider<CaseIgnorableV1Marker>
        + DataProvider<CasedV1Marker>
        + DataProvider<ChangesWhenCasefoldedV1Marker>
        + DataProvider<ChangesWhenCasemappedV1Marker>
        + DataProvider<ChangesWhenLowercasedV1Marker>
        + DataProvider<ChangesWhenNfkcCasefoldedV1Marker>
        + DataProvider<ChangesWhenTitlecasedV1Marker>
        + DataProvider<ChangesWhenUppercasedV1Marker>
        + DataProvider<DashV1Marker>
        + DataProvider<DefaultIgnorableCodePointV1Marker>
        + DataProvider<DeprecatedV1Marker>
        + DataProvider<DiacriticV1Marker>
        + DataProvider<EmojiV1Marker>
        + DataProvider<EmojiComponentV1Marker>
        + DataProvider<EmojiModifierV1Marker>
        + DataProvider<EmojiModifierBaseV1Marker>
        + DataProvider<EmojiPresentationV1Marker>
        + DataProvider<ExtendedPictographicV1Marker>
        + DataProvider<ExtenderV1Marker>
        + DataProvider<GraphemeBaseV1Marker>
        + DataProvider<GraphemeClusterBreakV1Marker>
        + DataProvider<GraphemeClusterBreakNameToValueV1Marker>
        + DataProvider<GraphemeExtendV1Marker>
        + DataProvider<HexDigitV1Marker>
        + DataProvider<IdsBinaryOperatorV1Marker>
        + DataProvider<IdsTrinaryOperatorV1Marker>
        + DataProvider<IdContinueV1Marker>
        + DataProvider<IdStartV1Marker>
        + DataProvider<IdeographicV1Marker>
        + DataProvider<JoinControlV1Marker>
        + DataProvider<LogicalOrderExceptionV1Marker>
        + DataProvider<LowercaseV1Marker>
        + DataProvider<MathV1Marker>
        + DataProvider<NoncharacterCodePointV1Marker>
        + DataProvider<PatternSyntaxV1Marker>
        + DataProvider<PatternWhiteSpaceV1Marker>
        + DataProvider<QuotationMarkV1Marker>
        + DataProvider<RadicalV1Marker>
        + DataProvider<RegionalIndicatorV1Marker>
        + DataProvider<SentenceBreakV1Marker>
        + DataProvider<SentenceBreakNameToValueV1Marker>
        + DataProvider<SentenceTerminalV1Marker>
        + DataProvider<SoftDottedV1Marker>
        + DataProvider<TerminalPunctuationV1Marker>
        + DataProvider<UnifiedIdeographV1Marker>
        + DataProvider<UppercaseV1Marker>
        + DataProvider<VariationSelectorV1Marker>
        + DataProvider<WhiteSpaceV1Marker>
        + DataProvider<WordBreakV1Marker>
        + DataProvider<WordBreakNameToValueV1Marker>
        + DataProvider<XidContinueV1Marker>
        + DataProvider<GeneralCategoryMaskNameToValueV1Marker>
        + DataProvider<GeneralCategoryV1Marker>
        + DataProvider<ScriptNameToValueV1Marker>
        + DataProvider<ScriptV1Marker>
        + DataProvider<ScriptWithExtensionsPropertyV1Marker>
        + DataProvider<XidStartV1Marker>,
    NP: ?Sized,
{
    fn iter_ids(&self) -> Result<HashSet<DataIdentifierCow>, DataError> {
        let exclusive_data = self.collection.data.borrow();
        Ok(exclusive_data
            .0
            .keys()
            .cloned()
            .chain(exclusive_data.1.keys().cloned())
            .filter_map(|s| {
                Some(DataIdentifierCow::from_marker_attributes_owned(
                    DataMarkerAttributes::try_from_string(s).ok()?,
                ))
            })
            .collect())
    }
}

struct CompileError {
    /// offset is the index to an arbitrary byte in the last character in the source that makes sense
    /// to display as location for the error, e.g., the unexpected character itself or
    /// for an unknown property name the last character of the name.
    offset: Option<usize>,
    /// The type of compile error
    kind: CompileErrorKind,
}

impl CompileError {
    fn explain(self, source: &str) -> DataError {
        let e = DataError::custom("Invalid transliterator");
        if let Some(mut col_number) = self.offset {
            let mut line_number = 1;
            if let Some(snippet) = source
                .lines()
                .filter_map(|line| {
                    if let Some(snippet) = line.get(col_number..) {
                        Some(snippet)
                    } else {
                        col_number -= line.len();
                        col_number -= 1; // \n
                        line_number += 1;
                        None
                    }
                })
                .next()
            {
                e.with_display_context(&format!("at {line_number}:{col_number} '{snippet}'"));
            }
        }
        e.with_debug_context(&self.kind);
        e
    }
}

/// The kind of compile error that occurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompileErrorKind {
    /// An unexpected character was encountered. This variant implies the other variants
    /// (notably `UnknownProperty` and `Unimplemented`) do not apply.
    UnexpectedChar(char),
    /// A reference to an unknown variable.
    UnknownVariable,
    /// The source is incomplete.
    Eof,
    /// Something unexpected went wrong with our code. Please file a bug report on GitHub.
    Internal(&'static str),
    /// The provided syntax is not supported by us. Please file an issue on GitHub if you need
    /// this feature.
    Unimplemented,
    /// The provided escape sequence is not a valid Unicode code point.
    InvalidEscape,
    /// The provided transform ID is invalid.
    InvalidId,
    /// The provided number is invalid, which likely means it's too big.
    InvalidNumber,
    /// Duplicate variable definition.
    DuplicateVariable,
    /// Invalid UnicodeSet syntax. See `crate::unicodeset_parse`'s [`ParseError`](crate::unicodeset_parse::ParseError).
    UnicodeSetError(crate::unicodeset_parse::ParseError),

    // errors originating from compilation step
    /// A global filter (forward or backward) in an unexpected position.
    UnexpectedGlobalFilter,
    /// A global filter (forward or backward) may not contain strings.
    GlobalFilterWithStrings,
    /// An element appeared on the source side of a rule, but that is prohibited.
    UnexpectedElementInSource(&'static str),
    /// An element appeared on the target side of a rule, but that is prohibited.
    UnexpectedElementInTarget(&'static str),
    /// An element appeared in a variable definition, but that is prohibited.
    UnexpectedElementInVariableDefinition(&'static str),
    /// The start anchor `^` was not placed at the beginning of a source.
    AnchorStartNotAtStart,
    /// The end anchor `$` was not placed at the end of a source.
    AnchorEndNotAtEnd,
    /// A variable that contains source-only matchers (e.g., UnicodeSets) was used on the target side.
    SourceOnlyVariable,
    /// No matching segment for this backreference was found.
    BackReferenceOutOfRange,
    /// The cursor is in an invalid position.
    InvalidCursor,
    /// Multiple cursors were defined.
    DuplicateCursor,
    /// There are too many special matchers/replacers/variables in the source.
    TooManySpecials,
}

impl CompileErrorKind {
    fn with_offset(self, offset: usize) -> CompileError {
        CompileError {
            offset: Some(offset),
            kind: self,
        }
    }
}

impl From<CompileErrorKind> for CompileError {
    fn from(kind: CompileErrorKind) -> Self {
        CompileError { offset: None, kind }
    }
}

#[cfg(test)]
mod tests {
    use alloc::borrow::Cow;

    use super::*;
    use crate::transliterate::provider as ds;
    use icu_locale_core::locale;
    use std::collections::HashSet;
    use zerovec::VarZeroVec;

    fn parse_set(source: &str) -> super::parse::UnicodeSet {
        crate::unicodeset_parse::parse_unstable(source, &icu_properties::provider::Baked)
            .expect("Parsing failed")
            .0
    }

    fn parse_set_cp(source: &str) -> super::parse::FilterSet {
        crate::unicodeset_parse::parse_unstable(source, &icu_properties::provider::Baked)
            .expect("Parsing failed")
            .0
            .code_points()
            .clone()
    }

    #[test]
    fn test_source_to_struct() {
        let source = r"
        use variable range 0xFFF 0xFFFF ;
        :: [1] ;
        :: Latin-InterIndic ;
        $a = [a] [b]+ ;
        $unused = [c{string}]+? ;
        $b = $a? 'literal chars' ;
        x } [a-z] > |@@@y ;
        $a > ab ;
        'reverse output:' &RevFnCall($1 'padding') @@ | < ($b) ;
        ^ left } $ <> ^ { rig|ht } [0-9] $ ;
        :: [\ ] Remove (AnyRev-AddRandomSpaces/FiftyPercent) ;
        # splits up the forward rules
        forward rule that > splits up rule groups ;
        :: InterIndic-Devanagari ;
        :: NFC ;
        ";

        let mut collection = RuleCollection::default();
        collection.register_source(&locale!("fwd"), source.into(), [], false, true);
        collection.register_source(&locale!("rev"), source.into(), [], true, true);
        collection.register_source(
            &"und-t-d0-addrndsp-m0-fifty-s0-anyrev".parse().unwrap(),
            "unparsed dummy".into(),
            ["AnyRev-AddRandomSpaces/FiftyPercent"],
            false,
            true,
        );

        let forward: DataResponse<TransliteratorRulesV1Marker> = collection
            .as_provider()
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes(
                    DataMarkerAttributes::from_str_or_panic("fwd"),
                ),
                ..Default::default()
            })
            .unwrap();

        let reverse: DataResponse<TransliteratorRulesV1Marker> = collection
            .as_provider()
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes(
                    DataMarkerAttributes::from_str_or_panic("rev"),
                ),

                ..Default::default()
            })
            .unwrap();
        {
            let expected_filter = parse_set_cp("[1]");

            let expected_id_group1 = vec![ds::SimpleId {
                filter: parse::FilterSet::all(),
                id: Cow::Borrowed("x-latin-interindic"),
            }];
            let expected_id_group2 = vec![ds::SimpleId {
                filter: parse_set_cp(r"[\ ]"),
                id: Cow::Borrowed("x-any-remove"),
            }];
            let expected_id_group3 = vec![
                ds::SimpleId {
                    filter: parse::FilterSet::all(),
                    id: Cow::Borrowed("x-interindic-devanagari"),
                },
                ds::SimpleId {
                    filter: parse::FilterSet::all(),
                    id: Cow::Borrowed("x-any-nfc"),
                },
            ];

            let expected_id_group_list: Vec<VarZeroVec<'_, ds::SimpleIdULE>> = vec![
                VarZeroVec::from(&expected_id_group1),
                VarZeroVec::from(&expected_id_group2),
                VarZeroVec::from(&expected_id_group3),
            ];

            let expected_rule_group1 = vec![
                ds::Rule {
                    ante: Cow::Borrowed(""),
                    key: Cow::Borrowed("x"),
                    post: Cow::Borrowed("\u{F0002}"),
                    replacer: Cow::Borrowed("\u{F0007}y"), // `|@@@`
                },
                ds::Rule {
                    ante: Cow::Borrowed(""),
                    key: Cow::Borrowed("\u{F0000}"),
                    post: Cow::Borrowed(""),
                    replacer: Cow::Borrowed("ab"),
                },
                ds::Rule {
                    ante: Cow::Borrowed(""),
                    key: Cow::Borrowed("\u{FFFFC}left"),
                    post: Cow::Borrowed("\u{FFFFD}"),
                    replacer: Cow::Borrowed("rig\u{FFFFB}ht"), // `|`
                },
            ];
            let expected_rule_group2 = vec![ds::Rule {
                ante: Cow::Borrowed(""),
                key: Cow::Borrowed("forwardrulethat"),
                post: Cow::Borrowed(""),
                replacer: Cow::Borrowed("splitsuprulegroups"),
            }];

            let expected_rule_group_list: Vec<VarZeroVec<'_, ds::RuleULE>> = vec![
                VarZeroVec::from(&expected_rule_group1),
                VarZeroVec::from(&expected_rule_group2),
                VarZeroVec::new(), // empty rule group after the last transform rule
            ];

            let expected_compounds = vec![
                "\u{F0003}\u{F0001}", // [a] and [b]+ (the quantifier contains [b])
            ];

            let expected_quantifiers_kleene_plus = vec![
                "\u{F0004}", // [b] from [b]+
            ];

            let expected_unicode_sets =
                vec![parse_set("[a-z]"), parse_set("[a]"), parse_set("[b]")];

            let expected_var_table = ds::VarTable {
                compounds: VarZeroVec::from(&expected_compounds),
                quantifiers_opt: VarZeroVec::new(),
                quantifiers_kleene: VarZeroVec::new(),
                quantifiers_kleene_plus: VarZeroVec::from(&expected_quantifiers_kleene_plus),
                segments: VarZeroVec::new(),
                unicode_sets: VarZeroVec::from(&expected_unicode_sets),
                function_calls: VarZeroVec::new(),
                max_left_placeholder_count: 0,
                max_right_placeholder_count: 3,
            };

            let expected_rbt = ds::RuleBasedTransliterator {
                filter: expected_filter,
                id_group_list: VarZeroVec::from(&expected_id_group_list),
                rule_group_list: VarZeroVec::from(&expected_rule_group_list),
                variable_table: expected_var_table,
                visibility: true,
            };
            assert_eq!(*forward.payload.get(), expected_rbt);

            assert_eq!(
                forward.payload.get().deps().collect::<HashSet<_>>(),
                HashSet::from_iter([
                    Cow::Borrowed("x-any-nfc"),
                    Cow::Borrowed("x-any-remove"),
                    Cow::Borrowed("x-interindic-devanagari"),
                    Cow::Borrowed("x-latin-interindic"),
                ])
            );
        }
        {
            let expected_filter = parse::FilterSet::all();

            let expected_id_group1 = vec![
                ds::SimpleId {
                    filter: parse::FilterSet::all(),
                    id: Cow::Borrowed("x-any-nfd"),
                },
                ds::SimpleId {
                    filter: parse::FilterSet::all(),
                    id: Cow::Borrowed("x-devanagari-interindic"),
                },
                ds::SimpleId {
                    filter: parse::FilterSet::all(),
                    id: Cow::Borrowed("und-t-d0-addrndsp-m0-fifty-s0-anyrev"),
                },
            ];
            let expected_id_group2 = vec![ds::SimpleId {
                filter: parse::FilterSet::all(),
                id: Cow::Borrowed("x-interindic-latin"),
            }];

            let expected_id_group_list: Vec<VarZeroVec<'_, ds::SimpleIdULE>> = vec![
                VarZeroVec::from(&expected_id_group1),
                VarZeroVec::from(&expected_id_group2),
            ];

            let expected_rule_group1 = vec![
                ds::Rule {
                    ante: Cow::Borrowed(""),
                    key: Cow::Borrowed("\u{F0004}"),
                    post: Cow::Borrowed(""),
                    replacer: Cow::Borrowed("reverse output:\u{F0008}\u{F000A}"), /* `@@|` and function call */
                },
                ds::Rule {
                    ante: Cow::Borrowed("\u{FFFFC}"), // start anchor
                    key: Cow::Borrowed("right"),
                    post: Cow::Borrowed("\u{F0007}\u{FFFFD}"), // [0-9] and end anchor
                    replacer: Cow::Borrowed("left"),
                },
            ];

            let expected_rule_group_list: Vec<VarZeroVec<'_, ds::RuleULE>> =
                vec![VarZeroVec::from(&expected_rule_group1), VarZeroVec::new()];

            let expected_compounds = vec![
                "\u{F0005}\u{F0003}",     // $a = [a] [b]+ (quantifier contains [b])
                "\u{F0002}literal chars", // $b = $a? (quantifier contains $a)
            ];

            let expected_quantifers_opt = vec![
                "\u{F0000}", // $a from $a?
            ];

            let expected_quantifiers_kleene_plus = vec![
                "\u{F0006}", // [b] from [b]+
            ];

            let expected_segments = vec![ds::Segment {
                idx: 0,
                content: Cow::Borrowed("\u{F0001}"), // $b from ($b)
            }];

            let expected_unicode_sets =
                vec![parse_set("[a]"), parse_set("[b]"), parse_set("[0-9]")];

            let expected_function_calls = vec![ds::FunctionCall {
                arg: Cow::Borrowed("\u{F000B}padding"), // $1 and 'padding'
                translit: ds::SimpleId {
                    filter: parse::FilterSet::all(),
                    id: Cow::Borrowed("x-any-revfncall"),
                },
            }];

            let expected_var_table = ds::VarTable {
                compounds: VarZeroVec::from(&expected_compounds),
                quantifiers_opt: VarZeroVec::from(&expected_quantifers_opt),
                quantifiers_kleene: VarZeroVec::new(),
                quantifiers_kleene_plus: VarZeroVec::from(&expected_quantifiers_kleene_plus),
                segments: VarZeroVec::from(&expected_segments),
                unicode_sets: VarZeroVec::from(&expected_unicode_sets),
                function_calls: VarZeroVec::from(&expected_function_calls),
                max_left_placeholder_count: 2,
                max_right_placeholder_count: 0,
            };

            let expected_rbt = ds::RuleBasedTransliterator {
                filter: expected_filter,
                id_group_list: VarZeroVec::from(&expected_id_group_list),
                rule_group_list: VarZeroVec::from(&expected_rule_group_list),
                variable_table: expected_var_table,
                visibility: true,
            };
            assert_eq!(*reverse.payload.get(), expected_rbt);

            assert_eq!(
                reverse.payload.get().deps().collect::<HashSet<_>>(),
                HashSet::from_iter([
                    Cow::Borrowed("und-t-d0-addrndsp-m0-fifty-s0-anyrev"),
                    Cow::Borrowed("x-any-nfd"),
                    Cow::Borrowed("x-any-revfncall"),
                    Cow::Borrowed("x-devanagari-interindic"),
                    Cow::Borrowed("x-interindic-latin"),
                ])
            );
        }
    }
}
