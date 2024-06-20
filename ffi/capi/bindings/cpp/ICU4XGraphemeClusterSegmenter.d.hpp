#ifndef ICU4XGraphemeClusterSegmenter_D_HPP
#define ICU4XGraphemeClusterSegmenter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.d.hpp"
#include "ICU4XGraphemeClusterSegmenter.d.h"

class ICU4XDataProvider;
class ICU4XGraphemeClusterBreakIteratorLatin1;
class ICU4XGraphemeClusterBreakIteratorUtf16;
class ICU4XGraphemeClusterBreakIteratorUtf8;
class ICU4XDataError;


class ICU4XGraphemeClusterSegmenter {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XGraphemeClusterSegmenter>, ICU4XDataError> create(const ICU4XDataProvider& provider);

  inline std::unique_ptr<ICU4XGraphemeClusterBreakIteratorUtf8> segment_utf8(std::string_view input) const;

  inline std::unique_ptr<ICU4XGraphemeClusterBreakIteratorUtf16> segment_utf16(std::u16string_view input) const;

  inline std::unique_ptr<ICU4XGraphemeClusterBreakIteratorLatin1> segment_latin1(diplomat::span<const uint8_t> input) const;

  inline const capi::ICU4XGraphemeClusterSegmenter* AsFFI() const;
  inline capi::ICU4XGraphemeClusterSegmenter* AsFFI();
  inline static const ICU4XGraphemeClusterSegmenter* FromFFI(const capi::ICU4XGraphemeClusterSegmenter* ptr);
  inline static ICU4XGraphemeClusterSegmenter* FromFFI(capi::ICU4XGraphemeClusterSegmenter* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XGraphemeClusterSegmenter() = delete;
  ICU4XGraphemeClusterSegmenter(const ICU4XGraphemeClusterSegmenter&) = delete;
  ICU4XGraphemeClusterSegmenter(ICU4XGraphemeClusterSegmenter&&) noexcept = delete;
  ICU4XGraphemeClusterSegmenter operator=(const ICU4XGraphemeClusterSegmenter&) = delete;
  ICU4XGraphemeClusterSegmenter operator=(ICU4XGraphemeClusterSegmenter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XGraphemeClusterSegmenter_D_HPP
