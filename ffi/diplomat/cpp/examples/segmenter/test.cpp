// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "../../include/ICU4XDataProvider.hpp"
#include "../../include/ICU4XLineBreakSegmenter.hpp"

#include <iostream>
#include <string_view>

using std::cout;
using std::endl;

void print_ruler(size_t str_len) {
    for (size_t i = 0; i < str_len; i++) {
        if (i % 10 == 0) {
            cout << "0";
        } else if (i % 5 == 0) {
            cout << "5";
        } else {
            cout << ".";
        }
    }
    cout << endl;
}

template <typename Iterator>
void iterate_breakpoints(Iterator& iterator) {
    while (true) {
        int32_t breakpoint = iterator.next();
        if (breakpoint == -1) {
            break;
        }
        cout << " " << breakpoint;
    }
    cout << endl;
}

void test_line(const std::string_view& str) {
    const auto provider = ICU4XDataProvider::create_test();
    const auto segmenter = ICU4XLineBreakSegmenter::try_new(provider).ok().value();
    cout << "Finding line breakpoints in string:" << endl
         << str << endl;
    print_ruler(str.size());

    cout << "Line breakpoints:";
    auto iterator = segmenter.segment_utf8(str);
    iterate_breakpoints(iterator);
}

int main(int argc, char* argv[]) {
    std::string_view str;
    if (argc >= 2) {
        str = argv[1];
    } else {
        str = "The quick brown fox jumps over the lazy dog.";
    }

    test_line(str);
    cout << endl;

    return 0;
}
