use crate::regex_automata::{
    dfa::search,
    util::{
        id::{PatternID, StateID},
        matchtypes::{HalfMatch, MatchError},
        prefilter,
    },
};

pub unsafe trait Automaton {
                                                                                                                                                                        fn next_state(&self, current: StateID, input: u8) -> StateID;

                                                                unsafe fn next_state_unchecked(
        &self,
        current: StateID,
        input: u8,
    ) -> StateID;

                                                                                                                                                                                                                                    fn next_eoi_state(&self, current: StateID) -> StateID;

                                                                                                                        fn start_state_forward(
        &self,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> StateID;

                                                                                                                    fn start_state_reverse(
        &self,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> StateID;

                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        fn is_special_state(&self, id: StateID) -> bool;

                                                                                                                        fn is_dead_state(&self, id: StateID) -> bool;

                                                                                                                                                fn is_quit_state(&self, id: StateID) -> bool;

                                                                                                                fn is_match_state(&self, id: StateID) -> bool;

                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            fn is_start_state(&self, id: StateID) -> bool;

                                                                                                                                                                                            fn is_accel_state(&self, id: StateID) -> bool;

                                                                                                                                                fn pattern_count(&self) -> usize;

                                                                                                                                                                                                                                                                                                        fn match_count(&self, id: StateID) -> usize;

                                                                                fn match_pattern(&self, id: StateID, index: usize) -> PatternID;

                                                                                                                                                                                                                        fn accelerator(&self, _id: StateID) -> &[u8] {
        &[]
    }

                                                                                                                                                                                                                    #[inline]
    fn find_earliest_fwd(
        &self,
        bytes: &[u8],
    ) -> Result<Option<HalfMatch>, MatchError> {
        self.find_earliest_fwd_at(None, None, bytes, 0, bytes.len())
    }

                                                                                                                                                                                                                                            #[inline]
    fn find_earliest_rev(
        &self,
        bytes: &[u8],
    ) -> Result<Option<HalfMatch>, MatchError> {
        self.find_earliest_rev_at(None, bytes, 0, bytes.len())
    }

                                                                                                                                                                                                                                                                            #[inline]
    fn find_leftmost_fwd(
        &self,
        bytes: &[u8],
    ) -> Result<Option<HalfMatch>, MatchError> {
        self.find_leftmost_fwd_at(None, None, bytes, 0, bytes.len())
    }

                                                                                                                                                                                                                                                                    #[inline]
    fn find_leftmost_rev(
        &self,
        bytes: &[u8],
    ) -> Result<Option<HalfMatch>, MatchError> {
        self.find_leftmost_rev_at(None, bytes, 0, bytes.len())
    }

                                                                                                                                                                                                                                                                #[inline]
    fn find_overlapping_fwd(
        &self,
        bytes: &[u8],
        state: &mut OverlappingState,
    ) -> Result<Option<HalfMatch>, MatchError> {
        self.find_overlapping_fwd_at(None, None, bytes, 0, bytes.len(), state)
    }

                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        #[inline]
    fn find_earliest_fwd_at(
        &self,
        pre: Option<&mut prefilter::Scanner>,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Option<HalfMatch>, MatchError> {
        search::find_earliest_fwd(pre, self, pattern_id, bytes, start, end)
    }

                                                                                                                    #[inline]
    fn find_earliest_rev_at(
        &self,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Option<HalfMatch>, MatchError> {
        search::find_earliest_rev(self, pattern_id, bytes, start, end)
    }

                                                                                                    #[inline]
    fn find_leftmost_fwd_at(
        &self,
        pre: Option<&mut prefilter::Scanner>,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Option<HalfMatch>, MatchError> {
        search::find_leftmost_fwd(pre, self, pattern_id, bytes, start, end)
    }

                                                                                                        #[inline]
    fn find_leftmost_rev_at(
        &self,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Option<HalfMatch>, MatchError> {
        search::find_leftmost_rev(self, pattern_id, bytes, start, end)
    }

                                                                                                                                                                        #[inline]
    fn find_overlapping_fwd_at(
        &self,
        pre: Option<&mut prefilter::Scanner>,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
        state: &mut OverlappingState,
    ) -> Result<Option<HalfMatch>, MatchError> {
        search::find_overlapping_fwd(
            pre, self, pattern_id, bytes, start, end, state,
        )
    }
}

unsafe impl<'a, T: Automaton> Automaton for &'a T {
    #[inline]
    fn next_state(&self, current: StateID, input: u8) -> StateID {
        (**self).next_state(current, input)
    }

    #[inline]
    unsafe fn next_state_unchecked(
        &self,
        current: StateID,
        input: u8,
    ) -> StateID {
        (**self).next_state_unchecked(current, input)
    }

    #[inline]
    fn next_eoi_state(&self, current: StateID) -> StateID {
        (**self).next_eoi_state(current)
    }

    #[inline]
    fn start_state_forward(
        &self,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> StateID {
        (**self).start_state_forward(pattern_id, bytes, start, end)
    }

    #[inline]
    fn start_state_reverse(
        &self,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> StateID {
        (**self).start_state_reverse(pattern_id, bytes, start, end)
    }

    #[inline]
    fn is_special_state(&self, id: StateID) -> bool {
        (**self).is_special_state(id)
    }

    #[inline]
    fn is_dead_state(&self, id: StateID) -> bool {
        (**self).is_dead_state(id)
    }

    #[inline]
    fn is_quit_state(&self, id: StateID) -> bool {
        (**self).is_quit_state(id)
    }

    #[inline]
    fn is_match_state(&self, id: StateID) -> bool {
        (**self).is_match_state(id)
    }

    #[inline]
    fn is_start_state(&self, id: StateID) -> bool {
        (**self).is_start_state(id)
    }

    #[inline]
    fn is_accel_state(&self, id: StateID) -> bool {
        (**self).is_accel_state(id)
    }

    #[inline]
    fn pattern_count(&self) -> usize {
        (**self).pattern_count()
    }

    #[inline]
    fn match_count(&self, id: StateID) -> usize {
        (**self).match_count(id)
    }

    #[inline]
    fn match_pattern(&self, id: StateID, index: usize) -> PatternID {
        (**self).match_pattern(id, index)
    }

    #[inline]
    fn accelerator(&self, id: StateID) -> &[u8] {
        (**self).accelerator(id)
    }

    #[inline]
    fn find_earliest_fwd(
        &self,
        bytes: &[u8],
    ) -> Result<Option<HalfMatch>, MatchError> {
        (**self).find_earliest_fwd(bytes)
    }

    #[inline]
    fn find_earliest_rev(
        &self,
        bytes: &[u8],
    ) -> Result<Option<HalfMatch>, MatchError> {
        (**self).find_earliest_rev(bytes)
    }

    #[inline]
    fn find_leftmost_fwd(
        &self,
        bytes: &[u8],
    ) -> Result<Option<HalfMatch>, MatchError> {
        (**self).find_leftmost_fwd(bytes)
    }

    #[inline]
    fn find_leftmost_rev(
        &self,
        bytes: &[u8],
    ) -> Result<Option<HalfMatch>, MatchError> {
        (**self).find_leftmost_rev(bytes)
    }

    #[inline]
    fn find_overlapping_fwd(
        &self,
        bytes: &[u8],
        state: &mut OverlappingState,
    ) -> Result<Option<HalfMatch>, MatchError> {
        (**self).find_overlapping_fwd(bytes, state)
    }

    #[inline]
    fn find_earliest_fwd_at(
        &self,
        pre: Option<&mut prefilter::Scanner>,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Option<HalfMatch>, MatchError> {
        (**self).find_earliest_fwd_at(pre, pattern_id, bytes, start, end)
    }

    #[inline]
    fn find_earliest_rev_at(
        &self,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Option<HalfMatch>, MatchError> {
        (**self).find_earliest_rev_at(pattern_id, bytes, start, end)
    }

    #[inline]
    fn find_leftmost_fwd_at(
        &self,
        pre: Option<&mut prefilter::Scanner>,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Option<HalfMatch>, MatchError> {
        (**self).find_leftmost_fwd_at(pre, pattern_id, bytes, start, end)
    }

    #[inline]
    fn find_leftmost_rev_at(
        &self,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Option<HalfMatch>, MatchError> {
        (**self).find_leftmost_rev_at(pattern_id, bytes, start, end)
    }

    #[inline]
    fn find_overlapping_fwd_at(
        &self,
        pre: Option<&mut prefilter::Scanner>,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
        state: &mut OverlappingState,
    ) -> Result<Option<HalfMatch>, MatchError> {
        (**self)
            .find_overlapping_fwd_at(pre, pattern_id, bytes, start, end, state)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OverlappingState {
                                    id: Option<StateID>,
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

    pub(crate) fn id(&self) -> Option<StateID> {
        self.id
    }

    pub(crate) fn set_id(&mut self, id: StateID) {
        self.id = Some(id);
    }

    pub(crate) fn last_match(&mut self) -> Option<&mut StateMatch> {
        self.last_match.as_mut()
    }

    pub(crate) fn set_last_match(&mut self, last_match: StateMatch) {
        self.last_match = Some(last_match);
    }
}

pub(crate) fn fmt_state_indicator<A: Automaton>(
    f: &mut core::fmt::Formatter<'_>,
    dfa: A,
    id: StateID,
) -> core::fmt::Result {
    if dfa.is_dead_state(id) {
        write!(f, "D")?;
        if dfa.is_start_state(id) {
            write!(f, ">")?;
        } else {
            write!(f, " ")?;
        }
    } else if dfa.is_quit_state(id) {
        write!(f, "Q ")?;
    } else if dfa.is_start_state(id) {
        if dfa.is_accel_state(id) {
            write!(f, "A>")?;
        } else {
            write!(f, " >")?;
        }
    } else if dfa.is_match_state(id) {
        if dfa.is_accel_state(id) {
            write!(f, "A*")?;
        } else {
            write!(f, " *")?;
        }
    } else if dfa.is_accel_state(id) {
        write!(f, "A ")?;
    } else {
        write!(f, "  ")?;
    }
    Ok(())
}
