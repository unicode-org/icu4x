#ifndef ICU4XLocaleExpander_D_HPP
#define ICU4XLocaleExpander_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XError.d.hpp"
#include "ICU4XTransformResult.d.hpp"

class ICU4XDataProvider;
class ICU4XLocale;
class ICU4XError;
class ICU4XTransformResult;


namespace capi {
    typedef struct ICU4XLocaleExpander ICU4XLocaleExpander;
}

class ICU4XLocaleExpander {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XLocaleExpander>, ICU4XError> create(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XLocaleExpander>, ICU4XError> create_extended(const ICU4XDataProvider& provider);

  inline ICU4XTransformResult maximize(ICU4XLocale& locale) const;

  inline ICU4XTransformResult minimize(ICU4XLocale& locale) const;

  inline ICU4XTransformResult minimize_favor_script(ICU4XLocale& locale) const;

  inline const capi::ICU4XLocaleExpander* AsFFI() const;
  inline capi::ICU4XLocaleExpander* AsFFI();
  inline static const ICU4XLocaleExpander* FromFFI(const capi::ICU4XLocaleExpander* ptr);
  inline static ICU4XLocaleExpander* FromFFI(capi::ICU4XLocaleExpander* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XLocaleExpander() = delete;
  ICU4XLocaleExpander(const ICU4XLocaleExpander&) = delete;
  ICU4XLocaleExpander(ICU4XLocaleExpander&&) noexcept = delete;
  ICU4XLocaleExpander operator=(const ICU4XLocaleExpander&) = delete;
  ICU4XLocaleExpander operator=(ICU4XLocaleExpander&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XLocaleExpander_D_HPP
