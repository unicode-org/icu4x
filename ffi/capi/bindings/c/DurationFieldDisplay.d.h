#ifndef DurationFieldDisplay_D_H
#define DurationFieldDisplay_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum DurationFieldDisplay {
  DurationFieldDisplay_Auto = 0,
  DurationFieldDisplay_Always = 1,
} DurationFieldDisplay;

typedef struct DurationFieldDisplay_option {union { DurationFieldDisplay ok; }; bool is_ok; } DurationFieldDisplay_option;



#endif // DurationFieldDisplay_D_H
