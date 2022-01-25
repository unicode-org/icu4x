use crate::regex_automata::{
    nfa,
    util::{
        id::{PatternID, StateID},
        start::Start,
    },
};

#[derive(Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Clone, Debug)]
enum ErrorKind {
    NFA(nfa::thompson::Error),
    Unsupported(&'static str),
    TooManyStates,
    TooManyStartStates,
    TooManyMatchPatternIDs,
    DFAExceededSizeLimit { limit: usize },
    DeterminizeExceededSizeLimit { limit: usize },
}

impl Error {
    fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub(crate) fn nfa(err: nfa::thompson::Error) -> Error {
        Error {
            kind: ErrorKind::NFA(err),
        }
    }

    pub(crate) fn unsupported_dfa_word_boundary_unicode() -> Error {
        let msg = "cannot build DFAs for regexes with Unicode word \
                   boundaries; switch to ASCII word boundaries, or \
                   heuristically enable Unicode word boundaries or use a \
                   different regex engine";
        Error {
            kind: ErrorKind::Unsupported(msg),
        }
    }

    pub(crate) fn too_many_states() -> Error {
        Error {
            kind: ErrorKind::TooManyStates,
        }
    }

    pub(crate) fn too_many_start_states() -> Error {
        Error {
            kind: ErrorKind::TooManyStartStates,
        }
    }

    pub(crate) fn too_many_match_pattern_ids() -> Error {
        Error {
            kind: ErrorKind::TooManyMatchPatternIDs,
        }
    }

    pub(crate) fn dfa_exceeded_size_limit(limit: usize) -> Error {
        Error {
            kind: ErrorKind::DFAExceededSizeLimit { limit },
        }
    }

    pub(crate) fn determinize_exceeded_size_limit(limit: usize) -> Error {
        Error {
            kind: ErrorKind::DeterminizeExceededSizeLimit { limit },
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.kind() {
            ErrorKind::NFA(ref err) => Some(err),
            ErrorKind::Unsupported(_) => None,
            ErrorKind::TooManyStates => None,
            ErrorKind::TooManyStartStates => None,
            ErrorKind::TooManyMatchPatternIDs => None,
            ErrorKind::DFAExceededSizeLimit { .. } => None,
            ErrorKind::DeterminizeExceededSizeLimit { .. } => None,
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.kind() {
            ErrorKind::NFA(_) => write!(f, "error building NFA"),
            ErrorKind::Unsupported(ref msg) => {
                write!(f, "unsupported regex feature for DFAs: {}", msg)
            }
            ErrorKind::TooManyStates => write!(
                f,
                "number of DFA states exceeds limit of {}",
                StateID::LIMIT,
            ),
            ErrorKind::TooManyStartStates => {
                let stride = Start::count();
                // The start table has `stride` entries for starting states for
                // the entire DFA, and then `stride` entries for each pattern
                // if start states for each pattern are enabled (which is the
                // only way this error can occur). Thus, the total number of
                // patterns that can fit in the table is `stride` less than
                // what we can allocate.
                let limit = ((core::isize::MAX as usize) - stride) / stride;
                write!(
                    f,
                    "compiling DFA with start states exceeds pattern \
                     pattern limit of {}",
                    limit,
                )
            }
            ErrorKind::TooManyMatchPatternIDs => write!(
                f,
                "compiling DFA with total patterns in all match states \
                 exceeds limit of {}",
                PatternID::LIMIT,
            ),
            ErrorKind::DFAExceededSizeLimit { limit } => write!(
                f,
                "DFA exceeded size limit of {:?} during determinization",
                limit,
            ),
            ErrorKind::DeterminizeExceededSizeLimit { limit } => {
                write!(f, "determinization exceeded size limit of {:?}", limit)
            }
        }
    }
}
