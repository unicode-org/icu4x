use core::iter::Iterator;
use core::num::NonZero;

use calendrical_calculations::rata_die::RataDie;

use crate::{AsCalendar, Date};

#[derive(Debug, Clone)]
pub struct IsoConvRangeFromIter<A: AsCalendar + Clone> {
    cur: RataDie,
    step: NonZero<i64>,
    calendar: A,
    is_first: bool,
    is_ended: bool,
}

impl<A: AsCalendar + Clone> IsoConvRangeFromIter<A> {
    /// # Panic
    /// If step is `0`.
    pub fn new(start: Date<A>, step: i32) -> Self {
        #[allow(clippy::panic)]
        if step == 0 {
            panic!("step was 0")
        }
        // Safety: step is not 0
        let step = unsafe { NonZero::new_unchecked(step as i64) };

        let cur_fixed = start.to_fixed();
        Self {
            cur: cur_fixed,
            step,
            calendar: start.calendar,
            is_first: true,
            is_ended: false,
        }
    }

    #[inline]
    fn cur_date(&self) -> Date<A> {
        let iso = crate::Iso::iso_from_fixed(self.cur);
        Date::new_from_iso(iso, self.calendar.clone())
    }

    /// Creates an iterator starting at the same point, but stepping by the given amount at each iteration.
    ///
    /// # Panic
    /// The method will panic if the given step is `0`.
    ///
    /// # Note
    ///  Creating of `core::iter::StepBy` is private, so it can't be impl in the Iterator trait
    pub fn step_by(mut self, step: usize) -> Self
    where
        Self: Sized,
    {
        const MAX_DELTA: i64 = RataDie::MAX.to_i64_date() - RataDie::MIN.to_i64_date();

        // The doc say: "The method will panic if the given step is `0`."
        assert!(step != 0);

        let (step, overflow) = self.step.get().overflowing_mul(step as i64);
        if overflow || MAX_DELTA < step.abs() {
            // there no diff which value step is, it's more than MAX_DELTA and therefore it's ended from any cur point
            self.is_ended = true;
        } else {
            // Safety: non-zero * non-zero IS non-zero
            self.step = unsafe { NonZero::new_unchecked(step) };
        }

        self
    }
}

impl<A: AsCalendar + Clone> Iterator for IsoConvRangeFromIter<A> {
    type Item = Date<A>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_ended {
            return None;
        }
        if self.is_first {
            self.is_first = false;
            return Some(self.cur_date());
        }

        let rata_die = self.cur.to_i64_date();
        let new_rata_die = rata_die + self.step.get();

        if new_rata_die < RataDie::MIN.to_i64_date() || RataDie::MAX.to_i64_date() < new_rata_die {
            self.is_ended = true;
            None
        } else {
            self.cur = RataDie::new(new_rata_die);
            Some(self.cur_date())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IsoConvRangeFromIter;
    use crate::{chinese::Chinese, Iso};
    use crate::{Calendar, Date};
    use calendrical_calculations::rata_die::RataDie;

    #[test]
    fn test_ranges() {
        let cal = Chinese::new();
        let date = Date::try_new_chinese_with_calendar(4660, 6, 11, cal.clone()).unwrap();
        let rata_die_init = cal.date_to_iso(&date.inner).to_fixed();

        macro_rules! test_range_iter {
            ($iter_amount: literal for [$($steps: literal),+]) => {
                $(
                    let mut range_iter = IsoConvRangeFromIter::new(date.clone(), $steps);
                    for i in 0..$iter_amount {
                        #[allow(clippy::neg_multiply)]
                        let must = Iso::iso_from_fixed(RataDie::new(rata_die_init.to_i64_date() + i * $steps));
                        let must = cal.date_from_iso(must);
                        let real = range_iter.next().unwrap().inner;
                        assert_eq!(real, must);
                        assert_eq!(real.0.0.year_info, must.0.0.year_info);
                    }
                )+
            };
        }

        test_range_iter!(200 for [148, -975]);
        test_range_iter!(2_000 for [-7, 28]);
        test_range_iter!(10_000 for [1, -1, 3, -3]);
    }

    #[test]
    fn test_step_by() {
        let date = Date::try_new_iso(2024, 11, 28).unwrap();

        macro_rules! test_range_iter {
            ($iter_amount: literal for $step: literal BY $step_by: literal) => {
                let mut range_iter_a =
                    IsoConvRangeFromIter::new(date.clone(), $step).step_by($step_by);
                #[allow(clippy::neg_multiply)]
                let mut range_iter_b = IsoConvRangeFromIter::new(date.clone(), $step * $step_by);
                for _ in 0..$iter_amount {
                    let a = range_iter_a.next().unwrap();
                    let b = range_iter_b.next().unwrap();
                    assert_eq!(a, b);
                }
            };
        }

        test_range_iter!(1_000 for 7 BY 13);
        test_range_iter!(1_000 for -9 BY 11);
    }
}
