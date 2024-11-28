use core::iter::Iterator;
use core::num::NonZero;

use calendrical_calculations::rata_die::RataDie;

use crate::{AsCalendar, Date};

#[derive(Debug, Clone)]
pub struct DateRangeFromIter<A: AsCalendar + Clone> {
    cur: RataDie,
    step: NonZero<i64>,
    calendar: A,
    is_first: bool,
    is_ended: bool,
}

impl<A: AsCalendar + Clone> DateRangeFromIter<A> {
    /// Creates new DateRangeFromIter
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

    /// Skips next `n` elements of iteration.
    pub fn skip(mut self, n: usize) -> Self {
        match n {
            0 => self,
            1 => {
                self.next();
                self
            }
            _ => {
                let step_save = self.step;
                let steps = n - (self.is_first as usize);
                let mut new_self = self.step_by(steps);
                new_self.is_first = false;
                new_self.next();
                new_self.step = step_save;
                new_self
            }
        }
    }
}

impl<A: AsCalendar + Clone> Iterator for DateRangeFromIter<A> {
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

impl<A: AsCalendar + Clone> From<core::ops::RangeFrom<Date<A>>> for DateRangeFromIter<A> {
    fn from(value: core::ops::RangeFrom<Date<A>>) -> Self {
        Self::new(value.start, 1)
    }
}

// ────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
enum IsFirst {
    NoOne,
    OnlyFrom,
    OnlyTo,
    Both,
}
impl core::fmt::Debug for IsFirst {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoOne => write!(f, "(F, F)"),
            Self::OnlyFrom => write!(f, "(T, F)"),
            Self::OnlyTo => write!(f, "(F, T)"),
            Self::Both => write!(f, "(T, T)"),
        }
    }
}
impl IsFirst {
    fn new() -> Self {
        Self::Both
    }

    fn is_first(self, is_backward: bool) -> bool {
        match self {
            Self::NoOne => false,
            Self::OnlyFrom => !is_backward,
            Self::OnlyTo => is_backward,
            Self::Both => true,
        }
    }

    fn upd(&mut self, is_backward: bool) {
        *self = match self {
            Self::NoOne => Self::NoOne,

            Self::OnlyFrom if !is_backward => Self::NoOne,
            Self::OnlyFrom /* if is_backward */ => Self::OnlyFrom,

            Self::OnlyTo if is_backward => Self::NoOne,
            Self::OnlyTo /* if !is_backward */ => Self::OnlyTo,

            Self::Both if is_backward => Self::OnlyFrom,
            Self::Both /* if !is_backward */ => Self::OnlyTo,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DateRangeIter<A: AsCalendar + Clone> {
    from: RataDie,
    to: RataDie,
    step: NonZero<i64>,
    calendar: A,
    is_first: IsFirst,
    is_ended: bool,
}
impl<A: AsCalendar + Clone + PartialEq> DateRangeIter<A> {
    fn check_calendar(start: &Date<A>, end: &Date<A>) {
        #[allow(clippy::panic)]
        if start.calendar != end.calendar {
            panic!("Start and end dates have different calendars!")
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

    /// Creates new Exclusive DateRangeIter
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
            is_first: IsFirst::new(),
            is_ended,
        }
    }

    #[inline]
    fn cur_date(&self) -> Date<A> {
        let date = if self.is_backward() {
            self.to
        } else {
            self.from
        };
        let iso = crate::Iso::iso_from_fixed(date);
        Date::new_from_iso(iso, self.calendar.clone())
    }

    #[inline(always)]
    fn is_backward(&self) -> bool {
        self.step.get() < 0
    }

    fn step_reverse(&mut self) {
        // Because of `Self::round_bound` we can just change the sign of step

        let rev_step = -self.step.get();
        // Safety: `- non-zero` IS `non-zero``
        self.step = unsafe { NonZero::new_unchecked(rev_step) }
    }

    fn is_first(&self, is_backward: bool) -> bool {
        self.is_first.is_first(is_backward)
    } 

    fn end_iter(&mut self) -> Option<<Self as Iterator>::Item> {
        self.is_ended = true;
        None
    } 
}

impl<A: AsCalendar + Clone> DateRangeIter<A> {
    /// Reverses direction of iteration.
    pub fn rev(mut self) -> Self {
        self.step_reverse();
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
        if self.next().is_some() {
            self.is_ended = false;
            self.is_first = IsFirst::new();

            let delta = self.to.to_i64_date() - self.from.to_i64_date();
            let (step, overflow) = self.step.get().overflowing_mul(step as i64);

            if overflow || delta < step.abs() {
                let sign = if is_backward { -1 } else { 1 };
                // Safety: `(|x| + 1) * sign` IS non-zero
                self.step = unsafe { NonZero::new_unchecked((delta.abs() + 1) * sign) };
            } else {
                let (from, to) = Self::round_bound(self.from, self.to, step);
                // Safety: `non-zero * non-zero` IS non-zero
                self.step = unsafe { NonZero::new_unchecked(step) };
                self.from = from;
                self.to = to;
            }
        }
        // else : `self.is_ended` setted to `true`

        self
    }
}

impl<A: AsCalendar + Clone> Iterator for DateRangeIter<A> {
    type Item = Date<A>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_ended {
            return None;
        }

        let is_backward = self.is_backward();
        if self.is_first(is_backward) {
            self.is_first.upd(is_backward);
            println!("X: from:{}   to:{}", self.from.to_i64_date(), self.to.to_i64_date());
            return Some(self.cur_date());
        }

        let step = self.step.get();
        if is_backward {
            let rata_die = self.to.to_i64_date();
            let new_rata_die = rata_die + step;

            let inclusive = self.is_first(!is_backward);
            let is_eq = new_rata_die == self.from.to_i64_date(); 

            println!("B: {inclusive} {is_eq}   from:{}   to:{new_rata_die}", self.from.to_i64_date());

            if new_rata_die < self.from.to_i64_date() {
                self.end_iter()
            } else if !inclusive && is_eq {
                self.end_iter()
            } else {
                // Because `new_rata_die` is MOE than `self.from` => it's in `RataDie` bounds
                self.to = RataDie::new(new_rata_die);
                self.is_ended = is_eq;
                Some(self.cur_date())
            }
        } else {
            let rata_die = self.from.to_i64_date();
            let new_rata_die = rata_die + step;

            let inclusive = self.is_first(!is_backward);
            let is_eq = self.to.to_i64_date() == new_rata_die; 

            println!("F: {inclusive} {is_eq}   from:{new_rata_die}   to:{}", self.to.to_i64_date());

            if self.to.to_i64_date() < new_rata_die {
                self.end_iter()
            } else if !inclusive && is_eq {
                self.end_iter()
            } else {
                // Because `new_rata_die` is LOE than `self.to` => it's in `RataDie` bounds
                self.from = RataDie::new(new_rata_die);
                self.is_ended = is_eq;
                Some(self.cur_date())
            }
        }
    }
}

impl<A: AsCalendar + Clone> DoubleEndedIterator for DateRangeIter<A> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.step_reverse();
        let ret = self.next();
        self.step_reverse();
        ret
    }
}

impl<A: AsCalendar + Clone + PartialEq> From<core::ops::Range<Date<A>>> for DateRangeIter<A> {
    fn from(value: core::ops::Range<Date<A>>) -> Self {
        Self::new_exclusive(value.start, value.end, 1)
    }
}

impl<A: AsCalendar + Clone + PartialEq> From<core::ops::RangeInclusive<Date<A>>> for DateRangeIter<A> {
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

        macro_rules! test_range_iter {
            ($iter_amount: literal for [$($steps: literal),+]) => {
                $(
                    let mut range_iter = DateRangeFromIter::new(date.clone(), $steps);
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
                    DateRangeFromIter::new(date.clone(), $step).step_by($step_by);
                #[allow(clippy::neg_multiply)]
                let mut range_iter_b = DateRangeFromIter::new(date.clone(), $step * $step_by);
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

    // TODO:MAKE OK
    #[test]
    fn test_todo_del() {
        let date = Date::try_new_iso(2025, 3, 1).unwrap();
        // 1, 4, 7, 10, 13, 16, 19, ...
        let mut range = DateRangeFromIter::new(date.clone(), 3);
        let range = range.skip(2).skip(3); // TODO: out of bound incorrect impl
        for (i, date) in (0..5).zip(range) {
            println!("{date:?}");
        }
    }
    
    // TODO:MAKE OK
    #[test]
    fn test_todo_del_2() {
        let date_from = Date::try_new_iso(2025, 3, 1).unwrap();
        let date_to = Date::try_new_iso(2025, 3, 12).unwrap();
        // 1, 4, 7, 10
        let mut range = DateRangeIter::new_inclusive(date_from.clone(), date_to.clone(), 3);
        // for (i, date) in (0..10).zip(range) {
        //     println!("{date:?}");
        // }
        for _ in 0..1 {
            println!("backward: {:?}", range.next_back());
        }
        for _ in 0..4 {
            println!("forward: {:?}", range.next());
        }
        for _ in 0..3 {
            println!("backward: {:?}", range.next_back());
        }
        for _ in 0..2 {
            println!("forward: {:?}", range.next());
        }
    }
       
    // TODO:MAKE OK
    #[test]
    fn test_todo_del_3() {
        let date_from = Date::try_new_iso(2025, 3, 1).unwrap();
        let date_to = Date::try_new_iso(2025, 3, 11).unwrap();
        // 1, 4, 7, 10
        let mut range = DateRangeIter::new_inclusive(date_from.clone(), date_to.clone(), 3);
        
        // // 1, 10
        // let mut range = range.step_by(3); 
        
        println!("forward[A]: {:?}", range.next());
        // 4
        let mut range = range.step_by(3); 
        
        // 1
        // let mut range = range.step_by(4); 

        // for (i, date) in (0..10).zip(range) {
        //     println!("{date:?}");
        // }
        for _ in 0..1 {
            println!("forward: {:?}", range.next());
        }
        for _ in 0..2 {
            println!("backward: {:?}", range.next_back());
        }
        for _ in 0..2 {
            println!("forward: {:?}", range.next());
        }
        for _ in 0..2 {
            println!("backward: {:?}", range.next_back());
        }
    }
}
