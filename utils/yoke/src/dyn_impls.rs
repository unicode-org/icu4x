use crate::Yoke;
use crate::Yokeable;
use stable_deref_trait::StableDeref;
use std::borrow::Cow;
use std::mem;
use std::ops::Deref;

pub trait Foo<'a> {
    fn foo(&self) -> char;
}

/// Example implementation of trait Foo
impl<'s> Foo<'s> for Cow<'s, str> {
    fn foo(&self) -> char {
        self.chars().next().unwrap_or('?')
    }
}

pub struct FooWrap<'a> {
    pub inner: &'a dyn Foo<'a>,
}

unsafe impl<'a> Yokeable<'a> for FooWrap<'static> {
    type Output = FooWrap<'a>;

    fn transform(&'a self) -> &'a Self::Output {
        // needs unsafe because the compiler has trouble with covariant trait object lifetimes
        unsafe { mem::transmute(self) }
    }

    unsafe fn make(from: Self::Output) -> Self {
        mem::transmute(from)
    }

    fn with_mut<F>(&'a mut self, f: F)
    where
        F: 'static + for<'b> FnOnce(&'b mut Self::Output),
    {
        // Cast away the lifetime of Self
        unsafe { f(mem::transmute::<&'a mut Self, &'a mut Self::Output>(self)) }
    }
}

trait DynHelper {
    type Yokeable: for<'a> Yokeable<'a>;
    type Cart: StableDeref;

    fn attach<'de>(
        data: &'de <Self::Cart as Deref>::Target,
    ) -> <Self::Yokeable as Yokeable<'de>>::Output;

    fn make_cart(input: Box<Self>) -> Self::Cart;
}

impl<'a> DynHelper for dyn Foo<'a> + 'a {
    type Yokeable = FooWrap<'static>;
    type Cart = Box<dyn Foo<'static>>;

    fn attach<'de>(data: &'de dyn Foo<'static>) -> FooWrap<'de> {
        let inner: &'de dyn Foo<'de> = unsafe { mem::transmute(data) };
        FooWrap { inner }
    }

    fn make_cart(input: Box<dyn Foo<'a> + 'a>) -> Box<dyn Foo<'static>> {
        unsafe { mem::transmute(input) }
    }
}

// unsafe impl<T> DynHelper for T where T: Sized {
//     type Yokeable = ();
//     type Cart = ();
// }

impl<'b, Y, C> Foo<'b> for Yoke<Y, C>
where
    Y: for<'a> Yokeable<'a>,
    <Y as Yokeable<'b>>::Output: Foo<'b> + 'b,
{
    fn foo(&self) -> char {
        'y'
    }
}

fn yoke_from_box<'b, D>(input: Box<D>) -> Yoke<<D as DynHelper>::Yokeable, <D as DynHelper>::Cart>
where
    D: DynHelper + ?Sized,
{
    let cart: <D as DynHelper>::Cart = <D as DynHelper>::make_cart(input);
    Yoke::<<D as DynHelper>::Yokeable, <D as DynHelper>::Cart>::attach_to_cart_badly(
        cart,
        <D as DynHelper>::attach,
    )
}

#[test]
fn test_dyn() {
    let source_data = "zyx".to_string();

    let string_yoke: Yoke<Cow<'static, str>, &str> =
        Yoke::<Cow<'static, str>, &str>::attach_to_cart_badly(&source_data, |s| Cow::Borrowed(s));

    let boxed: Box<dyn Foo<'_>> = Box::new(string_yoke);

    let dyn_yoke: Yoke<FooWrap<'static>, Box<dyn Foo<'static>>> = yoke_from_box(boxed);

    assert_eq!(dyn_yoke.get().inner.foo(), 'y');
}
