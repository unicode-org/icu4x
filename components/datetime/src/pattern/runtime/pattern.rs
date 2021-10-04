// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::super::{reference, PatternItem, TimeGranularity};
use crate::pattern::reference::pattern::dump_buffer_into_formatter;
use alloc::fmt::{self, Write};
use alloc::string::String;
use alloc::{vec, vec::Vec};
use zerovec::ZeroVec;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "provider_serde",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Pattern<'data> {
    #[cfg_attr(feature = "provider_serde", serde(borrow))]
    pub items: ZeroVec<'data, PatternItem>,
    pub(crate) time_granularity: TimeGranularity,
}

impl From<Vec<PatternItem>> for Pattern<'_> {
    fn from(items: Vec<PatternItem>) -> Self {
        Self {
            time_granularity: items.iter().map(Into::into).max().unwrap_or_default(),
            items: ZeroVec::clone_from_slice(&items),
        }
    }
}

impl From<reference::Pattern> for Pattern<'_> {
    fn from(input: reference::Pattern) -> Self {
        Self {
            items: ZeroVec::clone_from_slice(&input.items),
            time_granularity: input.time_granularity,
        }
    }
}

impl Default for Pattern<'_> {
    fn default() -> Self {
        Self {
            items: ZeroVec::Owned(vec![]),
            time_granularity: TimeGranularity::default(),
        }
    }
}

/// This trait is implemented in order to provide the machinery to convert a [`Pattern`] to a UTS 35
/// pattern string. It could also be implemented as the Writeable trait, but at the time of writing
/// this was not done, as this code would need to implement the [`write_len()`] method, which would
/// need to duplicate the branching logic of the [`fmt`](std::fmt) method here. This code is used in generating
/// the data providers and is not as performance sensitive.
impl fmt::Display for Pattern<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer = String::new();
        for pattern_item in self.items.iter() {
            match pattern_item {
                PatternItem::Field(field) => {
                    dump_buffer_into_formatter(&buffer, formatter)?;
                    buffer.clear();
                    let ch: char = field.symbol.into();
                    for _ in 0..field.length as usize {
                        formatter.write_char(ch)?;
                    }
                }
                PatternItem::Literal(ch) => {
                    buffer.push(ch);
                }
            }
        }
        dump_buffer_into_formatter(&buffer, formatter)?;
        buffer.clear();
        Ok(())
    }
}
