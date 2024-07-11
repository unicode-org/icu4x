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


namespace capi {
    extern "C" {
    
    typedef struct ICU4XCustomTimeZone_create_from_string_result {union {CustomTimeZone* ok; TimeZoneInvalidOffsetError err;}; bool is_ok;} ICU4XCustomTimeZone_create_from_string_result;
    ICU4XCustomTimeZone_create_from_string_result ICU4XCustomTimeZone_create_from_string(const char* s_data, size_t s_len);
    
    CustomTimeZone* ICU4XCustomTimeZone_create_empty();
    
    CustomTimeZone* ICU4XCustomTimeZone_create_utc();
    
    CustomTimeZone* ICU4XCustomTimeZone_create_gmt();
    
    CustomTimeZone* ICU4XCustomTimeZone_create_bst();
    
    typedef struct ICU4XCustomTimeZone_try_set_gmt_offset_seconds_result {union { TimeZoneInvalidOffsetError err;}; bool is_ok;} ICU4XCustomTimeZone_try_set_gmt_offset_seconds_result;
    ICU4XCustomTimeZone_try_set_gmt_offset_seconds_result ICU4XCustomTimeZone_try_set_gmt_offset_seconds(CustomTimeZone* self, int32_t offset_seconds);
    
    void ICU4XCustomTimeZone_clear_gmt_offset(CustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_gmt_offset_seconds_result {union {int32_t ok; }; bool is_ok;} ICU4XCustomTimeZone_gmt_offset_seconds_result;
    ICU4XCustomTimeZone_gmt_offset_seconds_result ICU4XCustomTimeZone_gmt_offset_seconds(const CustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_is_gmt_offset_positive_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_is_gmt_offset_positive_result;
    ICU4XCustomTimeZone_is_gmt_offset_positive_result ICU4XCustomTimeZone_is_gmt_offset_positive(const CustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_is_gmt_offset_zero_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_is_gmt_offset_zero_result;
    ICU4XCustomTimeZone_is_gmt_offset_zero_result ICU4XCustomTimeZone_is_gmt_offset_zero(const CustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_gmt_offset_has_minutes_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_gmt_offset_has_minutes_result;
    ICU4XCustomTimeZone_gmt_offset_has_minutes_result ICU4XCustomTimeZone_gmt_offset_has_minutes(const CustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_gmt_offset_has_seconds_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_gmt_offset_has_seconds_result;
    ICU4XCustomTimeZone_gmt_offset_has_seconds_result ICU4XCustomTimeZone_gmt_offset_has_seconds(const CustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_try_set_time_zone_id_result {union { TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XCustomTimeZone_try_set_time_zone_id_result;
    ICU4XCustomTimeZone_try_set_time_zone_id_result ICU4XCustomTimeZone_try_set_time_zone_id(CustomTimeZone* self, const char* id_data, size_t id_len);
    
    typedef struct ICU4XCustomTimeZone_try_set_iana_time_zone_id_result {union { TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XCustomTimeZone_try_set_iana_time_zone_id_result;
    ICU4XCustomTimeZone_try_set_iana_time_zone_id_result ICU4XCustomTimeZone_try_set_iana_time_zone_id(CustomTimeZone* self, const TimeZoneIdMapper* mapper, const char* id_data, size_t id_len);
    
    void ICU4XCustomTimeZone_clear_time_zone_id(CustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_time_zone_id_result { bool is_ok;} ICU4XCustomTimeZone_time_zone_id_result;
    ICU4XCustomTimeZone_time_zone_id_result ICU4XCustomTimeZone_time_zone_id(const CustomTimeZone* self, DiplomatWrite* write);
    
    typedef struct ICU4XCustomTimeZone_try_set_metazone_id_result {union { TimeZoneInvalidIdError err;}; bool is_ok;} ICU4XCustomTimeZone_try_set_metazone_id_result;
    ICU4XCustomTimeZone_try_set_metazone_id_result ICU4XCustomTimeZone_try_set_metazone_id(CustomTimeZone* self, const char* id_data, size_t id_len);
    
    void ICU4XCustomTimeZone_clear_metazone_id(CustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_metazone_id_result { bool is_ok;} ICU4XCustomTimeZone_metazone_id_result;
    ICU4XCustomTimeZone_metazone_id_result ICU4XCustomTimeZone_metazone_id(const CustomTimeZone* self, DiplomatWrite* write);
    
    typedef struct ICU4XCustomTimeZone_try_set_zone_variant_result { bool is_ok;} ICU4XCustomTimeZone_try_set_zone_variant_result;
    ICU4XCustomTimeZone_try_set_zone_variant_result ICU4XCustomTimeZone_try_set_zone_variant(CustomTimeZone* self, const char* id_data, size_t id_len);
    
    void ICU4XCustomTimeZone_clear_zone_variant(CustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_zone_variant_result { bool is_ok;} ICU4XCustomTimeZone_zone_variant_result;
    ICU4XCustomTimeZone_zone_variant_result ICU4XCustomTimeZone_zone_variant(const CustomTimeZone* self, DiplomatWrite* write);
    
    void ICU4XCustomTimeZone_set_standard_time(CustomTimeZone* self);
    
    void ICU4XCustomTimeZone_set_daylight_time(CustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_is_standard_time_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_is_standard_time_result;
    ICU4XCustomTimeZone_is_standard_time_result ICU4XCustomTimeZone_is_standard_time(const CustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_is_daylight_time_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_is_daylight_time_result;
    ICU4XCustomTimeZone_is_daylight_time_result ICU4XCustomTimeZone_is_daylight_time(const CustomTimeZone* self);
    
    void ICU4XCustomTimeZone_maybe_calculate_metazone(CustomTimeZone* self, const MetazoneCalculator* metazone_calculator, const IsoDateTime* local_datetime);
    
    
    void ICU4XCustomTimeZone_destroy(CustomTimeZone* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<CustomTimeZone>, TimeZoneInvalidOffsetError> CustomTimeZone::create_from_string(std::string_view s) {
  auto result = capi::ICU4XCustomTimeZone_create_from_string(s.data(),
    s.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<CustomTimeZone>, TimeZoneInvalidOffsetError>(diplomat::Ok<std::unique_ptr<CustomTimeZone>>(std::unique_ptr<CustomTimeZone>(CustomTimeZone::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CustomTimeZone>, TimeZoneInvalidOffsetError>(diplomat::Err<TimeZoneInvalidOffsetError>(TimeZoneInvalidOffsetError::FromFFI(result.err)));
}

inline std::unique_ptr<CustomTimeZone> CustomTimeZone::create_empty() {
  auto result = capi::ICU4XCustomTimeZone_create_empty();
  return std::unique_ptr<CustomTimeZone>(CustomTimeZone::FromFFI(result));
}

inline std::unique_ptr<CustomTimeZone> CustomTimeZone::create_utc() {
  auto result = capi::ICU4XCustomTimeZone_create_utc();
  return std::unique_ptr<CustomTimeZone>(CustomTimeZone::FromFFI(result));
}

inline std::unique_ptr<CustomTimeZone> CustomTimeZone::create_gmt() {
  auto result = capi::ICU4XCustomTimeZone_create_gmt();
  return std::unique_ptr<CustomTimeZone>(CustomTimeZone::FromFFI(result));
}

inline std::unique_ptr<CustomTimeZone> CustomTimeZone::create_bst() {
  auto result = capi::ICU4XCustomTimeZone_create_bst();
  return std::unique_ptr<CustomTimeZone>(CustomTimeZone::FromFFI(result));
}

inline diplomat::result<std::monostate, TimeZoneInvalidOffsetError> CustomTimeZone::try_set_gmt_offset_seconds(int32_t offset_seconds) {
  auto result = capi::ICU4XCustomTimeZone_try_set_gmt_offset_seconds(this->AsFFI(),
    offset_seconds);
  return result.is_ok ? diplomat::result<std::monostate, TimeZoneInvalidOffsetError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, TimeZoneInvalidOffsetError>(diplomat::Err<TimeZoneInvalidOffsetError>(TimeZoneInvalidOffsetError::FromFFI(result.err)));
}

inline void CustomTimeZone::clear_gmt_offset() {
  capi::ICU4XCustomTimeZone_clear_gmt_offset(this->AsFFI());
}

inline std::optional<int32_t> CustomTimeZone::gmt_offset_seconds() const {
  auto result = capi::ICU4XCustomTimeZone_gmt_offset_seconds(this->AsFFI());
  return result.is_ok ? std::optional<int32_t>(result.ok) : std::nullopt;
}

inline std::optional<bool> CustomTimeZone::is_gmt_offset_positive() const {
  auto result = capi::ICU4XCustomTimeZone_is_gmt_offset_positive(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline std::optional<bool> CustomTimeZone::is_gmt_offset_zero() const {
  auto result = capi::ICU4XCustomTimeZone_is_gmt_offset_zero(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline std::optional<bool> CustomTimeZone::gmt_offset_has_minutes() const {
  auto result = capi::ICU4XCustomTimeZone_gmt_offset_has_minutes(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline std::optional<bool> CustomTimeZone::gmt_offset_has_seconds() const {
  auto result = capi::ICU4XCustomTimeZone_gmt_offset_has_seconds(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline diplomat::result<std::monostate, TimeZoneInvalidIdError> CustomTimeZone::try_set_time_zone_id(std::string_view id) {
  auto result = capi::ICU4XCustomTimeZone_try_set_time_zone_id(this->AsFFI(),
    id.data(),
    id.size());
  return result.is_ok ? diplomat::result<std::monostate, TimeZoneInvalidIdError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, TimeZoneInvalidIdError>(diplomat::Err<TimeZoneInvalidIdError>(TimeZoneInvalidIdError::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, TimeZoneInvalidIdError> CustomTimeZone::try_set_iana_time_zone_id(const TimeZoneIdMapper& mapper, std::string_view id) {
  auto result = capi::ICU4XCustomTimeZone_try_set_iana_time_zone_id(this->AsFFI(),
    mapper.AsFFI(),
    id.data(),
    id.size());
  return result.is_ok ? diplomat::result<std::monostate, TimeZoneInvalidIdError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, TimeZoneInvalidIdError>(diplomat::Err<TimeZoneInvalidIdError>(TimeZoneInvalidIdError::FromFFI(result.err)));
}

inline void CustomTimeZone::clear_time_zone_id() {
  capi::ICU4XCustomTimeZone_clear_time_zone_id(this->AsFFI());
}

inline std::optional<std::string> CustomTimeZone::time_zone_id() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XCustomTimeZone_time_zone_id(this->AsFFI(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline diplomat::result<std::monostate, TimeZoneInvalidIdError> CustomTimeZone::try_set_metazone_id(std::string_view id) {
  auto result = capi::ICU4XCustomTimeZone_try_set_metazone_id(this->AsFFI(),
    id.data(),
    id.size());
  return result.is_ok ? diplomat::result<std::monostate, TimeZoneInvalidIdError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, TimeZoneInvalidIdError>(diplomat::Err<TimeZoneInvalidIdError>(TimeZoneInvalidIdError::FromFFI(result.err)));
}

inline void CustomTimeZone::clear_metazone_id() {
  capi::ICU4XCustomTimeZone_clear_metazone_id(this->AsFFI());
}

inline std::optional<std::string> CustomTimeZone::metazone_id() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XCustomTimeZone_metazone_id(this->AsFFI(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline std::optional<std::monostate> CustomTimeZone::try_set_zone_variant(std::string_view id) {
  auto result = capi::ICU4XCustomTimeZone_try_set_zone_variant(this->AsFFI(),
    id.data(),
    id.size());
  return result.is_ok ? std::optional<std::monostate>() : std::nullopt;
}

inline void CustomTimeZone::clear_zone_variant() {
  capi::ICU4XCustomTimeZone_clear_zone_variant(this->AsFFI());
}

inline std::optional<std::string> CustomTimeZone::zone_variant() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XCustomTimeZone_zone_variant(this->AsFFI(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline void CustomTimeZone::set_standard_time() {
  capi::ICU4XCustomTimeZone_set_standard_time(this->AsFFI());
}

inline void CustomTimeZone::set_daylight_time() {
  capi::ICU4XCustomTimeZone_set_daylight_time(this->AsFFI());
}

inline std::optional<bool> CustomTimeZone::is_standard_time() const {
  auto result = capi::ICU4XCustomTimeZone_is_standard_time(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline std::optional<bool> CustomTimeZone::is_daylight_time() const {
  auto result = capi::ICU4XCustomTimeZone_is_daylight_time(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline void CustomTimeZone::maybe_calculate_metazone(const MetazoneCalculator& metazone_calculator, const IsoDateTime& local_datetime) {
  capi::ICU4XCustomTimeZone_maybe_calculate_metazone(this->AsFFI(),
    metazone_calculator.AsFFI(),
    local_datetime.AsFFI());
}

inline const capi::CustomTimeZone* CustomTimeZone::AsFFI() const {
  return reinterpret_cast<const capi::CustomTimeZone*>(this);
}

inline capi::CustomTimeZone* CustomTimeZone::AsFFI() {
  return reinterpret_cast<capi::CustomTimeZone*>(this);
}

inline const CustomTimeZone* CustomTimeZone::FromFFI(const capi::CustomTimeZone* ptr) {
  return reinterpret_cast<const CustomTimeZone*>(ptr);
}

inline CustomTimeZone* CustomTimeZone::FromFFI(capi::CustomTimeZone* ptr) {
  return reinterpret_cast<CustomTimeZone*>(ptr);
}

inline void CustomTimeZone::operator delete(void* ptr) {
  capi::ICU4XCustomTimeZone_destroy(reinterpret_cast<capi::CustomTimeZone*>(ptr));
}


#endif // CustomTimeZone_HPP
