#ifndef DurationFieldDisplay_D_H
#define DurationFieldDisplay_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum DurationFieldDisplay {
  DurationFieldDisplay_Default = 0,
  DurationFieldDisplay_Auto = 1,
  DurationFieldDisplay_Always = 2,
} DurationFieldDisplay;

typedef struct DurationFieldDisplay_option {union { DurationFieldDisplay ok; }; bool is_ok; } DurationFieldDisplay_option;



#endif // DurationFieldDisplay_D_H
