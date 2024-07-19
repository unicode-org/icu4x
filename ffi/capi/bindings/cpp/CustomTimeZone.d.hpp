#ifndef CustomTimeZone_D_HPP
#define CustomTimeZone_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct IsoDateTime; }
class IsoDateTime;
namespace diplomat::capi { struct MetazoneCalculator; }
class MetazoneCalculator;
namespace diplomat::capi { struct TimeZoneIdMapper; }
class TimeZoneIdMapper;
struct TimeZoneInvalidIdError;
struct TimeZoneInvalidOffsetError;


namespace diplomat {
namespace capi {
    struct CustomTimeZone;
} // namespace capi
} // namespace

class CustomTimeZone {
public:

  inline static diplomat::result<std::unique_ptr<CustomTimeZone>, TimeZoneInvalidOffsetError> from_string(std::string_view s);

  inline static std::unique_ptr<CustomTimeZone> empty();

  inline static std::unique_ptr<CustomTimeZone> utc();

  inline static std::unique_ptr<CustomTimeZone> gmt();

  inline static std::unique_ptr<CustomTimeZone> bst();

  inline diplomat::result<std::monostate, TimeZoneInvalidOffsetError> try_set_gmt_offset_seconds(int32_t offset_seconds);

  inline void set_gmt_offset_eighths_of_hour(int8_t offset_eighths_of_hour);

  inline void clear_gmt_offset();

  inline std::optional<int32_t> gmt_offset_seconds() const;

  inline std::optional<bool> is_gmt_offset_positive() const;

  inline std::optional<bool> is_gmt_offset_zero() const;

  inline std::optional<bool> gmt_offset_has_minutes() const;

  inline std::optional<bool> gmt_offset_has_seconds() const;

  inline diplomat::result<std::monostate, TimeZoneInvalidIdError> try_set_time_zone_id(std::string_view id);

  inline diplomat::result<std::monostate, TimeZoneInvalidIdError> try_set_iana_time_zone_id(const TimeZoneIdMapper& mapper, std::string_view id);

  inline void clear_time_zone_id();

  inline std::optional<std::string> time_zone_id() const;

  inline diplomat::result<std::monostate, TimeZoneInvalidIdError> try_set_metazone_id(std::string_view id);

  inline void clear_metazone_id();

  inline std::optional<std::string> metazone_id() const;

  inline std::optional<std::monostate> try_set_zone_variant(std::string_view id);

  inline void clear_zone_variant();

  inline std::optional<std::string> zone_variant() const;

  inline void set_standard_time();

  inline void set_daylight_time();

  inline std::optional<bool> is_standard_time() const;

  inline std::optional<bool> is_daylight_time() const;

  inline void maybe_calculate_metazone(const MetazoneCalculator& metazone_calculator, const IsoDateTime& local_datetime);

  inline const diplomat::capi::CustomTimeZone* AsFFI() const;
  inline diplomat::capi::CustomTimeZone* AsFFI();
  inline static const CustomTimeZone* FromFFI(const diplomat::capi::CustomTimeZone* ptr);
  inline static CustomTimeZone* FromFFI(diplomat::capi::CustomTimeZone* ptr);
  inline static void operator delete(void* ptr);
private:
  CustomTimeZone() = delete;
  CustomTimeZone(const CustomTimeZone&) = delete;
  CustomTimeZone(CustomTimeZone&&) noexcept = delete;
  CustomTimeZone operator=(const CustomTimeZone&) = delete;
  CustomTimeZone operator=(CustomTimeZone&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // CustomTimeZone_D_HPP
