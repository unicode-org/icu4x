// We also can compile parts of this file for bench purpose (to cmp algo perfs) => `#[cfg(test)]`

#[cfg(test)]
mod iso;
pub mod iso_old_algos;
pub mod iso_old_file;

#[cfg(test)]
mod julian;
pub mod julian_old_file;

#[cfg(test)]
pub mod helpful_consts {
    use core::ops::RangeInclusive;

    pub const N_YEAR_BOUND: i32 = 1234; // more than one cycle (400 years)
    pub const MIN_YEAR_BOUND_RANGE: RangeInclusive<i32> = i32::MIN..=(i32::MIN + N_YEAR_BOUND);
    pub const MAX_YEAR_BOUND_RANGE: RangeInclusive<i32> = (i32::MAX - N_YEAR_BOUND)..=i32::MAX;

    pub const MONTH_DAYS: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
}
