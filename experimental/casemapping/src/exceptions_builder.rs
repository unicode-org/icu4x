// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::Error;
use crate::provider::exception_header::{
    ExceptionBits, ExceptionBitsULE, ExceptionSlot, SlotPresence,
};
use crate::provider::exceptions::CaseMappingExceptions;
use std::collections::HashMap;
use zerovec::ule::{AsULE, ULE};
use zerovec::{VarZeroVec, ZeroVec};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ExceptionHeader {
    /// The various slots that are present, masked by ExceptionSlot
    ///
    /// We still store this as a bitmask since it's more convenient to access as one
    pub slot_presence: SlotPresence,
    pub bits: ExceptionBits,
}

impl ExceptionHeader {
    /// Construct from an ICU4C-format u16.
    pub(crate) fn from_integer(int: u16) -> Self {
        let slot_presence =
            SlotPresence(u8::try_from(int & ExceptionHeaderULE::SLOTS_MASK).unwrap_or(0));
        let bits = ExceptionBits::from_integer(
            u8::try_from(int >> ExceptionHeaderULE::BITS_SHIFT).unwrap_or(0),
        );
        Self {
            slot_presence,
            bits,
        }
    }

    /// Convert to an ICU4C-format u16
    #[cfg(feature = "datagen")]
    pub(crate) fn to_integer(self) -> u16 {
        let mut sixteen: u16 = self.slot_presence.0.into();
        sixteen |= (u16::from(self.bits.to_integer())) << ExceptionHeaderULE::BITS_SHIFT;
        sixteen
    }

    // The number of optional slots for this exception
    pub(crate) fn num_slots(&self) -> u16 {
        self.slot_presence.0.count_ones() as u16
    }

    // Returns true if the given slot exists for this exception
    pub(crate) fn has_slot(&self, slot: ExceptionSlot) -> bool {
        self.slot_presence.has_slot(slot)
    }

    // Returns the number of slots between this header and the given slot.
    pub(crate) fn slot_offset(&self, slot: ExceptionSlot) -> usize {
        debug_assert!(self.has_slot(slot));
        let slot_bit = 1 << (slot as u8);
        let previous_slot_mask = slot_bit - 1;
        let previous_slots = self.slot_presence.0 & previous_slot_mask;
        let slot_num = previous_slots.count_ones() as usize;

        if self.bits.double_width_slots {
            slot_num * 2
        } else {
            slot_num
        }
    }
}

/// Packed exception header (format from icu4c, documented in casepropsbuilder.cpp)
///
/// ```text
///       Bits:
///         0..7  Flag bits indicating which optional slots are present (if any):
///               0: Lowercase mapping (code point)
///               1: Case folding (code point)
///               2: Uppercase mapping (code point)
///               3: Titlecase mapping (code point)
///               4: Delta to simple case mapping (code point) (sign stored separately)
///               5: RESERVED
///               6: Closure mappings (string; see below)
///               7: Full mappings (strings; see below)
///            8  Double-width slots. If set, then each optional slot is stored as two
///               elements of the array (high and low halves of 32-bit values) instead of
///               a single element.
///            9  Has no simple case folding, even if there is a simple lowercase mapping
///           10  The value in the delta slot is negative
///           11  Is case-sensitive (not exposed)
///       12..13  Dot type
///           14  Has conditional special casing
///           15  Has conditional case folding
/// ```
///
/// In this struct the RESERVED bit is still allowed to be set, and it will produce a different
/// exception header, but it will not have any other effects.
#[derive(Copy, Clone, PartialEq, Eq, ULE)]
#[repr(packed)]
pub struct ExceptionHeaderULE {
    slot_presence: SlotPresence,
    bits: ExceptionBitsULE,
}

impl ExceptionHeaderULE {
    const SLOTS_MASK: u16 = 0xff;
    const BITS_SHIFT: u16 = 8;
}

impl AsULE for ExceptionHeader {
    type ULE = ExceptionHeaderULE;
    fn from_unaligned(u: ExceptionHeaderULE) -> Self {
        Self {
            slot_presence: u.slot_presence,
            bits: ExceptionBits::from_integer(u.bits.0),
        }
    }

    fn to_unaligned(self) -> ExceptionHeaderULE {
        ExceptionHeaderULE {
            slot_presence: self.slot_presence,
            bits: ExceptionBitsULE(self.bits.to_integer()),
        }
    }
}
// CaseMappingExceptionsBuilder consumes the exceptions data produced by
// casepropsbuilder.cpp in ICU4C. It generates an instance of CaseMappingExceptions. The
// primary difference is that the ICU4C representation stores full mapping and closure
// strings inline in the data, while CaseMappingExceptions uses a side table. As a result,
// the starting index of each exception in the resulting CaseMappingExceptions may have
// changed, so we also produce a map from old indices to new indices that will be used to
// update the data stored in the code point trie.
pub struct CaseMappingExceptionsBuilder<'a> {
    raw_data: &'a [u16],
    raw_data_idx: usize,
    double_slots: bool,
    slot_data: Vec<u16>,
    strings: Vec<String>,
}

impl<'a> CaseMappingExceptionsBuilder<'a> {
    const MAPPINGS_ALL_LENGTHS_MASK: u32 = 0xffff;
    const FULL_MAPPINGS_LENGTH_MASK: u32 = 0xf;
    const FULL_MAPPINGS_LENGTH_SHIFT: u32 = 4;

    const CLOSURE_MAX_LENGTH: u32 = 0xf;

    pub fn new(raw_data: &'a [u16]) -> Self {
        Self {
            raw_data,
            raw_data_idx: 0,
            double_slots: false,
            slot_data: vec![],
            strings: vec![],
        }
    }

    fn done(&self) -> bool {
        self.raw_data_idx >= self.raw_data.len()
    }
    fn read_raw(&mut self) -> Result<u16, Error> {
        let result = self
            .raw_data
            .get(self.raw_data_idx)
            .ok_or(Error::Validation("Incomplete data"))?;
        self.raw_data_idx += 1;
        Ok(*result)
    }
    fn write_raw(&mut self, value: u16) {
        self.slot_data.push(value);
    }

    fn read_slot(&mut self) -> Result<u32, Error> {
        if self.double_slots {
            let hi = self.read_raw()? as u32;
            let lo = self.read_raw()? as u32;
            Ok(hi << 16 | lo)
        } else {
            Ok(self.read_raw()? as u32)
        }
    }
    fn write_slot(&mut self, value: u32) {
        if self.double_slots {
            let hi = (value >> 16) as u16;
            let lo = (value & 0xffff) as u16;
            self.write_raw(hi);
            self.write_raw(lo);
        } else {
            debug_assert!(value & 0xffff == value);
            self.write_raw(value as u16);
        }
    }

    // After reading a string out of the raw data, advance raw_data_idx.
    fn skip_string(&mut self, s: &str) {
        for c in s.chars() {
            self.raw_data_idx += c.len_utf16();
        }
    }

    pub(crate) fn build(
        mut self,
    ) -> Result<(CaseMappingExceptions<'static>, HashMap<u16, u16>), Error> {
        let mut index_map = HashMap::new();
        // The format of the raw data from ICU4C is the same as the format described in
        // exceptions.rs, with the exception of full mapping and closure strings. The
        // header and non-string slots can be copied over without modification. For string
        // slots, we read the length information from the ICU4C slot (described below),
        // read the strings, add the strings to the CaseMappingExceptions string table,
        // and write an updated slot value containing the index of the string in the
        // table. In the case of full mappings, we store the index of the lowercase
        // mapping; the remaining mappings are stored at sequential indices.
        //
        // Full mappings: If there is at least one full (string) case mapping, then the
        // lengths of the mappings are encoded as nibbles in the full mappings slot:
        //     Bits:
        //        0..4   Length of lowercase string
        //        5..7   Length of case folding string
        //        8..11  Length of uppercase string
        //        12..15 Length of titlecase string
        // Mappings that do not exist have length 0. The strings themselves are stored in
        // the above order immediately following the last optional slot, encoded as UTF16.
        //
        // Case closure: If the case closure for a code point includes code points that
        // are not included in the simple or full mappings, then bits 0..3 of the closure
        // mappings slot will contain the number of codepoints in the closure string.
        // (Other bits are reserved.) The closure string itself is encoded as UTF16 and
        // stored following the full mappings data (if it exists) or the final optional
        // slot.
        while !self.done() {
            let old_idx = self.raw_data_idx as u16;
            let new_idx = self.slot_data.len() as u16;
            index_map.insert(old_idx, new_idx);

            // Copy header.
            let header = ExceptionHeader::from_integer(self.read_raw()?);
            self.write_raw(header.to_integer());
            self.double_slots = header.bits.double_width_slots;

            // Copy unmodified slots.
            for slot in [
                ExceptionSlot::Lower,
                ExceptionSlot::Fold,
                ExceptionSlot::Upper,
                ExceptionSlot::Title,
                ExceptionSlot::Delta,
            ] {
                if header.has_slot(slot) {
                    let value = self.read_slot()?;
                    self.write_slot(value);
                }
            }

            // Read the closure and full mappings slots, if they exist.
            let closure_length = if header.has_slot(ExceptionSlot::Closure) {
                Some((self.read_slot()? & Self::CLOSURE_MAX_LENGTH) as usize)
            } else {
                None
            };
            let mappings_lengths = if header.has_slot(ExceptionSlot::FullMappings) {
                Some(self.read_slot()? & Self::MAPPINGS_ALL_LENGTHS_MASK)
            } else {
                None
            };

            // Copy the full mappings strings into the strings table, if they exist.
            let mappings_idx = if let Some(mut lengths) = mappings_lengths {
                let idx = self.strings.len() as u32;
                for _ in 0..4 {
                    let len = lengths & Self::FULL_MAPPINGS_LENGTH_MASK;
                    lengths >>= Self::FULL_MAPPINGS_LENGTH_SHIFT;

                    let start = self.raw_data_idx;
                    let end = start + len as usize;
                    let slice = &self
                        .raw_data
                        .get(start..end)
                        .ok_or(Error::Validation("Incomplete string data"))?;
                    let string =
                        char::decode_utf16(slice.iter().copied()).collect::<Result<String, _>>()?;
                    self.skip_string(&string);
                    self.strings.push(string);
                }
                Some(idx)
            } else {
                None
            };

            // Copy the closure string into the strings table, if it exists.
            let closure_idx = if let Some(len) = closure_length {
                let idx = self.strings.len() as u32;

                let start = self.raw_data_idx;
                let slice = &self
                    .raw_data
                    .get(start..)
                    .ok_or(Error::Validation("Incomplete string data"))?;
                let string = char::decode_utf16(slice.iter().copied())
                    .take(len)
                    .collect::<Result<String, _>>()?;
                self.skip_string(&string);
                self.strings.push(string);
                Some(idx)
            } else {
                None
            };

            // Write the new indices, if they exist.
            if let Some(idx) = closure_idx {
                self.write_slot(idx);
            }
            if let Some(idx) = mappings_idx {
                self.write_slot(idx);
            }
        }

        let slots = ZeroVec::alloc_from_slice(&self.slot_data);
        let strings = VarZeroVec::from(&self.strings);
        Ok((CaseMappingExceptions { slots, strings }, index_map))
    }
}
