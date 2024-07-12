#ifndef ComposingNormalizer_D_HPP
#define ComposingNormalizer_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct DataProvider; }
class DataProvider;
class DataError;


namespace diplomat {
namespace capi {
    struct ComposingNormalizer;
} // namespace capi
} // namespace

class ComposingNormalizer {
public:

  inline static diplomat::result<std::unique_ptr<ComposingNormalizer>, DataError> create_nfc(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ComposingNormalizer>, DataError> create_nfkc(const DataProvider& provider);

  inline std::string normalize(std::string_view s) const;

  inline bool is_normalized(std::string_view s) const;

  inline bool is_normalized_utf16(std::u16string_view s) const;

  inline size_t is_normalized_up_to(std::string_view s) const;

  inline size_t is_normalized_utf16_up_to(std::u16string_view s) const;

  inline const diplomat::capi::ComposingNormalizer* AsFFI() const;
  inline diplomat::capi::ComposingNormalizer* AsFFI();
  inline static const ComposingNormalizer* FromFFI(const diplomat::capi::ComposingNormalizer* ptr);
  inline static ComposingNormalizer* FromFFI(diplomat::capi::ComposingNormalizer* ptr);
  inline static void operator delete(void* ptr);
private:
  ComposingNormalizer() = delete;
  ComposingNormalizer(const ComposingNormalizer&) = delete;
  ComposingNormalizer(ComposingNormalizer&&) noexcept = delete;
  ComposingNormalizer operator=(const ComposingNormalizer&) = delete;
  ComposingNormalizer operator=(ComposingNormalizer&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ComposingNormalizer_D_HPP
