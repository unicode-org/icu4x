#ifndef DurationFormatterOptionsV1_D_H
#define DurationFormatterOptionsV1_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DurationBaseStyle.d.h"
#include "DurationFieldDisplay.d.h"
#include "DurationUnitStyle.d.h"




typedef struct DurationFormatterOptionsV1 {
  DurationBaseStyle base;
  DurationUnitStyle_option year;
  DurationFieldDisplay_option year_visibility;
  DurationUnitStyle_option month;
  DurationFieldDisplay_option month_visibility;
  DurationUnitStyle_option week;
  DurationFieldDisplay_option week_visibility;
  DurationUnitStyle_option day;
  DurationFieldDisplay_option day_visibility;
  DurationUnitStyle_option hour;
  DurationFieldDisplay_option hour_visibility;
  DurationUnitStyle_option minute;
  DurationFieldDisplay_option minute_visibility;
  DurationUnitStyle_option second;
  DurationFieldDisplay_option second_visibility;
  DurationUnitStyle_option millisecond;
  DurationFieldDisplay_option millisecond_visibility;
  DurationUnitStyle_option microsecond;
  DurationFieldDisplay_option microsecond_visibility;
  DurationUnitStyle_option nanosecond;
  DurationFieldDisplay_option nanosecond_visibility;
  OptionU8 fractional_digits;
} DurationFormatterOptionsV1;

typedef struct DurationFormatterOptionsV1_option {union { DurationFormatterOptionsV1 ok; }; bool is_ok; } DurationFormatterOptionsV1_option;



#endif // DurationFormatterOptionsV1_D_H
