#ifndef icu4x_Time_HPP
#define icu4x_Time_HPP

#include "Time.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "CalendarError.hpp"
#include "CalendarParseError.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_Time_create_mv1_result {union {icu4x::capi::Time* ok; icu4x::capi::CalendarError err;}; bool is_ok;} icu4x_Time_create_mv1_result;
    icu4x_Time_create_mv1_result icu4x_Time_create_mv1(uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond);
    
    typedef struct icu4x_Time_from_string_mv1_result {union {icu4x::capi::Time* ok; icu4x::capi::CalendarParseError err;}; bool is_ok;} icu4x_Time_from_string_mv1_result;
    icu4x_Time_from_string_mv1_result icu4x_Time_from_string_mv1(const char* v_data, size_t v_len);
    
    typedef struct icu4x_Time_midnight_mv1_result {union {icu4x::capi::Time* ok; icu4x::capi::CalendarError err;}; bool is_ok;} icu4x_Time_midnight_mv1_result;
    icu4x_Time_midnight_mv1_result icu4x_Time_midnight_mv1(void);
    
    uint8_t icu4x_Time_hour_mv1(const icu4x::capi::Time* self);
    
    uint8_t icu4x_Time_minute_mv1(const icu4x::capi::Time* self);
    
    uint8_t icu4x_Time_second_mv1(const icu4x::capi::Time* self);
    
    uint32_t icu4x_Time_nanosecond_mv1(const icu4x::capi::Time* self);
    
    
    void icu4x_Time_destroy_mv1(Time* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::Time>, icu4x::CalendarError> icu4x::Time::create(uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond) {
  auto result = icu4x::capi::icu4x_Time_create_mv1(hour,
    minute,
    second,
    nanosecond);
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::Time>, icu4x::CalendarError>(diplomat::Ok<std::unique_ptr<icu4x::Time>>(std::unique_ptr<icu4x::Time>(icu4x::Time::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::Time>, icu4x::CalendarError>(diplomat::Err<icu4x::CalendarError>(icu4x::CalendarError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::Time>, icu4x::CalendarParseError> icu4x::Time::from_string(std::string_view v) {
  auto result = icu4x::capi::icu4x_Time_from_string_mv1(v.data(),
    v.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::Time>, icu4x::CalendarParseError>(diplomat::Ok<std::unique_ptr<icu4x::Time>>(std::unique_ptr<icu4x::Time>(icu4x::Time::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::Time>, icu4x::CalendarParseError>(diplomat::Err<icu4x::CalendarParseError>(icu4x::CalendarParseError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<icu4x::Time>, icu4x::CalendarError> icu4x::Time::midnight() {
  auto result = icu4x::capi::icu4x_Time_midnight_mv1();
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::Time>, icu4x::CalendarError>(diplomat::Ok<std::unique_ptr<icu4x::Time>>(std::unique_ptr<icu4x::Time>(icu4x::Time::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::Time>, icu4x::CalendarError>(diplomat::Err<icu4x::CalendarError>(icu4x::CalendarError::FromFFI(result.err)));
}

inline uint8_t icu4x::Time::hour() const {
  auto result = icu4x::capi::icu4x_Time_hour_mv1(this->AsFFI());
  return result;
}

inline uint8_t icu4x::Time::minute() const {
  auto result = icu4x::capi::icu4x_Time_minute_mv1(this->AsFFI());
  return result;
}

inline uint8_t icu4x::Time::second() const {
  auto result = icu4x::capi::icu4x_Time_second_mv1(this->AsFFI());
  return result;
}

inline uint32_t icu4x::Time::nanosecond() const {
  auto result = icu4x::capi::icu4x_Time_nanosecond_mv1(this->AsFFI());
  return result;
}

inline const icu4x::capi::Time* icu4x::Time::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::Time*>(this);
}

inline icu4x::capi::Time* icu4x::Time::AsFFI() {
  return reinterpret_cast<icu4x::capi::Time*>(this);
}

inline const icu4x::Time* icu4x::Time::FromFFI(const icu4x::capi::Time* ptr) {
  return reinterpret_cast<const icu4x::Time*>(ptr);
}

inline icu4x::Time* icu4x::Time::FromFFI(icu4x::capi::Time* ptr) {
  return reinterpret_cast<icu4x::Time*>(ptr);
}

inline void icu4x::Time::operator delete(void* ptr) {
  icu4x::capi::icu4x_Time_destroy_mv1(reinterpret_cast<icu4x::capi::Time*>(ptr));
}


#endif // icu4x_Time_HPP
