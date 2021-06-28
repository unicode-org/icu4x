// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_PLURALRULES_H
#define ICU4X_PLURALRULES_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "provider.h"
#include "locale.h"

#ifdef __cplusplus
extern "C" {
#endif

// opaque
typedef struct ICU4XPluralRules ICU4XPluralRules;

typedef struct {
    ICU4XPluralRules* rules;
    bool success;
} ICU4XCreatePluralRulesResult;

typedef enum {
    ICU4XPluralRuleType_Cardinal,
    ICU4XPluralRuleType_Ordinal
} ICU4XPluralRuleType;

typedef enum {
    ICU4XPluralCategory_Zero,
    ICU4XPluralCategory_One,
    ICU4XPluralCategory_Two,
    ICU4XPluralCategory_Few,
    ICU4XPluralCategory_Many,
    ICU4XPluralCategory_Other,
} ICU4XPluralCategory;

typedef struct {
    bool zero;
    bool one;
    bool two;
    bool few;
    bool many;
    bool other;
} ICU4XPluralCategories;

typedef struct {
    uint64_t i;
    size_t v;
    size_t w;
    uint64_t f;
    uint64_t t;
    size_t c;
} ICU4XPluralOperands;

typedef struct {
    ICU4XPluralOperands operands;
    bool success;
} ICU4XCreatePluralOperandsResult;

ICU4XCreatePluralRulesResult icu4x_plural_rules_create(const ICU4XLocale* locale, const ICU4XDataProvider* provider, ICU4XPluralRuleType ty);
ICU4XCreatePluralOperandsResult icu4x_plural_operands_create(const char* number, size_t len);
ICU4XPluralCategory icu4x_plural_rules_select(const ICU4XPluralRules* rules, const ICU4XPluralOperands* op);
ICU4XPluralCategories icu4x_plural_rules_categories(const ICU4XPluralRules* rules);
void icu4x_plural_rules_destroy(ICU4XPluralRules* rules);

#ifdef __cplusplus
}
#endif

#endif // ICU4X_PLURALRULES_H
