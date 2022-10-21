extern crate icu_provider;
extern crate icu_locid;

use icu_provider::hello_world::{HelloWorldFormatter, HelloWorldProvider};
use icu_locid::locale;

fn main() {
    let fmt = HelloWorldFormatter::try_new_unstable(
        &HelloWorldProvider,
        &locale!("eo").into(),
    )
    .expect("locale exists");
    println!("{}", fmt.format());
}
