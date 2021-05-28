use crate::Yoke;
use crate::Yokeable;
use crate::yokeable::Foo;
use crate::yokeable::FooWrap;

impl<'b, Y, C> Foo<'b> for Yoke<Y, C>
where
    Y: for<'a> Yokeable<'a>,
    <Y as Yokeable<'b>>::Output: Foo<'b> + 'b
{
}

fn foo_make<'de>(input: &'de dyn Foo<'static>) -> <FooWrap<'static> as Yokeable<'de>>::Output {
    let inner: &'de dyn Foo<'de> = unsafe { std::mem::transmute(input) };
    FooWrap {
        inner,
    }
}

fn make_dyn<'b, 'q, Y, C>(input: Yoke<Y, C>) -> Yoke<FooWrap<'static>, Box<dyn Foo<'static>>>
where
    Y: for<'a> Yokeable<'a>,
    <Y as Yokeable<'b>>::Output: Foo<'b> + 'b
{
    let boxed: Box<dyn Foo<'b>> = Box::new(input);
    let cart: Box<dyn Foo<'static>> = unsafe { std::mem::transmute(boxed) };
    Yoke::<FooWrap<'static>, Box<dyn Foo<'static>>>::attach_to_cart_badly(cart, foo_make)
}
