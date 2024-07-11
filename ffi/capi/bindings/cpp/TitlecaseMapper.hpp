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


namespace capi {
    extern "C" {
    
    typedef struct ICU4XTitlecaseMapper_create_result {union {TitlecaseMapper* ok; DataError err;}; bool is_ok;} ICU4XTitlecaseMapper_create_result;
    ICU4XTitlecaseMapper_create_result ICU4XTitlecaseMapper_create(const DataProvider* provider);
    
    void ICU4XTitlecaseMapper_titlecase_segment_v1(const TitlecaseMapper* self, const char* s_data, size_t s_len, const Locale* locale, TitlecaseOptionsV1 options, DiplomatWrite* write);
    
    
    void ICU4XTitlecaseMapper_destroy(TitlecaseMapper* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<TitlecaseMapper>, DataError> TitlecaseMapper::create(const DataProvider& provider) {
  auto result = capi::ICU4XTitlecaseMapper_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<TitlecaseMapper>, DataError>(diplomat::Ok<std::unique_ptr<TitlecaseMapper>>(std::unique_ptr<TitlecaseMapper>(TitlecaseMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<TitlecaseMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::string, diplomat::Utf8Error> TitlecaseMapper::titlecase_segment_v1(std::string_view s, const Locale& locale, TitlecaseOptionsV1 options) const {
  if (!capi::diplomat_is_str(s.data(), s.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XTitlecaseMapper_titlecase_segment_v1(this->AsFFI(),
    s.data(),
    s.size(),
    locale.AsFFI(),
    options.AsFFI(),
    &write);
  return diplomat::Ok<std::string>(std::move(output));
}

inline const capi::TitlecaseMapper* TitlecaseMapper::AsFFI() const {
  return reinterpret_cast<const capi::TitlecaseMapper*>(this);
}

inline capi::TitlecaseMapper* TitlecaseMapper::AsFFI() {
  return reinterpret_cast<capi::TitlecaseMapper*>(this);
}

inline const TitlecaseMapper* TitlecaseMapper::FromFFI(const capi::TitlecaseMapper* ptr) {
  return reinterpret_cast<const TitlecaseMapper*>(ptr);
}

inline TitlecaseMapper* TitlecaseMapper::FromFFI(capi::TitlecaseMapper* ptr) {
  return reinterpret_cast<TitlecaseMapper*>(ptr);
}

inline void TitlecaseMapper::operator delete(void* ptr) {
  capi::ICU4XTitlecaseMapper_destroy(reinterpret_cast<capi::TitlecaseMapper*>(ptr));
}


#endif // TitlecaseMapper_HPP
