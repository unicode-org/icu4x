#ifndef ICU4XCaseMapCloser_H
#define ICU4XCaseMapCloser_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XCodePointSetBuilder.d.h"
#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"

#include "ICU4XCaseMapCloser.d.h"






typedef struct ICU4XCaseMapCloser_create_result {union {ICU4XCaseMapCloser* ok; ICU4XDataError err;}; bool is_ok;} ICU4XCaseMapCloser_create_result;
ICU4XCaseMapCloser_create_result ICU4XCaseMapCloser_create(const ICU4XDataProvider* provider);

void ICU4XCaseMapCloser_add_case_closure_to(const ICU4XCaseMapCloser* self, char32_t c, ICU4XCodePointSetBuilder* builder);

bool ICU4XCaseMapCloser_add_string_case_closure_to(const ICU4XCaseMapCloser* self, const char* s_data, size_t s_len, ICU4XCodePointSetBuilder* builder);


void ICU4XCaseMapCloser_destroy(ICU4XCaseMapCloser* self);





#endif // ICU4XCaseMapCloser_H
