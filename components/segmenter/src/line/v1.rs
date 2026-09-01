// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{LineBreakStrictness, LineBreakWordOption, ResolvedLineBreakOptions};
use crate::complex::*;
use crate::provider::*;
use crate::rule_segmenter_v1::{ComplexRunSegmenter, ResultCache, result_cache_from_offsets};
use crate::scaffold::*;

#[doc(hidden)]
impl RuleBreakData<'_> {
    pub const LINE_PROPERTY_AI: u8 = 1;
    pub const LINE_PROPERTY_AL: u8 = 3;
    pub const LINE_PROPERTY_BA: u8 = 8;
    pub const LINE_PROPERTY_BK: u8 = 10;
    pub const LINE_PROPERTY_CJ: u8 = 12;
    pub const LINE_PROPERTY_CM: u8 = 14;
    pub const LINE_PROPERTY_CR: u8 = 16;
    pub const LINE_PROPERTY_EX: u8 = 19;
    pub const LINE_PROPERTY_H2: u8 = 21;
    pub const LINE_PROPERTY_H3: u8 = 22;
    pub const LINE_PROPERTY_HY: u8 = 24;
    pub const LINE_PROPERTY_ID: u8 = 25;
    pub const LINE_PROPERTY_IN: u8 = 27;
    pub const LINE_PROPERTY_JL: u8 = 29;
    pub const LINE_PROPERTY_JT: u8 = 30;
    pub const LINE_PROPERTY_JV: u8 = 31;
    pub const LINE_PROPERTY_LF: u8 = 32;
    pub const LINE_PROPERTY_NL: u8 = 33;
    pub const LINE_PROPERTY_NS: u8 = 34;
    pub const LINE_PROPERTY_NU: u8 = 35;
    pub const LINE_PROPERTY_PO_EAW: u8 = 39;
    pub const LINE_PROPERTY_PR_EAW: u8 = 41;
    pub const LINE_PROPERTY_SP: u8 = 47;
    pub const LINE_PROPERTY_ZW: u8 = 53;
    pub const LINE_PROPERTY_ZWJ: u8 = 54;
}

#[cfg_attr(not(test), allow(dead_code))]
#[doc(hidden)]
impl RuleBreakData<'_> {
    pub const LINE_PROPERTY_AK: u8 = 2;
    pub const LINE_PROPERTY_AL_DOTTED_CIRCLE: u8 = 4;
    pub const LINE_PROPERTY_AP: u8 = 5;
    pub const LINE_PROPERTY_AS: u8 = 6;
    pub const LINE_PROPERTY_B2: u8 = 7;
    pub const LINE_PROPERTY_BB: u8 = 9;
    pub const LINE_PROPERTY_CB: u8 = 11;
    pub const LINE_PROPERTY_CL: u8 = 13;
    pub const LINE_PROPERTY_CP: u8 = 15;
    pub const LINE_PROPERTY_EB: u8 = 17;
    pub const LINE_PROPERTY_EM: u8 = 18;
    pub const LINE_PROPERTY_GL: u8 = 20;
    pub const LINE_PROPERTY_HL: u8 = 23;
    pub const LINE_PROPERTY_ID_CN: u8 = 26;
    pub const LINE_PROPERTY_IS: u8 = 28;
    pub const LINE_PROPERTY_OP_EA: u8 = 36;
    pub const LINE_PROPERTY_OP_OP30: u8 = 37;
    pub const LINE_PROPERTY_PO: u8 = 38;
    pub const LINE_PROPERTY_PR: u8 = 40;
    pub const LINE_PROPERTY_QU: u8 = 42;
    pub const LINE_PROPERTY_QU_PF: u8 = 43;
    pub const LINE_PROPERTY_QU_PI: u8 = 44;
    pub const LINE_PROPERTY_RI: u8 = 45;
    pub const LINE_PROPERTY_SY: u8 = 48;
    pub const LINE_PROPERTY_VF: u8 = 49;
    pub const LINE_PROPERTY_VI: u8 = 50;
    pub const LINE_PROPERTY_WJ: u8 = 51;
    pub const LINE_PROPERTY_XX: u8 = 52;
}

fn is_break_utf32_by_normal(codepoint: u32, ja_zh: bool) -> bool {
    matches!(codepoint, 0x301C | 0x30A0 if ja_zh)
}

#[inline]
fn is_break_utf32_by_loose(
    right_codepoint: u32,
    left_prop: u8,
    right_prop: u8,
    ja_zh: bool,
) -> Option<bool> {
    Some(match (right_prop, right_codepoint, left_prop) {
        // breaks before hyphens
        (RuleBreakData::LINE_PROPERTY_BA, 0x2010 | 0x2013, RuleBreakData::LINE_PROPERTY_ID) => true,
        // breaks before certain CJK hyphen-like characters
        (RuleBreakData::LINE_PROPERTY_NS, 0x301C | 0x30A0, _) => ja_zh,
        // breaks before iteration marks
        (
            RuleBreakData::LINE_PROPERTY_NS,
            0x3005 | 0x303B | 0x309D | 0x309E | 0x30FD | 0x30FE,
            _,
        ) => true,
        // breaks before certain centered punctuation marks:
        (
            RuleBreakData::LINE_PROPERTY_NS,
            0x30FB | 0xFF1A | 0xFF1B | 0xFF65 | 0x203C | 0x2047..=0x2049,
            _,
        ) => ja_zh,
        // breaks between inseparable characters such as U+2025, U+2026 i.e. characters with the Unicode Line Break property IN
        (RuleBreakData::LINE_PROPERTY_IN, _, RuleBreakData::LINE_PROPERTY_IN) => true,
        // breaks before certain centered punctuation marks:
        (RuleBreakData::LINE_PROPERTY_EX, 0xFF01 | 0xFF1F, _) => ja_zh,
        // breaks before suffixes:
        // Characters with the Unicode Line Break property PO and the East Asian Width property
        (RuleBreakData::LINE_PROPERTY_PO_EAW, _, _) => ja_zh,
        // breaks after prefixes:
        // Characters with the Unicode Line Break property PR and the East Asian Width property
        (_, _, RuleBreakData::LINE_PROPERTY_PR_EAW) => ja_zh,
        _ => return None,
    })
}

#[derive(Debug)]
pub(super) struct LineBreakIteratorV1<'data, 's, Y: RuleBreakType> {
    input: Y::IterAttr<'s>,
    iter: Y::IterAttr<'s>,
    len: usize,
    current_pos_data: Option<(usize, Y::CharType)>,
    result_cache: ResultCache,
    data: &'data RuleBreakData<'data>,
    options: ResolvedLineBreakOptions,
    complex: ComplexPayloadsBorrowed<'data>,
    // Should return None if there is no complex handling
    pub(crate) handle_complex:
        fn(&mut LineBreakIteratorV1<'data, 's, Y>, Y::CharType) -> Option<usize>,
}

impl<'data, 's, Y: RuleBreakType> LineBreakIteratorV1<'data, 's, Y> {
    pub(crate) fn new(
        iter: Y::IterAttr<'s>,
        len: usize,
        data: &'data RuleBreakData<'data>,
        options: ResolvedLineBreakOptions,
        complex: ComplexPayloadsBorrowed<'data>,
        handle_complex: fn(&mut LineBreakIteratorV1<'data, 's, Y>, Y::CharType) -> Option<usize>,
    ) -> Self {
        Self {
            input: iter.clone(),
            iter,
            len,
            current_pos_data: None,
            result_cache: Default::default(),
            data,
            options,
            complex,
            handle_complex,
        }
    }
}

impl<Y: RuleBreakType> Iterator for LineBreakIteratorV1<'_, '_, Y> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.options.strictness == LineBreakStrictness::Anywhere {
            use crate::grapheme::*;
            let mut grapheme_iter =
                GraphemeClusterBreakIterator(GraphemeClusterBreakIteratorInner::V1(
                    crate::rule_segmenter_v1::RuleBreakIterator {
                        input: self.input.clone(),
                        iter: self.iter.clone(),
                        len: self.len,
                        current_pos_data: self.current_pos_data,
                        data: match self.complex.grapheme.0 {
                            GraphemeClusterSegmenterBorrowedInner::V1(data) => data,
                            #[cfg(feature = "unstable")]
                            GraphemeClusterSegmenterBorrowedInner::V2(_) => unreachable!(),
                        },
                        result_cache: Default::default(),
                        complex: None,
                        boundary_property: 0,
                        locale_override: None,
                        handle_complex: crate::rule_segmenter_v1::empty_handle_complex::<Y>,
                    },
                ));
            let r = grapheme_iter.next();
            #[cfg_attr(not(feature = "unstable"), allow(irrefutable_let_patterns))]
            let GraphemeClusterBreakIterator(GraphemeClusterBreakIteratorInner::V1(grapheme_iter)) =
                grapheme_iter
            else {
                unreachable!();
            };
            self.iter = grapheme_iter.iter;
            self.len = grapheme_iter.len;
            self.current_pos_data = grapheme_iter.current_pos_data;
            return r;
        }

        match self.check_eof() {
            StringBoundaryPosType::Start => return Some(0),
            StringBoundaryPosType::End => return None,
            _ => (),
        }

        // If we have break point cache by previous run, return this result
        if let Some(&first_pos) = self.result_cache.as_slice().first() {
            let mut i = 0;
            loop {
                if i == first_pos {
                    self.result_cache.next();
                    return self.get_current_position();
                }
                i += self.get_current_codepoint().map_or(0, Y::char_len);
                self.advance_iter();
                if self.is_eof() {
                    self.result_cache = Default::default();
                    return Some(self.len);
                }
            }
        }

        // The state prior to a sequence of CM and ZWJ affected by rule LB9.
        let mut lb9_left: Option<u8> = None;
        // Whether LB9 was applied to a ZWJ, so that breaks at the current
        // position must be suppressed.
        let mut lb8a_after_lb9 = false;

        'a: loop {
            debug_assert!(!self.is_eof());

            let left_codepoint = self.get_current_codepoint()?;
            self.advance_iter();
            let Some(right_codepoint) = self.get_current_codepoint() else {
                return Some(self.len);
            };

            let left_prop = lb9_left.unwrap_or_else(|| self.get_linebreak_property(left_codepoint));
            let right_prop = self.get_linebreak_property(right_codepoint);

            // UAX14 doesn't have Thai etc, so use another way.
            if Y::CAN_CONTAIN_SA
                && self.get_linebreak_property(left_codepoint) == self.data.complex_property
                && right_prop == self.data.complex_property
            {
                let result = (self.handle_complex)(self, left_codepoint);
                if result.is_some() {
                    return result;
                }
                // I may have to fetch text until non-SA character?.
            }

            let after_zwj = lb8a_after_lb9
                || (lb9_left.is_none() && left_prop == RuleBreakData::LINE_PROPERTY_ZWJ);

            if (right_prop == RuleBreakData::LINE_PROPERTY_CM
                || right_prop == RuleBreakData::LINE_PROPERTY_ZWJ)
                && left_prop != RuleBreakData::LINE_PROPERTY_BK
                && left_prop != RuleBreakData::LINE_PROPERTY_CR
                && left_prop != RuleBreakData::LINE_PROPERTY_LF
                && left_prop != RuleBreakData::LINE_PROPERTY_NL
                && left_prop != RuleBreakData::LINE_PROPERTY_SP
                && left_prop != RuleBreakData::LINE_PROPERTY_ZW
            {
                lb9_left = Some(left_prop);
                lb8a_after_lb9 = right_prop == RuleBreakData::LINE_PROPERTY_ZWJ;
                continue;
            } else {
                lb9_left = None;
                lb8a_after_lb9 = false;
            }

            // CSS word-break property handling
            #[allow(clippy::single_match)]
            if self.options.word_option == LineBreakWordOption::KeepAll {
                //  typographic letter units shouldn't be break
                if matches!(
                    left_prop,
                    RuleBreakData::LINE_PROPERTY_AI
                        | RuleBreakData::LINE_PROPERTY_AL
                        | RuleBreakData::LINE_PROPERTY_ID
                        | RuleBreakData::LINE_PROPERTY_NU
                        | RuleBreakData::LINE_PROPERTY_HY
                        | RuleBreakData::LINE_PROPERTY_H2
                        | RuleBreakData::LINE_PROPERTY_H3
                        | RuleBreakData::LINE_PROPERTY_JL
                        | RuleBreakData::LINE_PROPERTY_JV
                        | RuleBreakData::LINE_PROPERTY_JT
                        | RuleBreakData::LINE_PROPERTY_CJ
                ) && matches!(
                    right_prop,
                    RuleBreakData::LINE_PROPERTY_AI
                        | RuleBreakData::LINE_PROPERTY_AL
                        | RuleBreakData::LINE_PROPERTY_ID
                        | RuleBreakData::LINE_PROPERTY_NU
                        | RuleBreakData::LINE_PROPERTY_HY
                        | RuleBreakData::LINE_PROPERTY_H2
                        | RuleBreakData::LINE_PROPERTY_H3
                        | RuleBreakData::LINE_PROPERTY_JL
                        | RuleBreakData::LINE_PROPERTY_JV
                        | RuleBreakData::LINE_PROPERTY_JT
                        | RuleBreakData::LINE_PROPERTY_CJ
                ) {
                    continue;
                }
            }

            // CSS line-break property handling
            match self.options.strictness {
                LineBreakStrictness::Normal
                    if is_break_utf32_by_normal(right_codepoint.into(), self.options.ja_zh)
                        && !after_zwj =>
                {
                    return self.get_current_position();
                }
                LineBreakStrictness::Loose => {
                    if let Some(breakable) = is_break_utf32_by_loose(
                        right_codepoint.into(),
                        left_prop,
                        right_prop,
                        self.options.ja_zh,
                    ) {
                        if breakable && !after_zwj {
                            return self.get_current_position();
                        }
                        continue;
                    }
                }
                _ => (),
            };

            // If break_state is equals or grater than 0, it is alias of property.
            match self.data.get_break_state_from_table(left_prop, right_prop) {
                BreakState::Break | BreakState::NoMatch => {
                    if after_zwj {
                        continue;
                    } else {
                        return self.get_current_position();
                    }
                }
                BreakState::Keep => continue,
                BreakState::Index(mut index) | BreakState::Intermediate(mut index) => {
                    let mut previous_iter = self.iter.clone();
                    let mut previous_pos_data = self.current_pos_data;
                    let mut previous_is_after_zwj = after_zwj;

                    // Since we are building up a state in this inner loop, we do not
                    // need an analogue of lb9_left; continuing the inner loop preserves
                    // `index` which is the current state, and thus implements the
                    // “treat as” rule.
                    let mut left_prop_pre_lb9 = right_prop;

                    // current state isn't resolved due to intermediating.
                    // Example, [AK] [AS] is processing LB28a, but if not matched after fetching
                    // data, we should break after [AK].
                    let is_intermediate_rule_no_match = if lb8a_after_lb9 {
                        // left was ZWJ so we don't break between ZWJ.
                        true
                    } else {
                        index > self.data.last_codepoint_property
                    };

                    loop {
                        self.advance_iter();
                        let after_zwj = left_prop_pre_lb9 == RuleBreakData::LINE_PROPERTY_ZWJ;

                        let previous_break_state_is_cp_prop =
                            index <= self.data.last_codepoint_property;

                        let Some(prop) = self.get_current_linebreak_property() else {
                            // Reached EOF. But we are analyzing multiple characters now, so next break may be previous point.
                            let break_state = self
                                .data
                                .get_break_state_from_table(index, self.data.eot_property);
                            if break_state == BreakState::NoMatch {
                                self.iter = previous_iter;
                                self.current_pos_data = previous_pos_data;
                                if previous_is_after_zwj {
                                    // Do not break [AK] [ZWJ] ÷ [AS] (eot).
                                    continue 'a;
                                } else {
                                    return self.get_current_position();
                                }
                            }
                            // EOF
                            return Some(self.len);
                        };

                        if (prop == RuleBreakData::LINE_PROPERTY_CM
                            || prop == RuleBreakData::LINE_PROPERTY_ZWJ)
                            && left_prop_pre_lb9 != RuleBreakData::LINE_PROPERTY_BK
                            && left_prop_pre_lb9 != RuleBreakData::LINE_PROPERTY_CR
                            && left_prop_pre_lb9 != RuleBreakData::LINE_PROPERTY_LF
                            && left_prop_pre_lb9 != RuleBreakData::LINE_PROPERTY_NL
                            && left_prop_pre_lb9 != RuleBreakData::LINE_PROPERTY_SP
                            && left_prop_pre_lb9 != RuleBreakData::LINE_PROPERTY_ZW
                        {
                            left_prop_pre_lb9 = prop;
                            continue;
                        }

                        match self.data.get_break_state_from_table(index, prop) {
                            BreakState::Keep => continue 'a,
                            BreakState::NoMatch => {
                                self.iter = previous_iter;
                                self.current_pos_data = previous_pos_data;
                                if after_zwj {
                                    // Break [AK] ÷ [AS] [ZWJ] [XX],
                                    // but not [AK] [ZWJ] ÷ [AS] [ZWJ] [XX].
                                    if is_intermediate_rule_no_match && !previous_is_after_zwj {
                                        return self.get_current_position();
                                    }
                                    continue 'a;
                                } else if previous_is_after_zwj {
                                    // Do not break [AK] [ZWJ] ÷ [AS] [XX].
                                    continue 'a;
                                } else {
                                    return self.get_current_position();
                                }
                            }
                            BreakState::Break => {
                                if after_zwj {
                                    continue 'a;
                                } else {
                                    return self.get_current_position();
                                }
                            }
                            BreakState::Intermediate(i) => {
                                index = i;
                                previous_iter = self.iter.clone();
                                previous_pos_data = self.current_pos_data;
                                previous_is_after_zwj = after_zwj;
                            }
                            BreakState::Index(i) => {
                                index = i;
                                if previous_break_state_is_cp_prop {
                                    previous_iter = self.iter.clone();
                                    previous_pos_data = self.current_pos_data;
                                    previous_is_after_zwj = after_zwj;
                                }
                            }
                        }
                        left_prop_pre_lb9 = prop;
                    }
                }
            }
        }
    }
}

enum StringBoundaryPosType {
    Start,
    Middle,
    End,
}

impl<Y: RuleBreakType> LineBreakIteratorV1<'_, '_, Y> {
    fn advance_iter(&mut self) {
        self.current_pos_data = self.iter.next();
    }

    fn is_eof(&self) -> bool {
        self.current_pos_data.is_none()
    }

    #[inline]
    fn check_eof(&mut self) -> StringBoundaryPosType {
        if self.is_eof() {
            self.advance_iter();
            if self.is_eof() {
                if self.len == 0 {
                    // Empty string. Since `self.current_pos_data` is always going to be empty,
                    // we never read `self.len` except for here, so we can use it to mark that
                    // we have already returned the single empty-string breakpoint.
                    self.len = 1;
                    StringBoundaryPosType::Start
                } else {
                    StringBoundaryPosType::End
                }
            } else {
                StringBoundaryPosType::Start
            }
        } else {
            StringBoundaryPosType::Middle
        }
    }

    fn get_current_position(&self) -> Option<usize> {
        self.current_pos_data.map(|(pos, _)| pos)
    }

    fn get_current_codepoint(&self) -> Option<Y::CharType> {
        self.current_pos_data.map(|(_, codepoint)| codepoint)
    }

    fn get_linebreak_property(&self, codepoint: Y::CharType) -> u8 {
        match (
            (self.options.word_option, self.options.strictness),
            self.data.property_table.get32(codepoint.into()),
        ) {
            // CJ is treated as NS by default, yielding strict line breaking.
            // https://www.unicode.org/reports/tr14/#CJ
            (
                (LineBreakWordOption::BreakAll, _)
                | (_, LineBreakStrictness::Loose | LineBreakStrictness::Normal),
                RuleBreakData::LINE_PROPERTY_CJ,
            ) => RuleBreakData::LINE_PROPERTY_ID, // All CJ's General_Category is Other_Letter (Lo).
            ((LineBreakWordOption::BreakAll, _), p) if p == self.data.complex_property => {
                RuleBreakData::LINE_PROPERTY_ID
            }
            (
                (LineBreakWordOption::BreakAll, _),
                RuleBreakData::LINE_PROPERTY_AL | RuleBreakData::LINE_PROPERTY_NU,
            ) => RuleBreakData::LINE_PROPERTY_ID,
            (_, prop) => prop,
        }
    }

    fn get_current_linebreak_property(&self) -> Option<u8> {
        self.get_current_codepoint()
            .map(|c| self.get_linebreak_property(c))
    }
}

pub(super) fn line_handle_complex<T>(
    iter: &mut LineBreakIteratorV1<'_, '_, T>,
    left_codepoint: T::CharType,
) -> Option<usize>
where
    T: ComplexRunSegmenter,
{
    // word segmenter doesn't define break rules for some scripts such as Thai.
    let start_iter = iter.iter.clone();
    let start_point = iter.current_pos_data;
    loop {
        debug_assert!(!iter.is_eof());
        iter.advance_iter();
        if let Some(current_codepoint) = iter.get_current_codepoint() {
            if iter.get_linebreak_property(current_codepoint) != iter.data.complex_property {
                break;
            }
        } else {
            // EOF
            break;
        }
    }
    let run_end = iter.current_pos_data.map_or(iter.len, |(pos, _)| pos);

    // Restore iterator to move to head of complex string
    iter.iter = start_iter;
    iter.current_pos_data = start_point;
    let run_start = start_point.map_or(iter.len, |(pos, _)| pos) - T::char_len(left_codepoint);
    let breaks = T::segment_complex_run(iter.complex, &iter.input, run_start, run_end);
    let previous_offset = start_point.map_or(iter.len, |(pos, _)| pos) - run_start;
    iter.result_cache = result_cache_from_offsets(breaks, previous_offset);
    let first_pos = *iter.result_cache.as_slice().first()?;
    let mut i = 0;
    loop {
        if i == first_pos {
            iter.result_cache.next();
            return iter.get_current_position();
        }
        debug_assert!(
            i < first_pos,
            "we should always arrive at first_pos: near index {:?}",
            iter.get_current_position()
        );
        i += iter.get_current_codepoint().map_or(0, T::char_len);
        iter.advance_iter();
        if iter.is_eof() {
            iter.result_cache = Default::default();
            return Some(iter.len);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use icu_provider::prelude::*;

    #[test]
    fn linebreak_property() {
        let super::super::LineBreakIteratorInner::V1(iterator) =
            LineSegmenter::new_for_non_complex_scripts(Default::default())
                .segment_str("input")
                .0
        else {
            unreachable!()
        };

        assert_eq!(
            iterator.get_linebreak_property('\u{0020}'),
            RuleBreakData::LINE_PROPERTY_SP
        );
        assert_eq!(
            iterator.get_linebreak_property('\u{0022}'),
            RuleBreakData::LINE_PROPERTY_QU
        );
        assert_eq!(
            iterator.get_linebreak_property('('),
            RuleBreakData::LINE_PROPERTY_OP_OP30
        );
        assert_eq!(
            iterator.get_linebreak_property('\u{0030}'),
            RuleBreakData::LINE_PROPERTY_NU
        );
        assert_eq!(
            iterator.get_linebreak_property('['),
            RuleBreakData::LINE_PROPERTY_OP_OP30
        );
        assert_eq!(
            iterator.get_linebreak_property('\u{1f3fb}'),
            RuleBreakData::LINE_PROPERTY_EM
        );
        assert_eq!(
            iterator.get_linebreak_property('\u{20000}'),
            RuleBreakData::LINE_PROPERTY_ID
        );
        assert_eq!(
            iterator.get_linebreak_property('\u{e0020}'),
            RuleBreakData::LINE_PROPERTY_CM
        );
        assert_eq!(
            iterator.get_linebreak_property('\u{3041}'),
            RuleBreakData::LINE_PROPERTY_CJ
        );
        assert_eq!(
            iterator.get_linebreak_property('\u{0025}'),
            RuleBreakData::LINE_PROPERTY_PO
        );
        assert_eq!(
            iterator.get_linebreak_property('\u{00A7}'),
            RuleBreakData::LINE_PROPERTY_AI
        );
        assert_eq!(
            iterator.get_linebreak_property('\u{50005}'),
            RuleBreakData::LINE_PROPERTY_XX
        );
        assert_eq!(
            iterator.get_linebreak_property('\u{17D6}'),
            RuleBreakData::LINE_PROPERTY_NS
        );
        assert_eq!(
            iterator.get_linebreak_property('\u{2014}'),
            RuleBreakData::LINE_PROPERTY_B2
        );
    }

    #[test]
    fn break_rule() {
        let payload = DataProvider::<SegmenterBreakLineV1>::load(&Baked, Default::default())
            .expect("Loading should succeed!")
            .payload;
        let lb_data: &RuleBreakData = payload.get();

        let is_break = |left, right| {
            matches!(
                lb_data.get_break_state_from_table(left, right),
                BreakState::Break | BreakState::NoMatch
            )
        };

        // LB4
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_BK,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            true
        );
        // LB5
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_CR,
                RuleBreakData::LINE_PROPERTY_LF
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_CR,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            true
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_LF,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            true
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_NL,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            true
        );
        // LB6
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_BK
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_CR
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_LF
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_NL
            ),
            false
        );
        // LB7
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_SP
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_ZW
            ),
            false
        );
        // LB8
        // LB8a and LB9 omitted: These are handled outside of the state table.
        // LB10
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_ZWJ,
                RuleBreakData::LINE_PROPERTY_SP
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_SP,
                RuleBreakData::LINE_PROPERTY_CM
            ),
            true
        );
        // LB11
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_WJ
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_WJ,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            false
        );
        // LB12
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_GL,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            false
        );
        // LB12a
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_GL
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_SP,
                RuleBreakData::LINE_PROPERTY_GL
            ),
            true
        );
        // LB13
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_CL
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_CP
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_EX
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_IS
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_SY
            ),
            false
        );
        // LB18
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_SP,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            true
        );
        // LB19
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_QU
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_QU,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            false
        );
        // LB20
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_CB
            ),
            true
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_CB,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            true
        );
        // LB20
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_BA
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_HY
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_NS
            ),
            false
        );
        // LB21
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_BA
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_BB,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_ID,
                RuleBreakData::LINE_PROPERTY_BA
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_ID,
                RuleBreakData::LINE_PROPERTY_NS
            ),
            false
        );
        // LB21a
        // LB21b
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_SY,
                RuleBreakData::LINE_PROPERTY_HL
            ),
            false
        );
        // LB22
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_IN
            ),
            false
        );
        // LB 23
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_NU
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_HL,
                RuleBreakData::LINE_PROPERTY_NU
            ),
            false
        );
        // LB 23a
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_PR,
                RuleBreakData::LINE_PROPERTY_ID
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_PR,
                RuleBreakData::LINE_PROPERTY_EB
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_PR,
                RuleBreakData::LINE_PROPERTY_EM
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_ID,
                RuleBreakData::LINE_PROPERTY_PO
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_EB,
                RuleBreakData::LINE_PROPERTY_PO
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_EM,
                RuleBreakData::LINE_PROPERTY_PO
            ),
            false
        );
        // LB26
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_JL,
                RuleBreakData::LINE_PROPERTY_JL
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_JL,
                RuleBreakData::LINE_PROPERTY_JV
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_JL,
                RuleBreakData::LINE_PROPERTY_H2
            ),
            false
        );
        // LB27
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_JL,
                RuleBreakData::LINE_PROPERTY_IN
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_JL,
                RuleBreakData::LINE_PROPERTY_PO
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_PR,
                RuleBreakData::LINE_PROPERTY_JL
            ),
            false
        );
        // LB28
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_AL,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_HL,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            false
        );
        // LB29
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_IS,
                RuleBreakData::LINE_PROPERTY_AL
            ),
            false
        );
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_IS,
                RuleBreakData::LINE_PROPERTY_HL
            ),
            false
        );
        // LB30b
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_EB,
                RuleBreakData::LINE_PROPERTY_EM
            ),
            false
        );
        // LB31
        assert_eq!(
            is_break(
                RuleBreakData::LINE_PROPERTY_ID,
                RuleBreakData::LINE_PROPERTY_ID
            ),
            true
        );
    }
}
