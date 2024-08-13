#ifndef icu4x_TitlecaseMapper_HPP
#define icu4x_TitlecaseMapper_HPP

#include "TitlecaseMapper.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "Locale.hpp"
#include "TitlecaseOptionsV1.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_TitlecaseMapper_create_mv1_result {union {icu4x::capi::TitlecaseMapper* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_TitlecaseMapper_create_mv1_result;
    icu4x_TitlecaseMapper_create_mv1_result icu4x_TitlecaseMapper_create_mv1(const icu4x::capi::DataProvider* provider);
    
    void icu4x_TitlecaseMapper_titlecase_segment_v1_mv1(const icu4x::capi::TitlecaseMapper* self, const char* s_data, size_t s_len, const icu4x::capi::Locale* locale, icu4x::capi::TitlecaseOptionsV1 options, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_TitlecaseMapper_destroy_mv1(TitlecaseMapper* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::TitlecaseMapper>, icu4x::DataError> icu4x::TitlecaseMapper::create(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_TitlecaseMapper_create_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::TitlecaseMapper>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::TitlecaseMapper>>(std::unique_ptr<icu4x::TitlecaseMapper>(icu4x::TitlecaseMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::TitlecaseMapper>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline diplomat::result<std::string, diplomat::Utf8Error> icu4x::TitlecaseMapper::titlecase_segment_v1(std::string_view s, const icu4x::Locale& locale, icu4x::TitlecaseOptionsV1 options) const {
  if (!diplomat::capi::diplomat_is_str(s.data(), s.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  icu4x::capi::icu4x_TitlecaseMapper_titlecase_segment_v1_mv1(this->AsFFI(),
    s.data(),
    s.size(),
    locale.AsFFI(),
    options.AsFFI(),
    &write);
  return diplomat::Ok<std::string>(std::move(output));
}

inline const icu4x::capi::TitlecaseMapper* icu4x::TitlecaseMapper::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::TitlecaseMapper*>(this);
}

inline icu4x::capi::TitlecaseMapper* icu4x::TitlecaseMapper::AsFFI() {
  return reinterpret_cast<icu4x::capi::TitlecaseMapper*>(this);
}

inline const icu4x::TitlecaseMapper* icu4x::TitlecaseMapper::FromFFI(const icu4x::capi::TitlecaseMapper* ptr) {
  return reinterpret_cast<const icu4x::TitlecaseMapper*>(ptr);
}

inline icu4x::TitlecaseMapper* icu4x::TitlecaseMapper::FromFFI(icu4x::capi::TitlecaseMapper* ptr) {
  return reinterpret_cast<icu4x::TitlecaseMapper*>(ptr);
}

inline void icu4x::TitlecaseMapper::operator delete(void* ptr) {
  icu4x::capi::icu4x_TitlecaseMapper_destroy_mv1(reinterpret_cast<icu4x::capi::TitlecaseMapper*>(ptr));
}


#endif // icu4x_TitlecaseMapper_HPP
