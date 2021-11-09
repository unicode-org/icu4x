// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/ICU4XUnicodeSetProperty.hpp"

#include <iostream>

const std::string_view path = "../../../../../provider/testdata/data/json/";

int test_set_property(ICU4XUnicodeSetPropertyResult result, char32_t included, char32_t excluded) {
    if (!result.success) {
        std::cout << "Failed to create ICU4XUnicodeSetProperty" << std::endl;
        return 1;
    }
    bool contains1 = result.data.value().contains(included);
    bool contains2 = result.data.value().contains(excluded);
    std::cout << std::hex; // print hex for U+####
    if (contains1 && !contains2) {
        std::cout << "Set correctly contains U+" << included << " and not U+" << excluded << std::endl;
    } else {
        std::cout << "Set returns wrong result on U+" << included << " or U+" << excluded << std::endl;
        return 1;
    }
    return 0;
}

int main() {
    ICU4XDataProvider dp = ICU4XDataProvider::create_fs(path).provider.value();
    int result;

    result = test_set_property(
        ICU4XUnicodeSetProperty::try_get_ascii_hex_digit(dp),
        u'3',
        u'੩'
    );
    if (result != 0) {
        return result;
    }

    return 0;
}
