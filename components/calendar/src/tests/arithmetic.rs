// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::options::{DateAddOptions, DateDifferenceOptions, Overflow};
use crate::types::{DateDuration, DateDurationUnit};
use crate::{AsCalendar, Date};

super::test_all_cals!(
    #[ignore]
    fn test_arithmetic<C: Calendar>(cal: Ref<C>) {
        fn new_duration(years: i32, months: i32, weeks: i32, days: i32) -> DateDuration {
            let is_negative = years < 0 || months < 0 || weeks < 0 || days < 0;
            // Verify no mixed signs
            if is_negative {
                assert!(years <= 0 && months <= 0 && weeks <= 0 && days <= 0);
            } else {
                assert!(years >= 0 && months >= 0 && weeks >= 0 && days >= 0);
            }
            DateDuration {
                is_negative,
                years: years.unsigned_abs(),
                months: months.unsigned_abs(),
                weeks: weeks.unsigned_abs(),
                days: days.unsigned_abs() as u64,
            }
        }

        let mut durations = Vec::new();

        // Check +/- 65 days
        for i in -65..=65 {
            durations.push(new_duration(0, 0, 0, i));
        }

        // Check +/- 30 months
        for i in -30..=30 {
            durations.push(new_duration(0, i, 0, 0));
        }

        // Check +/- 30 months with +/- 1 day
        for i in -30i32..=30 {
            let days = i.signum();
            durations.push(new_duration(0, i, 0, days));
        }

        // Check +/- 10 years
        for i in -10..=10 {
            durations.push(new_duration(i, 0, 0, 0));
        }

        // Check +/- 10 years with +/- 1 month
        for i in -10i32..=10 {
            let months = i.signum();
            durations.push(new_duration(i, months, 0, 0));
        }

        // Check +/- 10 years with +/- 1 day
        for i in -10i32..=10 {
            let days = i.signum();
            durations.push(new_duration(i, 0, 0, days));
        }

        // Check +/- 10 years with +/- 1 month and +/- 1 day
        for i in -10i32..=10 {
            let s = i.signum();
            durations.push(new_duration(i, s, 0, s));
        }

        let start_date = Date::try_new_iso(2000, 1, 1).unwrap();
        let end_date = Date::try_new_iso(2004, 12, 31).unwrap();
        let start_rd = start_date.to_rata_die();
        let end_rd = end_date.to_rata_die();

        let add_options = DateAddOptions {
            overflow: Some(Overflow::Constrain),
        };

        for rd_offset in 0..=(end_rd - start_rd) {
            let date = Date::from_rata_die(start_rd + rd_offset, cal);

            for duration in &durations {
                let mut diff_options = DateDifferenceOptions::default();
                if duration.years != 0 {
                    diff_options.largest_unit = Some(DateDurationUnit::Years);
                } else if duration.months != 0 {
                    diff_options.largest_unit = Some(DateDurationUnit::Months);
                } else {
                    diff_options.largest_unit = Some(DateDurationUnit::Days);
                }

                let added_date = date
                    .try_added_with_options(*duration, add_options)
                    .unwrap_or_else(|_| {
                        panic!(
                            "Failed to add duration {:?} to date {:?} in calendar {:?}",
                            duration,
                            date,
                            cal.as_calendar().debug_name()
                        )
                    });

                let calculated_duration = date
                    .try_until_with_options(&added_date, diff_options)
                    .unwrap_or_else(|_| {
                        panic!(
                            "Failed to calculate difference between {:?} and {:?} in calendar {:?}",
                            date,
                            added_date,
                            cal.as_calendar().debug_name()
                        )
                    });

                // Round-trip check
                let added_back = date
                    .try_added_with_options(calculated_duration, add_options)
                    .unwrap();
                assert_eq!(
                    added_back, added_date,
                    "Round trip failed for {:?} + {:?} -> {:?}. Got duration {:?} which led to {:?}",
                    date, duration, added_date, calculated_duration, added_back
                );
            }
        }
    }
);
