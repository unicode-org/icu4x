// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "unicode/errorcode.h"
#include "unicode/ucptrie.h"
#include <memory>

extern "C" UErrorCode* create_uerrorcode() {
    return (UErrorCode*) calloc(1, sizeof(UErrorCode));
}

extern "C" UErrorCode read_uerrorcode(UErrorCode* ptr) {
    return *ptr;
}

extern "C" uint32_t read_ucptrie_highStart(UCPTrie* ptr) {
    return ptr->highStart;
}

extern "C" uint32_t read_ucptrie_shifted12HighStart(UCPTrie* ptr) {
    return ptr->shifted12HighStart;
}

extern "C" uint32_t read_ucptrie_index3NullOffset(UCPTrie* ptr) {
    return ptr->index3NullOffset;
}

extern "C" uint32_t read_ucptrie_dataNullOffset(UCPTrie* ptr) {
    return ptr->dataNullOffset;
}

extern "C" uint32_t read_ucptrie_nullValue(UCPTrie* ptr) {
    return ptr->nullValue;
}

extern "C" const uint16_t* get_index_ptr(UCPTrie* ptr) {
    return ptr->index;
}

extern "C" uint32_t get_index_length(UCPTrie* ptr) {
    return ptr->indexLength;
}

extern "C" const void* get_data_ptr(UCPTrie* ptr) {
    // UCPTrieData is a union over different pointer types.
    // We don't care which one here.
    return ptr->data.ptr0;
}

extern "C" uint32_t get_data_length(UCPTrie* ptr) {
    return ptr->dataLength;
}
