#ifndef DateTimeFieldSetBuilder_D_H
#define DateTimeFieldSetBuilder_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DateFields.d.h"
#include "DateTimeAlignment.d.h"
#include "NeoDateTimeLength.d.h"
#include "TimePrecision.d.h"
#include "YearStyle.d.h"
#include "ZoneStyle.d.h"




typedef struct DateTimeFieldSetBuilder {
  NeoDateTimeLength_option length;
  DateFields_option date_fields;
  TimePrecision_option time_precision;
  ZoneStyle_option zone_style;
  DateTimeAlignment_option alignment;
  YearStyle_option year_style;
} DateTimeFieldSetBuilder;

typedef struct DateTimeFieldSetBuilder_option {union { DateTimeFieldSetBuilder ok; }; bool is_ok; } DateTimeFieldSetBuilder_option;



#endif // DateTimeFieldSetBuilder_D_H
