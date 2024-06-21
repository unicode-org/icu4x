#ifndef ICU4XTime_HPP
#define ICU4XTime_HPP

#include "ICU4XTime.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCalendarError.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XTime_create_result {union {ICU4XTime* ok; ICU4XCalendarError err;}; bool is_ok;} ICU4XTime_create_result;
    ICU4XTime_create_result ICU4XTime_create(uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond);
    
    typedef struct ICU4XTime_create_midnight_result {union {ICU4XTime* ok; ICU4XCalendarError err;}; bool is_ok;} ICU4XTime_create_midnight_result;
    ICU4XTime_create_midnight_result ICU4XTime_create_midnight();
    
    uint8_t ICU4XTime_hour(const ICU4XTime* self);
    
    uint8_t ICU4XTime_minute(const ICU4XTime* self);
    
    uint8_t ICU4XTime_second(const ICU4XTime* self);
    
    uint32_t ICU4XTime_nanosecond(const ICU4XTime* self);
    
    
    void ICU4XTime_destroy(ICU4XTime* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XTime>, ICU4XCalendarError> ICU4XTime::create(uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond) {
  auto result = capi::ICU4XTime_create(hour,
    minute,
    second,
    nanosecond);
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XTime>, ICU4XCalendarError>(diplomat::Ok<std::unique_ptr<ICU4XTime>>(std::unique_ptr<ICU4XTime>(ICU4XTime::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XTime>, ICU4XCalendarError>(diplomat::Err<ICU4XCalendarError>(ICU4XCalendarError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XTime>, ICU4XCalendarError> ICU4XTime::create_midnight() {
  auto result = capi::ICU4XTime_create_midnight();
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XTime>, ICU4XCalendarError>(diplomat::Ok<std::unique_ptr<ICU4XTime>>(std::unique_ptr<ICU4XTime>(ICU4XTime::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XTime>, ICU4XCalendarError>(diplomat::Err<ICU4XCalendarError>(ICU4XCalendarError::FromFFI(result.err)));
}

inline uint8_t ICU4XTime::hour() const {
  auto result = capi::ICU4XTime_hour(this->AsFFI());
  return result;
}

inline uint8_t ICU4XTime::minute() const {
  auto result = capi::ICU4XTime_minute(this->AsFFI());
  return result;
}

inline uint8_t ICU4XTime::second() const {
  auto result = capi::ICU4XTime_second(this->AsFFI());
  return result;
}

inline uint32_t ICU4XTime::nanosecond() const {
  auto result = capi::ICU4XTime_nanosecond(this->AsFFI());
  return result;
}

inline const capi::ICU4XTime* ICU4XTime::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XTime*>(this);
}

inline capi::ICU4XTime* ICU4XTime::AsFFI() {
  return reinterpret_cast<capi::ICU4XTime*>(this);
}

inline const ICU4XTime* ICU4XTime::FromFFI(const capi::ICU4XTime* ptr) {
  return reinterpret_cast<const ICU4XTime*>(ptr);
}

inline ICU4XTime* ICU4XTime::FromFFI(capi::ICU4XTime* ptr) {
  return reinterpret_cast<ICU4XTime*>(ptr);
}

inline void ICU4XTime::operator delete(void* ptr) {
  capi::ICU4XTime_destroy(reinterpret_cast<capi::ICU4XTime*>(ptr));
}


#endif // ICU4XTime_HPP
