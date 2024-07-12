#ifndef LocaleCanonicalizer_D_HPP
#define LocaleCanonicalizer_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct DataProvider; }
class DataProvider;
namespace diplomat::capi { struct Locale; }
class Locale;
class Error;
class TransformResult;


namespace diplomat {
namespace capi {
    struct LocaleCanonicalizer;
} // namespace capi
} // namespace

class LocaleCanonicalizer {
public:

  inline static diplomat::result<std::unique_ptr<LocaleCanonicalizer>, Error> create(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<LocaleCanonicalizer>, Error> create_extended(const DataProvider& provider);

  inline TransformResult canonicalize(Locale& locale) const;

  inline const diplomat::capi::LocaleCanonicalizer* AsFFI() const;
  inline diplomat::capi::LocaleCanonicalizer* AsFFI();
  inline static const LocaleCanonicalizer* FromFFI(const diplomat::capi::LocaleCanonicalizer* ptr);
  inline static LocaleCanonicalizer* FromFFI(diplomat::capi::LocaleCanonicalizer* ptr);
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
