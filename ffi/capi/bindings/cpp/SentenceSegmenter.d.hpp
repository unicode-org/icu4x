#ifndef SentenceSegmenter_D_HPP
#define SentenceSegmenter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {typedef struct DataProvider DataProvider; }
class DataProvider;
namespace capi {typedef struct SentenceBreakIteratorLatin1 SentenceBreakIteratorLatin1; }
class SentenceBreakIteratorLatin1;
namespace capi {typedef struct SentenceBreakIteratorUtf16 SentenceBreakIteratorUtf16; }
class SentenceBreakIteratorUtf16;
namespace capi {typedef struct SentenceBreakIteratorUtf8 SentenceBreakIteratorUtf8; }
class SentenceBreakIteratorUtf8;
class DataError;


namespace diplomat {
namespace capi {
    typedef struct SentenceSegmenter SentenceSegmenter;
} // namespace capi
} // namespace

class SentenceSegmenter {
public:

  inline static diplomat::result<std::unique_ptr<SentenceSegmenter>, DataError> create(const DataProvider& provider);

  inline std::unique_ptr<SentenceBreakIteratorUtf8> segment_utf8(std::string_view input) const;

  inline std::unique_ptr<SentenceBreakIteratorUtf16> segment_utf16(std::u16string_view input) const;

  inline std::unique_ptr<SentenceBreakIteratorLatin1> segment_latin1(diplomat::span<const uint8_t> input) const;

  inline const diplomat::capi::SentenceSegmenter* AsFFI() const;
  inline diplomat::capi::SentenceSegmenter* AsFFI();
  inline static const SentenceSegmenter* FromFFI(const diplomat::capi::SentenceSegmenter* ptr);
  inline static SentenceSegmenter* FromFFI(diplomat::capi::SentenceSegmenter* ptr);
  inline static void operator delete(void* ptr);
private:
  SentenceSegmenter() = delete;
  SentenceSegmenter(const SentenceSegmenter&) = delete;
  SentenceSegmenter(SentenceSegmenter&&) noexcept = delete;
  SentenceSegmenter operator=(const SentenceSegmenter&) = delete;
  SentenceSegmenter operator=(SentenceSegmenter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // SentenceSegmenter_D_HPP
