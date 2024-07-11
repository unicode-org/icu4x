#ifndef DecomposingNormalizer_D_HPP
#define DecomposingNormalizer_D_HPP

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
    typedef struct DecomposingNormalizer DecomposingNormalizer;
}

class DecomposingNormalizer {
public:

  inline static diplomat::result<std::unique_ptr<DecomposingNormalizer>, DataError> create_nfd(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<DecomposingNormalizer>, DataError> create_nfkd(const DataProvider& provider);

  inline std::string normalize(std::string_view s) const;

  inline bool is_normalized(std::string_view s) const;

  inline bool is_normalized_utf16(std::u16string_view s) const;

  inline size_t is_normalized_up_to(std::string_view s) const;

  inline size_t is_normalized_utf16_up_to(std::u16string_view s) const;

  inline const capi::DecomposingNormalizer* AsFFI() const;
  inline capi::DecomposingNormalizer* AsFFI();
  inline static const DecomposingNormalizer* FromFFI(const capi::DecomposingNormalizer* ptr);
  inline static DecomposingNormalizer* FromFFI(capi::DecomposingNormalizer* ptr);
  inline static void operator delete(void* ptr);
private:
  DecomposingNormalizer() = delete;
  DecomposingNormalizer(const DecomposingNormalizer&) = delete;
  DecomposingNormalizer(DecomposingNormalizer&&) noexcept = delete;
  DecomposingNormalizer operator=(const DecomposingNormalizer&) = delete;
  DecomposingNormalizer operator=(DecomposingNormalizer&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // DecomposingNormalizer_D_HPP
