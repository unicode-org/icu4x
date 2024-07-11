#ifndef LocaleCanonicalizer_D_HPP
#define LocaleCanonicalizer_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "Error.d.hpp"
#include "TransformResult.d.hpp"

class DataProvider;
class Locale;
class Error;
class TransformResult;


namespace capi {
    typedef struct LocaleCanonicalizer LocaleCanonicalizer;
}

class LocaleCanonicalizer {
public:

  inline static diplomat::result<std::unique_ptr<LocaleCanonicalizer>, Error> create(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<LocaleCanonicalizer>, Error> create_extended(const DataProvider& provider);

  inline TransformResult canonicalize(Locale& locale) const;

  inline const capi::LocaleCanonicalizer* AsFFI() const;
  inline capi::LocaleCanonicalizer* AsFFI();
  inline static const LocaleCanonicalizer* FromFFI(const capi::LocaleCanonicalizer* ptr);
  inline static LocaleCanonicalizer* FromFFI(capi::LocaleCanonicalizer* ptr);
  inline static void operator delete(void* ptr);
private:
  LocaleCanonicalizer() = delete;
  LocaleCanonicalizer(const LocaleCanonicalizer&) = delete;
  LocaleCanonicalizer(LocaleCanonicalizer&&) noexcept = delete;
  LocaleCanonicalizer operator=(const LocaleCanonicalizer&) = delete;
  LocaleCanonicalizer operator=(LocaleCanonicalizer&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // LocaleCanonicalizer_D_HPP
