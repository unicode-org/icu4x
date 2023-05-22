// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include <iostream>

#include "unicode/errorcode.h"
#include "unicode/umutablecptrie.h"
#include "writesrc.h"

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
int construct_ucptrie(const int32_t defaultValue, const int32_t errorValue,
                      const UCPTrieType trieType,
                      const UCPTrieValueWidth valueWidth,
                      const uint32_t* values, const uint32_t n) {
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

int main() { return 0; }
