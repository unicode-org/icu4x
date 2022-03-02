// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/ICU4XLineBreakSegmenter.hpp"
#include "../../include/ICU4XDataProvider.hpp"

#include <iostream>
#include <string_view>

const std::string_view path = "../../../../../provider/testdata/data/json/";

int main(int argc, char* argv[]) {
    ICU4XDataProvider provider = ICU4XDataProvider::create_fs(path).provider.value();
    ICU4XLineBreakSegmenter segmenter = ICU4XLineBreakSegmenter::try_new(provider).ok().value();

    std::string_view str;
    if (argc >= 2) {
        str = argv[1];
    } else {
        str = "The quick brown fox jumps over the lazy dog.";
    }

    std::cout << "Segmenting string:" << std::endl << str << std::endl;
    for (size_t i=0; i<str.size(); i++) {
        if (i % 10 == 0) {
            std::cout << "0";
        } else if (i % 5 == 0) {
            std::cout << "5";
        } else {
            std::cout << ".";
        }
    }
    std::cout << std::endl;

    ICU4XLineBreakIteratorUtf8 iterator = segmenter.segment_utf8(str);

    std::cout << "Breakpoints:";
    while (true) {
        int32_t breakpoint = iterator.next();
        if (breakpoint == -1) {
            break;
        }
        std::cout << " " << breakpoint;
    }
    std::cout << std::endl;

    return 0;
}
