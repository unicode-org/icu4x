// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include <icu4x/CaseMapper.hpp>
#include <icu4x/Logger.hpp>
#include <icu4x/CodePointSetBuilder.hpp>
#include <icu4x/TitlecaseOptionsV1.hpp>

#include <iostream>

using namespace icu4x;

int main() {
    Logger::init_simple_logger();
    std::unique_ptr<Locale> und = Locale::from_string("und").ok().value();
    std::unique_ptr<Locale> greek = Locale::from_string("el").ok().value();
    std::unique_ptr<Locale> turkish = Locale::from_string("tr").ok().value();

    std::unique_ptr<CaseMapper> cm = CaseMapper::create();

    TitlecaseOptionsV1 tc_options = TitlecaseOptionsV1::default_options();

    std::string out = cm->lowercase("hEllO WorLd", *und.get()).ok().value();
    std::cout << "Lowercased value is " << out << std::endl;
    if (out != "hello world") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }
    out = cm->uppercase("hEllO WorLd", *und.get()).ok().value();
    std::cout << "Uppercased value is " << out << std::endl;
    if (out != "HELLO WORLD") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    out = cm->titlecase_segment_with_only_case_data_v1("hEllO WorLd", *und.get(), tc_options).ok().value();
    std::cout << "Titlecased value is " << out << std::endl;
    if (out != "Hello world") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }


    // locale-specific behavior

    out = cm->uppercase("Γειά σου Κόσμε", *und.get()).ok().value();
    std::cout << "Uppercased value is " << out << std::endl;
    if (out != "ΓΕΙΆ ΣΟΥ ΚΌΣΜΕ") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    out = cm->uppercase("Γειά σου Κόσμε", *greek.get()).ok().value();
    std::cout << "Uppercased value is " << out << std::endl;
    if (out != "ΓΕΙΑ ΣΟΥ ΚΟΣΜΕ") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    out = cm->uppercase("istanbul", *und.get()).ok().value();
    std::cout << "Uppercased value is " << out << std::endl;
    if (out != "ISTANBUL") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    out = cm->uppercase("istanbul", *turkish.get()).ok().value();
    std::cout << "Uppercased value is " << out << std::endl;
    if (out != "İSTANBUL") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }


    out = cm->fold("ISTANBUL").ok().value();
    std::cout << "Folded value is " << out << std::endl;
    if (out != "istanbul") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    out = cm->fold_turkic("ISTANBUL").ok().value();
    std::cout << "Turkic-folded value is " << out << std::endl;
    if (out != "ıstanbul") {
        std::cout << "Output does not match expected output" << std::endl;
        return 1;
    }

    std::unique_ptr<CodePointSetBuilder> builder = CodePointSetBuilder::create();

    cm->add_case_closure_to('s', *builder.get());

    auto set = builder->build();

    if (set->contains('s')) {
        std::cout << "Set contains 's', which was not expected" << std::endl;
        return 1;
    } 
    if (!set->contains('S')) {
        std::cout << "Set does not 'S', which was not expected" << std::endl;
        return 1;
    } 
    if (!set->contains(U'ſ')) {
        std::cout << "Set does not 'S', which was not expected" << std::endl;
        return 1;
    } 
    return 0;
}
