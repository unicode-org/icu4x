#ifndef ComposingNormalizer_D_HPP
#define ComposingNormalizer_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.d.hpp"

class DataProvider;
class DataError;


namespace capi {
    typedef struct ComposingNormalizer ComposingNormalizer;
}

class ComposingNormalizer {
public:

  inline static diplomat::result<std::unique_ptr<ComposingNormalizer>, DataError> create_nfc(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ComposingNormalizer>, DataError> create_nfkc(const DataProvider& provider);

  inline std::string normalize(std::string_view s) const;

  inline bool is_normalized(std::string_view s) const;

  inline bool is_normalized_utf16(std::u16string_view s) const;

  inline size_t is_normalized_up_to(std::string_view s) const;

  inline size_t is_normalized_utf16_up_to(std::u16string_view s) const;

  inline const capi::ComposingNormalizer* AsFFI() const;
  inline capi::ComposingNormalizer* AsFFI();
  inline static const ComposingNormalizer* FromFFI(const capi::ComposingNormalizer* ptr);
  inline static ComposingNormalizer* FromFFI(capi::ComposingNormalizer* ptr);
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
