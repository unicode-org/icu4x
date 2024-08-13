#ifndef icu4x_WordSegmenter_D_HPP
#define icu4x_WordSegmenter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct DataProvider; }
class DataProvider;
namespace capi { struct WordBreakIteratorLatin1; }
class WordBreakIteratorLatin1;
namespace capi { struct WordBreakIteratorUtf16; }
class WordBreakIteratorUtf16;
namespace capi { struct WordBreakIteratorUtf8; }
class WordBreakIteratorUtf8;
namespace capi { struct WordSegmenter; }
class WordSegmenter;
class DataError;
}


namespace icu4x {
namespace capi {
    struct WordSegmenter;
} // namespace capi
} // namespace

namespace icu4x {
class WordSegmenter {
public:

  inline static diplomat::result<std::unique_ptr<icu4x::WordSegmenter>, icu4x::DataError> create_auto(const icu4x::DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<icu4x::WordSegmenter>, icu4x::DataError> create_lstm(const icu4x::DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<icu4x::WordSegmenter>, icu4x::DataError> create_dictionary(const icu4x::DataProvider& provider);

  inline std::unique_ptr<icu4x::WordBreakIteratorUtf8> segment(std::string_view input) const;

  inline std::unique_ptr<icu4x::WordBreakIteratorUtf16> segment16(std::u16string_view input) const;

  inline std::unique_ptr<icu4x::WordBreakIteratorLatin1> segment_latin1(diplomat::span<const uint8_t> input) const;

  inline const icu4x::capi::WordSegmenter* AsFFI() const;
  inline icu4x::capi::WordSegmenter* AsFFI();
  inline static const icu4x::WordSegmenter* FromFFI(const icu4x::capi::WordSegmenter* ptr);
  inline static icu4x::WordSegmenter* FromFFI(icu4x::capi::WordSegmenter* ptr);
  inline static void operator delete(void* ptr);
private:
  WordSegmenter() = delete;
  WordSegmenter(const icu4x::WordSegmenter&) = delete;
  WordSegmenter(icu4x::WordSegmenter&&) noexcept = delete;
  WordSegmenter operator=(const icu4x::WordSegmenter&) = delete;
  WordSegmenter operator=(icu4x::WordSegmenter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_WordSegmenter_D_HPP
