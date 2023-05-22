// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include <iostream>

#include "unicode/errorcode.h"
#include "unicode/umutablecptrie.h"
#include "writesrc.h"

/**
 * list_to_ucptrie: converts a stream of integers to a UCPTrie
 *
 * - standard input should be a stream of integers, each integer representing
 the value for consecutive code points, starting from code point 0.
 * - standard output is the build UCPTrie as a TOML file
 */

extern "C" {
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
