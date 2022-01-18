// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub fn get_break_property_utf32(codepoint: u32, property_table: &[&[u8; 1024]; 897]) -> u8 {
    let codepoint = codepoint as usize;
    if codepoint >= 897 * 1024 {
        // Unknown
        return 0;
    }
    property_table[codepoint / 1024][(codepoint & 0x3ff)]
}

#[inline]
pub fn get_break_property_latin1(codepoint: u8, property_table: &[&[u8; 1024]; 897]) -> u8 {
    let codepoint = codepoint as usize;
    property_table[codepoint / 1024][(codepoint & 0x3ff)]
}

#[inline]
pub fn get_break_property_utf8(codepoint: char, property_table: &[&[u8; 1024]; 897]) -> u8 {
    get_break_property_utf32(codepoint as u32, property_table)
}

/// Base implementation of rule based segmenter
#[macro_export]
macro_rules! break_iterator_impl {
    ($name:ident, $iter_attr:ty, $char_type:ty) => {
        /// The struct implementing the [`Iterator`] trait over the segmenter break
        /// opportunities of the given string. Please see the [module-level
        /// documentation] for its usages.
        ///
        /// [`Iterator`]: core::iter::Iterator
        /// [module-level documentation]: ../index.html
        #[allow(dead_code)]
        pub struct $name<'a> {
            iter: $iter_attr,
            len: usize,
            current_pos_data: Option<(usize, $char_type)>,
            result_cache: alloc::vec::Vec<usize>,
            break_state_table: &'a [i8],
            property_table: &'a [&'a [u8; 1024]; 897],
            rule_property_count: usize,
            last_codepoint_property: i8,
            sot_property: u8,
            eot_property: u8,
            complex_property: u8,
        }

        impl<'a> Iterator for $name<'a> {
            type Item = usize;

            fn next(&mut self) -> Option<Self::Item> {
                // If we have break point cache by previous run, return this result
                if !self.result_cache.is_empty() {
                    let mut i = 0;
                    loop {
                        if i == *self.result_cache.first().unwrap() {
                            self.result_cache =
                                self.result_cache.iter().skip(1).map(|r| r - i).collect();
                            return Some(self.current_pos_data.unwrap().0);
                        }
                        i += self.get_current_position_character_len();
                        self.current_pos_data = self.iter.next();
                        if self.current_pos_data.is_none() {
                            // Reach EOF
                            self.result_cache.clear();
                            return Some(self.len);
                        }
                    }
                }

                if self.current_pos_data.is_none() {
                    let current_pos_data = self.iter.next()?;
                    self.current_pos_data = Some(current_pos_data);
                    // SOT x anything
                    let right_prop = self.get_break_property();
                    if self.is_break_from_table(self.sot_property, right_prop) {
                        return Some(current_pos_data.0);
                    }
                }

                loop {
                    let left_prop = self.get_break_property();
                    let left_codepoint = self.current_pos_data.unwrap().1;
                    self.current_pos_data = self.iter.next();

                    if self.current_pos_data.is_none() {
                        return Some(self.len);
                    }
                    let right_prop = self.get_break_property();

                    // Some segmenter rules doesn't have language-specific rules, we have to use LSTM (or dictionary) segmenter.
                    // If property is marked as SA, use it
                    if right_prop == self.complex_property {
                        if left_prop != self.complex_property {
                            // break before SA
                            return Some(self.current_pos_data.unwrap().0);
                        }
                        let break_offset = self.handle_complex_language(left_codepoint);
                        if break_offset.is_some() {
                            return break_offset;
                        }
                    }

                    // If break_state is equals or grater than 0, it is alias of property.
                    let mut break_state = self.get_break_state_from_table(left_prop, right_prop);

                    if break_state >= 0 as i8 {
                        // This isn't simple rule set. We need marker to restore iterator to previous position.
                        let mut previous_iter = self.iter.clone();
                        let mut previous_pos_data = self.current_pos_data;

                        loop {
                            self.current_pos_data = self.iter.next();
                            if self.current_pos_data.is_none() {
                                // Reached EOF. But we are analyzing multiple characters now, so next break may be previous point.
                                if self.get_break_state_from_table(
                                    break_state as u8,
                                    self.eot_property as u8,
                                ) == NOT_MATCH_RULE
                                {
                                    self.iter = previous_iter;
                                    self.current_pos_data = previous_pos_data;
                                    return Some(previous_pos_data.unwrap().0);
                                }
                                // EOF
                                return Some(self.len);
                            }

                            let previous_break_state = break_state;
                            let prop = self.get_break_property();
                            break_state = self.get_break_state_from_table(break_state as u8, prop);
                            if break_state < 0 {
                                break;
                            }
                            if previous_break_state >= 0
                                && previous_break_state <= self.last_codepoint_property
                            {
                                // Move marker
                                previous_iter = self.iter.clone();
                                previous_pos_data = self.current_pos_data;
                            }
                            if (break_state & INTERMEDIATE_MATCH_RULE) != 0 {
                                break_state -= INTERMEDIATE_MATCH_RULE;
                                previous_iter = self.iter.clone();
                                previous_pos_data = self.current_pos_data;
                            }
                        }
                        if break_state == KEEP_RULE {
                            continue;
                        }
                        if break_state == NOT_MATCH_RULE {
                            self.iter = previous_iter;
                            self.current_pos_data = previous_pos_data;
                            return Some(previous_pos_data.unwrap().0);
                        }
                        return Some(self.current_pos_data.unwrap().0);
                    }

                    if self.is_break_from_table(left_prop, right_prop) {
                        return Some(self.current_pos_data.unwrap().0);
                    }
                }
            }
        }

        impl<'a> $name<'a> {
            fn get_break_state_from_table(&self, left: u8, right: u8) -> i8 {
                self.break_state_table
                    [(left as usize) * self.rule_property_count + (right as usize)]
            }

            fn is_break_from_table(&self, left: u8, right: u8) -> bool {
                let rule = self.get_break_state_from_table(left, right);
                if rule == KEEP_RULE {
                    return false;
                }
                if rule >= 0 {
                    // need additional next characters to get break rule.
                    return false;
                }
                true
            }
        }
    };
}
