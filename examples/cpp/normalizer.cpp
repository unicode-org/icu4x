// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include <icu4x/ComposingNormalizer.hpp>
#include <icu4x/DecomposingNormalizer.hpp>
#include <icu4x/Logger.hpp>

#include <iostream>

using namespace icu4x;

int main() {
    Logger::init_simple_logger();

    std::unique_ptr<ComposingNormalizer> nfc = ComposingNormalizer::create_nfc();
    std::unique_ptr<DecomposingNormalizer> nfd = DecomposingNormalizer::create_nfd();

    std::string decomposed_utf8 = "a\u0308\u1100\u1161\U0001F60A";
    std::string composed_utf8 = "\u00E4\uAC00\U0001F60A";

    if (nfc->normalize(decomposed_utf8) != composed_utf8) {
        std::cout << "UTF-8 NFC output did not match expected" << std::endl;
        return 1;
    }
    if (nfd->normalize(composed_utf8) != decomposed_utf8) {
        std::cout << "UTF-8 NFD output did not match expected" << std::endl;
        return 1;
    }

    std::u16string decomposed_utf16 = u"a\u0308\u1100\u1161\U0001F60A";
    std::u16string composed_utf16 = u"\u00E4\uAC00\U0001F60A";

    if (nfc->is_normalized16(decomposed_utf16) || !nfc->is_normalized16(composed_utf16)) {
        std::cout << "UTF-16 NFC is_normalized16 did not match expected" << std::endl;
        return 1;
    }
    if (nfc->normalize16(decomposed_utf16) != composed_utf16) {
        std::cout << "UTF-16 NFC normalize16 output did not match expected" << std::endl;
        return 1;
    }
    if (nfd->is_normalized16(composed_utf16) || !nfd->is_normalized16(decomposed_utf16)) {
        std::cout << "UTF-16 NFD is_normalized16 did not match expected" << std::endl;
        return 1;
    }
    if (nfd->normalize16(composed_utf16) != decomposed_utf16) {
        std::cout << "UTF-16 NFD normalize16 output did not match expected" << std::endl;
        return 1;
    }

    return 0;
}
