#ifndef DurationFormatterOptions_D_H
#define DurationFormatterOptions_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DurationBaseStyle.d.h"
#include "DurationFieldDisplay.d.h"
#include "DurationUnitStyle.d.h"




typedef struct DurationFormatterOptions {
  DurationBaseStyle base;
  DurationUnitStyle year;
  DurationFieldDisplay year_visibility;
  DurationUnitStyle month;
  DurationFieldDisplay month_visibility;
  DurationUnitStyle week;
  DurationFieldDisplay week_visibility;
  DurationUnitStyle day;
  DurationFieldDisplay day_visibility;
  DurationUnitStyle hour;
  DurationFieldDisplay hour_visibility;
  DurationUnitStyle minute;
  DurationFieldDisplay minute_visibility;
  DurationUnitStyle second;
  DurationFieldDisplay second_visibility;
  DurationUnitStyle millisecond;
  DurationFieldDisplay millisecond_visibility;
  DurationUnitStyle microsecond;
  DurationFieldDisplay microsecond_visibility;
  DurationUnitStyle nanosecond;
  DurationFieldDisplay nanosecond_visibility;
  int8_t fractional_digits;
} DurationFormatterOptions;

typedef struct DurationFormatterOptions_option {union { DurationFormatterOptions ok; }; bool is_ok; } DurationFormatterOptions_option;



#endif // DurationFormatterOptions_D_H
