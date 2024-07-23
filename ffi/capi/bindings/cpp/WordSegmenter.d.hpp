#ifndef WordSegmenter_D_HPP
#define WordSegmenter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct DataProvider; }
class DataProvider;
namespace diplomat::capi { struct WordBreakIteratorLatin1; }
class WordBreakIteratorLatin1;
namespace diplomat::capi { struct WordBreakIteratorUtf16; }
class WordBreakIteratorUtf16;
namespace diplomat::capi { struct WordBreakIteratorUtf8; }
class WordBreakIteratorUtf8;
class DataError;


namespace diplomat {
namespace capi {
    struct WordSegmenter;
} // namespace capi
} // namespace

class WordSegmenter {
public:

  inline static diplomat::result<std::unique_ptr<WordSegmenter>, DataError> create_auto(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<WordSegmenter>, DataError> create_lstm(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<WordSegmenter>, DataError> create_dictionary(const DataProvider& provider);

  inline std::unique_ptr<WordBreakIteratorUtf8> segment(std::string_view input) const;

  inline std::unique_ptr<WordBreakIteratorUtf16> segment16(std::u16string_view input) const;

  inline std::unique_ptr<WordBreakIteratorLatin1> segment_latin1(diplomat::span<const uint8_t> input) const;

  inline const diplomat::capi::WordSegmenter* AsFFI() const;
  inline diplomat::capi::WordSegmenter* AsFFI();
  inline static const WordSegmenter* FromFFI(const diplomat::capi::WordSegmenter* ptr);
  inline static WordSegmenter* FromFFI(diplomat::capi::WordSegmenter* ptr);
  inline static void operator delete(void* ptr);
private:
  WordSegmenter() = delete;
  WordSegmenter(const WordSegmenter&) = delete;
  WordSegmenter(WordSegmenter&&) noexcept = delete;
  WordSegmenter operator=(const WordSegmenter&) = delete;
  WordSegmenter operator=(WordSegmenter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // WordSegmenter_D_HPP
