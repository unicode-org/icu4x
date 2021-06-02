use crate::Yoke;
use crate::Yokeable;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::mem;
use std::rc::Rc;

pub trait ZeroCopyClone: for<'a> Yokeable<'a> {
    fn zcc<'b, 's>(this: &'b <Self as Yokeable<'s>>::Output) -> <Self as Yokeable<'b>>::Output;
}

struct DataStruct<'s> {
    f1: Cow<'s, str>,
    f2: Cow<'s, str>,
}

unsafe impl<'a> Yokeable<'a> for DataStruct<'static> {
    type Output = DataStruct<'a>;

    fn transform(&'a self) -> &'a DataStruct<'a> {
        // Doesn't need unsafe: `'a` is covariant so this lifetime cast is always safe
        self
    }

    unsafe fn make(from: DataStruct<'a>) -> Self {
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

impl ZeroCopyClone for DataStruct<'static> {
    fn zcc<'b, 's>(this: &'b DataStruct<'s>) -> DataStruct<'b> {
        DataStruct {
            f1: Cow::Borrowed(this.f1.borrow()),
            f2: Cow::Borrowed(this.f2.borrow()),
        }
    }
}

impl ZeroCopyClone for &'static str {
    fn zcc<'b, 's>(this: &'b &'s str) -> &'b str {
        this
    }
}

impl<'b, B> Yoke<B, &'b <B as Yokeable<'b>>::Output>
where
    B: ZeroCopyClone,
{
    fn from_borrowed(b: &'b <B as Yokeable<'b>>::Output) -> Self {
        Self::attach_to_cart_badly(b, B::zcc)
    }
}

impl<'b, B> Yoke<B, Rc<<B as Yokeable<'b>>::Output>>
where
    B: ZeroCopyClone,
{
    fn from_rc(b: Rc<<B as Yokeable<'b>>::Output>) -> Self {
        Self::attach_to_cart_badly(b, B::zcc)
    }
}

#[test]
fn test_borrowing() {
    let data_struct = DataStruct {
        f1: Cow::Owned("foo".to_string()),
        f2: Cow::Owned("bar".to_string()),
    };

    let yoke = Yoke::<DataStruct<'static>, &DataStruct<'_>>::from_borrowed(&data_struct);

    assert_eq!(yoke.get().f1, "foo");
    assert!(matches!(yoke.get().f1, Cow::Borrowed(_)));

    let yoke = Yoke::<DataStruct<'static>, Rc<DataStruct<'_>>>::from_rc(Rc::from(data_struct));

    assert_eq!(yoke.get().f1, "foo");
    assert!(matches!(yoke.get().f1, Cow::Borrowed(_)));
}
