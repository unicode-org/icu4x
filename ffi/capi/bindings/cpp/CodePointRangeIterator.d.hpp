#ifndef CodePointRangeIterator_D_HPP
#define CodePointRangeIterator_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CodePointRangeIteratorResult.d.hpp"

struct CodePointRangeIteratorResult;


namespace capi {
    typedef struct CodePointRangeIterator CodePointRangeIterator;
}

class CodePointRangeIterator {
public:

  inline CodePointRangeIteratorResult next();

  inline const capi::CodePointRangeIterator* AsFFI() const;
  inline capi::CodePointRangeIterator* AsFFI();
  inline static const CodePointRangeIterator* FromFFI(const capi::CodePointRangeIterator* ptr);
  inline static CodePointRangeIterator* FromFFI(capi::CodePointRangeIterator* ptr);
  inline static void operator delete(void* ptr);
private:
  CodePointRangeIterator() = delete;
  CodePointRangeIterator(const CodePointRangeIterator&) = delete;
  CodePointRangeIterator(CodePointRangeIterator&&) noexcept = delete;
  CodePointRangeIterator operator=(const CodePointRangeIterator&) = delete;
  CodePointRangeIterator operator=(CodePointRangeIterator&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // CodePointRangeIterator_D_HPP
