#ifndef ICU4XRegionDisplayNames_HPP
#define ICU4XRegionDisplayNames_HPP

#include "ICU4XRegionDisplayNames.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XLocale.hpp"
#include "ICU4XLocaleParseError.hpp"


namespace capi {
    extern "C" {
    
    struct ICU4XRegionDisplayNames_create_result {union {ICU4XRegionDisplayNames* ok; ICU4XDataError err;}; bool is_ok;};
    struct ICU4XRegionDisplayNames_create_result ICU4XRegionDisplayNames_create(const ICU4XDataProvider* provider, const ICU4XLocale* locale);
    
    struct ICU4XRegionDisplayNames_of_result {union { ICU4XLocaleParseError err;}; bool is_ok;};
    struct ICU4XRegionDisplayNames_of_result ICU4XRegionDisplayNames_of(const ICU4XRegionDisplayNames* self, const char* region_data, size_t region_len, DiplomatWrite* write);
    
    
    void ICU4XRegionDisplayNames_destroy(ICU4XRegionDisplayNames* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XRegionDisplayNames>, ICU4XDataError> ICU4XRegionDisplayNames::create(const ICU4XDataProvider& provider, const ICU4XLocale& locale) {
  auto result = capi::ICU4XRegionDisplayNames_create(provider.AsFFI(),
    locale.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XRegionDisplayNames>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XRegionDisplayNames>>(std::unique_ptr<ICU4XRegionDisplayNames>(ICU4XRegionDisplayNames::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XRegionDisplayNames>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::string, ICU4XLocaleParseError> ICU4XRegionDisplayNames::of(std::string_view region) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XRegionDisplayNames_of(this->AsFFI(),
    region.data(),
    region.size(),
    &write);
  return result.is_ok ? diplomat::result<std::string, ICU4XLocaleParseError>(diplomat::Ok<std::string>(std::move(output))) : diplomat::result<std::string, ICU4XLocaleParseError>(diplomat::Err<ICU4XLocaleParseError>(ICU4XLocaleParseError::FromFFI(result.err)));
}

inline const capi::ICU4XRegionDisplayNames* ICU4XRegionDisplayNames::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XRegionDisplayNames*>(this);
}

inline capi::ICU4XRegionDisplayNames* ICU4XRegionDisplayNames::AsFFI() {
  return reinterpret_cast<capi::ICU4XRegionDisplayNames*>(this);
}

inline const ICU4XRegionDisplayNames* ICU4XRegionDisplayNames::FromFFI(const capi::ICU4XRegionDisplayNames* ptr) {
  return reinterpret_cast<const ICU4XRegionDisplayNames*>(ptr);
}

inline ICU4XRegionDisplayNames* ICU4XRegionDisplayNames::FromFFI(capi::ICU4XRegionDisplayNames* ptr) {
  return reinterpret_cast<ICU4XRegionDisplayNames*>(ptr);
}

inline void ICU4XRegionDisplayNames::operator delete(void* ptr) {
  capi::ICU4XRegionDisplayNames_destroy(reinterpret_cast<capi::ICU4XRegionDisplayNames*>(ptr));
}


#endif // ICU4XRegionDisplayNames_HPP
