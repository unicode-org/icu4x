#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord,
)]
pub struct LazyStateID(u32);

impl LazyStateID {
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    const MAX_BIT: usize = 31;

    #[cfg(target_pointer_width = "16")]
    const MAX_BIT: usize = 15;

    const MASK_UNKNOWN: usize = 1 << (LazyStateID::MAX_BIT);
    const MASK_DEAD: usize = 1 << (LazyStateID::MAX_BIT - 1);
    const MASK_QUIT: usize = 1 << (LazyStateID::MAX_BIT - 2);
    const MASK_START: usize = 1 << (LazyStateID::MAX_BIT - 3);
    const MASK_MATCH: usize = 1 << (LazyStateID::MAX_BIT - 4);
    const MAX: usize = LazyStateID::MASK_MATCH - 1;

                    #[inline]
    pub(crate) fn new(id: usize) -> Result<LazyStateID, LazyStateIDError> {
        if id > LazyStateID::MAX {
            return Err(LazyStateIDError { attempted: id as u64 });
        }
        Ok(LazyStateID::new_unchecked(id))
    }

                        #[inline]
    const fn new_unchecked(id: usize) -> LazyStateID {
        LazyStateID(id as u32)
    }

            #[inline]
    pub(crate) fn as_usize(&self) -> Option<usize> {
        if self.is_tagged() {
            None
        } else {
            Some(self.as_usize_unchecked())
        }
    }

                        #[inline]
    pub(crate) fn as_usize_untagged(&self) -> usize {
        self.as_usize_unchecked() & LazyStateID::MAX
    }

            #[inline]
    pub(crate) const fn as_usize_unchecked(&self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub(crate) const fn to_unknown(&self) -> LazyStateID {
        LazyStateID::new_unchecked(
            self.as_usize_unchecked() | LazyStateID::MASK_UNKNOWN,
        )
    }

    #[inline]
    pub(crate) const fn to_dead(&self) -> LazyStateID {
        LazyStateID::new_unchecked(
            self.as_usize_unchecked() | LazyStateID::MASK_DEAD,
        )
    }

    #[inline]
    pub(crate) const fn to_quit(&self) -> LazyStateID {
        LazyStateID::new_unchecked(
            self.as_usize_unchecked() | LazyStateID::MASK_QUIT,
        )
    }

            #[inline]
    pub(crate) const fn to_start(&self) -> LazyStateID {
        LazyStateID::new_unchecked(
            self.as_usize_unchecked() | LazyStateID::MASK_START,
        )
    }

            #[inline]
    pub(crate) const fn to_match(&self) -> LazyStateID {
        LazyStateID::new_unchecked(
            self.as_usize_unchecked() | LazyStateID::MASK_MATCH,
        )
    }

                    #[inline]
    pub const fn is_tagged(&self) -> bool {
        self.as_usize_unchecked() > LazyStateID::MAX
    }

                    #[inline]
    pub const fn is_unknown(&self) -> bool {
        self.as_usize_unchecked() & LazyStateID::MASK_UNKNOWN > 0
    }

                    #[inline]
    pub const fn is_dead(&self) -> bool {
        self.as_usize_unchecked() & LazyStateID::MASK_DEAD > 0
    }

                            #[inline]
    pub const fn is_quit(&self) -> bool {
        self.as_usize_unchecked() & LazyStateID::MASK_QUIT > 0
    }

            #[inline]
    pub const fn is_start(&self) -> bool {
        self.as_usize_unchecked() & LazyStateID::MASK_START > 0
    }

            #[inline]
    pub const fn is_match(&self) -> bool {
        self.as_usize_unchecked() & LazyStateID::MASK_MATCH > 0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LazyStateIDError {
    attempted: u64,
}

impl LazyStateIDError {
        pub(crate) fn attempted(&self) -> u64 {
        self.attempted
    }
}

#[cfg(feature = "std")]
impl std::error::Error for LazyStateIDError {}

impl core::fmt::Display for LazyStateIDError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "failed to create LazyStateID from {:?}, which exceeds {:?}",
            self.attempted(),
            LazyStateID::MAX,
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OverlappingState {
                                    id: Option<LazyStateID>,
            last_match: Option<StateMatch>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct StateMatch {
        pub(crate) match_index: usize,
                                pub(crate) offset: usize,
}

impl OverlappingState {
            pub fn start() -> OverlappingState {
        OverlappingState { id: None, last_match: None }
    }

    pub(crate) fn id(&self) -> Option<LazyStateID> {
        self.id
    }

    pub(crate) fn set_id(&mut self, id: LazyStateID) {
        self.id = Some(id);
    }

    pub(crate) fn last_match(&mut self) -> Option<&mut StateMatch> {
        self.last_match.as_mut()
    }

    pub(crate) fn set_last_match(&mut self, last_match: StateMatch) {
        self.last_match = Some(last_match);
    }
}
