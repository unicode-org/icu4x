#ifndef WeekInformation_D_H
#define WeekInformation_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct WeekInformation WeekInformation;


typedef struct DiplomatWeekInformationView {
  const WeekInformation** data;
  size_t len;
} DiplomatWeekInformationView;



#endif // WeekInformation_D_H
