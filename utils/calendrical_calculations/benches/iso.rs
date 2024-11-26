use core::hint::black_box as bb;

use calendrical_calculations::{helpers::I32CastError, rata_die::RataDie};
use criterion::{criterion_group, criterion_main, Criterion};

#[cfg(feature = "bench")]
use calendrical_calculations::bench_support::iso_old as old;
use calendrical_calculations::iso as new;

#[cfg(feature = "bench")]
use calendrical_calculations::bench_support::julian_old as j_old;
use calendrical_calculations::julian as j_new;

const MONTH_DAYS: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
type IsoFromFixedFn = fn(RataDie) -> Result<(i32, u8, u8), I32CastError>;

fn prep_gen_ymd_vec(y: i32, delta: i32) -> Vec<(i32, u8, u8)> {
    let mut ret = Vec::with_capacity(delta as usize * 2 * 366);
    for year in y - delta..=y + delta {
        for month in 1..=12u8 {
            for day in 1..=MONTH_DAYS[(month as usize) - 1] {
                ret.push((year, month, day));
            }
        }
    }
    bb(ret)
}

fn prep_gen_yd_vec(y: i32, delta: i32, from: u16, to: u16) -> Vec<(i32, u16)> {
    let mut ret = Vec::with_capacity(delta as usize * 2 * 365);
    for year in y - delta..=y + delta {
        for day in from..=to {
            ret.push((year, day))
        }
    }
    bb(ret)
}

fn prep_gen_rata_die_vec(n: i64) -> Vec<RataDie> {
    let mut ret = Vec::with_capacity(n as usize);
    (1..=n).for_each(|n| ret.push(RataDie::new(n)));
    bb(ret)
}

fn fixed_from(f: fn(i32, u8, u8) -> RataDie, ymd_vec: &Vec<(i32, u8, u8)>) -> i64 {
    // The problem here is that LTO(it is turned on for benches in the workspace)
    // is very good at cyclic optimization when we goes year by year:
    // ```
    //  for year in (y - delta)..=(y + delta) {
    //      for month in 1..=12u8 {
    //          for day in 1..=MONTH_DAYS[(month as usize) - 1] {
    //              ...
    // ```
    // and seems like the optimizer find out that in such cycles
    // result will be differ a little, and loop unroling stage
    // transforms it in something very optimized
    // (like incrimenting by one. It seems really so, because for new algo
    //       mentioned cycle was optimized to perf of empty cycle with `black_box`'es)
    //
    // And we want to test perf of the algos, not how good the optimizer work with
    // cases when the dates goes one by one.
    //
    // And there we get another problem: the algo(at least the new one) is +- as fast
    // as {cycle + `black_box`} overheads ://
    // So we should to call the function multiple times to at least reduce
    // the influence of perf measurements of the cycle ¯\_(ツ)_/¯

    let mut sum = 0;
    for (year, month, day) in ymd_vec {
        let (year, month, day) = (bb(*year), bb(*month), bb(*day));

        // If you comment next lines
        // Then you will get ~10% of NEW algo (lets say this ~10% is `X ms`)
        // So real perf ratio is:
        // `(OLD - X)/(NEW - X)`
        // And in my case this is +- about the asm instr len differ
        sum += f(year, month, day).to_i64_date();
        sum += f(year + 7, month, (day + 2) >> 1).to_i64_date();
        sum += f(year + 37, (month + 3) >> 1, (day + 3) >> 1).to_i64_date();
        sum += f(year + 137, (month + 7) >> 1, (day + 19) >> 1).to_i64_date();
    }

    bb(sum)
}

fn day_of_week(f: fn(i32, u8, u8) -> u8, ymd_vec: &Vec<(i32, u8, u8)>) -> i32 {
    let mut sum = 0;
    for (year, month, day) in ymd_vec {
        let (year, month, day) = (bb(*year), bb(*month), bb(*day));

        sum += f(year, month, day) as i32;
        sum += f(year + 31, month, (day + 7) >> 1) as i32;
        sum += f(year + 141, (month + 5) >> 1, day) as i32;
        sum += f(year + 243, (month + 2) >> 1, (day + 17) >> 1) as i32;
    }

    sum
}

fn day_of_year(f: fn(i32, u8, u8) -> u16, ymd_vec: &Vec<(i32, u8, u8)>) -> i32 {
    let mut sum = 0;
    for (year, month, day) in ymd_vec {
        let (year, month, day) = (bb(*year), bb(*month), bb(*day));

        sum += f(year, month, day) as i32;
        sum += f(year + 31, month, (day + 7) >> 1) as i32;
        sum += f(year + 141, (month + 5) >> 1, day) as i32;
        sum += f(year + 243, (month + 2) >> 1, (day + 17) >> 1) as i32;
    }

    sum
}

fn from_fixed(f: IsoFromFixedFn, rd_vec: &Vec<RataDie>) -> i64 {
    let map = |r: Result<(i32, u8, u8), I32CastError>| {
        let x = r.unwrap();
        x.0 as i64 + (x.1 + x.2) as i64
    };

    let mut sum = 0;
    for rd in rd_vec {
        let rd = bb(*rd).to_i64_date();

        sum += map(f(RataDie::new(rd)));
        sum += map(f(RataDie::new(rd + 1313)));
        sum += map(f(RataDie::new(rd + 7429)));
        sum += map(f(RataDie::new(rd - 5621)));
    }

    sum
}

fn iso_from_year_day(f: fn(i32, u16) -> (u8, u8), rd_vec: &Vec<(i32, u16)>) -> i64 {
    let map = |x: (u8, u8)| (x.0 + x.1) as i64;
    let mut sum = 0;

    for (year, day_of_year) in rd_vec {
        let (year, day_of_year) = (bb(*year), bb(*day_of_year));

        sum += map(f(year, day_of_year));
        sum += map(f(year + 21, day_of_year));
        sum += map(f(year + 97, (day_of_year + 127) >> 1));
        sum += map(f(year + 137, (day_of_year + 12) >> 1));
    }

    sum
}

#[no_mangle]
fn bench_fixed_from(c: &mut Criterion) {
    const Y: i32 = 1_000;
    const DELTA: i32 = 2_000;
    let ymd_vec = prep_gen_ymd_vec(Y, DELTA);

    {
        let mut group = c.benchmark_group("fixed_from_iso");

        #[cfg(feature = "bench")]
        group.bench_function("OLD", |b| {
            b.iter(|| fixed_from(old::fixed_from_iso, bb(&ymd_vec)))
        });
        group.bench_function("NEW", |b| {
            b.iter(|| fixed_from(new::fixed_from_iso, bb(&ymd_vec)))
        });
    }

    {
        let mut group = c.benchmark_group("fixed_from_julian");

        #[cfg(feature = "bench")]
        group.bench_function("OLD", |b| {
            b.iter(|| fixed_from(j_old::fixed_from_julian, bb(&ymd_vec)))
        });
        group.bench_function("NEW", |b| {
            b.iter(|| fixed_from(j_new::fixed_from_julian, bb(&ymd_vec)))
        });
    }
}

fn bench_day_of_week(c: &mut Criterion) {
    const Y: i32 = 1_000;
    const DELTA: i32 = 2_000;
    let ymd_vec = prep_gen_ymd_vec(Y, DELTA);

    let mut group = c.benchmark_group("day_of_week");

    #[cfg(feature = "bench")]
    group.bench_function("iso/OLD", |b| {
        b.iter(|| day_of_week(old::day_of_week, bb(&ymd_vec)))
    });
    group.bench_function("iso/NEW", |b| {
        b.iter(|| day_of_week(new::day_of_week, bb(&ymd_vec)))
    });
}

fn bench_day_of_year(c: &mut Criterion) {
    const Y: i32 = 1_000;
    const DELTA: i32 = 2_000;
    let ymd_vec = prep_gen_ymd_vec(Y, DELTA);

    let mut group = c.benchmark_group("day_of_year");

    #[cfg(feature = "bench")]
    group.bench_function("iso/OLD", |b| {
        b.iter(|| day_of_year(old::day_of_year, bb(&ymd_vec)))
    });
    group.bench_function("iso/NEW", |b| {
        b.iter(|| day_of_year(new::day_of_year, bb(&ymd_vec)))
    });

    #[cfg(feature = "bench")]
    group.bench_function("julian/OLD", |b| {
        b.iter(|| day_of_year(j_old::day_of_year, bb(&ymd_vec)))
    });
    group.bench_function("julian/NEW", |b| {
        b.iter(|| day_of_year(j_new::day_of_year, bb(&ymd_vec)))
    });
}

fn bench_from_fixed(c: &mut Criterion) {
    const N: i64 = 10_000;
    let rd_vec = prep_gen_rata_die_vec(N);

    let mut group = c.benchmark_group("from_fixed");

    #[cfg(feature = "bench")]
    group.bench_function("iso/OLD", |b| {
        b.iter(|| from_fixed(old::iso_from_fixed, bb(&rd_vec)))
    });
    group.bench_function("iso/NEW", |b| {
        b.iter(|| from_fixed(new::iso_from_fixed, bb(&rd_vec)))
    });

    #[cfg(feature = "bench")]
    group.bench_function("julian/OLD", |b| {
        b.iter(|| from_fixed(j_old::julian_from_fixed, bb(&rd_vec)))
    });
    group.bench_function("julian/NEW", |b| {
        b.iter(|| from_fixed(j_new::julian_from_fixed, bb(&rd_vec)))
    });
}

fn bench_iso_from_year_day(c: &mut Criterion) {
    const Y: i32 = 1_000;
    const DELTA: i32 = 2_000;

    let mut group = c.benchmark_group("from_year_day");

    let yd_vec = prep_gen_yd_vec(Y, DELTA, 1, 365);
    #[cfg(feature = "bench")]
    group.bench_function("iso/OLD/AVG", |b| {
        b.iter(|| iso_from_year_day(old::iso_from_year_day, bb(&yd_vec)))
    });
    group.bench_function("iso/NEW/AVG", |b| {
        b.iter(|| iso_from_year_day(new::iso_from_year_day, bb(&yd_vec)))
    });

    // In range of first two months old algo is faster
    // And they ~ the same perf in 3rd/4th months
    let yd_vec = prep_gen_yd_vec(Y, DELTA, 3, 57);
    #[cfg(feature = "bench")]
    group.bench_function("iso/OLD/START", |b| {
        b.iter(|| iso_from_year_day(old::iso_from_year_day, bb(&yd_vec)))
    });
    group.bench_function("iso/NEW/START", |b| {
        b.iter(|| iso_from_year_day(new::iso_from_year_day, bb(&yd_vec)))
    });

    let yd_vec = prep_gen_yd_vec(Y, DELTA, 300, 360);
    #[cfg(feature = "bench")]
    group.bench_function("iso/OLD/END", |b| {
        b.iter(|| iso_from_year_day(old::iso_from_year_day, bb(&yd_vec)))
    });
    group.bench_function("iso/NEW/END", |b| {
        b.iter(|| iso_from_year_day(new::iso_from_year_day, bb(&yd_vec)))
    });
}

criterion_group!(benchmark_fixed_from, bench_fixed_from);
criterion_group!(benchmark_year_from_fixed, bench_day_of_week);
criterion_group!(benchmark_day_of_year, bench_day_of_year);
criterion_group!(benchmark_from_fixed, bench_from_fixed);
criterion_group!(benchmark_iso_from_year_day, bench_iso_from_year_day);

criterion_main!(
    benchmark_fixed_from,
    benchmark_year_from_fixed,
    benchmark_day_of_year,
    benchmark_from_fixed,
    benchmark_iso_from_year_day
);
