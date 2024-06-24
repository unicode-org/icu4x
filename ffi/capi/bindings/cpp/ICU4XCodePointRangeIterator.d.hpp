#ifndef ICU4XCodePointRangeIterator_D_HPP
#define ICU4XCodePointRangeIterator_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCodePointRangeIteratorResult.d.hpp"

struct ICU4XCodePointRangeIteratorResult;


namespace capi {
    typedef struct ICU4XCodePointRangeIterator ICU4XCodePointRangeIterator;
}

class ICU4XCodePointRangeIterator {
public:

  inline ICU4XCodePointRangeIteratorResult next();

  inline const capi::ICU4XCodePointRangeIterator* AsFFI() const;
  inline capi::ICU4XCodePointRangeIterator* AsFFI();
  inline static const ICU4XCodePointRangeIterator* FromFFI(const capi::ICU4XCodePointRangeIterator* ptr);
  inline static ICU4XCodePointRangeIterator* FromFFI(capi::ICU4XCodePointRangeIterator* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XCodePointRangeIterator() = delete;
  ICU4XCodePointRangeIterator(const ICU4XCodePointRangeIterator&) = delete;
  ICU4XCodePointRangeIterator(ICU4XCodePointRangeIterator&&) noexcept = delete;
  ICU4XCodePointRangeIterator operator=(const ICU4XCodePointRangeIterator&) = delete;
  ICU4XCodePointRangeIterator operator=(ICU4XCodePointRangeIterator&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XCodePointRangeIterator_D_HPP
