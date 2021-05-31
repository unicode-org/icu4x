use crate::Yoke;
use crate::Yokeable;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::mem;

pub trait ZeroCopyCloneV2 {
    type Yokeable: for<'a> Yokeable<'a>;

    fn zero_copy_clone_v2<'b>(&'b self) -> <Self::Yokeable as Yokeable<'b>>::Output;

    fn borrow_into_yoke<'b>(&'b self) -> Yoke<Self::Yokeable, &'b Self> {
        Yoke::<Self::Yokeable, &'b Self>::attach_to_cart_badly(self, Self::zero_copy_clone_v2)
    }
}

pub trait ZeroCopyCloneV4: for<'a> Yokeable<'a> {
    fn zero_copy_clone_v4<'b>(
        this: &'b <Self as Yokeable<'b>>::Output,
    ) -> <Self as Yokeable<'b>>::Output;
}

// pub trait ZeroCopyCloneV5<'d> {
//     type Yokeable: for<'a> Yokeable<'a> + Yokeable<'d, Output = Self>;

//     fn zero_copy_clone_v5<'b>(&self) -> <Self::Yokeable as Yokeable<'b>>::Output;
// }

pub trait ZeroCopyCloneV6<'d>: 'd {
    type Yokeable: for<'a> Yokeable<'a>;

    fn zero_copy_clone_v6<'b>(&'b self) -> <Self::Yokeable as Yokeable<'b>>::Output;
}

pub trait ZeroCopyCloneV7<'s> {
    fn zero_copy_clone_v7(&'s self) -> Self;
}

pub trait ZeroCopyCloneV8<'o, 'i: 'o>: 'i {
    type Output: 'o;

    fn zero_copy_clone_v8(&'o self) -> Self::Output;
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

impl<'s> ZeroCopyCloneV4 for DataStruct<'static> {
    fn zero_copy_clone_v4<'b>(this: &'b DataStruct<'b>) -> DataStruct<'b> {
        DataStruct {
            f1: Cow::Borrowed(this.f1.borrow()),
            f2: Cow::Borrowed(this.f2.borrow()),
        }
    }
}

impl<'s> ZeroCopyCloneV7<'s> for DataStruct<'s> {
    fn zero_copy_clone_v7(&'s self) -> DataStruct<'s> {
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
    B: ZeroCopyCloneV2,
{
    fn from_borrowed_v2(b: &'b B) -> Self {
        Self::attach_to_cart_badly(b, B::zero_copy_clone_v2)
    }
}

/*
fn zcc_v7_helper<'de, 'b, B>(v: &'de <B as Yokeable<'de>>::Output) -> <B as Yokeable<'de>>::Output
where
    B: for<'a> Yokeable<'a>,
    for<'a> <B as Yokeable<'a>>::Output: ZeroCopyCloneV7<'a>,
    'b: 'de,
{
    v.zero_copy_clone_v7()
}

impl<'b, B> Yoke<B, &'b <B as Yokeable<'b>>::Output>
where
    B: for<'a> Yokeable<'a>,
    <B as Yokeable<'b>>::Output: ZeroCopyCloneV7<'b>,
{
    fn from_borrowed_v7(b: &'b <B as Yokeable<'b>>::Output) -> Self {
        Self::attach_to_cart_badly(
            b,
            ZeroCopyCloneV7::zero_copy_clone_v7,
        )
    }
}

fn zcc_v8_helper<'de, 'b, B>(v: &'de <B as Yokeable<'b>>::Output) -> <B as Yokeable<'de>>::Output
where
    B: for<'a> Yokeable<'a>,
    <B as Yokeable<'b>>::Output: ZeroCopyCloneV8<'de, 'b, Output = <B as Yokeable<'de>>::Output>,
    'b: 'de,
{
    v.zero_copy_clone_v8()
}

impl<'b, B> Yoke<B, &'b <B as Yokeable<'b>>::Output>
where
    B: for<'a> Yokeable<'a>,
    <B as Yokeable<'b>>::Output:
        for<'de> ZeroCopyCloneV8<'de, 'b, Output = <B as Yokeable<'de>>::Output>,
{
    fn from_borrowed_v8(b: &'b <B as Yokeable<'b>>::Output) -> Self {
        Self::attach_to_cart_badly(b, zcc_v8_helper)
    }
}
*/

#[test]
fn test_borrowing() {
    let data_struct = DataStruct {
        f1: Cow::Owned("foo".to_string()),
        f2: Cow::Owned("bar".to_string()),
    };

    let yoke = data_struct.borrow_into_yoke();

    assert_eq!(yoke.get().f1, "foo");
    assert!(matches!(yoke.get().f1, Cow::Borrowed(_)));

    let yoke = Yoke::<DataStruct<'static>, &DataStruct<'_>>::from_borrowed_v2(&data_struct);

    assert_eq!(yoke.get().f1, "foo");
    assert!(matches!(yoke.get().f1, Cow::Borrowed(_)));
}
