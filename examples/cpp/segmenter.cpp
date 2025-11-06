// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include <icu4x/GraphemeClusterSegmenter.hpp>
#include <icu4x/LineSegmenter.hpp>
#include <icu4x/SentenceSegmenter.hpp>
#include <icu4x/WordSegmenter.hpp>
#include <icu4x/Logger.hpp>

#include <iostream>
#include <string_view>

using namespace icu4x;

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
    const auto segmenter_auto =
        LineSegmenter::create_auto();
    const auto segmenter_lstm =
        LineSegmenter::create_lstm();
    const auto segmenter_dictionary =
        LineSegmenter::create_dictionary();

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
    const auto segmenter = GraphemeClusterSegmenter::create();
    cout << "Finding grapheme cluster breakpoints in string:" << endl
         << str << endl;
    print_ruler(str.size());

    cout << "Grapheme cluster breakpoints:";
    auto iterator = segmenter->segment(str);
    iterate_breakpoints(*iterator.get());
}

void test_word(const std::string_view& str) {
    const auto segmenter_auto =
        WordSegmenter::create_auto();
    const auto segmenter_lstm =
        WordSegmenter::create_lstm();
    const auto segmenter_dictionary =
        WordSegmenter::create_dictionary();

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

void test_word_with_options(const std::string_view& str) {
    std::unique_ptr<Locale> locale = Locale::from_string("sv").ok().value();
    const auto segmenter_auto =
        WordSegmenter::create_auto_with_content_locale(*locale.get()).ok().value();
    const auto segmenter_lstm =
        WordSegmenter::create_lstm_with_content_locale(*locale.get()).ok().value();
    const auto segmenter_dictionary =
        WordSegmenter::create_dictionary_with_content_locale(*locale.get()).ok().value();

    const WordSegmenter* segmenters[] = {segmenter_auto.get(), segmenter_lstm.get(),
                                              segmenter_dictionary.get()};
    for (const auto* segmenter : segmenters) {
        cout << "Finding word breakpoints for sv in string:" << endl << str << endl;
        print_ruler(str.size());

        cout << "Word breakpoints:";
        auto iterator = segmenter->segment(str);
        iterate_word_breakpoints(*iterator.get());
    }
}

void test_sentence(const std::string_view& str) {
    const auto segmenter = SentenceSegmenter::create();
    cout << "Finding sentence breakpoints in string:" << endl
         << str << endl;
    print_ruler(str.size());

    cout << "Sentence breakpoints:";
    auto iterator = segmenter->segment(str);
    iterate_breakpoints(*iterator.get());
}

void test_sentence_with_options(const std::string_view& str) {
    std::unique_ptr<Locale> locale = Locale::from_string("el").ok().value();
    const auto segmenter =
        SentenceSegmenter::create_with_content_locale(*locale.get()).ok().value();
    cout << "Finding sentence breakpoints for el in string:" << endl
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

    test_word_with_options(str);
    cout << endl;

    test_sentence(str);
    cout << endl;

    test_sentence_with_options(str);
    cout << endl;
    return 0;
}
