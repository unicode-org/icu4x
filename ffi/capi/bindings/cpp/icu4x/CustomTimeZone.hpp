#ifndef icu4x_CustomTimeZone_HPP
#define icu4x_CustomTimeZone_HPP

#include "CustomTimeZone.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "IsoDateTime.hpp"
#include "MetazoneCalculator.hpp"
#include "TimeZoneIdMapper.hpp"
#include "TimeZoneInvalidIdError.hpp"
#include "TimeZoneInvalidOffsetError.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_CustomTimeZone_from_string_mv1_result {union {icu4x::capi::CustomTimeZone* ok; }; bool is_ok;} icu4x_CustomTimeZone_from_string_mv1_result;
    icu4x_CustomTimeZone_from_string_mv1_result icu4x_CustomTimeZone_from_string_mv1(diplomat::capi::DiplomatStringView s);
    
    icu4x::capi::CustomTimeZone* icu4x_CustomTimeZone_empty_mv1(void);
    
    icu4x::capi::CustomTimeZone* icu4x_CustomTimeZone_utc_mv1(void);
    
    icu4x::capi::CustomTimeZone* icu4x_CustomTimeZone_gmt_mv1(void);
    
    icu4x::capi::CustomTimeZone* icu4x_CustomTimeZone_bst_mv1(void);
    
    typedef struct icu4x_CustomTimeZone_try_set_gmt_offset_seconds_mv1_result { bool is_ok;} icu4x_CustomTimeZone_try_set_gmt_offset_seconds_mv1_result;
    icu4x_CustomTimeZone_try_set_gmt_offset_seconds_mv1_result icu4x_CustomTimeZone_try_set_gmt_offset_seconds_mv1(icu4x::capi::CustomTimeZone* self, int32_t offset_seconds);
    
    void icu4x_CustomTimeZone_set_gmt_offset_eighths_of_hour_mv1(icu4x::capi::CustomTimeZone* self, int8_t offset_eighths_of_hour);
    
    void icu4x_CustomTimeZone_clear_gmt_offset_mv1(icu4x::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_gmt_offset_seconds_mv1_result {union {int32_t ok; }; bool is_ok;} icu4x_CustomTimeZone_gmt_offset_seconds_mv1_result;
    icu4x_CustomTimeZone_gmt_offset_seconds_mv1_result icu4x_CustomTimeZone_gmt_offset_seconds_mv1(const icu4x::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_is_gmt_offset_positive_mv1_result {union {bool ok; }; bool is_ok;} icu4x_CustomTimeZone_is_gmt_offset_positive_mv1_result;
    icu4x_CustomTimeZone_is_gmt_offset_positive_mv1_result icu4x_CustomTimeZone_is_gmt_offset_positive_mv1(const icu4x::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_is_gmt_offset_zero_mv1_result {union {bool ok; }; bool is_ok;} icu4x_CustomTimeZone_is_gmt_offset_zero_mv1_result;
    icu4x_CustomTimeZone_is_gmt_offset_zero_mv1_result icu4x_CustomTimeZone_is_gmt_offset_zero_mv1(const icu4x::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_gmt_offset_has_minutes_mv1_result {union {bool ok; }; bool is_ok;} icu4x_CustomTimeZone_gmt_offset_has_minutes_mv1_result;
    icu4x_CustomTimeZone_gmt_offset_has_minutes_mv1_result icu4x_CustomTimeZone_gmt_offset_has_minutes_mv1(const icu4x::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_gmt_offset_has_seconds_mv1_result {union {bool ok; }; bool is_ok;} icu4x_CustomTimeZone_gmt_offset_has_seconds_mv1_result;
    icu4x_CustomTimeZone_gmt_offset_has_seconds_mv1_result icu4x_CustomTimeZone_gmt_offset_has_seconds_mv1(const icu4x::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_try_set_time_zone_id_mv1_result { bool is_ok;} icu4x_CustomTimeZone_try_set_time_zone_id_mv1_result;
    icu4x_CustomTimeZone_try_set_time_zone_id_mv1_result icu4x_CustomTimeZone_try_set_time_zone_id_mv1(icu4x::capi::CustomTimeZone* self, diplomat::capi::DiplomatStringView id);
    
    typedef struct icu4x_CustomTimeZone_try_set_iana_time_zone_id_mv1_result { bool is_ok;} icu4x_CustomTimeZone_try_set_iana_time_zone_id_mv1_result;
    icu4x_CustomTimeZone_try_set_iana_time_zone_id_mv1_result icu4x_CustomTimeZone_try_set_iana_time_zone_id_mv1(icu4x::capi::CustomTimeZone* self, const icu4x::capi::TimeZoneIdMapper* mapper, diplomat::capi::DiplomatStringView id);
    
    void icu4x_CustomTimeZone_clear_time_zone_id_mv1(icu4x::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_time_zone_id_mv1_result { bool is_ok;} icu4x_CustomTimeZone_time_zone_id_mv1_result;
    icu4x_CustomTimeZone_time_zone_id_mv1_result icu4x_CustomTimeZone_time_zone_id_mv1(const icu4x::capi::CustomTimeZone* self, diplomat::capi::DiplomatWrite* write);
    
    typedef struct icu4x_CustomTimeZone_try_set_metazone_id_mv1_result { bool is_ok;} icu4x_CustomTimeZone_try_set_metazone_id_mv1_result;
    icu4x_CustomTimeZone_try_set_metazone_id_mv1_result icu4x_CustomTimeZone_try_set_metazone_id_mv1(icu4x::capi::CustomTimeZone* self, diplomat::capi::DiplomatStringView id);
    
    void icu4x_CustomTimeZone_clear_metazone_id_mv1(icu4x::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_metazone_id_mv1_result { bool is_ok;} icu4x_CustomTimeZone_metazone_id_mv1_result;
    icu4x_CustomTimeZone_metazone_id_mv1_result icu4x_CustomTimeZone_metazone_id_mv1(const icu4x::capi::CustomTimeZone* self, diplomat::capi::DiplomatWrite* write);
    
    typedef struct icu4x_CustomTimeZone_try_set_zone_variant_mv1_result { bool is_ok;} icu4x_CustomTimeZone_try_set_zone_variant_mv1_result;
    icu4x_CustomTimeZone_try_set_zone_variant_mv1_result icu4x_CustomTimeZone_try_set_zone_variant_mv1(icu4x::capi::CustomTimeZone* self, diplomat::capi::DiplomatStringView id);
    
    void icu4x_CustomTimeZone_clear_zone_variant_mv1(icu4x::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_zone_variant_mv1_result { bool is_ok;} icu4x_CustomTimeZone_zone_variant_mv1_result;
    icu4x_CustomTimeZone_zone_variant_mv1_result icu4x_CustomTimeZone_zone_variant_mv1(const icu4x::capi::CustomTimeZone* self, diplomat::capi::DiplomatWrite* write);
    
    void icu4x_CustomTimeZone_set_standard_time_mv1(icu4x::capi::CustomTimeZone* self);
    
    void icu4x_CustomTimeZone_set_daylight_time_mv1(icu4x::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_is_standard_time_mv1_result {union {bool ok; }; bool is_ok;} icu4x_CustomTimeZone_is_standard_time_mv1_result;
    icu4x_CustomTimeZone_is_standard_time_mv1_result icu4x_CustomTimeZone_is_standard_time_mv1(const icu4x::capi::CustomTimeZone* self);
    
    typedef struct icu4x_CustomTimeZone_is_daylight_time_mv1_result {union {bool ok; }; bool is_ok;} icu4x_CustomTimeZone_is_daylight_time_mv1_result;
    icu4x_CustomTimeZone_is_daylight_time_mv1_result icu4x_CustomTimeZone_is_daylight_time_mv1(const icu4x::capi::CustomTimeZone* self);
    
    void icu4x_CustomTimeZone_maybe_calculate_metazone_mv1(icu4x::capi::CustomTimeZone* self, const icu4x::capi::MetazoneCalculator* metazone_calculator, const icu4x::capi::IsoDateTime* local_datetime);
    
    
    void icu4x_CustomTimeZone_destroy_mv1(CustomTimeZone* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<icu4x::CustomTimeZone>, icu4x::TimeZoneInvalidOffsetError> icu4x::CustomTimeZone::from_string(std::string_view s) {
  auto result = icu4x::capi::icu4x_CustomTimeZone_from_string_mv1({s.data(), s.size()});
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::CustomTimeZone>, icu4x::TimeZoneInvalidOffsetError>(diplomat::Ok<std::unique_ptr<icu4x::CustomTimeZone>>(std::unique_ptr<icu4x::CustomTimeZone>(icu4x::CustomTimeZone::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::CustomTimeZone>, icu4x::TimeZoneInvalidOffsetError>(diplomat::Err<icu4x::TimeZoneInvalidOffsetError>(icu4x::TimeZoneInvalidOffsetError {}));
}

inline std::unique_ptr<icu4x::CustomTimeZone> icu4x::CustomTimeZone::empty() {
  auto result = icu4x::capi::icu4x_CustomTimeZone_empty_mv1();
  return std::unique_ptr<icu4x::CustomTimeZone>(icu4x::CustomTimeZone::FromFFI(result));
}

inline std::unique_ptr<icu4x::CustomTimeZone> icu4x::CustomTimeZone::utc() {
  auto result = icu4x::capi::icu4x_CustomTimeZone_utc_mv1();
  return std::unique_ptr<icu4x::CustomTimeZone>(icu4x::CustomTimeZone::FromFFI(result));
}

inline std::unique_ptr<icu4x::CustomTimeZone> icu4x::CustomTimeZone::gmt() {
  auto result = icu4x::capi::icu4x_CustomTimeZone_gmt_mv1();
  return std::unique_ptr<icu4x::CustomTimeZone>(icu4x::CustomTimeZone::FromFFI(result));
}

inline std::unique_ptr<icu4x::CustomTimeZone> icu4x::CustomTimeZone::bst() {
  auto result = icu4x::capi::icu4x_CustomTimeZone_bst_mv1();
  return std::unique_ptr<icu4x::CustomTimeZone>(icu4x::CustomTimeZone::FromFFI(result));
}

inline diplomat::result<std::monostate, icu4x::TimeZoneInvalidOffsetError> icu4x::CustomTimeZone::try_set_gmt_offset_seconds(int32_t offset_seconds) {
  auto result = icu4x::capi::icu4x_CustomTimeZone_try_set_gmt_offset_seconds_mv1(this->AsFFI(),
    offset_seconds);
  return result.is_ok ? diplomat::result<std::monostate, icu4x::TimeZoneInvalidOffsetError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, icu4x::TimeZoneInvalidOffsetError>(diplomat::Err<icu4x::TimeZoneInvalidOffsetError>(icu4x::TimeZoneInvalidOffsetError {}));
}

inline void icu4x::CustomTimeZone::set_gmt_offset_eighths_of_hour(int8_t offset_eighths_of_hour) {
  icu4x::capi::icu4x_CustomTimeZone_set_gmt_offset_eighths_of_hour_mv1(this->AsFFI(),
    offset_eighths_of_hour);
}

inline void icu4x::CustomTimeZone::clear_gmt_offset() {
  icu4x::capi::icu4x_CustomTimeZone_clear_gmt_offset_mv1(this->AsFFI());
}

inline std::optional<int32_t> icu4x::CustomTimeZone::gmt_offset_seconds() const {
  auto result = icu4x::capi::icu4x_CustomTimeZone_gmt_offset_seconds_mv1(this->AsFFI());
  return result.is_ok ? std::optional<int32_t>(result.ok) : std::nullopt;
}

inline std::optional<bool> icu4x::CustomTimeZone::is_gmt_offset_positive() const {
  auto result = icu4x::capi::icu4x_CustomTimeZone_is_gmt_offset_positive_mv1(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline std::optional<bool> icu4x::CustomTimeZone::is_gmt_offset_zero() const {
  auto result = icu4x::capi::icu4x_CustomTimeZone_is_gmt_offset_zero_mv1(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline std::optional<bool> icu4x::CustomTimeZone::gmt_offset_has_minutes() const {
  auto result = icu4x::capi::icu4x_CustomTimeZone_gmt_offset_has_minutes_mv1(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline std::optional<bool> icu4x::CustomTimeZone::gmt_offset_has_seconds() const {
  auto result = icu4x::capi::icu4x_CustomTimeZone_gmt_offset_has_seconds_mv1(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline diplomat::result<std::monostate, icu4x::TimeZoneInvalidIdError> icu4x::CustomTimeZone::try_set_time_zone_id(std::string_view id) {
  auto result = icu4x::capi::icu4x_CustomTimeZone_try_set_time_zone_id_mv1(this->AsFFI(),
    {id.data(), id.size()});
  return result.is_ok ? diplomat::result<std::monostate, icu4x::TimeZoneInvalidIdError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, icu4x::TimeZoneInvalidIdError>(diplomat::Err<icu4x::TimeZoneInvalidIdError>(icu4x::TimeZoneInvalidIdError {}));
}

inline diplomat::result<std::monostate, icu4x::TimeZoneInvalidIdError> icu4x::CustomTimeZone::try_set_iana_time_zone_id(const icu4x::TimeZoneIdMapper& mapper, std::string_view id) {
  auto result = icu4x::capi::icu4x_CustomTimeZone_try_set_iana_time_zone_id_mv1(this->AsFFI(),
    mapper.AsFFI(),
    {id.data(), id.size()});
  return result.is_ok ? diplomat::result<std::monostate, icu4x::TimeZoneInvalidIdError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, icu4x::TimeZoneInvalidIdError>(diplomat::Err<icu4x::TimeZoneInvalidIdError>(icu4x::TimeZoneInvalidIdError {}));
}

inline void icu4x::CustomTimeZone::clear_time_zone_id() {
  icu4x::capi::icu4x_CustomTimeZone_clear_time_zone_id_mv1(this->AsFFI());
}

inline std::optional<std::string> icu4x::CustomTimeZone::time_zone_id() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = icu4x::capi::icu4x_CustomTimeZone_time_zone_id_mv1(this->AsFFI(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline diplomat::result<std::monostate, icu4x::TimeZoneInvalidIdError> icu4x::CustomTimeZone::try_set_metazone_id(std::string_view id) {
  auto result = icu4x::capi::icu4x_CustomTimeZone_try_set_metazone_id_mv1(this->AsFFI(),
    {id.data(), id.size()});
  return result.is_ok ? diplomat::result<std::monostate, icu4x::TimeZoneInvalidIdError>(diplomat::Ok<std::monostate>()) : diplomat::result<std::monostate, icu4x::TimeZoneInvalidIdError>(diplomat::Err<icu4x::TimeZoneInvalidIdError>(icu4x::TimeZoneInvalidIdError {}));
}

inline void icu4x::CustomTimeZone::clear_metazone_id() {
  icu4x::capi::icu4x_CustomTimeZone_clear_metazone_id_mv1(this->AsFFI());
}

inline std::optional<std::string> icu4x::CustomTimeZone::metazone_id() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = icu4x::capi::icu4x_CustomTimeZone_metazone_id_mv1(this->AsFFI(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline std::optional<std::monostate> icu4x::CustomTimeZone::try_set_zone_variant(std::string_view id) {
  auto result = icu4x::capi::icu4x_CustomTimeZone_try_set_zone_variant_mv1(this->AsFFI(),
    {id.data(), id.size()});
  return result.is_ok ? std::optional<std::monostate>() : std::nullopt;
}

inline void icu4x::CustomTimeZone::clear_zone_variant() {
  icu4x::capi::icu4x_CustomTimeZone_clear_zone_variant_mv1(this->AsFFI());
}

inline std::optional<std::string> icu4x::CustomTimeZone::zone_variant() const {
  std::string output;
  diplomat::capi::DiplomatWrite write = diplomat::WriteFromString(output);
  auto result = icu4x::capi::icu4x_CustomTimeZone_zone_variant_mv1(this->AsFFI(),
    &write);
  return result.is_ok ? std::optional<std::string>(std::move(output)) : std::nullopt;
}

inline void icu4x::CustomTimeZone::set_standard_time() {
  icu4x::capi::icu4x_CustomTimeZone_set_standard_time_mv1(this->AsFFI());
}

inline void icu4x::CustomTimeZone::set_daylight_time() {
  icu4x::capi::icu4x_CustomTimeZone_set_daylight_time_mv1(this->AsFFI());
}

inline std::optional<bool> icu4x::CustomTimeZone::is_standard_time() const {
  auto result = icu4x::capi::icu4x_CustomTimeZone_is_standard_time_mv1(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline std::optional<bool> icu4x::CustomTimeZone::is_daylight_time() const {
  auto result = icu4x::capi::icu4x_CustomTimeZone_is_daylight_time_mv1(this->AsFFI());
  return result.is_ok ? std::optional<bool>(result.ok) : std::nullopt;
}

inline void icu4x::CustomTimeZone::maybe_calculate_metazone(const icu4x::MetazoneCalculator& metazone_calculator, const icu4x::IsoDateTime& local_datetime) {
  icu4x::capi::icu4x_CustomTimeZone_maybe_calculate_metazone_mv1(this->AsFFI(),
    metazone_calculator.AsFFI(),
    local_datetime.AsFFI());
}

inline const icu4x::capi::CustomTimeZone* icu4x::CustomTimeZone::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::CustomTimeZone*>(this);
}

inline icu4x::capi::CustomTimeZone* icu4x::CustomTimeZone::AsFFI() {
  return reinterpret_cast<icu4x::capi::CustomTimeZone*>(this);
}

inline const icu4x::CustomTimeZone* icu4x::CustomTimeZone::FromFFI(const icu4x::capi::CustomTimeZone* ptr) {
  return reinterpret_cast<const icu4x::CustomTimeZone*>(ptr);
}

inline icu4x::CustomTimeZone* icu4x::CustomTimeZone::FromFFI(icu4x::capi::CustomTimeZone* ptr) {
  return reinterpret_cast<icu4x::CustomTimeZone*>(ptr);
}

inline void icu4x::CustomTimeZone::operator delete(void* ptr) {
  icu4x::capi::icu4x_CustomTimeZone_destroy_mv1(reinterpret_cast<icu4x::capi::CustomTimeZone*>(ptr));
}


#endif // icu4x_CustomTimeZone_HPP
