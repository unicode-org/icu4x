#ifndef LocaleExpander_D_HPP
#define LocaleExpander_D_HPP

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
    struct LocaleExpander;
} // namespace capi
} // namespace

class LocaleExpander {
public:

  inline static diplomat::result<std::unique_ptr<LocaleExpander>, Error> create(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<LocaleExpander>, Error> create_extended(const DataProvider& provider);

  inline TransformResult maximize(Locale& locale) const;

  inline TransformResult minimize(Locale& locale) const;

  inline TransformResult minimize_favor_script(Locale& locale) const;

  inline const diplomat::capi::LocaleExpander* AsFFI() const;
  inline diplomat::capi::LocaleExpander* AsFFI();
  inline static const LocaleExpander* FromFFI(const diplomat::capi::LocaleExpander* ptr);
  inline static LocaleExpander* FromFFI(diplomat::capi::LocaleExpander* ptr);
  inline static void operator delete(void* ptr);
private:
  LocaleExpander() = delete;
  LocaleExpander(const LocaleExpander&) = delete;
  LocaleExpander(LocaleExpander&&) noexcept = delete;
  LocaleExpander operator=(const LocaleExpander&) = delete;
  LocaleExpander operator=(LocaleExpander&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // LocaleExpander_D_HPP
