#ifndef ICU4XPluralRuleType_H
#define ICU4XPluralRuleType_H
#include <stdio.h>
#include <uchar.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum ICU4XPluralRuleType {
  ICU4XPluralRuleType_Cardinal = 0,
  ICU4XPluralRuleType_Ordinal = 1,
} ICU4XPluralRuleType;

void ICU4XPluralRuleType_destroy(ICU4XPluralRuleType* self);

#ifdef __cplusplus
}
#endif
#endif
