#ifndef ICU4XTitlecaseMapper_HPP
#define ICU4XTitlecaseMapper_HPP

#include "ICU4XTitlecaseMapper.d.hpp"

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
#include "ICU4XTitlecaseOptionsV1.hpp"


namespace capi {
    extern "C" {
    
    struct ICU4XTitlecaseMapper_create_result {union {ICU4XTitlecaseMapper* ok; ICU4XDataError err;}; bool is_ok;};
    struct ICU4XTitlecaseMapper_create_result ICU4XTitlecaseMapper_create(const ICU4XDataProvider* provider);
    
    void ICU4XTitlecaseMapper_titlecase_segment_v1(const ICU4XTitlecaseMapper* self, const char* s_data, size_t s_len, const ICU4XLocale* locale, ICU4XTitlecaseOptionsV1 options, DiplomatWrite* write);
    
    
    void ICU4XTitlecaseMapper_destroy(ICU4XTitlecaseMapper* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XTitlecaseMapper>, ICU4XDataError> ICU4XTitlecaseMapper::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XTitlecaseMapper_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XTitlecaseMapper>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XTitlecaseMapper>>(std::unique_ptr<ICU4XTitlecaseMapper>(ICU4XTitlecaseMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XTitlecaseMapper>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::string, diplomat::Utf8Error> ICU4XTitlecaseMapper::titlecase_segment_v1(std::string_view s, const ICU4XLocale& locale, ICU4XTitlecaseOptionsV1 options) const {
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

inline const capi::ICU4XTitlecaseMapper* ICU4XTitlecaseMapper::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XTitlecaseMapper*>(this);
}

inline capi::ICU4XTitlecaseMapper* ICU4XTitlecaseMapper::AsFFI() {
  return reinterpret_cast<capi::ICU4XTitlecaseMapper*>(this);
}

inline const ICU4XTitlecaseMapper* ICU4XTitlecaseMapper::FromFFI(const capi::ICU4XTitlecaseMapper* ptr) {
  return reinterpret_cast<const ICU4XTitlecaseMapper*>(ptr);
}

inline ICU4XTitlecaseMapper* ICU4XTitlecaseMapper::FromFFI(capi::ICU4XTitlecaseMapper* ptr) {
  return reinterpret_cast<ICU4XTitlecaseMapper*>(ptr);
}

inline void ICU4XTitlecaseMapper::operator delete(void* ptr) {
  capi::ICU4XTitlecaseMapper_destroy(reinterpret_cast<capi::ICU4XTitlecaseMapper*>(ptr));
}


#endif // ICU4XTitlecaseMapper_HPP
