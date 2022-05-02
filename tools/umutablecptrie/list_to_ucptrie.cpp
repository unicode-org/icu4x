#include "unicode/umutablecptrie.h"
#include "unicode/errorcode.h"
#include "writesrc.h"

#include <iostream>

/**
 * - standard input should be a stream of integers, in order of each code point.
 * - standard output is the build UCPTrie as a TOML file
 *
 * Usage:
 *
 * python3 -c 'import json; print("\n".join(str(l) for l in json.load(open("provider/testdata/data/json/segmenter/line@1/und.json"))["property_table"]))' | tools/umutablecptrie/list_to_ucptrie 0 0 1> provider/datagen/data/line_cptrie.toml
 */
int main(int argc, char const *argv[]) {
    if (argc != 4) {
        std::cerr << "Takes 3 positional arguments: default value, error value, and trie type" << std::endl;
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
    std::string line;
    while (std::getline(std::cin, line)) {
        int32_t value = atoi(line.data());
        // std::cerr << cp << " => " << value << std::endl;
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
        UCPTRIE_VALUE_BITS_8, // TODO: Auto-compute
        status));
    if (status.isFailure()) {
        std::cerr << status.errorName() << std::endl;
        return 1;
    }

    usrc_writeUCPTrie(stdout, "<unused>", utrie.getAlias(), UPRV_TARGET_SYNTAX_TOML);

    return 0;
}
