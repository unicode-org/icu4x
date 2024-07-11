#ifndef CodePointRangeIteratorResult_D_HPP
#define CodePointRangeIteratorResult_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    typedef struct CodePointRangeIteratorResult {
      uint32_t start;
      uint32_t end;
      bool done;
    } CodePointRangeIteratorResult;
} // namespace capi
} // namespace


struct CodePointRangeIteratorResult {
  uint32_t start;
  uint32_t end;
  bool done;

  inline diplomat::capi::CodePointRangeIteratorResult AsFFI() const;
  inline static CodePointRangeIteratorResult FromFFI(diplomat::capi::CodePointRangeIteratorResult c_struct);
};


#endif // CodePointRangeIteratorResult_D_HPP
