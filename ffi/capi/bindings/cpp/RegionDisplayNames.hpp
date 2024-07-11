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


namespace capi {
    extern "C" {
    
    typedef struct ICU4XRegionDisplayNames_create_result {union {RegionDisplayNames* ok; DataError err;}; bool is_ok;} ICU4XRegionDisplayNames_create_result;
    ICU4XRegionDisplayNames_create_result ICU4XRegionDisplayNames_create(const DataProvider* provider, const Locale* locale);
    
    typedef struct ICU4XRegionDisplayNames_of_result {union { LocaleParseError err;}; bool is_ok;} ICU4XRegionDisplayNames_of_result;
    ICU4XRegionDisplayNames_of_result ICU4XRegionDisplayNames_of(const RegionDisplayNames* self, const char* region_data, size_t region_len, DiplomatWrite* write);
    
    
    void ICU4XRegionDisplayNames_destroy(RegionDisplayNames* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<RegionDisplayNames>, DataError> RegionDisplayNames::create(const DataProvider& provider, const Locale& locale) {
  auto result = capi::ICU4XRegionDisplayNames_create(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<RegionDisplayNames>, DataError>(diplomat::Ok<std::unique_ptr<RegionDisplayNames>>(std::unique_ptr<RegionDisplayNames>(RegionDisplayNames::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<RegionDisplayNames>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::string, LocaleParseError> RegionDisplayNames::of(std::string_view region) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XRegionDisplayNames_of(this->AsFFI(),
    region.data(),
    region.size(),
    &write);
  return result.is_ok ? diplomat::result<std::string, LocaleParseError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, LocaleParseError>(diplomat::Err<LocaleParseError>(LocaleParseError::FromFFI(result.err)));
}

inline const capi::RegionDisplayNames* RegionDisplayNames::AsFFI() const {
  return reinterpret_cast<const capi::RegionDisplayNames*>(this);
}

inline capi::RegionDisplayNames* RegionDisplayNames::AsFFI() {
  return reinterpret_cast<capi::RegionDisplayNames*>(this);
}

inline const RegionDisplayNames* RegionDisplayNames::FromFFI(const capi::RegionDisplayNames* ptr) {
  return reinterpret_cast<const RegionDisplayNames*>(ptr);
}

inline RegionDisplayNames* RegionDisplayNames::FromFFI(capi::RegionDisplayNames* ptr) {
  return reinterpret_cast<RegionDisplayNames*>(ptr);
}

inline void RegionDisplayNames::operator delete(void* ptr) {
  capi::ICU4XRegionDisplayNames_destroy(reinterpret_cast<capi::RegionDisplayNames*>(ptr));
}


#endif // RegionDisplayNames_HPP
