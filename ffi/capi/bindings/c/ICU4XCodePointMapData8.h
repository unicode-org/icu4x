#ifndef ICU4XCodePointMapData8_H
#define ICU4XCodePointMapData8_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XCodePointRangeIterator.d.h"
#include "ICU4XCodePointSetData.d.h"
#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"

#include "ICU4XCodePointMapData8.d.h"






uint8_t ICU4XCodePointMapData8_get(const ICU4XCodePointMapData8* self, char32_t cp);

uint8_t ICU4XCodePointMapData8_get32(const ICU4XCodePointMapData8* self, uint32_t cp);

uint32_t ICU4XCodePointMapData8_general_category_to_mask(uint8_t gc);

ICU4XCodePointRangeIterator* ICU4XCodePointMapData8_iter_ranges_for_value(const ICU4XCodePointMapData8* self, uint8_t value);

ICU4XCodePointRangeIterator* ICU4XCodePointMapData8_iter_ranges_for_value_complemented(const ICU4XCodePointMapData8* self, uint8_t value);

ICU4XCodePointRangeIterator* ICU4XCodePointMapData8_iter_ranges_for_mask(const ICU4XCodePointMapData8* self, uint32_t mask);

ICU4XCodePointSetData* ICU4XCodePointMapData8_get_set_for_value(const ICU4XCodePointMapData8* self, uint8_t value);

struct ICU4XCodePointMapData8_load_general_category_result {union {ICU4XCodePointMapData8* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XCodePointMapData8_load_general_category_result ICU4XCodePointMapData8_load_general_category(const ICU4XDataProvider* provider);

struct ICU4XCodePointMapData8_load_bidi_class_result {union {ICU4XCodePointMapData8* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XCodePointMapData8_load_bidi_class_result ICU4XCodePointMapData8_load_bidi_class(const ICU4XDataProvider* provider);

struct ICU4XCodePointMapData8_load_east_asian_width_result {union {ICU4XCodePointMapData8* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XCodePointMapData8_load_east_asian_width_result ICU4XCodePointMapData8_load_east_asian_width(const ICU4XDataProvider* provider);

struct ICU4XCodePointMapData8_load_hangul_syllable_type_result {union {ICU4XCodePointMapData8* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XCodePointMapData8_load_hangul_syllable_type_result ICU4XCodePointMapData8_load_hangul_syllable_type(const ICU4XDataProvider* provider);

struct ICU4XCodePointMapData8_load_indic_syllabic_category_result {union {ICU4XCodePointMapData8* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XCodePointMapData8_load_indic_syllabic_category_result ICU4XCodePointMapData8_load_indic_syllabic_category(const ICU4XDataProvider* provider);

struct ICU4XCodePointMapData8_load_line_break_result {union {ICU4XCodePointMapData8* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XCodePointMapData8_load_line_break_result ICU4XCodePointMapData8_load_line_break(const ICU4XDataProvider* provider);

struct ICU4XCodePointMapData8_try_grapheme_cluster_break_result {union {ICU4XCodePointMapData8* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XCodePointMapData8_try_grapheme_cluster_break_result ICU4XCodePointMapData8_try_grapheme_cluster_break(const ICU4XDataProvider* provider);

struct ICU4XCodePointMapData8_load_word_break_result {union {ICU4XCodePointMapData8* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XCodePointMapData8_load_word_break_result ICU4XCodePointMapData8_load_word_break(const ICU4XDataProvider* provider);

struct ICU4XCodePointMapData8_load_sentence_break_result {union {ICU4XCodePointMapData8* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XCodePointMapData8_load_sentence_break_result ICU4XCodePointMapData8_load_sentence_break(const ICU4XDataProvider* provider);

struct ICU4XCodePointMapData8_load_joining_type_result {union {ICU4XCodePointMapData8* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XCodePointMapData8_load_joining_type_result ICU4XCodePointMapData8_load_joining_type(const ICU4XDataProvider* provider);


void ICU4XCodePointMapData8_destroy(ICU4XCodePointMapData8* self);





#endif // ICU4XCodePointMapData8_H
