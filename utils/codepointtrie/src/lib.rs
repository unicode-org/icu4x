// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! `icu_codepointtrie`  is one of the [`ICU4X`] components.
//!
//! This component provides a data structure for an time-efficient lookup of values
//! associated to code points.
//!
//! It is an implementation of the existing [ICU4C UCPTrie](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/ucptrie_8h.html)
//! / [ICU4J CodePointTrie](https://unicode-org.github.io/icu-docs/apidoc/dev/icu4j/) API.
//!
//! # Architecture
//!
//! Hello, architecture!...

pub mod codepointtrie;
pub mod error;
mod impl_const;
pub mod planes;
