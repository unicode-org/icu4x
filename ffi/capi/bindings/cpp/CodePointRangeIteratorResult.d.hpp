#ifndef CodePointRangeIteratorResult_D_HPP
#define CodePointRangeIteratorResult_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef struct CodePointRangeIteratorResult {
      uint32_t start;
      uint32_t end;
      bool done;
    } CodePointRangeIteratorResult;
}

struct CodePointRangeIteratorResult {
  uint32_t start;
  uint32_t end;
  bool done;

  inline capi::CodePointRangeIteratorResult AsFFI() const;
  inline static CodePointRangeIteratorResult FromFFI(capi::CodePointRangeIteratorResult c_struct);
};


#endif // CodePointRangeIteratorResult_D_HPP
