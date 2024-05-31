#ifndef ICU4XLocaleCanonicalizer_D_HPP
#define ICU4XLocaleCanonicalizer_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XError.d.hpp"
#include "ICU4XLocaleCanonicalizer.d.h"
#include "ICU4XTransformResult.d.hpp"

class ICU4XDataProvider;
class ICU4XLocale;
class ICU4XError;
class ICU4XTransformResult;


class ICU4XLocaleCanonicalizer {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XLocaleCanonicalizer>, ICU4XError> create(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XLocaleCanonicalizer>, ICU4XError> create_extended(const ICU4XDataProvider& provider);

  inline ICU4XTransformResult canonicalize(ICU4XLocale& locale) const;

  inline const capi::ICU4XLocaleCanonicalizer* AsFFI() const;
  inline capi::ICU4XLocaleCanonicalizer* AsFFI();
  inline static const ICU4XLocaleCanonicalizer* FromFFI(const capi::ICU4XLocaleCanonicalizer* ptr);
  inline static ICU4XLocaleCanonicalizer* FromFFI(capi::ICU4XLocaleCanonicalizer* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XLocaleCanonicalizer() = delete;
  ICU4XLocaleCanonicalizer(const ICU4XLocaleCanonicalizer&) = delete;
  ICU4XLocaleCanonicalizer(ICU4XLocaleCanonicalizer&&) noexcept = delete;
  ICU4XLocaleCanonicalizer operator=(const ICU4XLocaleCanonicalizer&) = delete;
  ICU4XLocaleCanonicalizer operator=(ICU4XLocaleCanonicalizer&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XLocaleCanonicalizer_D_HPP
