// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "unicode/umutablecptrie.h"
#include "unicode/errorcode.h"
#include "writesrc.h"

#include <iostream>

inline std::int32_t read_int32_t() {
    std::int32_t x(0);
    char ch;
	int sgn = 1;
	while (!isdigit(ch = getchar_unlocked())) {
		if (ch == '-') {
			sgn *= -1;
		}
	}
	x = ch - '0';
	while (isdigit(ch = getchar_unlocked())) {
		x = x * 10 + (ch - '0');
	}
	return x * sgn;
}

/**
 * list_to_ucptrie: converts a stream of integers to a UCPTrie
 *
 * - standard input should be a stream of integers, each integer representing the value for
     consecutive code points, starting from code point 0.
 * - standard output is the build UCPTrie as a TOML file
 */
int main(int argc, char const *argv[]) {
    if (argc != 5) {
        std::cerr << "Takes 4 positional arguments: default value, error value, trie type, and value width" << std::endl;
        return 1;
    }

    int32_t defaultValue = atoi(argv[1]);
    int32_t errorValue = atoi(argv[2]);

    UCPTrieType trieType;
    std::string trieTypeStr(argv[3]);
    if (trieTypeStr == "small") {
        trieType = UCPTRIE_TYPE_SMALL;
    } else if (trieTypeStr == "fast") {
        trieType = UCPTRIE_TYPE_FAST;
    } else {
        std::cerr << "Expected 'small' or 'fast' for 3rd argument" << std::endl;
        return 1;
    }

    UCPTrieValueWidth valueWidth;
    int32_t valueWidthInt = atoi(argv[4]);
    if (valueWidthInt == 8) {
        valueWidth = UCPTRIE_VALUE_BITS_8;
    } else if (valueWidthInt == 16) {
        valueWidth = UCPTRIE_VALUE_BITS_16;
    } else if (valueWidthInt == 32) {
        valueWidth = UCPTRIE_VALUE_BITS_32;
    } else {
        std::cerr << "Expected '8', '16', or '32' for 4th argument" << std::endl;
        return 1;
    }

    icu::ErrorCode status;

    icu::LocalUMutableCPTriePointer builder(umutablecptrie_open(
        defaultValue,
        errorValue,
        status));
    if (status.isFailure()) {
        std::cerr << status.errorName() << std::endl;
        return 1;
    }

    UChar32 cp = 0;
    int32_t n = read_int32_t();
    int32_t value = 0;
    while(n--) {
        value = read_int32_t();
        umutablecptrie_set(builder.getAlias(), cp, value, status);
        if (status.isFailure()) {
            std::cerr << status.errorName() << std::endl;
            return 1;
        }
        cp++;
    }
    std::cerr << "Saved values for code points up to " << cp << std::endl;

    icu::LocalUCPTriePointer utrie(umutablecptrie_buildImmutable(
        builder.getAlias(),
        trieType,
        valueWidth,
        status));
    if (status.isFailure()) {
        std::cerr << status.errorName() << std::endl;
        return 1;
    }

    // Note: writesrc.h appears to only print LF, not CRLF
    std::cout << "# This file is part of ICU4X. For terms of use, please see the file\n";
    std::cout << "# called LICENSE at the top level of the ICU4X source tree\n";
    std::cout << "# (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).\n";
    std::cout << "\n";
    std::cout << "# This file is auto-generated. Instructions: tools/list_to_ucptrie/README.md\n";
    std::cout << "\n";
    usrc_writeUCPTrie(stdout, "<unused>", utrie.getAlias(), UPRV_TARGET_SYNTAX_TOML);

    return 0;
}
