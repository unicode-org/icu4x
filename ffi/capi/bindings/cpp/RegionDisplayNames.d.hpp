#ifndef RegionDisplayNames_D_HPP
#define RegionDisplayNames_D_HPP

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
class DataError;
class LocaleParseError;


namespace diplomat {
namespace capi {
    struct RegionDisplayNames;
} // namespace capi
} // namespace

class RegionDisplayNames {
public:

  inline static diplomat::result<std::unique_ptr<RegionDisplayNames>, DataError> create(const DataProvider& provider, const Locale& locale);

  inline diplomat::result<std::string, LocaleParseError> of(std::string_view region) const;

  inline const diplomat::capi::RegionDisplayNames* AsFFI() const;
  inline diplomat::capi::RegionDisplayNames* AsFFI();
  inline static const RegionDisplayNames* FromFFI(const diplomat::capi::RegionDisplayNames* ptr);
  inline static RegionDisplayNames* FromFFI(diplomat::capi::RegionDisplayNames* ptr);
  inline static void operator delete(void* ptr);
private:
  RegionDisplayNames() = delete;
  RegionDisplayNames(const RegionDisplayNames&) = delete;
  RegionDisplayNames(RegionDisplayNames&&) noexcept = delete;
  RegionDisplayNames operator=(const RegionDisplayNames&) = delete;
  RegionDisplayNames operator=(RegionDisplayNames&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // RegionDisplayNames_D_HPP
