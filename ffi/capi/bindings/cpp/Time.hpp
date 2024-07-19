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
#include "ParseError.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_Time_create_mv1_result {union {diplomat::capi::Time* ok; diplomat::capi::CalendarError err;}; bool is_ok;} icu4x_Time_create_mv1_result;
    icu4x_Time_create_mv1_result icu4x_Time_create_mv1(uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond);
    
    typedef struct icu4x_Time_from_string_mv1_result {union {diplomat::capi::Time* ok; diplomat::capi::ParseError err;}; bool is_ok;} icu4x_Time_from_string_mv1_result;
    icu4x_Time_from_string_mv1_result icu4x_Time_from_string_mv1(const char* v_data, size_t v_len);
    
    typedef struct icu4x_Time_midnight_mv1_result {union {diplomat::capi::Time* ok; diplomat::capi::CalendarError err;}; bool is_ok;} icu4x_Time_midnight_mv1_result;
    icu4x_Time_midnight_mv1_result icu4x_Time_midnight_mv1();
    
    uint8_t icu4x_Time_hour_mv1(const diplomat::capi::Time* self);
    
    uint8_t icu4x_Time_minute_mv1(const diplomat::capi::Time* self);
    
    uint8_t icu4x_Time_second_mv1(const diplomat::capi::Time* self);
    
    uint32_t icu4x_Time_nanosecond_mv1(const diplomat::capi::Time* self);
    
    
    void icu4x_Time_destroy_mv1(Time* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<Time>, CalendarError> Time::create(uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond) {
  auto result = diplomat::capi::icu4x_Time_create_mv1(hour,
    minute,
    second,
    nanosecond);
  return result.is_ok ? diplomat::result<std::unique_ptr<Time>, CalendarError>(diplomat::Ok<std::unique_ptr<Time>>(std::unique_ptr<Time>(Time::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Time>, CalendarError>(diplomat::Err<CalendarError>(CalendarError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<Time>, ParseError> Time::from_string(std::string_view v) {
  auto result = diplomat::capi::icu4x_Time_from_string_mv1(v.data(),
    v.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<Time>, ParseError>(diplomat::Ok<std::unique_ptr<Time>>(std::unique_ptr<Time>(Time::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Time>, ParseError>(diplomat::Err<ParseError>(ParseError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<Time>, CalendarError> Time::midnight() {
  auto result = diplomat::capi::icu4x_Time_midnight_mv1();
  return result.is_ok ? diplomat::result<std::unique_ptr<Time>, CalendarError>(diplomat::Ok<std::unique_ptr<Time>>(std::unique_ptr<Time>(Time::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<Time>, CalendarError>(diplomat::Err<CalendarError>(CalendarError::FromFFI(result.err)));
}

inline uint8_t Time::hour() const {
  auto result = diplomat::capi::icu4x_Time_hour_mv1(this->AsFFI());
  return result;
}

inline uint8_t Time::minute() const {
  auto result = diplomat::capi::icu4x_Time_minute_mv1(this->AsFFI());
  return result;
}

inline uint8_t Time::second() const {
  auto result = diplomat::capi::icu4x_Time_second_mv1(this->AsFFI());
  return result;
}

inline uint32_t Time::nanosecond() const {
  auto result = diplomat::capi::icu4x_Time_nanosecond_mv1(this->AsFFI());
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
  diplomat::capi::icu4x_Time_destroy_mv1(reinterpret_cast<diplomat::capi::Time*>(ptr));
}


#endif // Time_HPP
