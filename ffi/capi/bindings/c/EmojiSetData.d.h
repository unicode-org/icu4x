#ifndef EmojiSetData_D_H
#define EmojiSetData_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct EmojiSetData EmojiSetData;


typedef struct DiplomatEmojiSetDataView {
  const EmojiSetData** data;
  size_t len;
} DiplomatEmojiSetDataView;



#endif // EmojiSetData_D_H
