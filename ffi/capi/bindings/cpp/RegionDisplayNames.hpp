#ifndef RegionDisplayNames_HPP
#define RegionDisplayNames_HPP

#include "RegionDisplayNames.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "Locale.hpp"
#include "LocaleParseError.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_RegionDisplayNames_create_mv1_result {union {diplomat::capi::RegionDisplayNames* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_RegionDisplayNames_create_mv1_result;
    icu4x_RegionDisplayNames_create_mv1_result icu4x_RegionDisplayNames_create_mv1(const diplomat::capi::DataProvider* provider, const diplomat::capi::Locale* locale);
    
    typedef struct icu4x_RegionDisplayNames_of_mv1_result {union { diplomat::capi::LocaleParseError err;}; bool is_ok;} icu4x_RegionDisplayNames_of_mv1_result;
    icu4x_RegionDisplayNames_of_mv1_result icu4x_RegionDisplayNames_of_mv1(const diplomat::capi::RegionDisplayNames* self, const char* region_data, size_t region_len, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_RegionDisplayNames_destroy_mv1(RegionDisplayNames* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<RegionDisplayNames>, DataError> RegionDisplayNames::create(const DataProvider& provider, const Locale& locale) {
  auto result = diplomat::capi::icu4x_RegionDisplayNames_create_mv1(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<RegionDisplayNames>, DataError>(diplomat::Ok<std::unique_ptr<RegionDisplayNames>>(std::unique_ptr<RegionDisplayNames>(RegionDisplayNames::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<RegionDisplayNames>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::string, LocaleParseError> RegionDisplayNames::of(std::string_view region) const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::icu4x_RegionDisplayNames_of_mv1(this->AsFFI(),
    region.data(),
    region.size(),
    &write);
  return result.is_ok ? diplomat::result<std::string, LocaleParseError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, LocaleParseError>(diplomat::Err<LocaleParseError>(LocaleParseError::FromFFI(result.err)));
}

inline const diplomat::capi::RegionDisplayNames* RegionDisplayNames::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::RegionDisplayNames*>(this);
}

inline diplomat::capi::RegionDisplayNames* RegionDisplayNames::AsFFI() {
  return reinterpret_cast<diplomat::capi::RegionDisplayNames*>(this);
}

inline const RegionDisplayNames* RegionDisplayNames::FromFFI(const diplomat::capi::RegionDisplayNames* ptr) {
  return reinterpret_cast<const RegionDisplayNames*>(ptr);
}

inline RegionDisplayNames* RegionDisplayNames::FromFFI(diplomat::capi::RegionDisplayNames* ptr) {
  return reinterpret_cast<RegionDisplayNames*>(ptr);
}

inline void RegionDisplayNames::operator delete(void* ptr) {
  diplomat::capi::icu4x_RegionDisplayNames_destroy_mv1(reinterpret_cast<diplomat::capi::RegionDisplayNames*>(ptr));
}


#endif // RegionDisplayNames_HPP
