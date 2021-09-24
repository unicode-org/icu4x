// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::types::Time;
use crate::{AsCalendar, Date};

/// A date+time for a given calendar
///
/// This can work with wrappers arount [`Calendar`] types,
/// e.g. `Rc<C>`, via the [`AsCalendar`] trait, much like
/// [`Date`]
pub struct DateTime<A: AsCalendar> {
    pub date: Date<A>,
    pub time: Time,
}

impl<A: AsCalendar> DateTime<A> {
    pub fn new(date: Date<A>, time: Time) -> Self {
        DateTime { date, time }
    }
}
