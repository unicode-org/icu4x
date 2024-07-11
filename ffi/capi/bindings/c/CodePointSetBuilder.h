#ifndef CodePointSetBuilder_H
#define CodePointSetBuilder_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CodePointSetData.d.h"

#include "CodePointSetBuilder.d.h"






CodePointSetBuilder* ICU4XCodePointSetBuilder_create();

CodePointSetData* ICU4XCodePointSetBuilder_build(CodePointSetBuilder* self);

void ICU4XCodePointSetBuilder_complement(CodePointSetBuilder* self);

bool ICU4XCodePointSetBuilder_is_empty(const CodePointSetBuilder* self);

void ICU4XCodePointSetBuilder_add_char(CodePointSetBuilder* self, char32_t ch);

void ICU4XCodePointSetBuilder_add_inclusive_range(CodePointSetBuilder* self, char32_t start, char32_t end);

void ICU4XCodePointSetBuilder_add_set(CodePointSetBuilder* self, const CodePointSetData* data);

void ICU4XCodePointSetBuilder_remove_char(CodePointSetBuilder* self, char32_t ch);

void ICU4XCodePointSetBuilder_remove_inclusive_range(CodePointSetBuilder* self, char32_t start, char32_t end);

void ICU4XCodePointSetBuilder_remove_set(CodePointSetBuilder* self, const CodePointSetData* data);

void ICU4XCodePointSetBuilder_retain_char(CodePointSetBuilder* self, char32_t ch);

void ICU4XCodePointSetBuilder_retain_inclusive_range(CodePointSetBuilder* self, char32_t start, char32_t end);

void ICU4XCodePointSetBuilder_retain_set(CodePointSetBuilder* self, const CodePointSetData* data);

void ICU4XCodePointSetBuilder_complement_char(CodePointSetBuilder* self, char32_t ch);

void ICU4XCodePointSetBuilder_complement_inclusive_range(CodePointSetBuilder* self, char32_t start, char32_t end);

void ICU4XCodePointSetBuilder_complement_set(CodePointSetBuilder* self, const CodePointSetData* data);


void ICU4XCodePointSetBuilder_destroy(CodePointSetBuilder* self);





#endif // CodePointSetBuilder_H
