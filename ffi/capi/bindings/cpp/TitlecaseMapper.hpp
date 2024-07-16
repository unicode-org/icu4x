#ifndef TitlecaseMapper_HPP
#define TitlecaseMapper_HPP

#include "TitlecaseMapper.d.hpp"

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
#include "TitlecaseOptionsV1.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XTitlecaseMapper_create_result {union {diplomat::capi::TitlecaseMapper* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XTitlecaseMapper_create_result;
    ICU4XTitlecaseMapper_create_result ICU4XTitlecaseMapper_create(const diplomat::capi::DataProvider* provider);
    
    void ICU4XTitlecaseMapper_titlecase_segment_v1(const diplomat::capi::TitlecaseMapper* self, const char* s_data, size_t s_len, const diplomat::capi::Locale* locale, diplomat::capi::TitlecaseOptionsV1 options, diplomat::capi::DiplomatWrite* write);
    
    
    void ICU4XTitlecaseMapper_destroy(TitlecaseMapper* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<TitlecaseMapper>, DataError> TitlecaseMapper::create(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XTitlecaseMapper_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<TitlecaseMapper>, DataError>(diplomat::Ok<std::unique_ptr<TitlecaseMapper>>(std::unique_ptr<TitlecaseMapper>(TitlecaseMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<TitlecaseMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::string, diplomat::Utf8Error> TitlecaseMapper::titlecase_segment_v1(std::string_view s, const Locale& locale, TitlecaseOptionsV1 options) const {
  if (!diplomat::capi::diplomat_is_str(s.data(), s.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  diplomat::capi::ICU4XTitlecaseMapper_titlecase_segment_v1(this->AsFFI(),
    s.data(),
    s.size(),
    locale.AsFFI(),
    options.AsFFI(),
    &write);
  return diplomat::Ok<std::string>(std::move(output));
}

inline const diplomat::capi::TitlecaseMapper* TitlecaseMapper::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::TitlecaseMapper*>(this);
}

inline diplomat::capi::TitlecaseMapper* TitlecaseMapper::AsFFI() {
  return reinterpret_cast<diplomat::capi::TitlecaseMapper*>(this);
}

inline const TitlecaseMapper* TitlecaseMapper::FromFFI(const diplomat::capi::TitlecaseMapper* ptr) {
  return reinterpret_cast<const TitlecaseMapper*>(ptr);
}

inline TitlecaseMapper* TitlecaseMapper::FromFFI(diplomat::capi::TitlecaseMapper* ptr) {
  return reinterpret_cast<TitlecaseMapper*>(ptr);
}

inline void TitlecaseMapper::operator delete(void* ptr) {
  diplomat::capi::ICU4XTitlecaseMapper_destroy(reinterpret_cast<diplomat::capi::TitlecaseMapper*>(ptr));
}


#endif // TitlecaseMapper_HPP
