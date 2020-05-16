use std::prelude::v1::*;

use downcast_rs::Downcast;
use downcast_rs::impl_downcast;
use std::fmt::{Debug};

// Please do not to make this trait public, because it is easy to use incorrectly. It is fine as
// an internal auto-implemented trait.
pub(super) trait CloneableAny: Debug + Downcast {
    fn clone_into_box(&self) -> Box<dyn CloneableAny>;
}

impl ToOwned for dyn CloneableAny {
    type Owned = Box<dyn CloneableAny>;

    fn to_owned(&self) -> Self::Owned {
        CloneableAny::clone_into_box(self)
    }
}

// Implement CloneableAny for all 'static types implementing Clone.
impl<S: 'static + Clone + Debug> CloneableAny for S {
    fn clone_into_box(&self) -> Box<dyn CloneableAny> {
        Box::new(self.clone())
    }
}

// Adds the Downcast methods to all 'static types implementing CloneableAny.
impl_downcast!(CloneableAny);
