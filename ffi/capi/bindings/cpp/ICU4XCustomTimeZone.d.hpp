#ifndef ICU4XCustomTimeZone_D_HPP
#define ICU4XCustomTimeZone_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XTimeZoneInvalidIdError.d.hpp"
#include "ICU4XTimeZoneInvalidOffsetError.d.hpp"

class ICU4XIsoDateTime;
class ICU4XMetazoneCalculator;
class ICU4XTimeZoneIdMapper;
class ICU4XTimeZoneInvalidIdError;
class ICU4XTimeZoneInvalidOffsetError;


namespace capi {
    typedef struct ICU4XCustomTimeZone ICU4XCustomTimeZone;
}

class ICU4XCustomTimeZone {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XCustomTimeZone>, ICU4XTimeZoneInvalidOffsetError> create_from_string(std::string_view s);

  inline static std::unique_ptr<ICU4XCustomTimeZone> create_empty();

  inline static std::unique_ptr<ICU4XCustomTimeZone> create_utc();

  inline static std::unique_ptr<ICU4XCustomTimeZone> create_gmt();

  inline diplomat::result<std::monostate, ICU4XTimeZoneInvalidOffsetError> try_set_gmt_offset_seconds(int32_t offset_seconds);

  inline void clear_gmt_offset();

  inline std::optional<int32_t> gmt_offset_seconds() const;

  inline std::optional<bool> is_gmt_offset_positive() const;

  inline std::optional<bool> is_gmt_offset_zero() const;

  inline std::optional<bool> gmt_offset_has_minutes() const;

  inline std::optional<bool> gmt_offset_has_seconds() const;

  inline diplomat::result<std::monostate, ICU4XTimeZoneInvalidIdError> try_set_time_zone_id(std::string_view id);

  inline diplomat::result<std::monostate, ICU4XTimeZoneInvalidIdError> try_set_iana_time_zone_id(const ICU4XTimeZoneIdMapper& mapper, std::string_view id);

  inline void clear_time_zone_id();

  inline std::optional<std::string> time_zone_id() const;

  inline diplomat::result<std::monostate, ICU4XTimeZoneInvalidIdError> try_set_metazone_id(std::string_view id);

  inline void clear_metazone_id();

  inline std::optional<std::string> metazone_id() const;

  inline std::optional<std::monostate> try_set_zone_variant(std::string_view id);

  inline void clear_zone_variant();

  inline std::optional<std::string> zone_variant() const;

  inline void set_standard_time();

  inline void set_daylight_time();

  inline std::optional<bool> is_standard_time() const;

  inline std::optional<bool> is_daylight_time() const;

  inline void maybe_calculate_metazone(const ICU4XMetazoneCalculator& metazone_calculator, const ICU4XIsoDateTime& local_datetime);

  inline const capi::ICU4XCustomTimeZone* AsFFI() const;
  inline capi::ICU4XCustomTimeZone* AsFFI();
  inline static const ICU4XCustomTimeZone* FromFFI(const capi::ICU4XCustomTimeZone* ptr);
  inline static ICU4XCustomTimeZone* FromFFI(capi::ICU4XCustomTimeZone* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XCustomTimeZone() = delete;
  ICU4XCustomTimeZone(const ICU4XCustomTimeZone&) = delete;
  ICU4XCustomTimeZone(ICU4XCustomTimeZone&&) noexcept = delete;
  ICU4XCustomTimeZone operator=(const ICU4XCustomTimeZone&) = delete;
  ICU4XCustomTimeZone operator=(ICU4XCustomTimeZone&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XCustomTimeZone_D_HPP
