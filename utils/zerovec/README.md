# ZeroVec

This crate provides `ZeroVec` and `VarZeroVec`, zero-copy deserializable counterparts for `Vec<T: Sized>` and `Vec<T: !Sized>` respectively. In general, `ZeroVec` can be used with integer-like types, and `VarZeroVec` can be used with `str`/`String`.