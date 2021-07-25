#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum ICU4XCanonicalizationResult {
  ICU4XCanonicalizationResult_Modified = 0,
  ICU4XCanonicalizationResult_Unmodified = 1,
} ICU4XCanonicalizationResult;

typedef struct ICU4XDataProvider ICU4XDataProvider;

typedef struct ICU4XFixedDecimal ICU4XFixedDecimal;

typedef struct ICU4XFixedDecimalFormat ICU4XFixedDecimalFormat;

typedef enum ICU4XFixedDecimalGroupingStrategy {
  ICU4XFixedDecimalGroupingStrategy_Auto = 0,
  ICU4XFixedDecimalGroupingStrategy_Never = 1,
  ICU4XFixedDecimalGroupingStrategy_Always = 2,
  ICU4XFixedDecimalGroupingStrategy_Min2 = 3,
} ICU4XFixedDecimalGroupingStrategy;

typedef enum ICU4XFixedDecimalSignDisplay {
  ICU4XFixedDecimalSignDisplay_Auto = 0,
  ICU4XFixedDecimalSignDisplay_Never = 1,
  ICU4XFixedDecimalSignDisplay_Always = 2,
  ICU4XFixedDecimalSignDisplay_ExceptZero = 3,
  ICU4XFixedDecimalSignDisplay_Negative = 4,
} ICU4XFixedDecimalSignDisplay;

typedef struct ICU4XLocale ICU4XLocale;

typedef struct ICU4XLocaleCanonicalizer ICU4XLocaleCanonicalizer;

typedef enum ICU4XLocaleResult {
  ICU4XLocaleResult_Ok = 0,
  ICU4XLocaleResult_Undefined = 1,
  ICU4XLocaleResult_Error = 2,
} ICU4XLocaleResult;

typedef enum ICU4XPluralCategory {
  ICU4XPluralCategory_Zero = 0,
  ICU4XPluralCategory_One = 1,
  ICU4XPluralCategory_Two = 2,
  ICU4XPluralCategory_Few = 3,
  ICU4XPluralCategory_Many = 4,
  ICU4XPluralCategory_Other = 5,
} ICU4XPluralCategory;

typedef enum ICU4XPluralRuleType {
  ICU4XPluralRuleType_Cardinal = 0,
  ICU4XPluralRuleType_Ordinal = 1,
} ICU4XPluralRuleType;

typedef struct ICU4XPluralRules ICU4XPluralRules;

typedef struct ICU4XCreateDataProviderResult {
    ICU4XDataProvider* provider;
    bool success;
} ICU4XCreateDataProviderResult;

typedef struct ICU4XCreateFixedDecimalResult {
    ICU4XFixedDecimal* fd;
    bool success;
} ICU4XCreateFixedDecimalResult;

typedef struct ICU4XPluralOperands {
    uint64_t i;
    size_t v;
    size_t w;
    uint64_t f;
    uint64_t t;
    size_t c;
} ICU4XPluralOperands;

typedef struct ICU4XCreatePluralOperandsResult {
    ICU4XPluralOperands operands;
    bool success;
} ICU4XCreatePluralOperandsResult;

typedef struct ICU4XCreatePluralRulesResult {
    ICU4XPluralRules* rules;
    bool success;
} ICU4XCreatePluralRulesResult;

typedef struct ICU4XFixedDecimalFormatOptions {
    ssize_t grouping_strategy;
    ssize_t sign_display;
} ICU4XFixedDecimalFormatOptions;

typedef struct ICU4XFixedDecimalFormatResult {
    ICU4XFixedDecimalFormat* fdf;
    bool success;
} ICU4XFixedDecimalFormatResult;

typedef struct ICU4XPluralCategories {
    bool zero;
    bool one;
    bool two;
    bool few;
    bool many;
    bool other;
} ICU4XPluralCategories;
void ICU4XCanonicalizationResult_destroy(ssize_t* self);
void ICU4XCreateDataProviderResult_destroy(ICU4XCreateDataProviderResult* self);
void ICU4XCreateFixedDecimalResult_destroy(ICU4XCreateFixedDecimalResult* self);
void ICU4XCreatePluralOperandsResult_destroy(ICU4XCreatePluralOperandsResult* self);
void ICU4XCreatePluralRulesResult_destroy(ICU4XCreatePluralRulesResult* self);

ICU4XCreateDataProviderResult ICU4XDataProvider_create_fs(const char* path_data, size_t path_len);

ICU4XCreateDataProviderResult ICU4XDataProvider_create_static();
void ICU4XDataProvider_destroy(ICU4XDataProvider* self);

ICU4XFixedDecimal* ICU4XFixedDecimal_create(int32_t v);

ICU4XCreateFixedDecimalResult ICU4XFixedDecimal_create_fromstr(const char* v_data, size_t v_len);

bool ICU4XFixedDecimal_multiply_pow10(ICU4XFixedDecimal* self, int16_t power);

void ICU4XFixedDecimal_negate(ICU4XFixedDecimal* self);

void ICU4XFixedDecimal_to_string(const ICU4XFixedDecimal* self, DiplomatWriteable* to);
void ICU4XFixedDecimal_destroy(ICU4XFixedDecimal* self);

ICU4XFixedDecimalFormatResult ICU4XFixedDecimalFormat_try_new(const ICU4XLocale* locale, const ICU4XDataProvider* provider, ICU4XFixedDecimalFormatOptions options);

void ICU4XFixedDecimalFormat_format_write(const ICU4XFixedDecimalFormat* self, const ICU4XFixedDecimal* value, DiplomatWriteable* write);
void ICU4XFixedDecimalFormat_destroy(ICU4XFixedDecimalFormat* self);

ICU4XFixedDecimalFormatOptions ICU4XFixedDecimalFormatOptions_default();
void ICU4XFixedDecimalFormatOptions_destroy(ICU4XFixedDecimalFormatOptions* self);
void ICU4XFixedDecimalFormatResult_destroy(ICU4XFixedDecimalFormatResult* self);
void ICU4XFixedDecimalGroupingStrategy_destroy(ssize_t* self);
void ICU4XFixedDecimalSignDisplay_destroy(ssize_t* self);

ICU4XLocale* ICU4XLocale_create(const char* name_data, size_t name_len);

ICU4XLocale* ICU4XLocale_clone(const ICU4XLocale* self);

void ICU4XLocale_basename(const ICU4XLocale* self, DiplomatWriteable* write);

void ICU4XLocale_get_unicode_extension(const ICU4XLocale* self, const char* bytes_data, size_t bytes_len, DiplomatWriteable* write);

void ICU4XLocale_language(const ICU4XLocale* self, DiplomatWriteable* write);

void ICU4XLocale_region(const ICU4XLocale* self, DiplomatWriteable* write);

void ICU4XLocale_script(const ICU4XLocale* self, DiplomatWriteable* write);

void ICU4XLocale_tostring(const ICU4XLocale* self, DiplomatWriteable* write);
void ICU4XLocale_destroy(ICU4XLocale* self);

ICU4XLocaleCanonicalizer* ICU4XLocaleCanonicalizer_create(const ICU4XDataProvider* provider);

ssize_t ICU4XLocaleCanonicalizer_canonicalize(const ICU4XLocaleCanonicalizer* self, ICU4XLocale* locale);

ssize_t ICU4XLocaleCanonicalizer_maximize(const ICU4XLocaleCanonicalizer* self, ICU4XLocale* locale);

ssize_t ICU4XLocaleCanonicalizer_minimize(const ICU4XLocaleCanonicalizer* self, ICU4XLocale* locale);
void ICU4XLocaleCanonicalizer_destroy(ICU4XLocaleCanonicalizer* self);
void ICU4XLocaleResult_destroy(ssize_t* self);
void ICU4XPluralCategories_destroy(ICU4XPluralCategories* self);
void ICU4XPluralCategory_destroy(ssize_t* self);

ICU4XCreatePluralOperandsResult ICU4XPluralOperands_create(const char* s_data, size_t s_len);
void ICU4XPluralOperands_destroy(ICU4XPluralOperands* self);
void ICU4XPluralRuleType_destroy(ssize_t* self);

ICU4XCreatePluralRulesResult ICU4XPluralRules_create(const ICU4XLocale* locale, const ICU4XDataProvider* provider, ssize_t ty);

ssize_t ICU4XPluralRules_select(const ICU4XPluralRules* self, const ICU4XPluralOperands* op);

ICU4XPluralCategories ICU4XPluralRules_categories(const ICU4XPluralRules* self);
void ICU4XPluralRules_destroy(ICU4XPluralRules* self);
#ifdef __cplusplus
}
#endif
