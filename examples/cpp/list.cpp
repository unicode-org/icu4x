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

#include <array>
#include <iostream>
#include <memory>
#include <optional>
#include <string>
#include <string_view>

using namespace icu4x;

int main() {
    Logger::init_simple_logger();
    std::unique_ptr<Locale> locale = Locale::from_string("en").ok().value();
    std::unique_ptr<ListFormatter> fmt =
        ListFormatter::create_and_with_length(*locale, ListLength::Wide).ok().value();

    std::unique_ptr<Parts> parts = Parts::create();
    if (!parts->is_empty() || parts->len() != 0 || (*parts)[0].has_value()) {
        std::cout << "Expected newly created Parts to be empty" << std::endl;
        return 1;
    }

    // 1. UTF-8 `format_to_parts`
    std::array<diplomat::string_view_for_slice, 3> items = {"Alice", "Bob", "Céline"};
    std::string out = fmt->format_to_parts(items, *parts);

    if (out != "Alice, Bob, and Céline") {
        std::cout << "Unexpected UTF-8 formatted output: " << out << std::endl;
        return 1;
    }

    if (parts->is_empty() || parts->len() != 5 || (*parts)[5].has_value()) {
        std::cout << "Expected 5 UTF-8 parts, got " << parts->len() << std::endl;
        return 1;
    }

    struct ExpectedPart8 {
        size_t start;
        size_t end;
        PartKind::Value kind;
        std::string_view text;
    };

    std::array<ExpectedPart8, 5> expected = {{
        {0, 5, PartKind::ListElement, "Alice"},
        {5, 7, PartKind::ListLiteral, ", "},
        {7, 10, PartKind::ListElement, "Bob"},
        {10, 16, PartKind::ListLiteral, ", and "},
        {16, 23, PartKind::ListElement, "Céline"},
    }};

    for (size_t i = 0; i < expected.size(); ++i) {
        std::optional<PartSpan> opt_span = (*parts)[i];
        if (!opt_span.has_value()) {
            std::cout << "Missing UTF-8 part at index " << i << std::endl;
            return 1;
        }
        const PartSpan& actual = opt_span.value();
        std::string_view substr(out.data() + actual.start, actual.end - actual.start);
        if (actual.start != expected[i].start || actual.end != expected[i].end ||
            actual.kind != expected[i].kind || substr != expected[i].text) {
            std::cout << "UTF-8 mismatch at part " << i << ": got (" << actual.start << ".."
                      << actual.end << ", '" << substr << "')" << std::endl;
            return 1;
        }
    }

    // 2. UTF-16 `format_to_parts16` (reusing the same `Parts` buffer, with surrogate pair U+1F973)
    std::array<diplomat::u16string_view_for_slice, 3> items16 = {
        u"Alice", u"Bob", u"Céline \U0001F973"};
    std::u16string out16 = fmt->format_to_parts16(items16, *parts);

    if (out16 != u"Alice, Bob, and Céline \U0001F973") {
        std::cout << "Unexpected UTF-16 formatted output" << std::endl;
        return 1;
    }

    if (parts->is_empty() || parts->len() != 5 || (*parts)[5].has_value()) {
        std::cout << "Expected 5 UTF-16 parts after reuse, got " << parts->len() << std::endl;
        return 1;
    }

    struct ExpectedPart16 {
        size_t start;
        size_t end;
        PartKind::Value kind;
        std::u16string_view text;
    };

    std::array<ExpectedPart16, 5> expected16 = {{
        {0, 5, PartKind::ListElement, u"Alice"},
        {5, 7, PartKind::ListLiteral, u", "},
        {7, 10, PartKind::ListElement, u"Bob"},
        {10, 16, PartKind::ListLiteral, u", and "},
        {16, 25, PartKind::ListElement, u"Céline \U0001F973"},
    }};

    for (size_t i = 0; i < expected16.size(); ++i) {
        std::optional<PartSpan> opt_span = (*parts)[i];
        if (!opt_span.has_value()) {
            std::cout << "Missing UTF-16 part at index " << i << std::endl;
            return 1;
        }
        const PartSpan& actual = opt_span.value();
        std::u16string_view substr(out16.data() + actual.start, actual.end - actual.start);
        if (actual.start != expected16[i].start || actual.end != expected16[i].end ||
            actual.kind != expected16[i].kind || substr != expected16[i].text) {
            std::cout << "UTF-16 mismatch at part " << i << ": got (" << actual.start << ".."
                      << actual.end << ")" << std::endl;
            return 1;
        }
    }

    std::cout << "ListFormatter::format_to_parts C++ test passed: " << out << std::endl;
    return 0;
}
