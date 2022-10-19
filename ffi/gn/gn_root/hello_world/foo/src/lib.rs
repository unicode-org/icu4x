#[derive(Debug)]
pub struct Foo {
    s: &'static str,
    i: &'static str
}

impl Foo {
    pub fn new(s: &'static str) -> Foo {
        Foo{s: s, i: "foo"}
    }
}