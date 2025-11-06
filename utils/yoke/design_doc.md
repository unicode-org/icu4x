# Yoke: Targeted Lifetime Erasure in Rust

## Problem statement

Zero-copy deserialization is a very effective way to speed up programs and avoid allocations. However, the requirement that data is borrowed from somewhere else means that:

1. All data types that contain zero-copy data, even indirectly, need to carry a lifetime parameter
2. Certain memory management techniques are hampered, like caching.

Similar to how `dyn` enables Rust programs to perform "type erasure": turning compile-time types into "erased" runtime ones, `Yoke` enables Rust programs to perform the analogous "lifetime erasure": turning specific compile-time lifetimes into "erased" runtime ones. This means we don't need lifetime parameters to track data ownership, and that we can carry reference-counted data that can be safely dropped from a cache.

## Background

[ICU4X](https://github.com/unicode-org/icu4x) is an internationalization library that has pluggable data loading as a core value proposition. Internationalization often needs a lot of data, and we want to make sure data loading can be fast and efficient. Zero-copy deserialization is quite attractive as a way to reduce this load.

## Requirements

 - It should be possible to use zero-copy deserialization without storage of the deserialized data introducing a lifetime (<span style="color:red">**required**</span>)
 - It should be possible to zero-copy deserialize data from some source and have that source data kept alive until the deserialized data is no longer needed, perhaps using things like `Rc<T>`, with dynamically-known lifetimes (<span style="color:red">**required**</span>)
 - It should be possible to manipulate the zero-copy deserialized data, not just read it (<span style="color:orange">**preferred**</span>)
 - It should be possible to conveniently use this abstraction without needing to write any unsafe code (<span style="color:orange">**preferred**</span>)
 - It would be nice if complicated lifetimes were not introduced at _any_ point (<span style="color:#729468">**optional**</span>)
 

## High level design

The `yoke` crate provides the [`Yoke<Y, C>`][`Yoke`] and [`Yokeable<'a>`][`Yokeable`] traits, which form its core value proposition.

`Yoke<Y, C>` allows one to "yoke" a zero-copy deserialized object (say, a `Cow<'a, str>`) to the source it was deserialized from, (say, an `Rc<[u8]>`), known as a "cart", producing a type that looks like `Yoke<Cow<'static, str>, Rc<[u8]>>` and can be moved around with impunity.

The `'static` is somewhat of a lie: it is actually a self-referential lifetime. The `Cow` is allowed to borrow data from the cart (the `Rc<[u8]>`), but the Rust compiler does not allow this, so we use `'static`. Since this `Cow` cannot be normally extracted from the `Yoke`, the lifetime is considered an implementation detail.

Most of the time the yokeable `Y` type will be some kind of zero-copy deserializable abstraction, potentially with an owned variant (like `Cow`, [`ZeroVec`](https://docs.rs/zerovec), or an aggregate containing such types), and the cart `C` will be some smart pointer like `Box<T>`, `Rc<T>`, or `Arc<T>`, potentially wrapped in an `Option<T>`.

### Basic functionality

The `Yokeable<'a>` trait is implemented on the `'static` version of any zero-copy type; for example, `Cow<'static, T>` implements `Yokeable<'a>` (for all `'a`). One can use `Yokeable::Output` on this trait to obtain the "lifetime'd" value of the `Cow<'static, T>`, e.g. `<Cow<'static, T> as Yokeable<'a>'>::Output` is `Cow<'a, T>`.

The key behind this crate is [`Yoke::get()`][get], with a signature as follows:

```rust
impl<Y, C> Yoke<Y, C> where Y: for<'a> Yokeable<'a> {
    pub fn get<'a>(&'a self) -> &'a <Self as Yokeable<'a>>::Output {
        // ...
    }
}
```

Essentially, calling [`.get()`][get] on a type like `Yoke<Cow<'static, str>, _>` will get you a short-lived `&'a Cow<'a, str>`, restricted to the lifetime of the borrow used during `.get()`. This is entirely safe since the `Cow` borrows from the cart type, which cannot be interfered with as long as the `Yoke` is borrowed by `.get()`. `.get()` protects access by essentially reifying the erased lifetime to a safe local one when necessary.


Constructing a `Yoke` can be done using one of the [`attach_to_cart()`][attach] methods with a signature like so:


```rust
impl<Y, C> Yoke<Y, C> where Y: for<'a> Yokeable<'a>, C: StableDeref {
    pub fn attach_to_cart<F>(cart: C, f: F) -> Self
        where F: for<'de> FnOnce(&'de <C as Deref>::Target) -> <Y as Yokeable<'de>>::Output {
        // ..
    }

}
```

which can be used as follows:

```rust
let data: Rc<[u8]> = load_data_from_file();

// constructs a Yoke by zero-copy deserializing from data
let yoke: Yoke<Cow<str>, Rc<[u8]>> = Yoke::attach_to_cart(data.clone(), |data| deserialize_cow_from_data(data));

// prints the data out
println!("{:?}", yoke.get());

// you can move `yoke` without needing to move data
```

`Yokeable` is implemented on all basic borrowed types `Cow<'static, T>`, `&'static T`, etc, and can be implemented on complex types using [`#[derive(Yokeable)]`][yokeable-derive]:

```rust
#[derive(Yokeable)]
struct AggregateZeroCopy<'a> {
    field1: Cow<'a, str>,
    field2: Cow<'a, [u32]>,
}
```

As usual with zero-copy deserialization, it is good to use types that have both owned and borrowed variants, like `Cow` and [`ZeroVec`](https://docs.rs/zerovec)

### Mutation

It is possible to mutate `Yoke`s using the [`.with_mut()`](with_mut) method. This cannot be used to introduce _new_ borrowed data, but one can freely mutate values (or parts of values) into their owned variants:

```rust
// yoke is a Yoke<Cow<str>, _>
yoke.with_mut(|cow| {
    let mut_str = cow.to_mut();
    mut_str.clear();
    mut_str.push_str("bar");
});
```

### Projection

There are various [`.project()`][project] methods that allow turning a `Yoke` into another `Yoke` containing a different type that may contain elements of the original yoked value. For example, one can turn a `Yoke<&str, _>` into a `Yoke<&[u8]>` like so:

```rust
yoke.project(move |yk, _| yoke.as_bytes());
```


This can also be used to, for example, focus a `Yoke` onto containing a subfield of the type it uses, e.g. we can extract `field1` of `AggregateZeroCopy` above with:

```rust
yoke.project(move |yk, _| yoke.field1);
```

 [get]: https://docs.rs/yoke/latest/yoke/struct.Yoke.html#method.get
 [attach]: https://docs.rs/yoke/latest/yoke/struct.Yoke.html#method.attach
 [with_mut]: https://docs.rs/yoke/latest/yoke/struct.Yoke.html#method.with_mut
 [project]: https://docs.rs/yoke/latest/yoke/struct.Yoke.html#method.project

## Detailed design

### Yokeable

The core design of [`Yoke`] relies on the [`Yokeable`] trait, with the following signature:

```rust
pub unsafe trait Yokeable<'a>: 'static {
    type Output: 'a;
    fn transform(&'a self) -> &'a Self::Output;
    fn transform_owned(self) -> Self::Output;
    unsafe fn make(from: Self::Output) -> Self;
    fn transform_mut<F>(&'a mut self, f: F)
        where F: 'static + for<'b> FnOnce(&'b mut Self::Output);
}
```

This trait should be implemented for all `'a` on the `'static` version of any type with [covariant] lifetimes, with the `Output` type having the lifetime swapped with `'a`. In essence, a [covariant] lifetime is one where replacing the lifetime with a shorter lifetime is always safe. This is true for, e.g. `'a` in `&'a T` and `Cow<'a, T>`, but it is not true in `Cell<&'a T>` or `fn(&'a u8)`. The covariance is crucial here, because `Yoke` uses this trait as a utility for replacing lifetimes, and the trait asserts that replacing lifetimes this way is safe.

A way of looking at `Self::Output` is `Self<'a>` if we look at `Self` as a higher kinded type here: the shorthand `Self<'a>` is equivalent to `<Self as Yokeable<'a>>::Output`). In this explanation we can use `Self<'static>` and `Self<'a>` (etc) to be clearer about what's happening.

The first two methods on the trait are basically an in-code representation of covariance: it should always be possible to replace `Self<'static>` with `Self<'a>`. In fact, in most cases these methods can be implemented with the body being just `self`.

`make()` is used when _constructing_ `Yoke`s and enables us to construct the "fake" `'static` lifetime by converting `Self<'a>` to `Self<'static>`. This is very unsafe, and is only called when `Yoke`s are being constructed since such an operation is basically handing over responsibility for respecting the lifetime to the `Yoke`.

`transform_mut()` is interesting: it's what enables [`Yoke::with_mut()`][with_mut], and it works by enforcing extremely strict constraints on what data can be smuggled in or out of the closure passed to it.

The bound is `F: 'static + for<'b> FnOnce(&'b mut Self::Output)`: the `'static` ensures that no borrowed data can be pulled from the outside, and the `for<'b>` ensures that data from the inside cannot leak since the closure has to be valid for _all_ lifetimes, not just the lifetime of `self`, which means that it needs to theoretically be able to handle data with much shorter lifetimes even if it's not actually going to be passed such data.

### Safety of various methods

The main things that need to be taken care of are:

 1. Borrowed data from within the `Yoke` may only escape with short-lived lifetimes, if at all
 2. When constructing or modifying a `Yoke` no borrowed data from the outside may sneak in, it must all come from the Cart.


Most of the methods derive safety from careful choices of bounds in their signatures.


[`.get()`][get] has the following signature:

```rust
impl Yoke<Y, C> where Y: for<'a> Yokeable<'a> {
    pub fn get<'a>(&'a self) -> &'a <Y as Yokeable<'a>>::Output {
        // ...
    }
}
```

by returning `&'a Y<'a>`, `.get()` enforces that the returned type cannot live longer than the method borrow, satisfying safety point 1. Point 2 is irrelevant since this is not mutating or constructing `Yoke`s

[`Yoke::attach_to_cart()`][attach] and friends have signatures like the following:

```rust
impl Yoke<Y, C> where Y: for<'a> Yokeable<'a>, C: StableDeref {
    pub fn attach_to_cart<F>(cart: C, f: F) -> Self
        where F: for<'de> FnOnce(&'de <C as Deref>::Target) -> <Y as Yokeable<'de>>::Output {
        // ..
    }
}
```

The `StableDeref` requirement on the `Cart` type ensures that moving the cart does not move any of the data found by `Deref`ing the cart, which is crucial to ensure that the closure is only operating on data already pinned by the allocation of the cart. The `for<'de>` on `F` pins down the lifetime flow by ensuring that any borrowed data in the output (`Y<'de>`) _must_ have been borrowed from the input (`&'de C::Target`). It can't be borrowed from anywhere else because the function must be written to work for all lifetimes `'de`. This satisfies safety point 2, and point 1 is irrelevant since the `Yoke` hasn't been constructed yet.

[`.with_mut()`][with_mut] just proxies to `Yokeable::transform_mut` and satisfies point 2 for the reasons stated above when describing that method. It also satisfies point 1 due to the `for<'a>` lifetime in the bound; if the function is allowed to be called with any lifetime, it cannot export that data since it will not statically know what the lifetime will be.

[`.project()`][project] has the following signature (the other project methods are a little bit more complicated to allow for captures, but they work off the same principles):

```rust
impl Yoke<Y, C> where Y: for<'a> Yokeable<'a> {
    pub fn project<P>(
        self,
        f: for<'a> fn(_: <Y as Yokeable<'a>>::Output, _: PhantomData<&'a ()>) -> <P as Yokeable<'a>>::Output
    ) -> Yoke<P, C>
        where P: for<'a> Yokeable<'a>
        // ...
    }
}
```

This is a fair bit more complicated. First off the bat, the `PhantomData` can be ignored completely; it exists to satisfy the compiler, which needs the lifetime `'a` to be used in a more concrete place.

What this function does is take a closure that, for all `'a`, can convert `Y<'a>` to `P<'a>`. The `for<'a>` here has the same effect as in `Yoke::attach_to_cart()`: it pins down a lifetime flow such that all borrowed data in `P<'a>` _must_ have come from `Y<'a>` and nowhere else, satisfying safety point 2. There's no chance for `f` to smuggle any borrowed out so safety point 1 is also satisfied. The `with_capture` variants of this are able to enforce that no data of the wrong lifetime is smuggled out by relying on the `for<'a>` again: any data being smuggled out would not have a statically known lifetime and cannot be stuffed into the capture since that would give the capture a statically unknown lifetime.


### ZeroFrom

The `zerofrom` crate comes with the [`ZeroFrom`] trait. Implementing this trait allows one to define a canonical, infallible implementation of Yoke's `attach_to_cart` function, enabling various additional constructors on Yoke for convenience, including `Yoke::attach_to_borrowed_cart()`, `Yoke::attach_to_box_cart()`, and `Yoke::attach_to_rc_cart()`.

Using this trait, for example, one can generically talk about taking a `Cow<'a, T>` and borrowing it to produce a `Cow<'b, T>` that is `Cow::Borrowed`, borrowing from the original `Cow`, regardless of whether or not the original `Cow` was owned or borrowed.

It has the following signature and also has a custom derive:

```rust
pub trait ZeroFrom<C: ?Sized>: for<'a> Yokeable<'a> {
    fn zero_from<'b>(cart: &'b C) -> <Self as Yokeable<'b>>::Output;
}
```


 [covariant]: https://doc.rust-lang.org/nomicon/subtyping.html
 [`Yoke`]: https://docs.rs/yoke/latest/yoke/struct.Yoke.html
 [`Yokeable`]: https://docs.rs/yoke/latest/yoke/trait.Yokeable.html
 [yokeable-derive]: https://docs.rs/yoke/latest/yoke/derive.Yokeable.html
 [`ZeroFrom`]: https://docs.rs/zerofrom/latest/zerofrom/trait.ZeroFrom.html
 [get]: https://docs.rs/yoke/latest/yoke/struct.Yoke.html#method.get
 [attach]: https://docs.rs/yoke/latest/yoke/struct.Yoke.html#method.attach
 [with_mut]: https://docs.rs/yoke/latest/yoke/struct.Yoke.html#method.with_mut
 [project]: https://docs.rs/yoke/latest/yoke/struct.Yoke.html#method.project
