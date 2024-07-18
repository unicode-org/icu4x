// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#include "DataProvider.h"
#include "LineSegmenter.h"
#include "LineBreakIteratorUtf8.h"
#include <string.h>
#include <stdio.h>

int main(int argc, char *argv[]) {
    DataProvider* provider = icu4x_DataProvider_create_compiled_mv1();

    icu4x_LineSegmenter_create_auto_mv1_result segmenter_result = icu4x_LineSegmenter_create_auto_mv1(provider);
    if (!segmenter_result.is_ok)  {
        printf("Failed to create icu4x_LineSegmenter_mv1\n");
        return 1;
    }
    LineSegmenter* segmenter = segmenter_result.ok;

    char output[40];
    DiplomatWrite write = diplomat_simple_write(output, 40);

    const char* data = "อักษรไทย เป็นอักษรที่ใช้เขียนภาษาไทยและภาษาของกลุ่มชาติพันธุ์ต่างๆ เช่น คำเมือง, อีสาน, ภาษาไทยใต้, มลายูปัตตานี เป็นต้น ในประเทศไทย มีพยัญชนะ 44 รูป สระ 21 รูป วรรณยุกต์ 4 รูป และเครื่องหมายอื่น ๆ อีกจำนวนหนึ่ง";

    LineBreakIteratorUtf8* iter = icu4x_LineSegmenter_segment_utf8_mv1(segmenter, data, strlen(data));

    printf("Breakpoints:");
    while (true) {
        int32_t breakpoint = icu4x_LineBreakIteratorUtf8_next_mv1(iter);
        if (breakpoint == -1) {
            break;
        }
        printf(" %d", breakpoint);
    }

    printf("\n");

    icu4x_LineBreakIteratorUtf8_destroy_mv1(iter);
    icu4x_LineSegmenter_destroy_mv1(segmenter);
    icu4x_DataProvider_destroy_mv1(provider);

    return 0;
}
