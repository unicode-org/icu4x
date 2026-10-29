#ifndef PartKind_D_H
#define PartKind_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum PartKind {
  PartKind_Unknown = 0,
  PartKind_Error = 1,
  PartKind_ListElement = 2,
  PartKind_ListLiteral = 3,
  PartKind_DecimalPlusSign = 4,
  PartKind_DecimalMinusSign = 5,
  PartKind_DecimalInteger = 6,
  PartKind_DecimalFraction = 7,
  PartKind_DecimalGroup = 8,
  PartKind_DecimalDecimal = 9,
  PartKind_DateTimeEra = 10,
  PartKind_DateTimeYear = 11,
  PartKind_DateTimeRelatedYear = 12,
  PartKind_DateTimeYearName = 13,
  PartKind_DateTimeMonth = 14,
  PartKind_DateTimeDay = 15,
  PartKind_DateTimeExtendedYear = 16,
  PartKind_DateTimeJulianDay = 17,
  PartKind_DateTimeWeekday = 18,
  PartKind_DateTimeDayPeriod = 19,
  PartKind_DateTimeHour = 20,
  PartKind_DateTimeMinute = 21,
  PartKind_DateTimeSecond = 22,
  PartKind_DateTimeTimeZoneName = 23,
  PartKind_DateTimeStartRange = 24,
  PartKind_DateTimeEndRange = 25,
  PartKind_RelativeTimeLiteral = 26,
  PartKind_DurationLiteral = 27,
  PartKind_UnitYear = 28,
  PartKind_UnitMonth = 29,
  PartKind_UnitWeek = 30,
  PartKind_UnitDay = 31,
  PartKind_UnitHour = 32,
  PartKind_UnitMinute = 33,
  PartKind_UnitSecond = 34,
  PartKind_UnitMillisecond = 35,
  PartKind_UnitMicrosecond = 36,
  PartKind_UnitNanosecond = 37,
} PartKind;

typedef struct PartKind_option {union { PartKind ok; }; bool is_ok; } PartKind_option;



#endif // PartKind_D_H
