// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::iter::Iterator;
use core::num::NonZero;

use crate::{AsCalendar, Date};
use calendrical_calculations::rata_die::RataDie;

/// Try to create a [`Date`] from given [`RataDie`] as i64.
#[inline]
fn date<A: AsCalendar + Clone>(rata_die_i64: i64, calendar: &A) -> Option<Date<A>> {
    let date = RataDie::new(rata_die_i64);
    match crate::Iso::iso_try_from_fixed(date) {
        Ok(iso) => Some(Date::new_from_iso(iso, calendar.clone())),
        _ => None,
    }
}

/// Iterator that contains all [`Date`] starting from the specified in [`Self::new`] one
/// and going through the specified step.
#[derive(Debug, Clone)]
pub struct DateRangeFromIter<A: AsCalendar + Clone> {
    /// [`RataDie`] of a [`Date`] that will be returned (if iterations not ended) on next [`Iterator::next`] call.
    cur: RataDie,
    step: NonZero<i64>,
    /// Calendar of the Dates.
    calendar: A,
    is_ended: bool,
}

impl<A: AsCalendar + Clone> DateRangeFromIter<A> {
    /// Creates new [`DateRangeFromIter`].
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
            is_ended: false,
        }
    }

    /// Creates an iterator starting at the same point, but stepping by the given amount at each iteration.\
    /// It's faster than `Iterator::step_by`, because we not iterate each skipped step actually.
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
        let is_backward = self.step.get() < 0;

        let (step, overflow) = self.step.get().overflowing_mul(step as i64);
        if overflow || MAX_DELTA < step.abs() {
            // there no diff which value step is, it's more than MAX_DELTA and therefore
            // next iteration will be last from any cur point
            let sign = if is_backward { -1 } else { 1 };
            // Safety:
            // 1. `(|x| + 1) * sign` IS non-zero
            // 2. delta.abs() << i64::MAX => delta + 1 is'not overflow
            self.step = unsafe { NonZero::new_unchecked((MAX_DELTA + 1) * sign) };
        } else {
            // Safety: non-zero * non-zero IS non-zero
            self.step = unsafe { NonZero::new_unchecked(step) };
        }

        self
    }

    /// Skips next `n` elements of iteration.
    pub fn skip(self, n: usize) -> Self {
        if n == 0 {
            self
        } else {
            let step_save = self.step;
            let mut new_self = self.step_by(n);
            // skip next n Dates:
            new_self.next();
            new_self.step = step_save;
            new_self
        }
    }
}

impl<A: AsCalendar + Clone> Iterator for DateRangeFromIter<A> {
    type Item = Date<A>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_ended {
            return None;
        }

        let rata_die = self.cur.to_i64_date();
        let next_rd = rata_die + self.step.get();

        if next_rd < RataDie::MIN.to_i64_date() || RataDie::MAX.to_i64_date() < next_rd {
            self.is_ended = true;
        } else {
            self.cur = RataDie::new(next_rd);
        }

        let ret = date(rata_die, &self.calendar);
        if ret.is_none() {
            self.is_ended = true;
        }
        ret
    }
}

impl<A: AsCalendar + Clone> From<core::ops::RangeFrom<Date<A>>> for DateRangeFromIter<A> {
    fn from(value: core::ops::RangeFrom<Date<A>>) -> Self {
        Self::new(value.start, 1)
    }
}

// ────────────────────────────────────────────────────────────────────────────

/// Iterator that contains all [`Date`] within a specified range.
#[derive(Debug, Clone)]
pub struct DateRangeIter<A: AsCalendar + Clone> {
    /// [`RataDie`] of a [`Date`] that will be returned (if iterations not ended)
    ///  on next [`Iterator::next`] call.
    from: RataDie,
    /// [`RataDie`] of a [`Date`] that will be returned (if iterations not ended)
    /// on next [`DoubleEndedIterator::next_back`] call.
    to: RataDie,
    step: NonZero<i64>,
    /// Calendar of the Dates.
    calendar: A,
    is_ended: bool,
}
impl<A: AsCalendar + Clone + PartialEq> DateRangeIter<A> {
    fn check_calendar(start: &Date<A>, end: &Date<A>) {
        #[allow(clippy::panic)]
        if start.calendar != end.calendar {
            panic!("Start({start:?}) and end({end:?}) dates have different calendars!")
        }
    }

    /// Creates new inclusive DateRangeIter
    /// # Panic
    /// * If step is `0`
    /// * If `start` and `end` have different calendars
    pub fn new_inclusive(start: Date<A>, end: Date<A>, step: i32) -> Self {
        Self::check_calendar(&start, &end);
        let to = end.to_fixed();
        Self::new(start, to, step)
    }

    /// Creates new exclusive DateRangeIter
    /// # Panic
    /// * If step is `0`
    /// * If `start` and `end` have different calendars
    pub fn new_exclusive(start: Date<A>, end: Date<A>, step: i32) -> Self {
        Self::check_calendar(&start, &end);

        let to = end.to_fixed();
        let (to, is_ended) = if to == RataDie::MIN {
            // In this case it's always empty range
            (to, true)
        } else {
            (RataDie::new(to.to_i64_date() - 1), false)
        };

        let mut ret = Self::new(start, to, step);
        ret.is_ended = is_ended;
        ret
    }
}

impl<A: AsCalendar + Clone> DateRangeIter<A> {
    /// Rounding the bounds to make it step-even.
    ///
    /// For example, if step is 3 and current bounds contains next Dates:\
    /// `1.12.2024`, `2.12.2024`, `3.12.2024`, `4.12.2024`, `5.12.2024`, i.e.
    /// `from` -> `1.12.2024`; `to` -> `5.12.2024`\
    /// After the rounding it will be:\
    /// `1.12.2024`, `2.12.2024`, `3.12.2024`, `4.12.2024`, i.e.
    /// `from` -> `1.12.2024`; `to` -> `4.12.2024`.
    ///
    /// In other words, make `from` and `to` such that you can get one from other by stepping.
    fn round_bound(from: RataDie, to: RataDie, step: i64) -> (RataDie, RataDie) {
        let from = from.to_i64_date();
        let to = to.to_i64_date();
        let delta = to - from;

        if step < 0 {
            let step = step.abs();
            let new_delta = (delta / step) * step;
            let from = to - new_delta;
            (RataDie::new(from), RataDie::new(to))
        } else {
            let new_delta = (delta / step) * step;
            let to = from + new_delta;
            (RataDie::new(from), RataDie::new(to))
        }
    }

    fn new(start: Date<A>, to: RataDie, step: i32) -> Self {
        #[allow(clippy::panic)]
        if step == 0 {
            panic!("step was 0")
        }

        let from = start.to_fixed();
        let is_ended = from > to;

        let step = step as i64;
        let (from, to) = Self::round_bound(from, to, step);

        // Safety: step is not 0
        let step = unsafe { NonZero::new_unchecked(step) };

        Self {
            from,
            to,
            step,
            calendar: start.calendar,
            is_ended,
        }
    }

    /// Is direction of stepping backward?
    #[inline(always)]
    fn is_backward(&self) -> bool {
        self.step.get() < 0
    }

    /// Reverse direction of the stepping.
    fn rev_dir(&mut self) {
        // Because of `Self::round_bound` we can just change the sign of step

        let rev_step = -self.step.get();
        // Safety: `- non-zero` IS `non-zero``
        self.step = unsafe { NonZero::new_unchecked(rev_step) }
    }

    /// Calculate `is_ended` value
    fn calc_is_ended(&mut self) -> bool {
        if !self.is_ended {
            self.is_ended = self.from > self.to;
        }
        self.is_ended
    }
}

impl<A: AsCalendar + Clone> DateRangeIter<A> {
    /// Reverses direction of iteration.
    pub fn rev(mut self) -> Self {
        self.rev_dir();
        self
    }

    /// Creates an iterator starting at the same point, but stepping by the given amount at each iteration.\
    /// It's faster than `Iterator::step_by`, because we not iterate each skipped step actually.
    ///
    /// # Panic
    /// The method will panic if the given step is `0`.
    ///
    /// # Note
    ///  Creating of `core::iter::StepBy` is private, so it can't be impl in the Iterator trait
    pub fn step_by(mut self, step: usize) -> Self {
        // The doc say: "The method will panic if the given step is `0`."
        assert!(step != 0);

        let is_backward = self.is_backward();

        let delta = self.to.to_i64_date() - self.from.to_i64_date();
        let (step, overflow) = self.step.get().overflowing_mul(step as i64);

        if overflow || delta.abs() < step.abs() {
            let sign = if is_backward { -1 } else { 1 };
            // Safety:
            // 1. `(|x| + 1) * sign` IS non-zero
            // 2. delta.abs() << i64::MAX => delta + 1 is'not overflow
            self.step = unsafe { NonZero::new_unchecked((delta.abs() + 1) * sign) };
        } else {
            // Safety: `non-zero * non-zero` IS non-zero
            self.step = unsafe { NonZero::new_unchecked(step) };
        }
        // We should to round the bounds because it can be non-even after changing the step
        let (from, to) = Self::round_bound(self.from, self.to, self.step.get());
        self.from = from;
        self.to = to;

        self
    }

    /// Skips next `n` elements of iteration.
    pub fn skip(self, n: usize) -> Self {
        if n == 0 {
            self
        } else {
            let step_save = self.step;
            let mut new_self = self.step_by(n);
            // skip n dates:
            new_self.next();
            // returns to prev step:
            new_self.step = step_save;
            new_self
        }
    }
}

impl<A: AsCalendar + Clone> Iterator for DateRangeIter<A> {
    type Item = Date<A>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.calc_is_ended() {
            return None;
        }

        let rata_die;
        let step = self.step.get();
        if self.is_backward() {
            rata_die = self.to.to_i64_date();
            let next_rata_die = rata_die + step;

            if next_rata_die < self.from.to_i64_date() {
                self.is_ended = true;
            } else {
                // Because `new_rata_die` is MOE than `self.from` => it's in `RataDie` bounds
                self.to = RataDie::new(next_rata_die);
            }
        } else {
            rata_die = self.from.to_i64_date();
            let next_rata_die = rata_die + step;

            if self.to.to_i64_date() < next_rata_die {
                self.is_ended = true;
            } else {
                // Because `new_rata_die` is LOE than `self.to` => it's in `RataDie` bounds
                self.from = RataDie::new(next_rata_die);
            }
        }

        let ret = date(rata_die, &self.calendar);
        if ret.is_none() {
            self.is_ended = true;
        }
        ret
    }
}

impl<A: AsCalendar + Clone> DoubleEndedIterator for DateRangeIter<A> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.rev_dir();
        let ret = self.next();
        self.rev_dir();
        ret
    }
}

impl<A: AsCalendar + Clone + PartialEq> From<core::ops::Range<Date<A>>> for DateRangeIter<A> {
    fn from(value: core::ops::Range<Date<A>>) -> Self {
        Self::new_exclusive(value.start, value.end, 1)
    }
}

impl<A: AsCalendar + Clone + PartialEq> From<core::ops::RangeInclusive<Date<A>>>
    for DateRangeIter<A>
{
    fn from(value: core::ops::RangeInclusive<Date<A>>) -> Self {
        let (start, end) = value.into_inner();
        Self::new_inclusive(start, end, 1)
    }
}

// ────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::{DateRangeFromIter, DateRangeIter};
    use crate::{chinese::Chinese, Iso};
    use crate::{Calendar, Date};
    use calendrical_calculations::rata_die::RataDie;

    #[test]
    fn test_ranges() {
        let cal = Chinese::new();
        let date = Date::try_new_chinese_with_calendar(4660, 6, 11, cal.clone()).unwrap();
        let rata_die_init = cal.date_to_iso(&date.inner).to_fixed();

        let test_range_iter = |iters: i64, steps: &[i64]| {
            for step in steps.iter().cloned() {
                let mut range_iter = DateRangeFromIter::new(date.clone(), step as i32);
                for i in 0..iters {
                    let date = rata_die_init.to_i64_date() + i * step;
                    let must = Iso::iso_from_fixed(RataDie::new(date));
                    let must = cal.date_from_iso(must);
                    let real = range_iter.next().unwrap().inner;
                    assert_eq!(real, must);
                    assert_eq!(real.0 .0.year_info, must.0 .0.year_info);
                }
            }
        };

        test_range_iter(200, &[148, -975]);
        test_range_iter(2_000, &[-7, 28]);
        test_range_iter(10_000, &[1, -1, 3, -3]);
    }

    #[test]
    fn test_step_by() {
        let date = Date::try_new_iso(2024, 11, 28).unwrap();

        let test_range_iter = |step: i32, step_by: usize, iters: usize| {
            let mut range_iter_a = DateRangeFromIter::new(date, step).step_by(step_by);

            let step = step * (step_by as i32);
            let mut range_iter_b = DateRangeFromIter::new(date, step);

            for _ in 0..iters {
                let a = range_iter_a.next().unwrap();
                let b = range_iter_b.next().unwrap();
                assert_eq!(a, b);
            }
        };

        test_range_iter(7, 13, 1_000);
        test_range_iter(-9, 11, 1_000);
    }

    #[test]
    fn test_range_from_iter_mutations() {
        let date_init = Date::try_new_iso(2025, 3, 1).unwrap();

        let assert_one = |date: Date<Iso>, day_must: u8| {
            let date = date.inner.0;
            assert_eq!(date.year, date_init.inner.0.year);
            assert_eq!(date.month, date_init.inner.0.month);
            assert_eq!(date.day, day_must);
        };

        // Example:
        // 1, 4, 7, 10, 13, 16, 19, ...
        let mut range = DateRangeFromIter::new(date_init, 3).skip(2).skip(3);
        // x, x, X,  X,  X, 16, 19, ...
        let range_emul = (1..=31).step_by(3).skip(2).skip(3);
        for day_must in range_emul {
            let date = range.next().unwrap();
            assert_one(date, day_must);
        }

        /// Generate something like upper `Example`
        ///
        /// macro because `.step_by` / `skip` return different struct
        macro_rules! test_iter_mutation {
            (CMD: $prev:expr; []) => { $prev };
            (CMD: $prev:expr; [STEP $n:literal / $($cmd:tt)*]) => {
                test_iter_mutation!(CMD: $prev.step_by($n); [$($cmd)*])
            };
            (CMD: $prev:expr; [SKIP $n:literal / $($cmd:tt)*]) => {
                test_iter_mutation!(CMD: $prev.skip($n); [$($cmd)*])
            };
            ($step_init:literal [$($cmd:tt)*]) => {{
                let mut range = test_iter_mutation!(CMD: DateRangeFromIter::new(date_init, $step_init); [$($cmd)*]);
                let range_emul = test_iter_mutation!(CMD: (1..=31).step_by($step_init); [$($cmd)*]);
                for day_must in range_emul {
                    let date = range.next().unwrap();
                    assert_one(date, day_must);
                }
            }};
        }

        test_iter_mutation!(7 []);
        test_iter_mutation!(2 [STEP 3 /]);
        test_iter_mutation!(4 [SKIP 3 /]);
        test_iter_mutation!(3 [SKIP 2 / SKIP 1 /]);
        test_iter_mutation!(3 [STEP 2 / STEP 2 /]);
        test_iter_mutation!(5 [STEP 2 / SKIP 1 /]);
        test_iter_mutation!(4 [STEP 2 / SKIP 3 /]);
        test_iter_mutation!(1 [SKIP 3 / STEP 2 / SKIP 1 / STEP 4 /]);
        test_iter_mutation!(1 [SKIP 2 / SKIP 3 / STEP 2 / STEP 1 / STEP 3 / SKIP 1 /]);
    }

    #[test]
    fn test_range_iter_mutations() {
        const YEAR: i32 = 2025;
        const MONTH: u8 = 3;
        fn assert_one(real: Option<Date<Iso>>, emul: Option<u8>) {
            // println!("{:?} : {:?}", emul, real);
            if let Some(real) = real {
                let inner = real.inner.0;
                let day_must = emul.unwrap();
                assert_eq!(inner.day, day_must);
                assert_eq!(inner.month, MONTH);
                assert_eq!(inner.year, YEAR);
            } else {
                assert!(emul.is_none())
            }
        }
        fn assert_seq<A, B>(a: &mut A, b: &mut B, seq: &[u8], mut backward: bool)
        where
            A: Iterator<Item = Date<Iso>> + DoubleEndedIterator,
            B: Iterator<Item = u8> + DoubleEndedIterator,
        {
            // println!("SEQ: {seq:?}");
            for n in seq.iter().copied() {
                if backward {
                    (0..n).for_each(|_| assert_one(a.next_back(), b.next_back()));
                } else {
                    (0..n).for_each(|_| assert_one(a.next(), b.next()));
                }
                backward = !backward;
            }
        }

        // Example:
        let date_from = Date::try_new_iso(YEAR, MONTH, 1).unwrap();
        let date_to = Date::try_new_iso(YEAR, MONTH, 12).unwrap();
        let step = 3;
        // 1, 4, 7, 10
        let mut range = DateRangeIter::new_inclusive(date_from, date_to, step);
        let mut range_emul = (date_from.inner.0.day..=date_to.inner.0.day).step_by(step as usize);
        // 10, 7, 1, 4, None ..
        assert_seq(&mut range, &mut range_emul, &[2, 1, 1, 3, 3], true);

        // println!("---------------");

        macro_rules! test_iter_mutation {
            (CMD: $prev:expr; []) => { $prev };
            (CMD: $prev:expr; [STEP $n:literal / $($cmd:tt)*]) => {
                test_iter_mutation!(CMD: $prev.step_by($n); [$($cmd)*])
            };
            (CMD: $prev:expr; [SKIP $n:literal / $($cmd:tt)*]) => {
                test_iter_mutation!(CMD: $prev.skip($n); [$($cmd)*])
            };
            (CMD: $prev:expr; [REV / $($cmd:tt)*]) => {
                test_iter_mutation!(CMD: $prev.rev(); [$($cmd)*])
            };
            (
                $from:literal .. $to:literal : $step_init:literal
                [$($cmd:tt)*] SEQ $backward:literal $seq:expr;
                $([$($cmds:tt)*] SEQ $backwards:literal $seqs:expr;)*
            ) => {{
                // Generate code like example before the macro
                // (with possible additional step_by/skip/rev transformation)
                let date_from = Date::try_new_iso(YEAR, MONTH, $from).unwrap();
                let date_to = Date::try_new_iso(YEAR, MONTH, $to).unwrap();
                let step = $step_init;

                let mut range = test_iter_mutation!(CMD: DateRangeIter::new_exclusive(date_from, date_to, step); [$($cmd)*]);
                let mut range_emul = test_iter_mutation!(CMD: (date_from.inner.0.day..date_to.inner.0.day).step_by(step as usize); [$($cmd)*]);
                assert_seq(&mut range, &mut range_emul, &$seq, $backward);
                $(
                    // println!("--   --");

                    // now make `step_by`/`skip`/`rev` after some iterations happen
                    let mut range = test_iter_mutation!(CMD: range; [$($cmds)*]);
                    let mut range_emul = test_iter_mutation!(CMD: range_emul; [$($cmds)*]);
                    // and after new iterator mutation, make some more iterations:
                    assert_seq(&mut range, &mut range_emul, &$seqs, $backwards);
                )*
                // println!("---------------");
            }};
        }

        test_iter_mutation!(2..27 : 2 [SKIP 2 / STEP 2 / REV / SKIP 1 /] SEQ false [1, 2, 1, 2];);
        test_iter_mutation!(
            2..30 : 1
            [SKIP 1 / REV / SKIP 1 /] SEQ true [2, 3];
            [STEP 2 / SKIP 1 / REV / SKIP 1 /] SEQ false [2, 2];
            [REV / STEP 2 / REV /] SEQ false [2, 2, 2];
        );
        test_iter_mutation!(1..12 : 3 [STEP 3 /] SEQ false [1, 1];);
        test_iter_mutation!(1..12 : 3 [STEP 3 /] SEQ false [2, 1];);
        test_iter_mutation!(1..12 : 3 [STEP 3 /] SEQ false [3, 1];);
        test_iter_mutation!(1..12 : 3 [STEP 3 /] SEQ true [1, 1];);
        test_iter_mutation!(1..12 : 3 [STEP 3 /] SEQ true [2, 1];);
        test_iter_mutation!(1..12 : 3 [STEP 3 /] SEQ true [3, 1];);
        test_iter_mutation!(1..12 : 3 [SKIP 1 / STEP 3 /] SEQ false [1, 1];);
        test_iter_mutation!(1..12 : 3 [SKIP 1 / STEP 3 /] SEQ true [1, 1];);
        test_iter_mutation!(1..12 : 3 [SKIP 1 / STEP 3 / REV / REV / REV /] SEQ true [1, 1];);
        test_iter_mutation!(1..12 : 3 [] SEQ false [1]; [REV / STEP 3 /] SEQ false [1, 1, 2];);
        test_iter_mutation!(1..12 : 3 [] SEQ false [1]; [REV / STEP 3 /] SEQ false [0, 1, 2];);
    }

    #[test]
    fn test_range_from_bound_cases() {
        let delta = 100;
        let n = 4;
        let step = delta / n;
        assert!(delta % n == 0);

        let max_date = Date::try_new_iso(i32::MAX, 12, 31).unwrap();
        let max_date_i64 = max_date.to_fixed().to_i64_date();

        let min_date = Date::try_new_iso(i32::MIN, 1, 1).unwrap();
        let min_date_i64 = min_date.to_fixed().to_i64_date();

        let max_date_test = |shift: i64| {
            assert!(shift.abs() < step);
            let inint_date_i64 = max_date_i64 - delta + shift;
            let date = RataDie::new(inint_date_i64);
            let date = Iso::iso_from_fixed(date);
            let mut iter = DateRangeFromIter::from(date..).step_by(step as usize);

            let mut expected = vec![];
            let to = if shift > 0 { n - 1 } else { n };
            for i in 0..=to {
                let date = RataDie::new(inint_date_i64 + step * i);
                let date = Iso::iso_from_fixed(date);
                expected.push(Some(date));
            }
            expected.push(None);

            for expected_one in expected {
                let real = iter.next();
                assert_eq!(real, expected_one);
            }
        };

        let min_date_test = |shift: i64| {
            assert!(shift.abs() < step);
            let inint_date_i64 = min_date_i64 + delta + shift;
            let date = RataDie::new(inint_date_i64);
            let date = Iso::iso_from_fixed(date);
            // backward direction:
            let mut iter = DateRangeFromIter::new(date, -1).step_by(step as usize);

            let mut expected = vec![];
            let to = if shift < 0 { n - 1 } else { n };
            for i in 0..=to {
                let date = RataDie::new(inint_date_i64 - step * i);
                let date = Iso::iso_from_fixed(date);
                expected.push(Some(date));
            }
            expected.push(None);

            for expected_one in expected {
                let real = iter.next();
                assert_eq!(real, expected_one);
            }
        };

        // Exact max date test:
        max_date_test(0);
        max_date_test(-1);
        // Last date right after the max:
        max_date_test(1);
        // Shifted max date test:
        max_date_test(-step / 3);
        max_date_test(step / 3);

        // Exact min date test:
        min_date_test(0);
        min_date_test(-1);
        // Last date right after the min:
        min_date_test(1);
        // Shifted min date test:
        min_date_test(-step / 3);
        min_date_test(step / 3);

        // Too big step test:
        let random_date = Date::try_new_iso(2024, 11, 29).unwrap();
        let iter = DateRangeFromIter::from(random_date..);
        // One step iter (Step is more than RataDie::MAX - RataDie::MIN):
        let mut iter = iter
            .step_by((1 << 20) + 2567)
            .step_by((1 << 20) + 123)
            .step_by((1 << 18) + 7);
        assert_eq!(iter.next(), Some(random_date));
        assert_eq!(iter.next(), None);
        assert!(iter.step.get() > 0);

        // Too big backward step test:
        let random_date = Date::try_new_iso(2024, 11, 29).unwrap();
        let iter = DateRangeFromIter::new(random_date, -((1 << 20) + 256));
        // One step iter (Step is more than RataDie::MAX - RataDie::MIN):
        let mut iter = iter.step_by((1 << 20) + 123).step_by((1 << 18) + 7);
        assert_eq!(iter.next(), Some(random_date));
        assert_eq!(iter.next(), None);
        assert!(iter.step.get() < 0);
    }

    #[test]
    fn test_range_bound_cases() {
        let random_date = Date::try_new_iso(2024, 11, 29).unwrap();
        let min_date = Date::try_new_iso(i32::MIN, 1, 1).unwrap();
        let max_date = Date::try_new_iso(i32::MAX, 12, 31).unwrap();

        // Empty iters:
        for date in [min_date, random_date, max_date] {
            let mut empty_iter = DateRangeIter::from(date..date);
            assert_eq!(empty_iter.next(), None);

            let mut empty_iter = DateRangeIter::from(date..min_date);
            assert_eq!(empty_iter.next(), None);

            let mut empty_iter = DateRangeIter::from(max_date..min_date);
            assert_eq!(empty_iter.next(), None);

            let mut empty_iter = DateRangeIter::from(date..min_date).rev();
            assert_eq!(empty_iter.next(), None);

            let mut empty_iter = DateRangeIter::from(max_date..min_date).rev();
            assert_eq!(empty_iter.next(), None);
        }

        // Single iters:
        for date in [min_date, random_date, max_date] {
            let mut single_iter = DateRangeIter::from(date..=date);
            assert_eq!(single_iter.next(), Some(date));
            assert_eq!(single_iter.next(), None);

            let mut single_iter = DateRangeIter::from(date..=date).rev();
            assert_eq!(single_iter.next(), Some(date));
            assert_eq!(single_iter.next(), None);

            let mut single_iter = DateRangeIter::from(date..=date).step_by(1337);
            assert_eq!(single_iter.next(), Some(date));
            assert_eq!(single_iter.next(), None);
        }

        let l_date = Date::try_new_iso(-9999, 1, 1).unwrap();
        let r_date = Date::try_new_iso(9999, 3, 3).unwrap();

        let bstep_1 = (1 << 23) + 13;
        let bstep_2 = 1 << 20;
        let bstep_3 = (1 << 15) + 137;

        // Too big steps:
        let iter_init = DateRangeIter::from(l_date..=r_date);
        // steps more than RataDie max min delta:
        let mut iter = iter_init
            .clone()
            .step_by(bstep_1)
            .step_by(bstep_2)
            .step_by(bstep_3);
        assert_eq!(iter.next(), Some(l_date));
        assert_eq!(iter.next(), None);
        let mut iter = iter_init
            .clone()
            .step_by(bstep_1)
            .rev()
            .step_by(bstep_2)
            .step_by(bstep_3);
        assert_eq!(iter.next(), Some(l_date));
        assert_eq!(iter.next(), None);
        let iter_rev = iter_init.clone().rev();
        let mut iter = iter_rev
            .clone()
            .step_by(bstep_1)
            .step_by(bstep_2)
            .step_by(bstep_3);
        assert_eq!(iter.next(), Some(r_date));
        assert_eq!(iter.next(), None);
        let mut iter = iter_rev
            .clone()
            .step_by(bstep_1)
            .rev()
            .step_by(bstep_2)
            .step_by(bstep_3);
        assert_eq!(iter.next(), Some(r_date));
        assert_eq!(iter.next(), None);
        // steps more than dates day delta:
        let big_step = (9999 * 2 + 3) * 366;
        let step_1 = 1379;
        let step_2 = (big_step / 1379) + 1;
        let mut iter = iter_init.clone().step_by(step_1).step_by(step_2);
        assert_eq!(iter.next(), Some(l_date));
        assert_eq!(iter.next(), None);
        let mut iter = iter_init
            .clone()
            .step_by(step_1)
            .rev()
            .rev()
            .step_by(step_2)
            .rev();
        assert_eq!(iter.next(), Some(l_date));
        assert_eq!(iter.next(), None);
        let iter_rev = iter_init.rev();
        let mut iter = iter_rev.clone().step_by(step_1).step_by(step_2);
        assert_eq!(iter.next(), Some(r_date));
        assert_eq!(iter.next(), None);
        let mut iter = iter_rev.clone().step_by(step_1).step_by(step_2).rev();
        assert_eq!(iter.next(), Some(r_date));
        assert_eq!(iter.next(), None);

        // Excluded upper boundary:
        let pre_r_date = Date::try_new_iso(9999, 3, 2).unwrap();
        let iter_init = DateRangeIter::from(l_date..r_date);
        // steps more than dates day delta:
        let mut iter = iter_init.clone().step_by(step_1).step_by(step_2);
        assert_eq!(iter.next(), Some(l_date));
        assert_eq!(iter.next(), None);
        let mut iter = iter_init
            .clone()
            .step_by(step_1)
            .rev()
            .rev()
            .step_by(step_2)
            .rev();
        assert_eq!(iter.next(), Some(l_date));
        assert_eq!(iter.next(), None);
        let iter_rev = iter_init.rev();
        let mut iter = iter_rev.clone().step_by(step_1).step_by(step_2);
        assert_eq!(iter.next(), Some(pre_r_date));
        assert_eq!(iter.next(), None);
        let mut iter = iter_rev.clone().step_by(step_1).step_by(step_2).rev();
        assert_eq!(iter.next(), Some(pre_r_date));
        assert_eq!(iter.next(), None);
    }
}
