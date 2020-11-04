#[derive(Debug)]
pub enum Error {
    /// This error shows that if size of a data component is larger that what is should be.
    /// Example: the list of grapheme clusters must have at most std::i16::MAX number of elements
    Limit,

    /// This error shows that if there is a syntax error.
    /// Example: If there is a letter other than {b, e, i, s} in a bies sequence.
    Syntax,
}
