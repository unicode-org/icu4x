#ifndef icu4x_TimeZone_HPP
#define icu4x_TimeZone_HPP

#include "TimeZone.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "TimeZoneInvalidIdError.hpp"
#include "TimeZoneInvalidOffsetError.hpp"
#include "TimeZoneUnknownError.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_TimeZone_from_string_mv1_result {union {icu4x::capi::TimeZone* ok; }; bool is_ok;} icu4x_TimeZone_from_string_mv1_result;
    icu4x_TimeZone_from_string_mv1_result icu4x_TimeZone_from_string_mv1(diplomat::capi::DiplomatStringView s);
    
    icu4x::capi::TimeZone* icu4x_TimeZone_utc_mv1(void);
    
    icu4x::capi::TimeZone* icu4x_TimeZone_create_mv1(int32_t offset_seconds, diplomat::capi::DiplomatStringView id);
    
    typedef struct icu4x_TimeZone_create_from_offset_seconds_mv1_result {union {icu4x::capi::TimeZone* ok; }; bool is_ok;} icu4x_TimeZone_create_from_offset_seconds_mv1_result;
    icu4x_TimeZone_create_from_offset_seconds_mv1_result icu4x_TimeZone_create_from_offset_seconds_mv1(int32_t offset_seconds);
    
    typedef struct icu4x_TimeZone_create_from_bcp47_id_mv1_result {union {icu4x::capi::TimeZone* ok; }; bool is_ok;} icu4x_TimeZone_create_from_bcp47_id_mv1_result;
    icu4x_TimeZone_create_from_bcp47_id_mv1_result icu4x_TimeZone_create_from_bcp47_id_mv1(diplomat::capi::DiplomatStringView id);
    
    typedef struct icu4x_TimeZone_offset_eighths_of_hour_mv1_result {union {int8_t ok; }; bool is_ok;} icu4x_TimeZone_offset_eighths_of_hour_mv1_result;
    icu4x_TimeZone_offset_eighths_of_hour_mv1_result icu4x_TimeZone_offset_eighths_of_hour_mv1(const icu4x::capi::TimeZone* self);
    
    typedef struct icu4x_TimeZone_offset_seconds_mv1_result {union {int32_t ok; }; bool is_ok;} icu4x_TimeZone_offset_seconds_mv1_result;
    icu4x_TimeZone_offset_seconds_mv1_result icu4x_TimeZone_offset_seconds_mv1(const icu4x::capi::TimeZone* self);
    
    typedef struct icu4x_TimeZone_is_offset_positive_mv1_result {union {bool ok; }; bool is_ok;} icu4x_TimeZone_is_offset_positive_mv1_result;
    icu4x_TimeZone_is_offset_positive_mv1_result icu4x_TimeZone_is_offset_positive_mv1(const icu4x::capi::TimeZone* self);
    
    typedef struct icu4x_TimeZone_is_offset_zero_mv1_result {union {bool ok; }; bool is_ok;} icu4x_TimeZone_is_offset_zero_mv1_result;
    icu4x_TimeZone_is_offset_zero_mv1_result icu4x_TimeZone_is_offset_zero_mv1(const icu4x::capi::TimeZone* self);
    
    typedef struct icu4x_TimeZone_offset_has_minutes_mv1_result {union {bool ok; }; bool is_ok;} icu4x_TimeZone_offset_has_minutes_mv1_result;
    icu4x_TimeZone_offset_has_minutes_mv1_result icu4x_TimeZone_offset_has_minutes_mv1(const icu4x::capi::TimeZone* self);
    
    typedef struct icu4x_TimeZone_offset_has_seconds_mv1_result {union {bool ok; }; bool is_ok;} icu4x_TimeZone_offset_has_seconds_mv1_result;
    icu4x_TimeZone_offset_has_seconds_mv1_result icu4x_TimeZone_offset_has_seconds_mv1(const icu4x::capi::TimeZone* self);
    
    typedef struct icu4x_TimeZone_bcp47_id_mv1_result { bool is_ok;} icu4x_TimeZone_bcp47_id_mv1_result;
    icu4x_TimeZone_bcp47_id_mv1_result icu4x_TimeZone_bcp47_id_mv1(const icu4x::capi::TimeZone* self, diplomat::capi::DiplomatWrite* write);
    
    
    void icu4x_TimeZone_destroy_mv1(TimeZone* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::TimeZone>, icu4x::TimeZoneUnknownError> icu4x::TimeZone::from_string(std::string_view s) {
  auto result = icu4x::capi::icu4x_TimeZone_from_string_mv1({s.data(), s.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::TimeZone>, icu4x::TimeZoneUnknownError>(diplomat::Ok<std::unique_ptr<icu4x::TimeZone>>(std::unique_ptr<icu4x::TimeZone>(icu4x::TimeZone::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::TimeZone>, icu4x::TimeZoneUnknownError>(diplomat::Err<icu4x::TimeZoneUnknownError>(icu4x::TimeZoneUnknownError {}));
}

inline std::unique_ptr<icu4x::TimeZone> icu4x::TimeZone::utc() {
  auto result = icu4x::capi::icu4x_TimeZone_utc_mv1();
  return std::unique_ptr<icu4x::TimeZone>(icu4x::TimeZone::FromFFI(result));
}

inline std::unique_ptr<icu4x::TimeZone> icu4x::TimeZone::create(int32_t offset_seconds, std::string_view id) {
  auto result = icu4x::capi::icu4x_TimeZone_create_mv1(offset_seconds,
    {id.data(), id.size()});
  return std::unique_ptr<icu4x::TimeZone>(icu4x::TimeZone::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<icu4x::TimeZone>, icu4x::TimeZoneInvalidOffsetError> icu4x::TimeZone::create_from_offset_seconds(int32_t offset_seconds) {
  auto result = icu4x::capi::icu4x_TimeZone_create_from_offset_seconds_mv1(offset_seconds);
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::TimeZone>, icu4x::TimeZoneInvalidOffsetError>(diplomat::Ok<std::unique_ptr<icu4x::TimeZone>>(std::unique_ptr<icu4x::TimeZone>(icu4x::TimeZone::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::TimeZone>, icu4x::TimeZoneInvalidOffsetError>(diplomat::Err<icu4x::TimeZoneInvalidOffsetError>(icu4x::TimeZoneInvalidOffsetError {}));
}

inline diplomat::result<std::unique_ptr<icu4x::TimeZone>, icu4x::TimeZoneInvalidIdError> icu4x::TimeZone::create_from_bcp47_id(std::string_view id) {
  auto result = icu4x::capi::icu4x_TimeZone_create_from_bcp47_id_mv1({id.data(), id.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::TimeZone>, icu4x::TimeZoneInvalidIdError>(diplomat::Ok<std::unique_ptr<icu4x::TimeZone>>(std::unique_ptr<icu4x::TimeZone>(icu4x::TimeZone::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::TimeZone>, icu4x::TimeZoneInvalidIdError>(diplomat::Err<icu4x::TimeZoneInvalidIdError>(icu4x::TimeZoneInvalidIdError {}));
}

inline std::optional<int8_t> icu4x::TimeZone::offset_eighths_of_hour() const {
  auto result = icu4x::capi::icu4x_TimeZone_offset_eighths_of_hour_mv1(this->AsFFI());
  return result.is_ok ? std::optional<int8_t>(result.ok) : std::nullopt;
}

inline std::optional<int32_t> icu4x::TimeZone::offset_seconds() const {
  auto result = icu4x::capi::icu4x_TimeZone_offset_seconds_mv1(this->AsFFI());
  return result.is_ok ? std::optional<int32_t>(result.ok) : std::nullopt;
}

inline std::optional<bool> icu4x::TimeZone::is_offset_positive() const {
  auto result = icu4x::capi::icu4x_TimeZone_is_offset_positive_mv1(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline std::optional<bool> icu4x::TimeZone::is_offset_zero() const {
  auto result = icu4x::capi::icu4x_TimeZone_is_offset_zero_mv1(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline std::optional<bool> icu4x::TimeZone::offset_has_minutes() const {
  auto result = icu4x::capi::icu4x_TimeZone_offset_has_minutes_mv1(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline std::optional<bool> icu4x::TimeZone::offset_has_seconds() const {
  auto result = icu4x::capi::icu4x_TimeZone_offset_has_seconds_mv1(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline std::optional<std::string> icu4x::TimeZone::bcp47_id() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = icu4x::capi::icu4x_TimeZone_bcp47_id_mv1(this->AsFFI(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline const icu4x::capi::TimeZone* icu4x::TimeZone::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::TimeZone*>(this);
}

inline icu4x::capi::TimeZone* icu4x::TimeZone::AsFFI() {
  return reinterpret_cast<icu4x::capi::TimeZone*>(this);
}

inline const icu4x::TimeZone* icu4x::TimeZone::FromFFI(const icu4x::capi::TimeZone* ptr) {
  return reinterpret_cast<const icu4x::TimeZone*>(ptr);
}

inline icu4x::TimeZone* icu4x::TimeZone::FromFFI(icu4x::capi::TimeZone* ptr) {
  return reinterpret_cast<icu4x::TimeZone*>(ptr);
}

inline void icu4x::TimeZone::operator delete(void* ptr) {
  icu4x::capi::icu4x_TimeZone_destroy_mv1(reinterpret_cast<icu4x::capi::TimeZone*>(ptr));
}


#endif // icu4x_TimeZone_HPP
