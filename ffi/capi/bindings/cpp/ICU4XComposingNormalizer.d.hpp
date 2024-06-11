#ifndef ICU4XComposingNormalizer_D_HPP
#define ICU4XComposingNormalizer_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XComposingNormalizer.d.h"
#include "ICU4XDataError.d.hpp"

class ICU4XDataProvider;
class ICU4XDataError;


class ICU4XComposingNormalizer {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XComposingNormalizer>, ICU4XDataError> create_nfc(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XComposingNormalizer>, ICU4XDataError> create_nfkc(const ICU4XDataProvider& provider);

  inline std::string normalize(std::string_view s) const;

  inline bool is_normalized(std::string_view s) const;

  inline const capi::ICU4XComposingNormalizer* AsFFI() const;
  inline capi::ICU4XComposingNormalizer* AsFFI();
  inline static const ICU4XComposingNormalizer* FromFFI(const capi::ICU4XComposingNormalizer* ptr);
  inline static ICU4XComposingNormalizer* FromFFI(capi::ICU4XComposingNormalizer* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XComposingNormalizer() = delete;
  ICU4XComposingNormalizer(const ICU4XComposingNormalizer&) = delete;
  ICU4XComposingNormalizer(ICU4XComposingNormalizer&&) noexcept = delete;
  ICU4XComposingNormalizer operator=(const ICU4XComposingNormalizer&) = delete;
  ICU4XComposingNormalizer operator=(ICU4XComposingNormalizer&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XComposingNormalizer_D_HPP
