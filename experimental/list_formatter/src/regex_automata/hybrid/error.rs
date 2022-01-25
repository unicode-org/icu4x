use crate::regex_automata::{hybrid::id::LazyStateIDError, nfa};

#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}

#[derive(Clone, Debug)]
enum BuildErrorKind {
    NFA(nfa::thompson::Error),
    InsufficientCacheCapacity { minimum: usize, given: usize },
    InsufficientStateIDCapacity { err: LazyStateIDError },
    Unsupported(&'static str),
}

impl BuildError {
    fn kind(&self) -> &BuildErrorKind {
        &self.kind
    }

    pub(crate) fn nfa(err: nfa::thompson::Error) -> BuildError {
        BuildError { kind: BuildErrorKind::NFA(err) }
    }

    pub(crate) fn insufficient_cache_capacity(
        minimum: usize,
        given: usize,
    ) -> BuildError {
        BuildError {
            kind: BuildErrorKind::InsufficientCacheCapacity { minimum, given },
        }
    }

    pub(crate) fn insufficient_state_id_capacity(
        err: LazyStateIDError,
    ) -> BuildError {
        BuildError {
            kind: BuildErrorKind::InsufficientStateIDCapacity { err },
        }
    }

    pub(crate) fn unsupported_dfa_word_boundary_unicode() -> BuildError {
        let msg = "cannot build lazy DFAs for regexes with Unicode word \
                   boundaries; switch to ASCII word boundaries, or \
                   heuristically enable Unicode word boundaries or use a \
                   different regex engine";
        BuildError { kind: BuildErrorKind::Unsupported(msg) }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.kind() {
            BuildErrorKind::NFA(ref err) => Some(err),
            BuildErrorKind::InsufficientCacheCapacity { .. } => None,
            // LazyStateIDError is an implementation detail, don't expose it.
            BuildErrorKind::InsufficientStateIDCapacity { .. } => None,
            BuildErrorKind::Unsupported(_) => None,
        }
    }
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.kind() {
            BuildErrorKind::NFA(_) => write!(f, "error building NFA"),
            BuildErrorKind::InsufficientCacheCapacity { minimum, given } => {
                write!(
                    f,
                    "given cache capacity ({}) is smaller than \
                     minimum required ({})",
                    given, minimum,
                )
            }
            BuildErrorKind::InsufficientStateIDCapacity { ref err } => {
                err.fmt(f)
            }
            BuildErrorKind::Unsupported(ref msg) => {
                write!(f, "unsupported regex feature for DFAs: {}", msg)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct CacheError(());

impl CacheError {
    pub(crate) fn too_many_cache_clears() -> CacheError {
        CacheError(())
    }
}

#[cfg(feature = "std")]
impl std::error::Error for CacheError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl core::fmt::Display for CacheError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "lazy DFA cache has been cleared too many times")
    }
}
