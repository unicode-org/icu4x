#ifndef CodePointMapData8_H
#define CodePointMapData8_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CodePointRangeIterator.d.h"
#include "CodePointSetData.d.h"
#include "DataError.d.h"
#include "DataProvider.d.h"

#include "CodePointMapData8.d.h"






uint8_t ICU4XCodePointMapData8_get(const CodePointMapData8* self, char32_t cp);

uint8_t ICU4XCodePointMapData8_get32(const CodePointMapData8* self, uint32_t cp);

uint32_t ICU4XCodePointMapData8_general_category_to_mask(uint8_t gc);

CodePointRangeIterator* ICU4XCodePointMapData8_iter_ranges_for_value(const CodePointMapData8* self, uint8_t value);

CodePointRangeIterator* ICU4XCodePointMapData8_iter_ranges_for_value_complemented(const CodePointMapData8* self, uint8_t value);

CodePointRangeIterator* ICU4XCodePointMapData8_iter_ranges_for_mask(const CodePointMapData8* self, uint32_t mask);

CodePointSetData* ICU4XCodePointMapData8_get_set_for_value(const CodePointMapData8* self, uint8_t value);

typedef struct ICU4XCodePointMapData8_load_general_category_result {union {CodePointMapData8* ok; DataError err;}; bool is_ok;} ICU4XCodePointMapData8_load_general_category_result;
ICU4XCodePointMapData8_load_general_category_result ICU4XCodePointMapData8_load_general_category(const DataProvider* provider);

typedef struct ICU4XCodePointMapData8_load_bidi_class_result {union {CodePointMapData8* ok; DataError err;}; bool is_ok;} ICU4XCodePointMapData8_load_bidi_class_result;
ICU4XCodePointMapData8_load_bidi_class_result ICU4XCodePointMapData8_load_bidi_class(const DataProvider* provider);

typedef struct ICU4XCodePointMapData8_load_east_asian_width_result {union {CodePointMapData8* ok; DataError err;}; bool is_ok;} ICU4XCodePointMapData8_load_east_asian_width_result;
ICU4XCodePointMapData8_load_east_asian_width_result ICU4XCodePointMapData8_load_east_asian_width(const DataProvider* provider);

typedef struct ICU4XCodePointMapData8_load_hangul_syllable_type_result {union {CodePointMapData8* ok; DataError err;}; bool is_ok;} ICU4XCodePointMapData8_load_hangul_syllable_type_result;
ICU4XCodePointMapData8_load_hangul_syllable_type_result ICU4XCodePointMapData8_load_hangul_syllable_type(const DataProvider* provider);

typedef struct ICU4XCodePointMapData8_load_indic_syllabic_category_result {union {CodePointMapData8* ok; DataError err;}; bool is_ok;} ICU4XCodePointMapData8_load_indic_syllabic_category_result;
ICU4XCodePointMapData8_load_indic_syllabic_category_result ICU4XCodePointMapData8_load_indic_syllabic_category(const DataProvider* provider);

typedef struct ICU4XCodePointMapData8_load_line_break_result {union {CodePointMapData8* ok; DataError err;}; bool is_ok;} ICU4XCodePointMapData8_load_line_break_result;
ICU4XCodePointMapData8_load_line_break_result ICU4XCodePointMapData8_load_line_break(const DataProvider* provider);

typedef struct ICU4XCodePointMapData8_try_grapheme_cluster_break_result {union {CodePointMapData8* ok; DataError err;}; bool is_ok;} ICU4XCodePointMapData8_try_grapheme_cluster_break_result;
ICU4XCodePointMapData8_try_grapheme_cluster_break_result ICU4XCodePointMapData8_try_grapheme_cluster_break(const DataProvider* provider);

typedef struct ICU4XCodePointMapData8_load_word_break_result {union {CodePointMapData8* ok; DataError err;}; bool is_ok;} ICU4XCodePointMapData8_load_word_break_result;
ICU4XCodePointMapData8_load_word_break_result ICU4XCodePointMapData8_load_word_break(const DataProvider* provider);

typedef struct ICU4XCodePointMapData8_load_sentence_break_result {union {CodePointMapData8* ok; DataError err;}; bool is_ok;} ICU4XCodePointMapData8_load_sentence_break_result;
ICU4XCodePointMapData8_load_sentence_break_result ICU4XCodePointMapData8_load_sentence_break(const DataProvider* provider);

typedef struct ICU4XCodePointMapData8_load_joining_type_result {union {CodePointMapData8* ok; DataError err;}; bool is_ok;} ICU4XCodePointMapData8_load_joining_type_result;
ICU4XCodePointMapData8_load_joining_type_result ICU4XCodePointMapData8_load_joining_type(const DataProvider* provider);


void ICU4XCodePointMapData8_destroy(CodePointMapData8* self);





#endif // CodePointMapData8_H
