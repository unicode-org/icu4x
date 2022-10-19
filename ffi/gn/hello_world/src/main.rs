fn main() {
    let f = baz::Foo::new("hello");
    println!("Hello world!");
    println!("I'm from a dependency: {:?}!", f);
}