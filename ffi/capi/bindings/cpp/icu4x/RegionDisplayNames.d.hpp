#ifndef icu4x_RegionDisplayNames_D_HPP
#define icu4x_RegionDisplayNames_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct DataProvider; }
class DataProvider;
namespace capi { struct Locale; }
class Locale;
namespace capi { struct RegionDisplayNames; }
class RegionDisplayNames;
struct DisplayNamesOptionsV1;
class DataError;
class LocaleParseError;
}


namespace icu4x {
namespace capi {
    struct RegionDisplayNames;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * See the [Rust documentation for `RegionDisplayNames`](https://docs.rs/icu/2.0.0/icu/experimental/displaynames/struct.RegionDisplayNames.html) for more information.
 */
class RegionDisplayNames {
public:

  /**
   * Creates a new `RegionDisplayNames` from locale data and an options bag using compiled data.
   *
   * See the [Rust documentation for `try_new`](https://docs.rs/icu/2.0.0/icu/experimental/displaynames/struct.RegionDisplayNames.html#method.try_new) for more information.
   */
  inline static diplomat::result<std::unique_ptr<icu4x::RegionDisplayNames>, icu4x::DataError> create_v1(const icu4x::Locale& locale, icu4x::DisplayNamesOptionsV1 options);

  /**
   * Creates a new `RegionDisplayNames` from locale data and an options bag using a particular data source.
   *
   * See the [Rust documentation for `try_new`](https://docs.rs/icu/2.0.0/icu/experimental/displaynames/struct.RegionDisplayNames.html#method.try_new) for more information.
   */
  inline static diplomat::result<std::unique_ptr<icu4x::RegionDisplayNames>, icu4x::DataError> create_v1_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::DisplayNamesOptionsV1 options);

  /**
   * Returns the locale specific display name of a region.
   * Note that the function returns an empty string in case the display name for a given
   * region code is not found.
   *
   * See the [Rust documentation for `of`](https://docs.rs/icu/2.0.0/icu/experimental/displaynames/struct.RegionDisplayNames.html#method.of) for more information.
   */
  inline diplomat::result<std::string, icu4x::LocaleParseError> of(std::string_view region) const;

  inline const icu4x::capi::RegionDisplayNames* AsFFI() const;
  inline icu4x::capi::RegionDisplayNames* AsFFI();
  inline static const icu4x::RegionDisplayNames* FromFFI(const icu4x::capi::RegionDisplayNames* ptr);
  inline static icu4x::RegionDisplayNames* FromFFI(icu4x::capi::RegionDisplayNames* ptr);
  inline static void operator delete(void* ptr);
private:
  RegionDisplayNames() = delete;
  RegionDisplayNames(const icu4x::RegionDisplayNames&) = delete;
  RegionDisplayNames(icu4x::RegionDisplayNames&&) noexcept = delete;
  RegionDisplayNames operator=(const icu4x::RegionDisplayNames&) = delete;
  RegionDisplayNames operator=(icu4x::RegionDisplayNames&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_RegionDisplayNames_D_HPP
