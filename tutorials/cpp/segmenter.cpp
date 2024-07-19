// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "DataProvider.hpp"
#include "GraphemeClusterSegmenter.hpp"
#include "LineSegmenter.hpp"
#include "SentenceSegmenter.hpp"
#include "WordSegmenter.hpp"
#include "Logger.hpp"

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

template <typename Iterator>
void iterate_word_breakpoints(Iterator& iterator) {
    while (true) {
        int32_t breakpoint = iterator.next();
        if (breakpoint == -1) {
            break;
        }
        cout << " " << breakpoint;
        switch (iterator.word_type()) {
            case SegmenterWordType::Value::None:
                cout << " (none";
                break;
            case SegmenterWordType::Value::Number:
                cout << " (number";
                break;
            case SegmenterWordType::Value::Letter:
                cout << " (letter";
                break;
            default:
                cout << " (unknown status";
                break;
        }
        if (iterator.is_word_like()) {
            cout << ", word-like";
        }
        cout << ")";
    }
    cout << endl;
}

void test_line(const std::string_view& str) {
    const auto provider = DataProvider::compiled();
    const auto segmenter_auto =
        LineSegmenter::create_auto(*provider.get()).ok().value();
    const auto segmenter_lstm =
        LineSegmenter::create_lstm(*provider.get()).ok().value();
    const auto segmenter_dictionary =
        LineSegmenter::create_dictionary(*provider.get()).ok().value();

    const LineSegmenter* segmenters[] = {segmenter_auto.get(), segmenter_lstm.get(),
                                              segmenter_dictionary.get()};
    for (const auto* segmenter : segmenters) {
        cout << "Finding line breakpoints in string:" << endl << str << endl;
        print_ruler(str.size());

        cout << "Line breakpoints:";
        auto iterator = segmenter->segment(str);
        iterate_breakpoints(*iterator.get());
    }
}

void test_grapheme(const std::string_view& str) {
    const auto provider = DataProvider::compiled();
    const auto segmenter = GraphemeClusterSegmenter::create(*provider.get()).ok().value();
    cout << "Finding grapheme cluster breakpoints in string:" << endl
         << str << endl;
    print_ruler(str.size());

    cout << "Grapheme cluster breakpoints:";
    auto iterator = segmenter->segment(str);
    iterate_breakpoints(*iterator.get());
}

void test_word(const std::string_view& str) {
    const auto provider = DataProvider::compiled();
    const auto segmenter_auto =
        WordSegmenter::create_auto(*provider.get()).ok().value();
    const auto segmenter_lstm =
        WordSegmenter::create_lstm(*provider.get()).ok().value();
    const auto segmenter_dictionary =
        WordSegmenter::create_dictionary(*provider.get()).ok().value();

    const WordSegmenter* segmenters[] = {segmenter_auto.get(), segmenter_lstm.get(),
                                              segmenter_dictionary.get()};
    for (const auto* segmenter : segmenters) {
        cout << "Finding word breakpoints in string:" << endl << str << endl;
        print_ruler(str.size());

        cout << "Word breakpoints:";
        auto iterator = segmenter->segment(str);
        iterate_word_breakpoints(*iterator.get());
    }
}

void test_sentence(const std::string_view& str) {
    const auto provider = DataProvider::compiled();
    const auto segmenter = SentenceSegmenter::create(*provider.get()).ok().value();
    cout << "Finding sentence breakpoints in string:" << endl
         << str << endl;
    print_ruler(str.size());

    cout << "Sentence breakpoints:";
    auto iterator = segmenter->segment(str);
    iterate_breakpoints(*iterator.get());
}

int main(int argc, char* argv[]) {
    Logger::init_simple_logger();
    std::string_view str;
    if (argc >= 2) {
        str = argv[1];
    } else {
        str = "The 101 quick brown foxes jump over the lazy dog.";
    }

    test_line(str);
    cout << endl;

    test_grapheme(str);
    cout << endl;

    test_word(str);
    cout << endl;

    test_sentence(str);
    cout << endl;
    return 0;
}
