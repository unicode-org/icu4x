#ifndef icu4x_CustomTimeZone_D_HPP
#define icu4x_CustomTimeZone_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct CustomTimeZone; }
class CustomTimeZone;
namespace capi { struct IsoDateTime; }
class IsoDateTime;
namespace capi { struct MetazoneCalculator; }
class MetazoneCalculator;
namespace capi { struct TimeZoneIdMapper; }
class TimeZoneIdMapper;
namespace capi { struct ZoneOffsetCalculator; }
class ZoneOffsetCalculator;
struct TimeZoneInvalidIdError;
struct TimeZoneInvalidOffsetError;
struct TimeZoneUnknownError;
}


namespace icu4x {
namespace capi {
    struct CustomTimeZone;
} // namespace capi
} // namespace

namespace icu4x {
class CustomTimeZone {
public:

  inline static diplomat::result<std::unique_ptr<icu4x::CustomTimeZone>, icu4x::TimeZoneUnknownError> from_string(std::string_view s);

  inline static std::unique_ptr<icu4x::CustomTimeZone> empty();

  inline static std::unique_ptr<icu4x::CustomTimeZone> utc();

  inline diplomat::result<std::monostate, icu4x::TimeZoneInvalidOffsetError> try_set_offset_seconds(int32_t offset_seconds);

  inline void set_offset_eighths_of_hour(int8_t offset_eighths_of_hour);

  inline std::optional<int8_t> offset_eighths_of_hour() const;

  inline void clear_offset();

  inline std::optional<int32_t> offset_seconds() const;

  inline std::optional<bool> is_offset_positive() const;

  inline std::optional<bool> is_offset_zero() const;

  inline std::optional<bool> offset_has_minutes() const;

  inline std::optional<bool> offset_has_seconds() const;

  inline diplomat::result<std::monostate, icu4x::TimeZoneInvalidIdError> try_set_time_zone_id(std::string_view id);

  inline diplomat::result<std::monostate, icu4x::TimeZoneInvalidIdError> try_set_iana_time_zone_id(const icu4x::TimeZoneIdMapper& mapper, std::string_view id);

  inline void clear_time_zone_id();

  inline std::optional<std::string> time_zone_id() const;

  inline diplomat::result<std::monostate, icu4x::TimeZoneInvalidIdError> try_set_metazone_id(std::string_view id);

  inline void clear_metazone_id();

  inline std::optional<std::string> metazone_id() const;

  inline std::optional<std::monostate> try_set_zone_variant(std::string_view id);

  inline void clear_zone_variant();

  inline std::optional<std::string> zone_variant() const;

  inline void set_standard_time();

  inline void set_daylight_time();

  inline std::optional<bool> is_standard_time() const;

  inline std::optional<bool> is_daylight_time() const;

  inline void maybe_calculate_metazone(const icu4x::MetazoneCalculator& metazone_calculator, const icu4x::IsoDateTime& local_datetime);

  inline void maybe_calculate_zone_variant(const icu4x::ZoneOffsetCalculator& zone_offset_calculator, const icu4x::IsoDateTime& local_datetime);

  inline const icu4x::capi::CustomTimeZone* AsFFI() const;
  inline icu4x::capi::CustomTimeZone* AsFFI();
  inline static const icu4x::CustomTimeZone* FromFFI(const icu4x::capi::CustomTimeZone* ptr);
  inline static icu4x::CustomTimeZone* FromFFI(icu4x::capi::CustomTimeZone* ptr);
  inline static void operator delete(void* ptr);
private:
  CustomTimeZone() = delete;
  CustomTimeZone(const icu4x::CustomTimeZone&) = delete;
  CustomTimeZone(icu4x::CustomTimeZone&&) noexcept = delete;
  CustomTimeZone operator=(const icu4x::CustomTimeZone&) = delete;
  CustomTimeZone operator=(icu4x::CustomTimeZone&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_CustomTimeZone_D_HPP
