// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include <icu4x/ListFormatter.hpp>
#include <icu4x/ListLength.hpp>
#include <icu4x/Locale.hpp>
#include <icu4x/Logger.hpp>
#include <icu4x/PartKind.hpp>
#include <icu4x/PartSpan.hpp>
#include <icu4x/Parts.hpp>
#include <icu4x/PartsSink.hpp>

#include <array>
#include <iostream>
#include <memory>
#include <string>
#include <string_view>

using namespace icu4x;

int main() {
    Logger::init_simple_logger();
    std::unique_ptr<Locale> locale = Locale::from_string("en").ok().value();
    std::unique_ptr<ListFormatter> fmt =
        ListFormatter::create_and_with_length(*locale, ListLength::Wide).ok().value();

    std::array<diplomat::string_view_for_slice, 3> items = {"Alice", "Bob", "Céline"};
    std::unique_ptr<PartsSink> sink = PartsSink::create();

    std::string out = fmt->format_to_parts(items, *sink);
    std::unique_ptr<Parts> parts = sink->drain();

    if (out != "Alice, Bob, and Céline") {
        std::cout << "Unexpected formatted output: " << out << std::endl;
        return 1;
    }

    icu4x::diplomat::span<const PartSpan> slice = parts->as_slice();
    if (slice.size() != 5) {
        std::cout << "Expected 5 parts, got " << slice.size() << std::endl;
        return 1;
    }

    struct ExpectedPart {
        size_t start;
        size_t end;
        PartKind::Value kind;
        std::string_view text;
    };

    std::array<ExpectedPart, 5> expected = {{
        {0, 5, PartKind::ListElement, "Alice"},
        {5, 7, PartKind::ListLiteral, ", "},
        {7, 10, PartKind::ListElement, "Bob"},
        {10, 16, PartKind::ListLiteral, ", and "},
        {16, 23, PartKind::ListElement, "Céline"},
    }};

    for (size_t i = 0; i < expected.size(); ++i) {
        const PartSpan& actual = slice.data()[i];
        std::string_view substr(out.data() + actual.start, actual.end - actual.start);
        if (actual.start != expected[i].start || actual.end != expected[i].end ||
            actual.kind != expected[i].kind || substr != expected[i].text) {
            std::cout << "Mismatch at part " << i << ": got (" << actual.start << ".."
                      << actual.end << ", '" << substr << "')" << std::endl;
            return 1;
        }
    }

    std::cout << "ListFormatter::format_to_parts C++ test passed: " << out << std::endl;
    return 0;
}
