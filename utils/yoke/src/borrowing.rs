use crate::Yoke;
use crate::Yokeable;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::mem;

trait ZeroCopyCloneV2 {
    type Yokeable: for<'a> Yokeable<'a>;

    fn zero_copy_clone_v2<'b>(&'b self) -> <Self::Yokeable as Yokeable<'b>>::Output;
}

struct DataStruct<'s> {
    f1: Cow<'s, str>,
    f2: Cow<'s, str>,
}

unsafe impl<'a> Yokeable<'a> for DataStruct<'static> {
    type Output = DataStruct<'a>;

    fn transform(&'a self) -> &'a DataStruct<'a> {
        unsafe { mem::transmute(self) }
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

fn helper<'de, B>(b: &'de B) -> <<B as ZeroCopyCloneV2>::Yokeable as Yokeable<'de>>::Output
where
    B: ZeroCopyCloneV2
{
    todo!()
}

fn yoke_from_borrowed<'b, B>(b: &'b B) -> Yoke<<B as ZeroCopyCloneV2>::Yokeable, &'b B>
where
    B: ZeroCopyCloneV2
{
    Yoke::<<B as ZeroCopyCloneV2>::Yokeable, &'b B>::attach_to_cart_badly(b, helper)
}
