#ifndef LineSegmenter_D_HPP
#define LineSegmenter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.d.hpp"
#include "LineBreakOptionsV1.d.hpp"

class DataProvider;
class LineBreakIteratorLatin1;
class LineBreakIteratorUtf16;
class LineBreakIteratorUtf8;
struct LineBreakOptionsV1;
class DataError;


namespace capi {
    typedef struct LineSegmenter LineSegmenter;
}

class LineSegmenter {
public:

  inline static diplomat::result<std::unique_ptr<LineSegmenter>, DataError> create_auto(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<LineSegmenter>, DataError> create_lstm(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<LineSegmenter>, DataError> create_dictionary(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<LineSegmenter>, DataError> create_auto_with_options_v1(const DataProvider& provider, LineBreakOptionsV1 options);

  inline static diplomat::result<std::unique_ptr<LineSegmenter>, DataError> create_lstm_with_options_v1(const DataProvider& provider, LineBreakOptionsV1 options);

  inline static diplomat::result<std::unique_ptr<LineSegmenter>, DataError> create_dictionary_with_options_v1(const DataProvider& provider, LineBreakOptionsV1 options);

  inline std::unique_ptr<LineBreakIteratorUtf8> segment_utf8(std::string_view input) const;

  inline std::unique_ptr<LineBreakIteratorUtf16> segment_utf16(std::u16string_view input) const;

  inline std::unique_ptr<LineBreakIteratorLatin1> segment_latin1(diplomat::span<const uint8_t> input) const;

  inline const capi::LineSegmenter* AsFFI() const;
  inline capi::LineSegmenter* AsFFI();
  inline static const LineSegmenter* FromFFI(const capi::LineSegmenter* ptr);
  inline static LineSegmenter* FromFFI(capi::LineSegmenter* ptr);
  inline static void operator delete(void* ptr);
private:
  LineSegmenter() = delete;
  LineSegmenter(const LineSegmenter&) = delete;
  LineSegmenter(LineSegmenter&&) noexcept = delete;
  LineSegmenter operator=(const LineSegmenter&) = delete;
  LineSegmenter operator=(LineSegmenter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // LineSegmenter_D_HPP
