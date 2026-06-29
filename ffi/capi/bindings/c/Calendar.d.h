#ifndef Calendar_D_H
#define Calendar_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct Calendar Calendar;


typedef struct DiplomatCalendarView {
  const Calendar** data;
  size_t len;
} DiplomatCalendarView;



#endif // Calendar_D_H
