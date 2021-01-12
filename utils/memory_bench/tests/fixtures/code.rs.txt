// The dhat injection will happen here.
use std::env;

fn do_something() {
    println!("It has another function in it.");
    println!("It uses an import. {}", env!("CARGO_PKG_VERSION"););
}

fn main() {
    // And here.
    println!("This is a test fixture");
}
