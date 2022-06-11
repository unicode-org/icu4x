// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/ICU4XCodePointSetData.hpp"
#include "../../include/ICU4XCodePointMapData16.hpp"

#include <iostream>

int test_set_property(ICU4XCodePointSetData data, char32_t included, char32_t excluded) {
    bool contains1 = data.contains(included);
    bool contains2 = data.contains(excluded);
    std::cout << std::hex; // print hex for U+####
    if (contains1 && !contains2) {
        std::cout << "Set correctly contains U+" << included << " and not U+" << excluded << std::endl;
    } else {
        std::cout << "Set returns wrong result on U+" << included << " or U+" << excluded << std::endl;
        return 1;
    }
    return 0;
}

int test_map_16_property(ICU4XCodePointMapData16 data, char32_t sample, uint32_t expected) {
    uint32_t actual = data.get(sample);
    std::cout << std::hex; // print hex for U+####
    if (actual == expected) {
        std::cout << "Code point U+" << sample << " correctly mapped to 0x" << actual << std::endl;
    } else {
        std::cout << "Code point U+" << sample << " incorrectly mapped to 0x" << actual << std::endl;
        return 1;
    }
    return 0;
}

int main() {
    ICU4XDataProvider dp = ICU4XDataProvider::create_test();
    int result;

    result = test_set_property(
        ICU4XCodePointSetData::try_get_ascii_hex_digit(dp).ok().value(),
        u'3',
        u'੩'
    );
    if (result != 0) {
        return result;
    }

    result = test_map_16_property(
        ICU4XCodePointMapData16::try_get_script(dp).ok().value(),
        u'木',
        17 // Script::Han
    );
    if (result != 0) {
        return result;
    }

    return 0;
}
