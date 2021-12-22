// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

macro_rules! impl_writeable_for_single_subtag {
    ($type:tt, $sample:literal) => {
        impl core::fmt::Display for $type {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl writeable::Writeable for $type {
            fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
                sink.write_str(self.as_str())
            }
            #[inline]
            fn write_len(&self) -> writeable::LengthHint {
                writeable::LengthHint::exact(self.0.len())
            }
        }

        #[test]
        fn test_writeable() {
            writeable::assert_writeable_eq!(&$type::from_str($sample).unwrap(), $sample);
        }
    };
}

macro_rules! impl_writeable_for_subtag_list {
    ($type:tt, $sample1:literal, $sample2:literal) => {
        impl core::fmt::Display for $type {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                writeable::Writeable::write_to(self, f)
            }
        }

        impl writeable::Writeable for $type {
            fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
                let mut initial = true;
                for subtag in self.iter() {
                    if initial {
                        initial = false;
                    } else {
                        sink.write_char('-')?;
                    }
                    writeable::Writeable::write_to(subtag, sink)?;
                }
                Ok(())
            }

            #[inline]
            fn write_len(&self) -> writeable::LengthHint {
                if self.0.is_none() {
                    writeable::LengthHint::exact(0)
                } else {
                    self.iter()
                        .map(writeable::Writeable::write_len)
                        .sum::<writeable::LengthHint>()
                        + (self.len() - 1)
                }
            }
        }

        #[test]
        fn test_writeable() {
            writeable::assert_writeable_eq!(&$type::default(), "");
            writeable::assert_writeable_eq!(
                &$type::from_vec_unchecked(alloc::vec![$sample1.parse().unwrap()]),
                $sample1,
            );
            writeable::assert_writeable_eq!(
                &$type::from_vec_unchecked(alloc::vec![
                    $sample1.parse().unwrap(),
                    $sample2.parse().unwrap()
                ]),
                core::concat!($sample1, "-", $sample2),
            );
        }
    };
}

macro_rules! impl_writeable_for_tinystr_list {
    ($type:tt, $if_empty:literal, $sample1:literal, $sample2:literal) => {
        impl core::fmt::Display for $type {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                writeable::Writeable::write_to(self, f)
            }
        }

        impl writeable::Writeable for $type {
            fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
                let mut initial = true;
                if self.0.len() == 0 {
                    sink.write_str($if_empty)?;
                }
                for subtag in self.0.iter() {
                    if initial {
                        initial = false;
                    } else {
                        sink.write_char('-')?;
                    }
                    sink.write_str(subtag.as_str())?;
                }
                Ok(())
            }

            #[inline]
            fn write_len(&self) -> writeable::LengthHint {
                if self.0.len() == 0 {
                    writeable::LengthHint::exact($if_empty.len())
                } else {
                    self.0
                        .iter()
                        .map(|s| s.len())
                        .sum::<writeable::LengthHint>()
                        + (self.0.len() - 1)
                }
            }
        }

        #[test]
        fn test_writeable() {
            writeable::assert_writeable_eq!(
                &$type::from_vec_unchecked(vec![$sample1.parse().unwrap()]),
                $sample1,
            );
            writeable::assert_writeable_eq!(
                &$type::from_vec_unchecked(vec![
                    $sample1.parse().unwrap(),
                    $sample2.parse().unwrap()
                ]),
                core::concat!($sample1, "-", $sample2),
            );
        }
    };
}

macro_rules! impl_writeable_for_key_value {
    ($type:tt, $key1:literal, $value1:literal, $key2:literal, $expected2:literal) => {
        impl core::fmt::Display for $type {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                writeable::Writeable::write_to(self, f)
            }
        }

        impl writeable::Writeable for $type {
            fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
                let mut initial = true;
                for (key, value) in self.iter() {
                    if initial {
                        initial = false;
                    } else {
                        sink.write_char('-')?;
                    }
                    writeable::Writeable::write_to(key, sink)?;
                    if !writeable::Writeable::write_len(value).is_zero() {
                        sink.write_char('-')?;
                        writeable::Writeable::write_to(value, sink)?;
                    }
                }
                Ok(())
            }

            #[inline]
            fn write_len(&self) -> writeable::LengthHint {
                if self.0.is_none() {
                    writeable::LengthHint::exact(0)
                } else {
                    self.iter()
                        .map(|(key, value)| {
                            writeable::Writeable::write_len(key)
                                + writeable::Writeable::write_len(value)
                                + if writeable::Writeable::write_len(value).is_zero() {
                                    0
                                } else {
                                    1
                                }
                        })
                        .sum::<writeable::LengthHint>()
                        + (self.len() - 1)
                }
            }
        }

        #[test]
        fn test_writeable() {
            writeable::assert_writeable_eq!(&$type::default(), "");
            writeable::assert_writeable_eq!(
                &$type::from_vec_unchecked(vec![(
                    $key1.parse().unwrap(),
                    $value1.parse().unwrap()
                )]),
                core::concat!($key1, "-", $value1),
            );
            writeable::assert_writeable_eq!(
                &$type::from_vec_unchecked(vec![
                    ($key1.parse().unwrap(), $value1.parse().unwrap()),
                    ($key2.parse().unwrap(), "true".parse().unwrap())
                ]),
                core::concat!($key1, "-", $value1, "-", $expected2),
            );
        }
    };
}
