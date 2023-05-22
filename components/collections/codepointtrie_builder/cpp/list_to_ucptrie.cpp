// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include <iostream>

#include "unicode/errorcode.h"
#include "unicode/umutablecptrie.h"
#include "writesrc.h"

constexpr uint32_t TRIE_FAST = 0;
constexpr uint32_t TRIE_SMALL = 1;

extern "C" {
/**
 * Construct a UCPTrie and write it to STDOUT in TOML.
 *
 * Parameters:
 * defaultValue: The initial value set for all codepoints.
 * errorValue: The value for out of range codepoints.
 * trieType: The type of UCPTrie. Can be '0' for FAST, '1' for SMALL.
 * valueWidth: The width of the values. Can be '0', '1', '2' for 16,
 *  32, 8 bits respectively.
 * values: A pointer to an array containing the value for each codepoint.
 * n: The length of values array.
 */
int construct_ucptrie(const uint32_t defaultValue, const uint32_t errorValue,
                      const uint32_t trieTypeUint,
                      const uint32_t valueWidthUint, const uint32_t* values,
                      const uint32_t n) {
  UCPTrieType trieType;
  if (trieTypeUint == TRIE_FAST) {
    trieType = UCPTRIE_TYPE_FAST;
  } else if (trieTypeUint == TRIE_SMALL) {
    trieType = UCPTRIE_TYPE_SMALL;
  } else {
    fprintf(stderr,
            "Expected '0' for 'fast' or '1' for 'small' as the 3rd argument. "
            "Got %d",
            trieTypeUint);
    return 1;
  }

  UCPTrieValueWidth valueWidth;
  if (valueWidthUint == 0) {
    valueWidth = UCPTRIE_VALUE_BITS_16;
  } else if (valueWidthUint == 1) {
    valueWidth = UCPTRIE_VALUE_BITS_32;
  } else if (valueWidthUint == 2) {
    valueWidth = UCPTRIE_VALUE_BITS_8;
  } else {
    fprintf(stderr,
            "Expected '0' for 16 bits, '1' for 32 bits, or '2' for 8 bits as "
            "the 4th argument. Got %d",
            valueWidthUint);
    return 1;
  }

  icu::ErrorCode status;

  icu::LocalUMutableCPTriePointer builder = icu::LocalUMutableCPTriePointer(
      umutablecptrie_open(defaultValue, errorValue, status));
  if (status.isFailure()) {
    fprintf(stderr, "LocalUMutableCPTriePointer builder failed: %s",
            status.errorName());
    return 1;
  }

  for (int i = 0; i < n; ++i) {
    umutablecptrie_set(builder.getAlias(), i, values[i], status);
    if (status.isFailure()) {
      fprintf(stderr, "Unable to set %d to value: %d", i, values[i]);
      return 1;
    }
  }
  icu::LocalUCPTriePointer utrie =
      icu::LocalUCPTriePointer(umutablecptrie_buildImmutable(
          builder.getAlias(), trieType, valueWidth, status));
  if (status.isFailure()) {
    fprintf(stderr,
            "LocalUCPTriePointer umutablecptrie_buildImmutable failed: %s",
            status.errorName());
    return 1;
  }

  // Note: writesrc.h appears to only print LF, not CRLF
  usrc_writeUCPTrie(stdout, "<unused>", utrie.getAlias(),
                    UPRV_TARGET_SYNTAX_TOML);
  return 0;
}
}
