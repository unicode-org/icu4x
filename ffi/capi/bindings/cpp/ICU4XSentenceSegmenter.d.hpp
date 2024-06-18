#ifndef ICU4XSentenceSegmenter_D_HPP
#define ICU4XSentenceSegmenter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.d.hpp"
#include "ICU4XSentenceSegmenter.d.h"

class ICU4XDataProvider;
class ICU4XSentenceBreakIteratorLatin1;
class ICU4XSentenceBreakIteratorUtf16;
class ICU4XSentenceBreakIteratorUtf8;
class ICU4XDataError;


class ICU4XSentenceSegmenter {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XSentenceSegmenter>, ICU4XDataError> create(const ICU4XDataProvider& provider);

  inline std::unique_ptr<ICU4XSentenceBreakIteratorUtf8> segment_utf8(std::string_view input) const;

  inline std::unique_ptr<ICU4XSentenceBreakIteratorUtf16> segment_utf16(std::u16string_view input) const;

  inline std::unique_ptr<ICU4XSentenceBreakIteratorLatin1> segment_latin1(diplomat::span<const uint8_t> input) const;

  inline const capi::ICU4XSentenceSegmenter* AsFFI() const;
  inline capi::ICU4XSentenceSegmenter* AsFFI();
  inline static const ICU4XSentenceSegmenter* FromFFI(const capi::ICU4XSentenceSegmenter* ptr);
  inline static ICU4XSentenceSegmenter* FromFFI(capi::ICU4XSentenceSegmenter* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XSentenceSegmenter() = delete;
  ICU4XSentenceSegmenter(const ICU4XSentenceSegmenter&) = delete;
  ICU4XSentenceSegmenter(ICU4XSentenceSegmenter&&) noexcept = delete;
  ICU4XSentenceSegmenter operator=(const ICU4XSentenceSegmenter&) = delete;
  ICU4XSentenceSegmenter operator=(ICU4XSentenceSegmenter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XSentenceSegmenter_D_HPP
