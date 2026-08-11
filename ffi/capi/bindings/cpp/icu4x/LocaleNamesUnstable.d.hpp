#ifndef ICU4X_LocaleNamesUnstable_D_HPP
#define ICU4X_LocaleNamesUnstable_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"
namespace icu4x {
namespace capi { struct DataProvider; }
class DataProvider;
namespace capi { struct Locale; }
class Locale;
class DataError;
} // namespace icu4x



namespace icu4x {
namespace capi {
    struct LocaleNamesUnstable;
} // namespace capi
} // namespace

namespace icu4x {
class LocaleNamesUnstable {
public:

  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_region_with_compiled_data_light(const icu4x::Locale& locale, std::string_view region);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_region_with_compiled_data_light_write(const icu4x::Locale& locale, std::string_view region, W& writeable_output);

  inline static icu4x::diplomat::result<std::string, icu4x::DataError> for_region_with_provider_light(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view region);
  template<typename W>
  inline static icu4x::diplomat::result<std::monostate, icu4x::DataError> for_region_with_provider_light_write(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::string_view region, W& writeable_output);

    inline const icu4x::capi::LocaleNamesUnstable* AsFFI() const;
    inline icu4x::capi::LocaleNamesUnstable* AsFFI();
    inline static const icu4x::LocaleNamesUnstable* FromFFI(const icu4x::capi::LocaleNamesUnstable* ptr);
    inline static icu4x::LocaleNamesUnstable* FromFFI(icu4x::capi::LocaleNamesUnstable* ptr);
    inline static void operator delete(void* ptr);
private:
    LocaleNamesUnstable() = delete;
    LocaleNamesUnstable(const icu4x::LocaleNamesUnstable&) = delete;
    LocaleNamesUnstable(icu4x::LocaleNamesUnstable&&) noexcept = delete;
    LocaleNamesUnstable operator=(const icu4x::LocaleNamesUnstable&) = delete;
    LocaleNamesUnstable operator=(icu4x::LocaleNamesUnstable&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ICU4X_LocaleNamesUnstable_D_HPP
