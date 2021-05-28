use crate::Yoke;
use crate::Yokeable;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::mem;

trait ZeroCopyCloneV2 {
    type Yokeable: for<'a> Yokeable<'a>;

    fn zero_copy_clone_v2<'b>(&'b self) -> <Self::Yokeable as Yokeable<'b>>::Output;

    fn borrow_into_yoke<'b>(&'b self) -> Yoke<Self::Yokeable, &'b Self> {
        Yoke::<Self::Yokeable, &'b Self>::attach_to_cart_badly(self, Self::zero_copy_clone_v2)
    }
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

impl<'s> ZeroCopyCloneV2 for DataStruct<'s> {
    type Yokeable = DataStruct<'static>;

    fn zero_copy_clone_v2<'b>(&'b self) -> <Self::Yokeable as Yokeable<'b>>::Output {
        DataStruct {
            f1: Cow::Borrowed(self.f1.borrow()),
            f2: Cow::Borrowed(self.f2.borrow()),
        }
    }
}

impl ZeroCopyCloneV2 for str {
    type Yokeable = &'static str;

    fn zero_copy_clone_v2<'b>(&'b self) -> <Self::Yokeable as Yokeable<'b>>::Output {
        self
    }
}

impl<'b, B> Yoke<<B as ZeroCopyCloneV2>::Yokeable, &'b B>
where
    B: ZeroCopyCloneV2
{
    fn from_borrowed(b: &'b B) -> Self {
        Self::attach_to_cart_badly(b, B::zero_copy_clone_v2)
    }
}

#[test]
fn test_borrowing() {
    let data_struct = DataStruct {
        f1: Cow::Owned("foo".to_string()),
        f2: Cow::Owned("bar".to_string()),
    };

    let yoke = data_struct.borrow_into_yoke();

    assert_eq!(yoke.get().f1, "foo");
    assert!(matches!(yoke.get().f1, Cow::Borrowed(_)));

    let yoke = Yoke::<DataStruct<'static>, &DataStruct<'_>>::from_borrowed(&data_struct);

    assert_eq!(yoke.get().f1, "foo");
    assert!(matches!(yoke.get().f1, Cow::Borrowed(_)));
}
