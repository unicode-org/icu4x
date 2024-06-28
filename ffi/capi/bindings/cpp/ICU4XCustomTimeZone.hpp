#ifndef ICU4XCustomTimeZone_HPP
#define ICU4XCustomTimeZone_HPP

#include "ICU4XCustomTimeZone.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XIsoDateTime.hpp"
#include "ICU4XMetazoneCalculator.hpp"
#include "ICU4XTimeZoneIdMapper.hpp"
#include "ICU4XTimeZoneInvalidIdError.hpp"
#include "ICU4XTimeZoneInvalidOffsetError.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XCustomTimeZone_create_from_string_result {union {ICU4XCustomTimeZone* ok; ICU4XTimeZoneInvalidOffsetError err;}; bool is_ok;} ICU4XCustomTimeZone_create_from_string_result;
    ICU4XCustomTimeZone_create_from_string_result ICU4XCustomTimeZone_create_from_string(const char* s_data, size_t s_len);
    
    ICU4XCustomTimeZone* ICU4XCustomTimeZone_create_empty();
    
    ICU4XCustomTimeZone* ICU4XCustomTimeZone_create_utc();
    
    ICU4XCustomTimeZone* ICU4XCustomTimeZone_create_gmt();
    
    typedef struct ICU4XCustomTimeZone_try_set_gmt_offset_seconds_result {union { ICU4XTimeZoneInvalidOffsetError err;}; bool is_ok;} ICU4XCustomTimeZone_try_set_gmt_offset_seconds_result;
    ICU4XCustomTimeZone_try_set_gmt_offset_seconds_result ICU4XCustomTimeZone_try_set_gmt_offset_seconds(ICU4XCustomTimeZone* self, int32_t offset_seconds);
    
    void ICU4XCustomTimeZone_clear_gmt_offset(ICU4XCustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_gmt_offset_seconds_result {union {int32_t ok; }; bool is_ok;} ICU4XCustomTimeZone_gmt_offset_seconds_result;
    ICU4XCustomTimeZone_gmt_offset_seconds_result ICU4XCustomTimeZone_gmt_offset_seconds(const ICU4XCustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_is_gmt_offset_positive_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_is_gmt_offset_positive_result;
    ICU4XCustomTimeZone_is_gmt_offset_positive_result ICU4XCustomTimeZone_is_gmt_offset_positive(const ICU4XCustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_is_gmt_offset_zero_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_is_gmt_offset_zero_result;
    ICU4XCustomTimeZone_is_gmt_offset_zero_result ICU4XCustomTimeZone_is_gmt_offset_zero(const ICU4XCustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_gmt_offset_has_minutes_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_gmt_offset_has_minutes_result;
    ICU4XCustomTimeZone_gmt_offset_has_minutes_result ICU4XCustomTimeZone_gmt_offset_has_minutes(const ICU4XCustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_gmt_offset_has_seconds_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_gmt_offset_has_seconds_result;
    ICU4XCustomTimeZone_gmt_offset_has_seconds_result ICU4XCustomTimeZone_gmt_offset_has_seconds(const ICU4XCustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_try_set_time_zone_id_result {union { ICU4XTimeZoneInvalidIdError err;}; bool is_ok;} ICU4XCustomTimeZone_try_set_time_zone_id_result;
    ICU4XCustomTimeZone_try_set_time_zone_id_result ICU4XCustomTimeZone_try_set_time_zone_id(ICU4XCustomTimeZone* self, const char* id_data, size_t id_len);
    
    typedef struct ICU4XCustomTimeZone_try_set_iana_time_zone_id_result {union { ICU4XTimeZoneInvalidIdError err;}; bool is_ok;} ICU4XCustomTimeZone_try_set_iana_time_zone_id_result;
    ICU4XCustomTimeZone_try_set_iana_time_zone_id_result ICU4XCustomTimeZone_try_set_iana_time_zone_id(ICU4XCustomTimeZone* self, const ICU4XTimeZoneIdMapper* mapper, const char* id_data, size_t id_len);
    
    void ICU4XCustomTimeZone_clear_time_zone_id(ICU4XCustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_time_zone_id_result { bool is_ok;} ICU4XCustomTimeZone_time_zone_id_result;
    ICU4XCustomTimeZone_time_zone_id_result ICU4XCustomTimeZone_time_zone_id(const ICU4XCustomTimeZone* self, DiplomatWrite* write);
    
    typedef struct ICU4XCustomTimeZone_try_set_metazone_id_result {union { ICU4XTimeZoneInvalidIdError err;}; bool is_ok;} ICU4XCustomTimeZone_try_set_metazone_id_result;
    ICU4XCustomTimeZone_try_set_metazone_id_result ICU4XCustomTimeZone_try_set_metazone_id(ICU4XCustomTimeZone* self, const char* id_data, size_t id_len);
    
    void ICU4XCustomTimeZone_clear_metazone_id(ICU4XCustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_metazone_id_result { bool is_ok;} ICU4XCustomTimeZone_metazone_id_result;
    ICU4XCustomTimeZone_metazone_id_result ICU4XCustomTimeZone_metazone_id(const ICU4XCustomTimeZone* self, DiplomatWrite* write);
    
    typedef struct ICU4XCustomTimeZone_try_set_zone_variant_result { bool is_ok;} ICU4XCustomTimeZone_try_set_zone_variant_result;
    ICU4XCustomTimeZone_try_set_zone_variant_result ICU4XCustomTimeZone_try_set_zone_variant(ICU4XCustomTimeZone* self, const char* id_data, size_t id_len);
    
    void ICU4XCustomTimeZone_clear_zone_variant(ICU4XCustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_zone_variant_result { bool is_ok;} ICU4XCustomTimeZone_zone_variant_result;
    ICU4XCustomTimeZone_zone_variant_result ICU4XCustomTimeZone_zone_variant(const ICU4XCustomTimeZone* self, DiplomatWrite* write);
    
    void ICU4XCustomTimeZone_set_standard_time(ICU4XCustomTimeZone* self);
    
    void ICU4XCustomTimeZone_set_daylight_time(ICU4XCustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_is_standard_time_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_is_standard_time_result;
    ICU4XCustomTimeZone_is_standard_time_result ICU4XCustomTimeZone_is_standard_time(const ICU4XCustomTimeZone* self);
    
    typedef struct ICU4XCustomTimeZone_is_daylight_time_result {union {bool ok; }; bool is_ok;} ICU4XCustomTimeZone_is_daylight_time_result;
    ICU4XCustomTimeZone_is_daylight_time_result ICU4XCustomTimeZone_is_daylight_time(const ICU4XCustomTimeZone* self);
    
    void ICU4XCustomTimeZone_maybe_calculate_metazone(ICU4XCustomTimeZone* self, const ICU4XMetazoneCalculator* metazone_calculator, const ICU4XIsoDateTime* local_datetime);
    
    
    void ICU4XCustomTimeZone_destroy(ICU4XCustomTimeZone* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XCustomTimeZone>, ICU4XTimeZoneInvalidOffsetError> ICU4XCustomTimeZone::create_from_string(std::string_view s) {
  auto result = capi::ICU4XCustomTimeZone_create_from_string(s.data(),
    s.size());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCustomTimeZone>, ICU4XTimeZoneInvalidOffsetError>(diplomat::Ok<std::unique_ptr<ICU4XCustomTimeZone>>(std::unique_ptr<ICU4XCustomTimeZone>(ICU4XCustomTimeZone::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCustomTimeZone>, ICU4XTimeZoneInvalidOffsetError>(diplomat::Err<ICU4XTimeZoneInvalidOffsetError>(ICU4XTimeZoneInvalidOffsetError::FromFFI(result.err)));
}

inline std::unique_ptr<ICU4XCustomTimeZone> ICU4XCustomTimeZone::create_empty() {
  auto result = capi::ICU4XCustomTimeZone_create_empty();
  return std::unique_ptr<ICU4XCustomTimeZone>(ICU4XCustomTimeZone::FromFFI(result));
}

inline std::unique_ptr<ICU4XCustomTimeZone> ICU4XCustomTimeZone::create_utc() {
  auto result = capi::ICU4XCustomTimeZone_create_utc();
  return std::unique_ptr<ICU4XCustomTimeZone>(ICU4XCustomTimeZone::FromFFI(result));
}

inline std::unique_ptr<ICU4XCustomTimeZone> ICU4XCustomTimeZone::create_gmt() {
  auto result = capi::ICU4XCustomTimeZone_create_gmt();
  return std::unique_ptr<ICU4XCustomTimeZone>(ICU4XCustomTimeZone::FromFFI(result));
}

inline diplomat::result<std::monostate, ICU4XTimeZoneInvalidOffsetError> ICU4XCustomTimeZone::try_set_gmt_offset_seconds(int32_t offset_seconds) {
  auto result = capi::ICU4XCustomTimeZone_try_set_gmt_offset_seconds(this->AsFFI(),
    offset_seconds);
  return result.is_ok ? diplomat::result<std::monostate, ICU4XTimeZoneInvalidOffsetError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XTimeZoneInvalidOffsetError>(diplomat::Err<ICU4XTimeZoneInvalidOffsetError>(ICU4XTimeZoneInvalidOffsetError::FromFFI(result.err)));
}

inline void ICU4XCustomTimeZone::clear_gmt_offset() {
  capi::ICU4XCustomTimeZone_clear_gmt_offset(this->AsFFI());
}

inline std::optional<int32_t> ICU4XCustomTimeZone::gmt_offset_seconds() const {
  auto result = capi::ICU4XCustomTimeZone_gmt_offset_seconds(this->AsFFI());
  return result.is_ok ? std::optional<int32_t>(result.ok) : std::nullopt;
}

inline std::optional<bool> ICU4XCustomTimeZone::is_gmt_offset_positive() const {
  auto result = capi::ICU4XCustomTimeZone_is_gmt_offset_positive(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline std::optional<bool> ICU4XCustomTimeZone::is_gmt_offset_zero() const {
  auto result = capi::ICU4XCustomTimeZone_is_gmt_offset_zero(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline std::optional<bool> ICU4XCustomTimeZone::gmt_offset_has_minutes() const {
  auto result = capi::ICU4XCustomTimeZone_gmt_offset_has_minutes(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline std::optional<bool> ICU4XCustomTimeZone::gmt_offset_has_seconds() const {
  auto result = capi::ICU4XCustomTimeZone_gmt_offset_has_seconds(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline diplomat::result<std::monostate, ICU4XTimeZoneInvalidIdError> ICU4XCustomTimeZone::try_set_time_zone_id(std::string_view id) {
  auto result = capi::ICU4XCustomTimeZone_try_set_time_zone_id(this->AsFFI(),
    id.data(),
    id.size());
  return result.is_ok ? diplomat::result<std::monostate, ICU4XTimeZoneInvalidIdError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XTimeZoneInvalidIdError>(diplomat::Err<ICU4XTimeZoneInvalidIdError>(ICU4XTimeZoneInvalidIdError::FromFFI(result.err)));
}

inline diplomat::result<std::monostate, ICU4XTimeZoneInvalidIdError> ICU4XCustomTimeZone::try_set_iana_time_zone_id(const ICU4XTimeZoneIdMapper& mapper, std::string_view id) {
  auto result = capi::ICU4XCustomTimeZone_try_set_iana_time_zone_id(this->AsFFI(),
    mapper.AsFFI(),
    id.data(),
    id.size());
  return result.is_ok ? diplomat::result<std::monostate, ICU4XTimeZoneInvalidIdError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XTimeZoneInvalidIdError>(diplomat::Err<ICU4XTimeZoneInvalidIdError>(ICU4XTimeZoneInvalidIdError::FromFFI(result.err)));
}

inline void ICU4XCustomTimeZone::clear_time_zone_id() {
  capi::ICU4XCustomTimeZone_clear_time_zone_id(this->AsFFI());
}

inline std::optional<std::string> ICU4XCustomTimeZone::time_zone_id() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XCustomTimeZone_time_zone_id(this->AsFFI(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline diplomat::result<std::monostate, ICU4XTimeZoneInvalidIdError> ICU4XCustomTimeZone::try_set_metazone_id(std::string_view id) {
  auto result = capi::ICU4XCustomTimeZone_try_set_metazone_id(this->AsFFI(),
    id.data(),
    id.size());
  return result.is_ok ? diplomat::result<std::monostate, ICU4XTimeZoneInvalidIdError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, ICU4XTimeZoneInvalidIdError>(diplomat::Err<ICU4XTimeZoneInvalidIdError>(ICU4XTimeZoneInvalidIdError::FromFFI(result.err)));
}

inline void ICU4XCustomTimeZone::clear_metazone_id() {
  capi::ICU4XCustomTimeZone_clear_metazone_id(this->AsFFI());
}

inline std::optional<std::string> ICU4XCustomTimeZone::metazone_id() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XCustomTimeZone_metazone_id(this->AsFFI(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline std::optional<std::monostate> ICU4XCustomTimeZone::try_set_zone_variant(std::string_view id) {
  auto result = capi::ICU4XCustomTimeZone_try_set_zone_variant(this->AsFFI(),
    id.data(),
    id.size());
  return result.is_ok ? std::optional<std::monostate>() : std::nullopt;
}

inline void ICU4XCustomTimeZone::clear_zone_variant() {
  capi::ICU4XCustomTimeZone_clear_zone_variant(this->AsFFI());
}

inline std::optional<std::string> ICU4XCustomTimeZone::zone_variant() const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = capi::ICU4XCustomTimeZone_zone_variant(this->AsFFI(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline void ICU4XCustomTimeZone::set_standard_time() {
  capi::ICU4XCustomTimeZone_set_standard_time(this->AsFFI());
}

inline void ICU4XCustomTimeZone::set_daylight_time() {
  capi::ICU4XCustomTimeZone_set_daylight_time(this->AsFFI());
}

inline std::optional<bool> ICU4XCustomTimeZone::is_standard_time() const {
  auto result = capi::ICU4XCustomTimeZone_is_standard_time(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline std::optional<bool> ICU4XCustomTimeZone::is_daylight_time() const {
  auto result = capi::ICU4XCustomTimeZone_is_daylight_time(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline void ICU4XCustomTimeZone::maybe_calculate_metazone(const ICU4XMetazoneCalculator& metazone_calculator, const ICU4XIsoDateTime& local_datetime) {
  capi::ICU4XCustomTimeZone_maybe_calculate_metazone(this->AsFFI(),
    metazone_calculator.AsFFI(),
    local_datetime.AsFFI());
}

inline const capi::ICU4XCustomTimeZone* ICU4XCustomTimeZone::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XCustomTimeZone*>(this);
}

inline capi::ICU4XCustomTimeZone* ICU4XCustomTimeZone::AsFFI() {
  return reinterpret_cast<capi::ICU4XCustomTimeZone*>(this);
}

inline const ICU4XCustomTimeZone* ICU4XCustomTimeZone::FromFFI(const capi::ICU4XCustomTimeZone* ptr) {
  return reinterpret_cast<const ICU4XCustomTimeZone*>(ptr);
}

inline ICU4XCustomTimeZone* ICU4XCustomTimeZone::FromFFI(capi::ICU4XCustomTimeZone* ptr) {
  return reinterpret_cast<ICU4XCustomTimeZone*>(ptr);
}

inline void ICU4XCustomTimeZone::operator delete(void* ptr) {
  capi::ICU4XCustomTimeZone_destroy(reinterpret_cast<capi::ICU4XCustomTimeZone*>(ptr));
}


#endif // ICU4XCustomTimeZone_HPP
