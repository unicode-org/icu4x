#ifndef CaseMapCloser_H
#define CaseMapCloser_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CodePointSetBuilder.d.h"
#include "DataError.d.h"
#include "DataProvider.d.h"

#include "CaseMapCloser.d.h"






typedef struct ICU4XCaseMapCloser_create_result {union {CaseMapCloser* ok; DataError err;}; bool is_ok;} ICU4XCaseMapCloser_create_result;
ICU4XCaseMapCloser_create_result ICU4XCaseMapCloser_create(const DataProvider* provider);

void ICU4XCaseMapCloser_add_case_closure_to(const CaseMapCloser* self, char32_t c, CodePointSetBuilder* builder);

bool ICU4XCaseMapCloser_add_string_case_closure_to(const CaseMapCloser* self, const char* s_data, size_t s_len, CodePointSetBuilder* builder);


void ICU4XCaseMapCloser_destroy(CaseMapCloser* self);





#endif // CaseMapCloser_H
