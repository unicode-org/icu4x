#ifndef ICU4XTimeFormatter_HPP
#define ICU4XTimeFormatter_HPP

#include "ICU4XTimeFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XDateTime.hpp"
#include "ICU4XError.hpp"
#include "ICU4XIsoDateTime.hpp"
#include "ICU4XLocale.hpp"
#include "ICU4XTime.hpp"
#include "ICU4XTimeLength.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XTimeFormatter_create_with_length_result {union {ICU4XTimeFormatter* ok; ICU4XError err;}; bool is_ok;} ICU4XTimeFormatter_create_with_length_result;
    ICU4XTimeFormatter_create_with_length_result ICU4XTimeFormatter_create_with_length(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XTimeLength length);
    
    void ICU4XTimeFormatter_format_time(const ICU4XTimeFormatter* self, const ICU4XTime* value, DiplomatWrite* write);
    
    void ICU4XTimeFormatter_format_datetime(const ICU4XTimeFormatter* self, const ICU4XDateTime* value, DiplomatWrite* write);
    
    void ICU4XTimeFormatter_format_iso_datetime(const ICU4XTimeFormatter* self, const ICU4XIsoDateTime* value, DiplomatWrite* write);
    
    
    void ICU4XTimeFormatter_destroy(ICU4XTimeFormatter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XTimeFormatter>, ICU4XError> ICU4XTimeFormatter::create_with_length(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XTimeLength length) {
  auto result = capi::ICU4XTimeFormatter_create_with_length(provider.AsFFI(),
    locale.AsFFI(),
    length.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XTimeFormatter>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XTimeFormatter>>(std::unique_ptr<ICU4XTimeFormatter>(ICU4XTimeFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XTimeFormatter>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline std::string ICU4XTimeFormatter::format_time(const ICU4XTime& value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XTimeFormatter_format_time(this->AsFFI(),
    value.AsFFI(),
    &write);
  return output;
}

inline std::string ICU4XTimeFormatter::format_datetime(const ICU4XDateTime& value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XTimeFormatter_format_datetime(this->AsFFI(),
    value.AsFFI(),
    &write);
  return output;
}

inline std::string ICU4XTimeFormatter::format_iso_datetime(const ICU4XIsoDateTime& value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XTimeFormatter_format_iso_datetime(this->AsFFI(),
    value.AsFFI(),
    &write);
  return output;
}

inline const capi::ICU4XTimeFormatter* ICU4XTimeFormatter::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XTimeFormatter*>(this);
}

inline capi::ICU4XTimeFormatter* ICU4XTimeFormatter::AsFFI() {
  return reinterpret_cast<capi::ICU4XTimeFormatter*>(this);
}

inline const ICU4XTimeFormatter* ICU4XTimeFormatter::FromFFI(const capi::ICU4XTimeFormatter* ptr) {
  return reinterpret_cast<const ICU4XTimeFormatter*>(ptr);
}

inline ICU4XTimeFormatter* ICU4XTimeFormatter::FromFFI(capi::ICU4XTimeFormatter* ptr) {
  return reinterpret_cast<ICU4XTimeFormatter*>(ptr);
}

inline void ICU4XTimeFormatter::operator delete(void* ptr) {
  capi::ICU4XTimeFormatter_destroy(reinterpret_cast<capi::ICU4XTimeFormatter*>(ptr));
}


#endif // ICU4XTimeFormatter_HPP
