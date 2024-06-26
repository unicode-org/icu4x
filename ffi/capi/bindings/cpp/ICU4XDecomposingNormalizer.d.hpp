#ifndef ICU4XDecomposingNormalizer_D_HPP
#define ICU4XDecomposingNormalizer_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.d.hpp"

class ICU4XDataProvider;
class ICU4XDataError;


namespace capi {
    typedef struct ICU4XDecomposingNormalizer ICU4XDecomposingNormalizer;
}

class ICU4XDecomposingNormalizer {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XDecomposingNormalizer>, ICU4XDataError> create_nfd(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XDecomposingNormalizer>, ICU4XDataError> create_nfkd(const ICU4XDataProvider& provider);

  inline std::string normalize(std::string_view s) const;

  inline bool is_normalized(std::string_view s) const;

  inline const capi::ICU4XDecomposingNormalizer* AsFFI() const;
  inline capi::ICU4XDecomposingNormalizer* AsFFI();
  inline static const ICU4XDecomposingNormalizer* FromFFI(const capi::ICU4XDecomposingNormalizer* ptr);
  inline static ICU4XDecomposingNormalizer* FromFFI(capi::ICU4XDecomposingNormalizer* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XDecomposingNormalizer() = delete;
  ICU4XDecomposingNormalizer(const ICU4XDecomposingNormalizer&) = delete;
  ICU4XDecomposingNormalizer(ICU4XDecomposingNormalizer&&) noexcept = delete;
  ICU4XDecomposingNormalizer operator=(const ICU4XDecomposingNormalizer&) = delete;
  ICU4XDecomposingNormalizer operator=(ICU4XDecomposingNormalizer&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XDecomposingNormalizer_D_HPP
