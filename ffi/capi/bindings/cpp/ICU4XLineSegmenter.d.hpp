#ifndef ICU4XLineSegmenter_D_HPP
#define ICU4XLineSegmenter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XError.d.hpp"
#include "ICU4XLineBreakOptionsV1.d.hpp"
#include "ICU4XLineSegmenter.d.h"

class ICU4XDataProvider;
class ICU4XLineBreakIteratorLatin1;
class ICU4XLineBreakIteratorUtf16;
class ICU4XLineBreakIteratorUtf8;
struct ICU4XLineBreakOptionsV1;
class ICU4XError;


class ICU4XLineSegmenter {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XError> create_auto(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XError> create_lstm(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XError> create_dictionary(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XError> create_auto_with_options_v1(const ICU4XDataProvider& provider, ICU4XLineBreakOptionsV1 options);

  inline static diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XError> create_lstm_with_options_v1(const ICU4XDataProvider& provider, ICU4XLineBreakOptionsV1 options);

  inline static diplomat::result<std::unique_ptr<ICU4XLineSegmenter>, ICU4XError> create_dictionary_with_options_v1(const ICU4XDataProvider& provider, ICU4XLineBreakOptionsV1 options);

  inline std::unique_ptr<ICU4XLineBreakIteratorUtf8> segment_utf8(std::string_view input) const;

  inline std::unique_ptr<ICU4XLineBreakIteratorUtf16> segment_utf16(std::u16string_view input) const;

  inline std::unique_ptr<ICU4XLineBreakIteratorLatin1> segment_latin1(diplomat::span<const uint8_t> input) const;

  inline const capi::ICU4XLineSegmenter* AsFFI() const;
  inline capi::ICU4XLineSegmenter* AsFFI();
  inline static const ICU4XLineSegmenter* FromFFI(const capi::ICU4XLineSegmenter* ptr);
  inline static ICU4XLineSegmenter* FromFFI(capi::ICU4XLineSegmenter* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XLineSegmenter() = delete;
  ICU4XLineSegmenter(const ICU4XLineSegmenter&) = delete;
  ICU4XLineSegmenter(ICU4XLineSegmenter&&) noexcept = delete;
  ICU4XLineSegmenter operator=(const ICU4XLineSegmenter&) = delete;
  ICU4XLineSegmenter operator=(ICU4XLineSegmenter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XLineSegmenter_D_HPP
