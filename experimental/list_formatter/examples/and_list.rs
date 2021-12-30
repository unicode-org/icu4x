use icu_list::{
    options::{Type, Width},
    ListFormatter,
};
use icu_locid_macros::langid;
use rand::seq::SliceRandom;

fn main() {
    let provider = icu_testdata::get_static_provider();

    let list_formatter =
        ListFormatter::try_new(langid!("es"), &provider, Type::And, Width::Wide).unwrap();

    let mut rng = rand::thread_rng();
    let mut data = ["España", "Francia", "Italia", "Suiza"];
    data.shuffle(&mut rng);

    println!("{}", list_formatter.format(&data));
}
