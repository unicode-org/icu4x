#[macro_export]
macro_rules! construct {
    ($c:expr, $struct:ident, $struct_name:expr, $data_str:expr) => {
        $c.bench_function($struct_name, |b| {
            b.iter(|| {
                for s in $data_str {
                    let _: Result<$struct, _> = black_box(s).parse();
                }
            })
        });
    };
}

#[macro_export]
macro_rules! to_string {
    ($c:expr, $struct:ident, $struct_name:expr, $data:expr) => {
        $c.bench_function($struct_name, |b| {
            b.iter(|| {
                for s in $data {
                    let _ = black_box(s).to_string();
                }
            })
        });
    };
}

#[macro_export]
macro_rules! compare_struct {
    ($c:expr, $struct:ident, $struct_name:expr, $data1:expr, $data2:expr) => {
        $c.bench_function(BenchmarkId::new("struct", $struct_name), |b| {
            b.iter(|| {
                for (lid1, lid2) in $data1.iter().zip($data2.iter()) {
                    let _ = black_box(lid1) == black_box(lid2);
                }
            })
        });
    };
}

#[macro_export]
macro_rules! compare_str {
    ($c:expr, $struct:ident, $struct_name:expr, $data1:expr, $data2:expr) => {
        $c.bench_function(BenchmarkId::new("str", $struct_name), |b| {
            b.iter(|| {
                for (lid, s) in $data1.iter().zip($data2.iter()) {
                    let _ = black_box(lid) == &black_box(s).as_str();
                }
            })
        });
    };
}

#[macro_export]
macro_rules! canonicalize {
    ($c:expr, $struct:ident, $struct_name:expr, $data:expr) => {
        $c.bench_function($struct_name, |b| {
            b.iter(|| {
                for s in $data {
                    let _ = black_box(s).to_string();
                }
                for s in $data {
                    let _ = $struct::canonicalize(black_box(s));
                }
            })
        });
    };
}
