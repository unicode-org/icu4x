// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "unicode/errorcode.h"
#include "unicode/brkiter.h"
#include "unicode/localpointer.h"
#include "unicode/uclean.h"

#include <iostream>
#include <string>
#include <sstream>

int main(int argc, char const *argv[]) {
    icu::ErrorCode status;
    u_init(status);

    std::basic_string<char> buffer;
    auto text = icu::UnicodeString();
    while (std::getline(std::cin, buffer)) {
        text.append(icu::UnicodeString::fromUTF8(buffer));
        text.append(u"\n");
    }

    std::cerr << "Segmenting string with " << text.length() << " UTF-16 code units" << std::endl;

    auto brkitr = icu::LocalPointer<icu::BreakIterator>(
        icu::BreakIterator::createWordInstance("de", status), status);

    for (size_t i=0; i<10000; i++) {
        brkitr->setText(text);
        while (brkitr->next() != icu::BreakIterator::DONE) {}
    }

    return 0;
}

