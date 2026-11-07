// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;
use yoke::either::EitherCart;
use yoke::erased::{ErasedArcCart, ErasedBoxCart, ErasedRcCart};
use yoke::{Yoke, Yokeable};

// Test for strong protection, should pass under miri with -Zmiri-retag-fields
// See https://github.com/unicode-org/icu4x/issues/3696

fn example(_: Yoke<&'static [u8], Vec<u8>>) {}

#[test]
fn run_test() {
    example(Yoke::attach_to_cart(vec![0, 1, 2], |data| data));
}

// Tests for Box and &mut carts under Miri (Stacked Borrows and Tree Borrows).
// See https://github.com/unicode-org/icu4x/issues/2095

#[cfg_attr(feature = "derive", derive(Yokeable))]
struct Words<'cart>(Vec<&'cart String>);

#[cfg(not(feature = "derive"))]
// SAFETY: `Words<'cart>` is covariant over `'cart`.
unsafe impl<'a> Yokeable<'a> for Words<'static> {
    type Output = Words<'a>;
    fn transform(&'a self) -> &'a Self::Output {
        self
    }
    fn transform_owned(self) -> Self::Output {
        self
    }
    unsafe fn make(from: Self::Output) -> Self {
        // SAFETY: `Self::Output` is `Words<'a>`, which has identical layout to `Words<'static>`.
        unsafe { core::mem::transmute(from) }
    }
    fn transform_mut<F>(&'a mut self, f: F)
    where
        F: 'static + for<'b> FnOnce(&'b mut Self::Output),
    {
        // SAFETY: Caller guarantees no borrowed data with shorter lifetime is inserted into `self`.
        f(unsafe { core::mem::transmute::<&'a mut Self, &'a mut Self::Output>(self) })
    }
}

fn words(cart: &[String]) -> Words<'_> {
    Words(cart.iter().collect())
}

#[test]
fn box_slice_string() {
    let in_arc: Arc<[String]> = Arc::from([String::from("word")]);
    let yoked: Yoke<Words<'static>, Arc<[String]>> =
        Yoke::attach_to_cart(in_arc, |cart: &[String]| words(cart));
    assert_eq!(yoked.get().0[0], "word");

    let in_box: Box<[String]> = vec![String::from("word")].into_boxed_slice();
    let yoked: Yoke<Words<'static>, Box<[String]>> =
        Yoke::attach_to_cart(in_box, |cart: &[String]| words(cart));
    assert_eq!(yoked.get().0[0], "word");
}

#[test]
fn box_noalias() {
    let x: Yoke<&'static u8, Box<u8>> = Yoke::attach_to_cart(Box::new(42), |data| data);
    assert_eq!(**x.get(), 42);
}

#[test]
fn box_try_attach_to_cart() {
    let x: Yoke<&'static u8, Box<u8>> =
        Yoke::try_attach_to_cart(Box::new(42), |data| Ok::<_, ()>(data)).unwrap();
    assert_eq!(**x.get(), 42);
}

#[test]
fn box_attach_to_zero_copy_cart() {
    let x: Yoke<&'static str, Box<str>> =
        Yoke::attach_to_zero_copy_cart(String::from("hello").into_boxed_str());
    assert_eq!(*x.get(), "hello");
}

#[test]
fn box_map_project() {
    let x: Yoke<&'static [u8], Box<[u8]>> =
        Yoke::attach_to_cart(vec![10, 20, 30].into_boxed_slice(), |data| data);
    let y: Yoke<&'static u8, Box<[u8]>> = x.map_project(|slice, _| &slice[1]);
    assert_eq!(**y.get(), 20);
}

#[test]
fn box_try_map_project() {
    let x: Yoke<&'static [u8], Box<[u8]>> =
        Yoke::attach_to_cart(vec![10, 20, 30].into_boxed_slice(), |data| data);
    let y: Yoke<&'static u8, Box<[u8]>> = x
        .try_map_project(|slice, _| Ok::<_, ()>(&slice[2]))
        .unwrap();
    assert_eq!(**y.get(), 30);
}

#[test]
fn box_map_project_explicit_capture() {
    let x: Yoke<&'static [u8], Box<[u8]>> =
        Yoke::attach_to_cart(vec![10, 20, 30].into_boxed_slice(), |data| data);
    let y: Yoke<&'static u8, Box<[u8]>> =
        x.map_project_with_explicit_capture(1usize, |slice, idx, _| &slice[idx]);
    assert_eq!(**y.get(), 20);
}

#[test]
fn box_try_map_project_explicit_capture() {
    let x: Yoke<&'static [u8], Box<[u8]>> =
        Yoke::attach_to_cart(vec![10, 20, 30].into_boxed_slice(), |data| data);
    let y: Yoke<&'static u8, Box<[u8]>> = x
        .try_map_project_with_explicit_capture(2usize, |slice, idx, _| Ok::<_, ()>(&slice[idx]))
        .unwrap();
    assert_eq!(**y.get(), 30);
}

#[test]
fn box_map_with_cart() {
    let x: Yoke<&'static u8, Box<[u8]>> =
        Yoke::attach_to_cart(vec![10, 20, 30].into_boxed_slice(), |data| &data[0]);
    let y: Yoke<(&'static u8, &'static u8), Box<[u8]>> =
        x.map_with_cart(|first, cart| (first, &cart[2]));
    assert_eq!((*y.get().0, *y.get().1), (10, 30));
}

#[test]
fn box_try_map_with_cart() {
    let x: Yoke<&'static u8, Box<[u8]>> =
        Yoke::attach_to_cart(vec![10, 20, 30].into_boxed_slice(), |data| &data[0]);
    let y: Yoke<(&'static u8, &'static u8), Box<[u8]>> = x
        .try_map_with_cart(|first, cart| Ok::<_, ()>((first, &cart[1])))
        .unwrap();
    assert_eq!((*y.get().0, *y.get().1), (10, 20));
}

#[test]
fn box_backing_cart_and_into_backing_cart() {
    let x: Yoke<&'static u8, Box<u8>> = Yoke::attach_to_cart(Box::new(42), |data| data);
    assert_eq!(**x.backing_cart(), 42);
    assert_eq!(**x.get(), 42);
    let cart: Box<u8> = x.into_backing_cart();
    assert_eq!(*cart, 42);
}

#[test]
fn box_with_mut() {
    let mut x: Yoke<Cow<'static, str>, Box<String>> =
        Yoke::attach_to_zero_copy_cart(Box::new(String::from("hello")));
    assert_eq!(x.get(), "hello");
    x.with_mut(|cow| {
        assert_eq!(&**cow, "hello");
    });
    assert_eq!(x.get(), "hello");
}

#[test]
fn box_wrap_cart_in_option() {
    let x: Yoke<&'static u8, Box<u8>> = Yoke::attach_to_cart(Box::new(42), |data| data);
    let y: Yoke<&'static u8, Option<Box<u8>>> = x.wrap_cart_in_option();
    assert_eq!(**y.get(), 42);
    let y = y.try_into_yokeable().unwrap_err();
    assert_eq!(**y.get(), 42);
    // Nested Option<Option<Box<u8>>> (non-niche outer Option!)
    let z: Yoke<&'static u8, Option<Option<Box<u8>>>> = y.wrap_cart_in_option();
    assert_eq!(**z.get(), 42);
}

#[test]
fn box_wrap_cart_in_either_then_option() {
    let x: Yoke<&'static u8, Box<u8>> = Yoke::attach_to_cart(Box::new(42), |data| data);
    let ya: Yoke<&'static u8, EitherCart<Box<u8>, Rc<u8>>> = x.wrap_cart_in_either_a();
    let yopt = ya.wrap_cart_in_option();
    assert_eq!(**yopt.get(), 42);
}

#[test]
fn box_wrap_cart_in_box() {
    let x: Yoke<&'static u8, Box<u8>> = Yoke::attach_to_cart(Box::new(42), |data| data);
    let y: Yoke<&'static u8, Box<Box<u8>>> = x.wrap_cart_in_box();
    assert_eq!(**y.get(), 42);
}

#[test]
fn box_wrap_cart_in_rc() {
    let x: Yoke<&'static u8, Box<u8>> = Yoke::attach_to_cart(Box::new(42), |data| data);
    let y: Yoke<&'static u8, Rc<Box<u8>>> = x.wrap_cart_in_rc();
    assert_eq!(**y.get(), 42);
    let y2 = y.clone();
    assert_eq!(**y.get(), 42);
    assert_eq!(**y2.get(), 42);
}

#[test]
fn box_wrap_cart_in_arc() {
    let x: Yoke<&'static u8, Box<u8>> = Yoke::attach_to_cart(Box::new(42), |data| data);
    let y: Yoke<&'static u8, Arc<Box<u8>>> = x.wrap_cart_in_arc();
    assert_eq!(**y.get(), 42);
    let y2 = y.clone();
    assert_eq!(**y.get(), 42);
    assert_eq!(**y2.get(), 42);
}

#[test]
fn box_wrap_cart_in_either() {
    let x: Yoke<&'static u8, Box<u8>> = Yoke::attach_to_cart(Box::new(42), |data| data);
    let ya: Yoke<&'static u8, EitherCart<Box<u8>, Rc<u8>>> = x.wrap_cart_in_either_a();
    assert_eq!(**ya.get(), 42);

    let x2: Yoke<&'static u8, Box<u8>> = Yoke::attach_to_cart(Box::new(99), |data| data);
    let yb: Yoke<&'static u8, EitherCart<Rc<u8>, Box<u8>>> = x2.wrap_cart_in_either_b();
    assert_eq!(**yb.get(), 99);
}

#[test]
fn box_erase_box_cart() {
    let x: Yoke<&'static u8, Box<u8>> = Yoke::attach_to_cart(Box::new(42), |data| data);
    let y: Yoke<&'static u8, ErasedBoxCart> = x.erase_box_cart();
    assert_eq!(**y.get(), 42);
}

#[test]
fn box_erase_rc_and_arc_cart() {
    let x: Yoke<&'static u8, Box<u8>> = Yoke::attach_to_cart(Box::new(42), |data| data);
    let y: Yoke<&'static u8, ErasedRcCart> = x.wrap_cart_in_rc().erase_rc_cart();
    assert_eq!(**y.get(), 42);

    let x2: Yoke<&'static u8, Box<u8>> = Yoke::attach_to_cart(Box::new(42), |data| data);
    let y2: Yoke<&'static u8, ErasedArcCart> = x2.wrap_cart_in_arc().erase_arc_cart();
    assert_eq!(**y2.get(), 42);
}

#[test]
fn box_convert_cart_into_option_pointer() {
    let x: Yoke<&'static u8, Box<u8>> = Yoke::attach_to_cart(Box::new(42), |data| data);
    let y = x.wrap_cart_in_option().convert_cart_into_option_pointer();
    assert_eq!(**y.get(), 42);
    let y = y.try_into_yokeable().unwrap_err();
    assert_eq!(**y.get(), 42);
}

#[test]
fn mut_ref_cart() {
    let mut val = 42u8;
    let x: Yoke<&'static u8, &mut u8> = Yoke::attach_to_cart(&mut val, |data| data);
    assert_eq!(**x.get(), 42);
    assert_eq!(**x.backing_cart(), 42);
    let y: Yoke<&'static u8, &mut u8> = x.map_project(|d, _| d);
    assert_eq!(**y.get(), 42);
}
