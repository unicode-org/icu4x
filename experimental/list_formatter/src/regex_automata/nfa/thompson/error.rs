use crate::regex_automata::util::id::{PatternID, StateID};

#[derive(Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Clone, Debug)]
enum ErrorKind {
    Syntax(regex_syntax::Error),
    TooManyPatterns { given: usize, limit: usize },
    TooManyStates { given: usize, limit: usize },
    ExceededSizeLimit { limit: usize },
    UnicodeWordUnavailable,
}

impl Error {
    fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub(crate) fn syntax(err: regex_syntax::Error) -> Error {
        Error {
            kind: ErrorKind::Syntax(err),
        }
    }

    pub(crate) fn too_many_patterns(given: usize) -> Error {
        let limit = PatternID::LIMIT;
        Error {
            kind: ErrorKind::TooManyPatterns { given, limit },
        }
    }

    pub(crate) fn too_many_states(given: usize) -> Error {
        let limit = StateID::LIMIT;
        Error {
            kind: ErrorKind::TooManyStates { given, limit },
        }
    }

    pub(crate) fn exceeded_size_limit(limit: usize) -> Error {
        Error {
            kind: ErrorKind::ExceededSizeLimit { limit },
        }
    }

    pub(crate) fn unicode_word_unavailable() -> Error {
        Error {
            kind: ErrorKind::UnicodeWordUnavailable,
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.kind() {
            ErrorKind::Syntax(ref err) => Some(err),
            ErrorKind::TooManyPatterns { .. } => None,
            ErrorKind::TooManyStates { .. } => None,
            ErrorKind::ExceededSizeLimit { .. } => None,
            ErrorKind::UnicodeWordUnavailable => None,
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.kind() {
            ErrorKind::Syntax(_) => write!(f, "error parsing regex"),
            ErrorKind::TooManyPatterns { given, limit } => write!(
                f,
                "attemped to compile {} patterns, \
                 which exceeds the limit of {}",
                given, limit,
            ),
            ErrorKind::TooManyStates { given, limit } => write!(
                f,
                "attemped to compile {} NFA states, \
                 which exceeds the limit of {}",
                given, limit,
            ),
            ErrorKind::ExceededSizeLimit { limit } => write!(
                f,
                "heap usage during NFA compilation exceeded limit of {:?}",
                limit,
            ),
            ErrorKind::UnicodeWordUnavailable => write!(
                f,
                "crate has been compiled without Unicode word boundary \
                 support, but the NFA contains Unicode word boundary \
                 assertions",
            ),
        }
    }
}
