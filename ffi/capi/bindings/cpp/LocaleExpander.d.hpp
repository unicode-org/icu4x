#ifndef LocaleExpander_D_HPP
#define LocaleExpander_D_HPP

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
    typedef struct LocaleExpander LocaleExpander;
}

class LocaleExpander {
public:

  inline static diplomat::result<std::unique_ptr<LocaleExpander>, Error> create(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<LocaleExpander>, Error> create_extended(const DataProvider& provider);

  inline TransformResult maximize(Locale& locale) const;

  inline TransformResult minimize(Locale& locale) const;

  inline TransformResult minimize_favor_script(Locale& locale) const;

  inline const capi::LocaleExpander* AsFFI() const;
  inline capi::LocaleExpander* AsFFI();
  inline static const LocaleExpander* FromFFI(const capi::LocaleExpander* ptr);
  inline static LocaleExpander* FromFFI(capi::LocaleExpander* ptr);
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
