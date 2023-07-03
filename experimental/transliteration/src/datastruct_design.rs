// This file just serves as a API-only design of the transliterator datastruct

use zerovec::*;

struct Transliterator {
    rules: VarZeroVec<str>,
}