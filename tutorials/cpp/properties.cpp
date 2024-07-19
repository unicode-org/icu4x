// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "CodePointSetData.hpp"
#include "UnicodeSetData.hpp"
#include "CodePointMapData16.hpp"
#include "CodePointMapData8.hpp"
#include "PropertyValueNameToEnumMapper.hpp"
#include "GeneralCategoryNameToMaskMapper.hpp"
#include "Logger.hpp"

#include <iostream>

int test_set_property(CodePointSetData* data, char32_t included, char32_t excluded) {
    bool contains1 = data->contains(included);
    bool contains2 = data->contains(excluded);
    std::cout << std::hex; // print hex for U+####
    if (contains1 && !contains2) {
        std::cout << "Set correctly contains U+" << included << " and not U+" << excluded << std::endl;
    } else {
        std::cout << "Set returns wrong result on U+" << included << " or U+" << excluded << std::endl;
        return 1;
    }
    return 0;
}

int test_map_16_property(CodePointMapData16* data, char32_t sample, uint32_t expected) {
    uint32_t actual = data->get(sample);
    std::cout << std::hex; // print hex for U+####
    if (actual == expected) {
        std::cout << "Code point U+" << sample << " correctly mapped to 0x" << actual << std::endl;
    } else {
        std::cout << "Code point U+" << sample << " incorrectly mapped to 0x" << actual << std::endl;
        return 1;
    }
    return 0;
}

int test_map_8_property(CodePointMapData8* data, char32_t sample, uint32_t expected) {
    uint32_t actual = data->get(sample);
    std::cout << std::hex; // print hex for U+####
    if (actual == expected) {
        std::cout << "Code point U+" << sample << " correctly mapped to 0x" << actual << std::endl;
    } else {
        std::cout << "Code point U+" << sample << " incorrectly mapped to 0x" << actual << std::endl;
        return 1;
    }
    return 0;
}

int main() {
    Logger::init_simple_logger();
    std::unique_ptr<DataProvider> dp = DataProvider::compiled();
    int result;

    result = test_set_property(
        CodePointSetData::load_ascii_hex_digit(*dp.get()).ok().value().get(),
        u'3',
        u'੩'
    );
    if (result != 0) {
        return result;
    }

    result = test_map_16_property(
        CodePointMapData16::load_script(*dp.get()).ok().value().get(),
        u'木',
        17 // Script::Han
    );
    if (result != 0) {
        return result;
    }

    result = test_map_8_property(
        CodePointMapData8::load_general_category(*dp.get()).ok().value().get(),
        u'木',
        5 // GeneralCategory::OtherLetter
    );
    if (result != 0) {
        return result;
    }

    result = test_map_8_property(
        CodePointMapData8::load_bidi_class(*dp.get()).ok().value().get(),
        u'ع',
        13 // GeneralCategory::ArabicLetter
    );
    if (result != 0) {
        return result;
    }

    std::unique_ptr<UnicodeSetData> basic_emoji = UnicodeSetData::load_basic_emoji(*dp.get()).ok().value();
    std::string letter = u8"hello";

    if (!basic_emoji->contains_char(U'🔥')) {
        std::cout << "Character 🔥 not found in Basic_Emoji set" << std::endl;
        result = 1;
    }

    if (!basic_emoji->contains(u8"🗺️")) {
        std::cout << "String \"🗺️\" (U+1F5FA U+FE0F) not found in Basic_Emoji set" << std::endl;
        result = 1;
    }
    if (basic_emoji->contains_char(U'a')) {
        std::cout << "Character a found in Basic_Emoji set" << std::endl;
        result = 1;
    }

    if (basic_emoji->contains(u8"aa")) {
        std::cout << "String \"aa\" found in Basic_Emoji set" << std::endl;
        result = 1;
    }

    if (result != 0) {
        return result;
    } else {
        std::cout << "Basic_Emoji set contains appropriate characters" << std::endl;
    }
    std::unique_ptr<Locale> locale = Locale::from_string("bn").ok().value();
    std::unique_ptr<UnicodeSetData> exemplars = UnicodeSetData::load_exemplars_main(*dp.get(), *locale.get()).ok().value();
    if (!exemplars->contains_char(U'ব')) {
        std::cout << "Character 'ব' not found in Bangla exemplar chars set" << std::endl;
        result = 1;
    }

    if (!exemplars->contains(u8"ক্ষ")) {
        std::cout << "String \"ক্ষ\" (U+0995U+09CDU+09B7) not found in Bangla exemplar chars set" << std::endl;
        result = 1;
    }
    if (exemplars->contains_char(U'a')) {
        std::cout << "Character a found in Bangla exemplar chars set" << std::endl;
        result = 1;
    }

    if (exemplars->contains(u8"aa")) {
        std::cout << "String \"aa\" not found in Bangla exemplar chars set" << std::endl;
        result = 1;
    }
    if (result != 0) {
        return result;
    } else {
        std::cout << "Bangla exemplar chars set contains appropriate characters" << std::endl;
    }


    std::unique_ptr<PropertyValueNameToEnumMapper> mapper = PropertyValueNameToEnumMapper::load_script(*dp.get()).ok().value();
    int32_t script = mapper->get_strict("Brah");
    if (script != 65) {
        std::cout << "Expected discriminant 64 for script name `Brah`, found " << script << std::endl;
        result = 1;
    }
    script = mapper->get_strict("Brahmi");
    if (script != 65) {
        std::cout << "Expected discriminant 64 for script name `Brahmi`, found " << script << std::endl;
        result = 1;
    }
    script = mapper->get_loose("brah");
    if (script != 65) {
        std::cout << "Expected discriminant 64 for (loose matched) script name `brah`, found " << script << std::endl;
        result = 1;
    }
    script = mapper->get_strict("Linear_Z");
    if (script != -1) {
        std::cout << "Expected no value for fake script name `Linear_Z`, found " << script << std::endl;
        result = 1;
    }
    if (result != 0) {
        return result;
    } else {
        std::cout << "Script name mapper returns correct values" << std::endl;
    }

    std::unique_ptr<GeneralCategoryNameToMaskMapper> mask_mapper = GeneralCategoryNameToMaskMapper::load(*dp.get()).ok().value();
    int32_t mask = mask_mapper->get_strict("Lu");
    if (mask != 0x02) {
        std::cout << "Expected discriminant 0x02 for mask name `Lu`, found " << mask << std::endl;
        result = 1;
    }
    mask = mask_mapper->get_strict("L");
    if (mask != 0x3e) {
        std::cout << "Expected discriminant 0x3e for mask name `Lu`, found " << mask << std::endl;
        result = 1;
    }
    mask = mask_mapper->get_strict("Letter");
    if (mask != 0x3e) {
        std::cout << "Expected discriminant 0x3e for mask name `Letter`, found " << mask << std::endl;
        result = 1;
    }
    mask = mask_mapper->get_loose("l");
    if (mask != 0x3e) {
        std::cout << "Expected discriminant 0x3e for mask name `l`, found " << mask << std::endl;
        result = 1;
    }
    mask = mask_mapper->get_strict("letter");
    if (mask != 0) {
        std::cout << "Expected no mask for (strict matched) name `letter`, found " << mask << std::endl;
        result = 1;
    }
    mask = mask_mapper->get_strict("EverythingLol");
    if (mask != 0) {
        std::cout << "Expected no mask for nonexistent name `EverythingLol`, found " << mask << std::endl;
        result = 1;
    }


    if (result != 0) {
        return result;
    } else {
        std::cout << "Mask name mapper returns correct values" << std::endl;
    }


    mask = mask_mapper->get_strict("Lu");
    std::unique_ptr<CodePointMapData8> gc = CodePointMapData8::load_general_category(*dp.get()).ok().value();
    auto ranges = gc->iter_ranges_for_mask(mask);
    auto next = ranges->next();
    if (next.done) {
        std::cout << "Got empty iterator!";
        result = 1;
    }
    if (next.start != U'A' || next.end != U'Z') {
        std::cout << "Expected range [" <<  U'A' << ", " <<  U'Z' << "], got range [" << next.start << ", " << next.end << "]" << std::endl;
        result = 1;
    }

    // Test iteration to completion for a small set
    mask = mask_mapper->get_strict("Control");
    ranges = gc->iter_ranges_for_mask(mask);
    next = ranges->next();

    if (next.start != 0 || next.end != 0x1f) {
        std::cout << "Expected range [0, 0x1f], got range [" << next.start << ", " << next.end << "]" << std::endl;
        result = 1;
    }

    std::cout << "Found ranges for gc=Control:";
    while (!next.done) {
        std::cout << " [" << next.start << ", " << next.end << "]";

        next = ranges->next();
    }
    std::cout << std::endl;

    if (result != 0) {
        return result;
    } else {
        std::cout << "Ranges iterator works" << std::endl;
    }
    return 0;
}
