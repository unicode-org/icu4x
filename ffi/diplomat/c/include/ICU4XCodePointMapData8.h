#ifndef ICU4XCodePointMapData8_H
#define ICU4XCodePointMapData8_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XCodePointMapData8 ICU4XCodePointMapData8;
#ifdef __cplusplus
} // namespace capi
#endif
#include "ICU4XDataProvider.h"
#include "diplomat_result_box_ICU4XCodePointMapData8_ICU4XError.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XCodePointMapData8_ICU4XError ICU4XCodePointMapData8_try_get_general_category(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XCodePointMapData8_ICU4XError ICU4XCodePointMapData8_try_get_bidi_class(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XCodePointMapData8_ICU4XError ICU4XCodePointMapData8_try_get_east_asian_width(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XCodePointMapData8_ICU4XError ICU4XCodePointMapData8_try_get_line_break(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XCodePointMapData8_ICU4XError ICU4XCodePointMapData8_try_grapheme_cluster_break(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XCodePointMapData8_ICU4XError ICU4XCodePointMapData8_try_get_word_break(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XCodePointMapData8_ICU4XError ICU4XCodePointMapData8_try_get_sentence_break(const ICU4XDataProvider* provider);

uint8_t ICU4XCodePointMapData8_get(const ICU4XCodePointMapData8* self, char32_t cp);
void ICU4XCodePointMapData8_destroy(ICU4XCodePointMapData8* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
