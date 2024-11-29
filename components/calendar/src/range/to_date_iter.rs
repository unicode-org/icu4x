use super::{DateRangeFromIter, DateRangeIter};
use crate::{AsCalendar, Date};
use core::ops::{Range, RangeFrom, RangeInclusive};

/// Convertion into [`Date`] iterator.
pub trait ToDateIter<A: AsCalendar> {
    /// Type of [`Date`] Iterator
    type Iter: Iterator<Item = Date<A>>;

    /// Convert rust range to date range iterator.
    fn to_date_iter(self) -> Self::Iter;
}

impl<A: AsCalendar + Clone> ToDateIter<A> for RangeFrom<Date<A>> {
    type Iter = DateRangeFromIter<A>;

    fn to_date_iter(self) -> Self::Iter {
        DateRangeFromIter::from(self)
    }
}

impl<A: AsCalendar + Clone + PartialEq> ToDateIter<A> for RangeInclusive<crate::Date<A>> {
    type Iter = DateRangeIter<A>;

    fn to_date_iter(self) -> Self::Iter {
        DateRangeIter::from(self)
    }
}

impl<A: AsCalendar + Clone + PartialEq> ToDateIter<A> for Range<Date<A>> {
    type Iter = DateRangeIter<A>;

    fn to_date_iter(self) -> Self::Iter {
        DateRangeIter::from(self)
    }
}
