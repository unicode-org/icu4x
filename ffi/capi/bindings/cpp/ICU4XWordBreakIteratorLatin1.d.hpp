#ifndef ICU4XWordBreakIteratorLatin1_D_HPP
#define ICU4XWordBreakIteratorLatin1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XSegmenterWordType.d.hpp"

class ICU4XSegmenterWordType;


namespace capi {
    typedef struct ICU4XWordBreakIteratorLatin1 ICU4XWordBreakIteratorLatin1;
}

class ICU4XWordBreakIteratorLatin1 {
public:

  inline int32_t next();

  inline ICU4XSegmenterWordType word_type() const;

  inline bool is_word_like() const;

  inline const capi::ICU4XWordBreakIteratorLatin1* AsFFI() const;
  inline capi::ICU4XWordBreakIteratorLatin1* AsFFI();
  inline static const ICU4XWordBreakIteratorLatin1* FromFFI(const capi::ICU4XWordBreakIteratorLatin1* ptr);
  inline static ICU4XWordBreakIteratorLatin1* FromFFI(capi::ICU4XWordBreakIteratorLatin1* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XWordBreakIteratorLatin1() = delete;
  ICU4XWordBreakIteratorLatin1(const ICU4XWordBreakIteratorLatin1&) = delete;
  ICU4XWordBreakIteratorLatin1(ICU4XWordBreakIteratorLatin1&&) noexcept = delete;
  ICU4XWordBreakIteratorLatin1 operator=(const ICU4XWordBreakIteratorLatin1&) = delete;
  ICU4XWordBreakIteratorLatin1 operator=(ICU4XWordBreakIteratorLatin1&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XWordBreakIteratorLatin1_D_HPP
