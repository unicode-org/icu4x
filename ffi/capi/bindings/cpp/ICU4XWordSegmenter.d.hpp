#ifndef ICU4XWordSegmenter_D_HPP
#define ICU4XWordSegmenter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XError.d.hpp"
#include "ICU4XWordSegmenter.d.h"

class ICU4XDataProvider;
class ICU4XWordBreakIteratorLatin1;
class ICU4XWordBreakIteratorUtf16;
class ICU4XWordBreakIteratorUtf8;
class ICU4XError;


class ICU4XWordSegmenter {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XWordSegmenter>, ICU4XError> create_auto(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XWordSegmenter>, ICU4XError> create_lstm(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XWordSegmenter>, ICU4XError> create_dictionary(const ICU4XDataProvider& provider);

  inline std::unique_ptr<ICU4XWordBreakIteratorUtf8> segment_utf8(std::string_view input) const;

  inline std::unique_ptr<ICU4XWordBreakIteratorUtf16> segment_utf16(std::u16string_view input) const;

  inline std::unique_ptr<ICU4XWordBreakIteratorLatin1> segment_latin1(diplomat::span<const uint8_t> input) const;

  inline const capi::ICU4XWordSegmenter* AsFFI() const;
  inline capi::ICU4XWordSegmenter* AsFFI();
  inline static const ICU4XWordSegmenter* FromFFI(const capi::ICU4XWordSegmenter* ptr);
  inline static ICU4XWordSegmenter* FromFFI(capi::ICU4XWordSegmenter* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XWordSegmenter() = delete;
  ICU4XWordSegmenter(const ICU4XWordSegmenter&) = delete;
  ICU4XWordSegmenter(ICU4XWordSegmenter&&) noexcept = delete;
  ICU4XWordSegmenter operator=(const ICU4XWordSegmenter&) = delete;
  ICU4XWordSegmenter operator=(ICU4XWordSegmenter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XWordSegmenter_D_HPP
