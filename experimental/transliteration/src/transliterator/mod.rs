// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod replaceable;

use core::ops::Range;

use crate::provider::{RuleBasedTransliterator, TransliteratorRulesV1Marker};
use crate::TransliteratorError;
use core::str;
use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use icu_collections::codepointinvlist::CodePointInversionList;
use icu_provider::_internal::locid::Locale;
use icu_provider::{DataError, DataLocale, DataPayload, DataProvider, DataRequest};

use litemap::LiteMap;
use replaceable::*;

use crate::provider::{FunctionCall, Rule, RuleULE, SimpleId, VarTable};
use alloc::vec::Vec;
use icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList;
use zerofrom::ZeroFrom;
use zerovec::VarZeroSlice;

type Filter<'a> = CodePointInversionList<'a>;

pub trait CustomTransliterator {
    /// Transliterate the portion of the input string specified by the byte indices in the range.
    /// 
    /// The returned `String` should just be the transliteration of `input[range]`.
    fn transliterate(&self, input: &str, range: Range<usize>) -> String;
}

struct NFCTransliterator {}

enum InternalTransliterator {
    RuleBased(DataPayload<TransliteratorRulesV1Marker>),
    NFC(NFCTransliterator),
    Dyn(Box<dyn CustomTransliterator>),
}

impl InternalTransliterator {
    fn transliterate(&self, mut rep: Replaceable, env: &Env) {
        match self {
            Self::RuleBased(rbt) => rbt.get().transliterate(rep, env),
            // TODO(#3910): internal hardcoded transliterators
            Self::NFC(_nfc) => (),
            Self::Dyn(custom) => {
                let replacement = custom.transliterate(rep.as_str(), rep.allowed_range());
                // SAFETY: rep.allowed_range() returns a range with valid UTF-8 bounds, and the bytes are valid UTF-8 as they come from a String
                unsafe { rep.splice(rep.allowed_range(), replacement.as_bytes()) };
            }
        }
    }
}

type Env = LiteMap<String, InternalTransliterator>;

struct Transliterator {
    transliterator: DataPayload<TransliteratorRulesV1Marker>,
    env: Env,
}

impl Transliterator {
    #[cfg(feature = "compiled_data")]
    pub fn try_new(locale: Locale) -> Result<Transliterator, TransliteratorError> {
        let provider = crate::provider::Baked;
        Self::try_new_unstable(locale, &provider)
    }

    pub fn try_new_unstable<P>(
        locale: Locale,
        provider: &P,
    ) -> Result<Transliterator, TransliteratorError>
    where
        P: DataProvider<TransliteratorRulesV1Marker>,
    {
        debug_assert!(!locale.extensions.transform.is_empty());

        let mut data_locale = DataLocale::default();
        data_locale.set_aux(locale.to_string().parse()?);
        let req = DataRequest {
            locale: &data_locale,
            metadata: Default::default(),
        };
        let payload = provider.load(req)?.take_payload()?;
        let rbt = payload.get();
        if !rbt.visibility {
            // transliterator is internal
            return Err(TransliteratorError::InternalOnly);
        }
        let mut env = LiteMap::new();
        Transliterator::load_dependencies(rbt, &mut env, provider)?;
        Ok(Transliterator {
            transliterator: payload,
            env,
        })
    }

    fn load_dependencies<P>(
        rbt: &RuleBasedTransliterator<'_>,
        env: &mut LiteMap<String, InternalTransliterator>,
        provider: &P,
    ) -> Result<(), TransliteratorError>
    where
        P: DataProvider<TransliteratorRulesV1Marker>,
    {
        for dep in rbt.dependencies.iter() {
            if !env.contains_key(dep) {
                let internal_t = Self::load_nested(dep, provider)?;
                if let InternalTransliterator::RuleBased(rbt) = &internal_t {
                    Self::load_dependencies(rbt.get(), env, provider)?;
                }
                env.insert(dep.to_string(), internal_t);
            }
        }
        Ok(())
    }

    // TODO: add hook for custom
    fn load_nested<P>(id: &str, provider: &P) -> Result<InternalTransliterator, TransliteratorError>
    where
        P: DataProvider<TransliteratorRulesV1Marker>,
    {
        if let Some(special) = id.strip_prefix("x-") {
            match special {
                "Any-NFC" => Ok(InternalTransliterator::NFC(NFCTransliterator {})),
                _ => Ok(InternalTransliterator::NFC(NFCTransliterator {})),
                s => Err(DataError::custom("unavailable transliterator")
                    .with_debug_context(s)
                    .into()),
            }
        } else {
            let mut data_locale = DataLocale::default();
            data_locale.set_aux(id.parse()?);
            let req = DataRequest {
                locale: &data_locale,
                metadata: Default::default(),
            };
            let rbt = provider.load(req)?.take_payload()?;
            Ok(InternalTransliterator::RuleBased(rbt))
        }
    }

    pub fn transliterate(&self, input: String) -> String {
        let mut buffer = input.into_bytes();
        // SAFETY: buffer is constructed from a String
        let rep = unsafe { Replaceable::new(&mut buffer) };
        self.transliterator.get().transliterate(rep, &self.env);
        debug_assert!(str::from_utf8(&buffer[..]).is_ok());
        // SAFETY: Replaceable's invariants ensure that buffer is always valid UTF-8
        unsafe { String::from_utf8_unchecked(buffer) }
    }
}

impl<'a> RuleBasedTransliterator<'a> {
    /// Transliteration using rules works as follows:
    ///  1. Split the input modifiable range of the Replaceable according into runs according to self.filter 
    ///  2. Transliterate each run in sequence 
    ///      i. Transliterate the first id_group, then the first rule_group, then the second id_group, etc.
    // muster-child signature of internal transliteration
    fn transliterate(&self, mut rep: Replaceable, env: &Env) {
        // assumes the cursor is at the right position.

        debug_assert_eq!(self.id_group_list.len(), self.rule_group_list.len());
        debug_assert!(rep.allowed_range().contains(&rep.cursor()));

        // TODO: https://unicode-org.atlassian.net/jira/software/c/projects/ICU/issues/ICU-22469
        while let Some(filtered_run) = unsafe { rep.next_filtered_run(rep.cursor(), &self.filter) } {


        }
    }




    //     // first: process the groups in order, i.e., id_group_list[0], rule_group_list[0], id_group_list[1], rule_group_list[1], ...
    //     for (id_group, rule_group) in self.id_group_list.iter().zip(self.rule_group_list.iter()) {
    //         // first handle id_group
    //         for single_id in id_group.iter() {
    //             let id = SimpleId::zero_from(single_id);
    //             let transliterated = self.transliterate_nested(
    //                 id,
    //                 unsafe { str::from_utf8_unchecked(&buf[..]) },
    //                 &filter,
    //                 env,
    //             );
    //             buf = transliterated.into_bytes();
    //         }

    //         // then handle rule_group
    //         let transliterated = self.transliterate_rule_group(
    //             rule_group,
    //             unsafe { str::from_utf8_unchecked(&buf[..]) },
    //             &filter,
    //             env,
    //         );
    //         buf = transliterated.into_bytes();
    //     }

    //     unsafe { String::from_utf8_unchecked(buf) }
    // }

    // fn transliterate_nested(
    //     &self,
    //     id: SimpleId<'a>,
    //     input: &str,
    //     filter: &FilterChain,
    //     env: &Env<'a>,
    // ) -> String {
    //     let filter = filter.add(&id.filter);
    //     // this get succeeds for valid RBT serializations
    //     if let Some(transliterator) = env.get(&id.id) {
    //         return transliterator.transliterate(input, &filter, env);
    //     }
    //     // GIGO, we don't want to panic
    //     String::new()
    // }

    // fn transliterate_rule_group(
    //     &self,
    //     rules: &VarZeroSlice<RuleULE>,
    //     input: &str,
    //     filter: &FilterChain,
    //     env: &Env<'a>,
    // ) -> String {
    //     // for (start_index, run_length) in filtered_runs...

    //     for rule in rules.iter() {
    //         let rule = Rule::zero_from(rule);
    //         let transliterated = self.transliterate_rule(rule, input, env);
    //     }

    //     String::new()
    // }

    // fn transliterate_rule(&self, rule: Rule<'a>, input: &str, env: &Env<'a>) -> String {
    //     String::new()
    // }

    // // returns the byte-length of the run after transliteration
    // fn transliterate_run(
    //     &self,
    //     inout: &mut Vec<u8>,
    //     start_index: usize,
    //     run_length: usize,
    //     env: &Env<'a>,
    // ) -> usize {
    //     let stop_when_remaining_length_is = inout.len() - start_index - run_length;
    //     let mut cursor = start_index;
    //     let mut remaining_length = run_length;
    //     0
    // }
}


enum QuantifierKind {
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
}

enum SpecialMatcher<'a> {
    Compound(&'a str),
    Quantifier(QuantifierKind, &'a str),
    Segment(&'a str),
    UnicodeSet(CodePointInversionListAndStringList<'a>),
    AnchorStart,
    AnchorEnd,
}

enum SpecialReplacer<'a> {
    Compound(&'a str),
    FunctionCall(FunctionCall<'a>),
    BackReference(u16),
    LeftPlaceholderCursor(u16),
    RightPlaceholderCursor(u16),
    PureCursor,
}

enum VarTableElement<'a> {
    Compound(&'a str),
    Quantifier(QuantifierKind, &'a str),
    Segment(&'a str),
    UnicodeSet(CodePointInversionListAndStringList<'a>),
    FunctionCall(FunctionCall<'a>),
    BackReference(u16),
    AnchorStart,
    AnchorEnd,
    LeftPlaceholderCursor(u16),
    RightPlaceholderCursor(u16),
    PureCursor,
}

impl<'a> VarTableElement<'a> {
    fn to_replacer(self) -> Option<SpecialReplacer<'a>> {
        Some(match self {
            VarTableElement::Compound(elt) => SpecialReplacer::Compound(elt),
            VarTableElement::FunctionCall(elt) => SpecialReplacer::FunctionCall(elt),
            VarTableElement::BackReference(elt) => SpecialReplacer::BackReference(elt),
            VarTableElement::LeftPlaceholderCursor(elt) => SpecialReplacer::LeftPlaceholderCursor(elt),
            VarTableElement::RightPlaceholderCursor(elt) => SpecialReplacer::RightPlaceholderCursor(elt),
            VarTableElement::PureCursor => SpecialReplacer::PureCursor,
            _ => return None,
        })
    }

    fn to_matcher(self) -> Option<SpecialMatcher<'a>> {
        Some(match self {
            VarTableElement::Compound(elt) => SpecialMatcher::Compound(elt),
            VarTableElement::Quantifier(kind, elt) => SpecialMatcher::Quantifier(kind, elt),
            VarTableElement::Segment(elt) => SpecialMatcher::Segment(elt),
            VarTableElement::UnicodeSet(elt) => SpecialMatcher::UnicodeSet(elt),
            VarTableElement::AnchorEnd => SpecialMatcher::AnchorEnd,
            VarTableElement::AnchorStart => SpecialMatcher::AnchorStart,
            _ => return None,
        })
    }
}

impl<'a> VarTable<'a> {
    // TODO: these must be the same as during datagen. Find some place to define them *once*
    const BASE: u32 = '\u{F0000}' as u32;
    const MAX_DYNAMIC: u32 = '\u{FFFF0}' as u32;
    const RESERVED_PURE_CURSOR: u32 = '\u{FFFFB}' as u32;
    const RESERVED_ANCHOR_START: u32 = '\u{FFFFC}' as u32;
    const RESERVED_ANCHOR_END: u32 = '\u{FFFFD}' as u32;

    fn lookup(&'a self, query: char) -> Option<VarTableElement<'a>> {
        let query = query as u32;
        if query < Self::BASE {
            return None;
        }
        if query > Self::MAX_DYNAMIC {
            return match query {
                Self::RESERVED_ANCHOR_END => Some(VarTableElement::AnchorEnd),
                Self::RESERVED_ANCHOR_START => Some(VarTableElement::AnchorStart),
                _ => None,
            };
        }
        let idx = query - Self::BASE;
        let mut idx = idx as usize;

        // TODO: these lookups must be in the same order as during datagen. Best way to enforce this?
        // note: might be worth trying to speed up these lookups by binary searching?
        let mut next_base = self.compounds.len();
        if idx < next_base {
            return Some(VarTableElement::Compound(&self.compounds[idx]));
        }
        // no underflow for all these idx subtractions, as idx is always >= next_base
        idx -= next_base;
        next_base = self.quantifiers_opt.len();
        if idx < next_base {
            return Some(VarTableElement::Quantifier(
                QuantifierKind::ZeroOrOne,
                &self.quantifiers_opt[idx],
            ));
        }
        idx -= next_base;
        next_base = self.quantifiers_kleene.len();
        if idx < next_base {
            return Some(VarTableElement::Quantifier(
                QuantifierKind::ZeroOrMore,
                &self.quantifiers_kleene[idx],
            ));
        }
        idx -= next_base;
        next_base = self.quantifiers_kleene_plus.len();
        if idx < next_base {
            return Some(VarTableElement::Quantifier(
                QuantifierKind::OneOrMore,
                &self.quantifiers_kleene_plus[idx],
            ));
        }
        idx -= next_base;
        next_base = self.segments.len();
        if idx < next_base {
            return Some(VarTableElement::Segment(&self.segments[idx]));
        }
        idx -= next_base;
        next_base = self.unicode_sets.len();
        if idx < next_base {
            return Some(VarTableElement::UnicodeSet(
                CodePointInversionListAndStringList::zero_from(&self.unicode_sets[idx]),
            ));
        }
        idx -= next_base;
        next_base = self.function_calls.len();
        if idx < next_base {
            return Some(VarTableElement::FunctionCall(FunctionCall::zero_from(
                &self.function_calls[idx],
            )));
        }
        idx -= next_base;
        next_base = self.max_left_placeholder_count as usize;
        if idx < next_base {
            // + 1 because index 0 represents 1 placeholder, etc.
            // cast is guaranteed because query is inside a range of less than 2^16 elements
            return Some(VarTableElement::LeftPlaceholderCursor(idx as u16 + 1));
        }
        idx -= next_base;
        next_base = self.max_right_placeholder_count as usize;
        if idx < next_base {
            // + 1 because index 0 represents 1 placeholder, etc.
            // cast is guaranteed because query is inside a range of less than 2^16 elements
            return Some(VarTableElement::RightPlaceholderCursor(idx as u16 + 1));
        }
        idx -= next_base;
        // idx must be a backreference (an u16 encoded as <itself> indices past the last valid other index)
        // + 1 because index 0 represents $1, etc.
        // cast is guaranteed because query is inside a range of less than 2^16 elements
        Some(VarTableElement::BackReference(idx as u16 + 1))
    }

    fn lookup_matcher(&'a self, query: char) -> Option<SpecialMatcher<'a>> {
        let elt = self.lookup(query)?;
        elt.to_matcher()
    }

    fn lookup_replacer(&'a self, query: char) -> Option<SpecialReplacer<'a>> {
        let elt = self.lookup(query)?;
        elt.to_replacer()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = Transliterator::try_new("de-t-de-d0-ascii".parse().unwrap()).unwrap();
        let input =
            r"Über ältere Lügner lästern ist sehr a\u{0308}rgerlich. Ja, SEHR ÄRGERLICH! - ꜵ";
        let output =
            "Ueber aeltere Luegner laestern ist sehr aergerlich. Ja, SEHR AERGERLICH! - ao";
        assert_eq!(t.transliterate(input.to_string()), output);
    }
}
