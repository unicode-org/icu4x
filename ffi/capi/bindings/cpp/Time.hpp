#ifndef Time_HPP
#define Time_HPP

#include "Time.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CalendarError.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XTime_create_result {union {diplomat::capi::Time* ok; diplomat::capi::CalendarError err;}; bool is_ok;} ICU4XTime_create_result;
    ICU4XTime_create_result ICU4XTime_create(uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond);
    
    typedef struct ICU4XTime_create_midnight_result {union {diplomat::capi::Time* ok; diplomat::capi::CalendarError err;}; bool is_ok;} ICU4XTime_create_midnight_result;
    ICU4XTime_create_midnight_result ICU4XTime_create_midnight();
    
    uint8_t ICU4XTime_hour(const diplomat::capi::Time* self);
    
    uint8_t ICU4XTime_minute(const diplomat::capi::Time* self);
    
    uint8_t ICU4XTime_second(const diplomat::capi::Time* self);
    
    uint32_t ICU4XTime_nanosecond(const diplomat::capi::Time* self);
    
    
    void ICU4XTime_destroy(Time* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<Time>, CalendarError> Time::create(uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond) {
  auto result = diplomat::capi::ICU4XTime_create(hour,
    minute,
    second,
    nanosecond);
  return result.is_ok ? diplomat::result<std::unique_ptr<Time>, CalendarError>(diplomat::Ok<std::unique_ptr<Time>>(std::unique_ptr<Time>(Time::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Time>, CalendarError>(diplomat::Err<CalendarError>(CalendarError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<Time>, CalendarError> Time::create_midnight() {
  auto result = diplomat::capi::ICU4XTime_create_midnight();
  return result.is_ok ? diplomat::result<std::unique_ptr<Time>, CalendarError>(diplomat::Ok<std::unique_ptr<Time>>(std::unique_ptr<Time>(Time::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Time>, CalendarError>(diplomat::Err<CalendarError>(CalendarError::FromFFI(result.err)));
}

inline uint8_t Time::hour() const {
  auto result = diplomat::capi::ICU4XTime_hour(this->AsFFI());
  return result;
}

inline uint8_t Time::minute() const {
  auto result = diplomat::capi::ICU4XTime_minute(this->AsFFI());
  return result;
}

inline uint8_t Time::second() const {
  auto result = diplomat::capi::ICU4XTime_second(this->AsFFI());
  return result;
}

inline uint32_t Time::nanosecond() const {
  auto result = diplomat::capi::ICU4XTime_nanosecond(this->AsFFI());
  return result;
}

inline const diplomat::capi::Time* Time::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Time*>(this);
}

inline diplomat::capi::Time* Time::AsFFI() {
  return reinterpret_cast<diplomat::capi::Time*>(this);
}

inline const Time* Time::FromFFI(const diplomat::capi::Time* ptr) {
  return reinterpret_cast<const Time*>(ptr);
}

inline Time* Time::FromFFI(diplomat::capi::Time* ptr) {
  return reinterpret_cast<Time*>(ptr);
}

inline void Time::operator delete(void* ptr) {
  diplomat::capi::ICU4XTime_destroy(reinterpret_cast<diplomat::capi::Time*>(ptr));
}


#endif // Time_HPP
