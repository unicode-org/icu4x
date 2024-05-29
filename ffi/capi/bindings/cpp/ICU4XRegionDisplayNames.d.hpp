#ifndef ICU4XRegionDisplayNames_D_HPP
#define ICU4XRegionDisplayNames_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.d.hpp"
#include "ICU4XLocaleParseError.d.hpp"
#include "ICU4XRegionDisplayNames.d.h"

class ICU4XDataProvider;
class ICU4XLocale;
class ICU4XDataError;
class ICU4XLocaleParseError;


class ICU4XRegionDisplayNames {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XRegionDisplayNames>, ICU4XDataError> create(const ICU4XDataProvider& provider, const ICU4XLocale& locale);

  inline diplomat::result<std::string, ICU4XLocaleParseError> of(std::string_view region) const;

  inline const capi::ICU4XRegionDisplayNames* AsFFI() const;
  inline capi::ICU4XRegionDisplayNames* AsFFI();
  inline static const ICU4XRegionDisplayNames* FromFFI(const capi::ICU4XRegionDisplayNames* ptr);
  inline static ICU4XRegionDisplayNames* FromFFI(capi::ICU4XRegionDisplayNames* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XRegionDisplayNames() = delete;
  ICU4XRegionDisplayNames(const ICU4XRegionDisplayNames&) = delete;
  ICU4XRegionDisplayNames(ICU4XRegionDisplayNames&&) noexcept = delete;
  ICU4XRegionDisplayNames operator=(const ICU4XRegionDisplayNames&) = delete;
  ICU4XRegionDisplayNames operator=(ICU4XRegionDisplayNames&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XRegionDisplayNames_D_HPP
