#ifndef CustomTimeZone_HPP
#define CustomTimeZone_HPP

#include "CustomTimeZone.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "IsoDateTime.hpp"
#include "MetazoneCalculator.hpp"
#include "TimeZoneIdMapper.hpp"
#include "TimeZoneInvalidIdError.hpp"
#include "TimeZoneInvalidOffsetError.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_CustomTimeZone_from_string_mv1_result {union {diplomat::capi::CustomTimeZone* ok; diplomat::capi::TimeZoneInvalidOffsetError err;}; bool is_ok;} icu4x_CustomTimeZone_from_string_mv1_result;
    icu4x_CustomTimeZone_from_string_mv1_result icu4x_CustomTimeZone_from_string_mv1(const char* s_data, size_t s_len);
    
    diplomat::capi::CustomTimeZone* icu4x_CustomTimeZone_empty_mv1();
    
    diplomat::capi::CustomTimeZone* icu4x_CustomTimeZone_utc_mv1();
    
    diplomat::capi::CustomTimeZone* icu4x_CustomTimeZone_gmt_mv1();
    
    diplomat::capi::CustomTimeZone* icu4x_CustomTimeZone_bst_mv1();
    
    typedef struct icu4x_CustomTimeZone_try_set_gmt_offset_seconds_mv1_result {union { diplomat::capi::TimeZoneInvalidOffsetError err;}; bool is_ok;} icu4x_CustomTimeZone_try_set_gmt_offset_seconds_mv1_result;
    icu4x_CustomTimeZone_try_set_gmt_offset_seconds_mv1_result icu4x_CustomTimeZone_try_set_gmt_offset_seconds_mv1(diplomat::capi::CustomTimeZone* self, int32_t offset_seconds);
    
    void icu4x_CustomTimeZone_set_gmt_offset_eighths_of_hour_mv1(diplomat::capi::CustomTimeZone* self, int8_t offset_eighths_of_hour);
    
    void icu4x_CustomTimeZone_clear_gmt_offset_mv1(diplomat::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_gmt_offset_seconds_mv1_result {union {int32_t ok; }; bool is_ok;} icu4x_CustomTimeZone_gmt_offset_seconds_mv1_result;
    icu4x_CustomTimeZone_gmt_offset_seconds_mv1_result icu4x_CustomTimeZone_gmt_offset_seconds_mv1(const diplomat::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_is_gmt_offset_positive_mv1_result {union {bool ok; }; bool is_ok;} icu4x_CustomTimeZone_is_gmt_offset_positive_mv1_result;
    icu4x_CustomTimeZone_is_gmt_offset_positive_mv1_result icu4x_CustomTimeZone_is_gmt_offset_positive_mv1(const diplomat::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_is_gmt_offset_zero_mv1_result {union {bool ok; }; bool is_ok;} icu4x_CustomTimeZone_is_gmt_offset_zero_mv1_result;
    icu4x_CustomTimeZone_is_gmt_offset_zero_mv1_result icu4x_CustomTimeZone_is_gmt_offset_zero_mv1(const diplomat::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_gmt_offset_has_minutes_mv1_result {union {bool ok; }; bool is_ok;} icu4x_CustomTimeZone_gmt_offset_has_minutes_mv1_result;
    icu4x_CustomTimeZone_gmt_offset_has_minutes_mv1_result icu4x_CustomTimeZone_gmt_offset_has_minutes_mv1(const diplomat::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_gmt_offset_has_seconds_mv1_result {union {bool ok; }; bool is_ok;} icu4x_CustomTimeZone_gmt_offset_has_seconds_mv1_result;
    icu4x_CustomTimeZone_gmt_offset_has_seconds_mv1_result icu4x_CustomTimeZone_gmt_offset_has_seconds_mv1(const diplomat::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_try_set_time_zone_id_mv1_result {union { diplomat::capi::TimeZoneInvalidIdError err;}; bool is_ok;} icu4x_CustomTimeZone_try_set_time_zone_id_mv1_result;
    icu4x_CustomTimeZone_try_set_time_zone_id_mv1_result icu4x_CustomTimeZone_try_set_time_zone_id_mv1(diplomat::capi::CustomTimeZone* self, const char* id_data, size_t id_len);
    
    typedef struct icu4x_CustomTimeZone_try_set_iana_time_zone_id_mv1_result {union { diplomat::capi::TimeZoneInvalidIdError err;}; bool is_ok;} icu4x_CustomTimeZone_try_set_iana_time_zone_id_mv1_result;
    icu4x_CustomTimeZone_try_set_iana_time_zone_id_mv1_result icu4x_CustomTimeZone_try_set_iana_time_zone_id_mv1(diplomat::capi::CustomTimeZone* self, const diplomat::capi::TimeZoneIdMapper* mapper, const char* id_data, size_t id_len);
    
    void icu4x_CustomTimeZone_clear_time_zone_id_mv1(diplomat::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_time_zone_id_mv1_result { bool is_ok;} icu4x_CustomTimeZone_time_zone_id_mv1_result;
    icu4x_CustomTimeZone_time_zone_id_mv1_result icu4x_CustomTimeZone_time_zone_id_mv1(const diplomat::capi::CustomTimeZone* self, diplomat::capi::DiplomatWrite* write);
    
    typedef struct icu4x_CustomTimeZone_try_set_metazone_id_mv1_result {union { diplomat::capi::TimeZoneInvalidIdError err;}; bool is_ok;} icu4x_CustomTimeZone_try_set_metazone_id_mv1_result;
    icu4x_CustomTimeZone_try_set_metazone_id_mv1_result icu4x_CustomTimeZone_try_set_metazone_id_mv1(diplomat::capi::CustomTimeZone* self, const char* id_data, size_t id_len);
    
    void icu4x_CustomTimeZone_clear_metazone_id_mv1(diplomat::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_metazone_id_mv1_result { bool is_ok;} icu4x_CustomTimeZone_metazone_id_mv1_result;
    icu4x_CustomTimeZone_metazone_id_mv1_result icu4x_CustomTimeZone_metazone_id_mv1(const diplomat::capi::CustomTimeZone* self, diplomat::capi::DiplomatWrite* write);
    
    typedef struct icu4x_CustomTimeZone_try_set_zone_variant_mv1_result { bool is_ok;} icu4x_CustomTimeZone_try_set_zone_variant_mv1_result;
    icu4x_CustomTimeZone_try_set_zone_variant_mv1_result icu4x_CustomTimeZone_try_set_zone_variant_mv1(diplomat::capi::CustomTimeZone* self, const char* id_data, size_t id_len);
    
    void icu4x_CustomTimeZone_clear_zone_variant_mv1(diplomat::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_zone_variant_mv1_result { bool is_ok;} icu4x_CustomTimeZone_zone_variant_mv1_result;
    icu4x_CustomTimeZone_zone_variant_mv1_result icu4x_CustomTimeZone_zone_variant_mv1(const diplomat::capi::CustomTimeZone* self, diplomat::capi::DiplomatWrite* write);
    
    void icu4x_CustomTimeZone_set_standard_time_mv1(diplomat::capi::CustomTimeZone* self);
    
    void icu4x_CustomTimeZone_set_daylight_time_mv1(diplomat::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_is_standard_time_mv1_result {union {bool ok; }; bool is_ok;} icu4x_CustomTimeZone_is_standard_time_mv1_result;
    icu4x_CustomTimeZone_is_standard_time_mv1_result icu4x_CustomTimeZone_is_standard_time_mv1(const diplomat::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_is_daylight_time_mv1_result {union {bool ok; }; bool is_ok;} icu4x_CustomTimeZone_is_daylight_time_mv1_result;
    icu4x_CustomTimeZone_is_daylight_time_mv1_result icu4x_CustomTimeZone_is_daylight_time_mv1(const diplomat::capi::CustomTimeZone* self);
    
    void icu4x_CustomTimeZone_maybe_calculate_metazone_mv1(diplomat::capi::CustomTimeZone* self, const diplomat::capi::MetazoneCalculator* metazone_calculator, const diplomat::capi::IsoDateTime* local_datetime);
    
    
    void icu4x_CustomTimeZone_destroy_mv1(CustomTimeZone* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<CustomTimeZone>, TimeZoneInvalidOffsetError> CustomTimeZone::from_string(std::string_view s) {
  auto result = diplomat::capi::icu4x_CustomTimeZone_from_string_mv1(s.data(),
    s.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<CustomTimeZone>, TimeZoneInvalidOffsetError>(diplomat::Ok<std::unique_ptr<CustomTimeZone>>(std::unique_ptr<CustomTimeZone>(CustomTimeZone::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CustomTimeZone>, TimeZoneInvalidOffsetError>(diplomat::Err<TimeZoneInvalidOffsetError>(TimeZoneInvalidOffsetError::FromFFI(result.err)));
}

inline std::unique_ptr<CustomTimeZone> CustomTimeZone::empty() {
  auto result = diplomat::capi::icu4x_CustomTimeZone_empty_mv1();
  return std::unique_ptr<CustomTimeZone>(CustomTimeZone::FromFFI(result));
}

inline std::unique_ptr<CustomTimeZone> CustomTimeZone::utc() {
  auto result = diplomat::capi::icu4x_CustomTimeZone_utc_mv1();
  return std::unique_ptr<CustomTimeZone>(CustomTimeZone::FromFFI(result));
}

inline std::unique_ptr<CustomTimeZone> CustomTimeZone::gmt() {
  auto result = diplomat::capi::icu4x_CustomTimeZone_gmt_mv1();
  return std::unique_ptr<CustomTimeZone>(CustomTimeZone::FromFFI(result));
}

inline std::unique_ptr<CustomTimeZone> CustomTimeZone::bst() {
  auto result = diplomat::capi::icu4x_CustomTimeZone_bst_mv1();
  return std::unique_ptr<CustomTimeZone>(CustomTimeZone::FromFFI(result));
}

inline diplomat::result<std::monostate, TimeZoneInvalidOffsetError> CustomTimeZone::try_set_gmt_offset_seconds(int32_t offset_seconds) {
  auto result = diplomat::capi::icu4x_CustomTimeZone_try_set_gmt_offset_seconds_mv1(this->AsFFI(),
    offset_seconds);
  return result.is_ok ? diplomat::result<std::monostate, TimeZoneInvalidOffsetError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, TimeZoneInvalidOffsetError>(diplomat::Err<TimeZoneInvalidOffsetError>(TimeZoneInvalidOffsetError::FromFFI(result.err)));
}

inline void CustomTimeZone::set_gmt_offset_eighths_of_hour(int8_t offset_eighths_of_hour) {
  diplomat::capi::icu4x_CustomTimeZone_set_gmt_offset_eighths_of_hour_mv1(this->AsFFI(),
    offset_eighths_of_hour);
}

inline void CustomTimeZone::clear_gmt_offset() {
  diplomat::capi::icu4x_CustomTimeZone_clear_gmt_offset_mv1(this->AsFFI());
}

inline std::optional<int32_t> CustomTimeZone::gmt_offset_seconds() const {
  auto result = diplomat::capi::icu4x_CustomTimeZone_gmt_offset_seconds_mv1(this->AsFFI());
  return result.is_ok ? std::optional<int32_t>(result.ok) : std::nullopt;
}

inline std::optional<bool> CustomTimeZone::is_gmt_offset_positive() const {
  auto result = diplomat::capi::icu4x_CustomTimeZone_is_gmt_offset_positive_mv1(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline std::optional<bool> CustomTimeZone::is_gmt_offset_zero() const {
  auto result = diplomat::capi::icu4x_CustomTimeZone_is_gmt_offset_zero_mv1(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline std::optional<bool> CustomTimeZone::gmt_offset_has_minutes() const {
  auto result = diplomat::capi::icu4x_CustomTimeZone_gmt_offset_has_minutes_mv1(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline std::optional<bool> CustomTimeZone::gmt_offset_has_seconds() const {
  auto result = diplomat::capi::icu4x_CustomTimeZone_gmt_offset_has_seconds_mv1(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline diplomat::result<std::monostate, TimeZoneInvalidIdError> CustomTimeZone::try_set_time_zone_id(std::string_view id) {
  auto result = diplomat::capi::icu4x_CustomTimeZone_try_set_time_zone_id_mv1(this->AsFFI(),
    id.data(),
    id.size());
  return result.is_ok ? diplomat::result<std::monostate, TimeZoneInvalidIdError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, TimeZoneInvalidIdError>(diplomat::Err<TimeZoneInvalidIdError>(TimeZoneInvalidIdError::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, TimeZoneInvalidIdError> CustomTimeZone::try_set_iana_time_zone_id(const TimeZoneIdMapper& mapper, std::string_view id) {
  auto result = diplomat::capi::icu4x_CustomTimeZone_try_set_iana_time_zone_id_mv1(this->AsFFI(),
    mapper.AsFFI(),
    id.data(),
    id.size());
  return result.is_ok ? diplomat::result<std::monostate, TimeZoneInvalidIdError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, TimeZoneInvalidIdError>(diplomat::Err<TimeZoneInvalidIdError>(TimeZoneInvalidIdError::FromFFI(result.err)));
}

inline void CustomTimeZone::clear_time_zone_id() {
  diplomat::capi::icu4x_CustomTimeZone_clear_time_zone_id_mv1(this->AsFFI());
}

inline std::optional<std::string> CustomTimeZone::time_zone_id() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::icu4x_CustomTimeZone_time_zone_id_mv1(this->AsFFI(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline diplomat::result<std::monostate, TimeZoneInvalidIdError> CustomTimeZone::try_set_metazone_id(std::string_view id) {
  auto result = diplomat::capi::icu4x_CustomTimeZone_try_set_metazone_id_mv1(this->AsFFI(),
    id.data(),
    id.size());
  return result.is_ok ? diplomat::result<std::monostate, TimeZoneInvalidIdError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, TimeZoneInvalidIdError>(diplomat::Err<TimeZoneInvalidIdError>(TimeZoneInvalidIdError::FromFFI(result.err)));
}

inline void CustomTimeZone::clear_metazone_id() {
  diplomat::capi::icu4x_CustomTimeZone_clear_metazone_id_mv1(this->AsFFI());
}

inline std::optional<std::string> CustomTimeZone::metazone_id() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::icu4x_CustomTimeZone_metazone_id_mv1(this->AsFFI(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline std::optional<std::monostate> CustomTimeZone::try_set_zone_variant(std::string_view id) {
  auto result = diplomat::capi::icu4x_CustomTimeZone_try_set_zone_variant_mv1(this->AsFFI(),
    id.data(),
    id.size());
  return result.is_ok ? std::optional<std::monostate>() : std::nullopt;
}

inline void CustomTimeZone::clear_zone_variant() {
  diplomat::capi::icu4x_CustomTimeZone_clear_zone_variant_mv1(this->AsFFI());
}

inline std::optional<std::string> CustomTimeZone::zone_variant() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = diplomat::capi::icu4x_CustomTimeZone_zone_variant_mv1(this->AsFFI(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline void CustomTimeZone::set_standard_time() {
  diplomat::capi::icu4x_CustomTimeZone_set_standard_time_mv1(this->AsFFI());
}

inline void CustomTimeZone::set_daylight_time() {
  diplomat::capi::icu4x_CustomTimeZone_set_daylight_time_mv1(this->AsFFI());
}

inline std::optional<bool> CustomTimeZone::is_standard_time() const {
  auto result = diplomat::capi::icu4x_CustomTimeZone_is_standard_time_mv1(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline std::optional<bool> CustomTimeZone::is_daylight_time() const {
  auto result = diplomat::capi::icu4x_CustomTimeZone_is_daylight_time_mv1(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline void CustomTimeZone::maybe_calculate_metazone(const MetazoneCalculator& metazone_calculator, const IsoDateTime& local_datetime) {
  diplomat::capi::icu4x_CustomTimeZone_maybe_calculate_metazone_mv1(this->AsFFI(),
    metazone_calculator.AsFFI(),
    local_datetime.AsFFI());
}

inline const diplomat::capi::CustomTimeZone* CustomTimeZone::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::CustomTimeZone*>(this);
}

inline diplomat::capi::CustomTimeZone* CustomTimeZone::AsFFI() {
  return reinterpret_cast<diplomat::capi::CustomTimeZone*>(this);
}

inline const CustomTimeZone* CustomTimeZone::FromFFI(const diplomat::capi::CustomTimeZone* ptr) {
  return reinterpret_cast<const CustomTimeZone*>(ptr);
}

inline CustomTimeZone* CustomTimeZone::FromFFI(diplomat::capi::CustomTimeZone* ptr) {
  return reinterpret_cast<CustomTimeZone*>(ptr);
}

inline void CustomTimeZone::operator delete(void* ptr) {
  diplomat::capi::icu4x_CustomTimeZone_destroy_mv1(reinterpret_cast<diplomat::capi::CustomTimeZone*>(ptr));
}


#endif // CustomTimeZone_HPP
