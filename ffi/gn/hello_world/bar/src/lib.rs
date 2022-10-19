#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Debug)]
pub struct Foo {
    s: &'static str,
    i: &'static str
}

impl Foo {
    pub fn new(s: &'static str) -> Foo {
        Foo{s: s, i: "bar"}
    }
}

pub fn answer() -> i32 {
  42
}