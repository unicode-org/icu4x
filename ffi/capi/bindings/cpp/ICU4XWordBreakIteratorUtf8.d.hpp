#ifndef ICU4XWordBreakIteratorUtf8_D_HPP
#define ICU4XWordBreakIteratorUtf8_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XSegmenterWordType.d.hpp"
#include "ICU4XWordBreakIteratorUtf8.d.h"

class ICU4XSegmenterWordType;


class ICU4XWordBreakIteratorUtf8 {
public:

  inline int32_t next();

  inline ICU4XSegmenterWordType word_type() const;

  inline bool is_word_like() const;

  inline const capi::ICU4XWordBreakIteratorUtf8* AsFFI() const;
  inline capi::ICU4XWordBreakIteratorUtf8* AsFFI();
  inline static const ICU4XWordBreakIteratorUtf8* FromFFI(const capi::ICU4XWordBreakIteratorUtf8* ptr);
  inline static ICU4XWordBreakIteratorUtf8* FromFFI(capi::ICU4XWordBreakIteratorUtf8* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XWordBreakIteratorUtf8() = delete;
  ICU4XWordBreakIteratorUtf8(const ICU4XWordBreakIteratorUtf8&) = delete;
  ICU4XWordBreakIteratorUtf8(ICU4XWordBreakIteratorUtf8&&) noexcept = delete;
  ICU4XWordBreakIteratorUtf8 operator=(const ICU4XWordBreakIteratorUtf8&) = delete;
  ICU4XWordBreakIteratorUtf8 operator=(ICU4XWordBreakIteratorUtf8&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XWordBreakIteratorUtf8_D_HPP
