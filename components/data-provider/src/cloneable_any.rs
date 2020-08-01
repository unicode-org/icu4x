use downcast_rs::impl_downcast;
use downcast_rs::Downcast;
use std::fmt::Debug;

#[cfg(feature = "erased-serde")]
use erased_serde;

// Please do not to make this trait public, because it is easy to use incorrectly. It is fine as
// an internal auto-implemented trait.
pub(super) trait CloneableAny: Debug + Downcast + erased_serde::Serialize {
    fn clone_into_box(&self) -> Box<dyn CloneableAny>;

    fn as_serialize(&self) -> &dyn erased_serde::Serialize;
}

impl ToOwned for dyn CloneableAny {
    type Owned = Box<dyn CloneableAny>;

    fn to_owned(&self) -> Self::Owned {
        CloneableAny::clone_into_box(self)
    }
}

// Implement CloneableAny for all 'static types implementing Clone.
impl<T: 'static + Clone + Debug + erased_serde::Serialize> CloneableAny for T {
    fn clone_into_box(&self) -> Box<dyn CloneableAny> {
        Box::new(self.clone())
    }

    fn as_serialize(&self) -> &dyn erased_serde::Serialize {
        self
    }
}

// Adds the Downcast methods to all 'static types implementing CloneableAny.
impl_downcast!(CloneableAny);
