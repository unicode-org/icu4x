#ifndef ICU4XCustomTimeZone_HPP
#define ICU4XCustomTimeZone_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XCustomTimeZone.h"

class ICU4XCustomTimeZone;
#include "ICU4XError.hpp"
class ICU4XIsoDateTime;
class ICU4XMetaZoneCalculator;

/**
 * A destruction policy for using ICU4XCustomTimeZone with std::unique_ptr.
 */
struct ICU4XCustomTimeZoneDeleter {
  void operator()(capi::ICU4XCustomTimeZone* l) const noexcept {
    capi::ICU4XCustomTimeZone_destroy(l);
  }
};

/**
 * 
 * 
 * See the [Rust documentation for `CustomTimeZone`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html) for more information.
 */
class ICU4XCustomTimeZone {
 public:
  static diplomat::result<ICU4XCustomTimeZone, ICU4XError> create_from_str(const std::string_view s);

  /**
   * Sets the `gmt_offset` field from offset seconds.
   * 
   * Errors if the offset seconds are out of range.
   * 
   * See the [Rust documentation for `try_from_offset_seconds`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.try_from_offset_seconds) for more information.
   * 
   *  Additional information: [1](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html)
   */
  diplomat::result<std::monostate, ICU4XError> try_set_gmt_offset_seconds(int32_t offset_seconds);

  /**
   * Clears the `gmt_offset` field.
   * 
   * See the [Rust documentation for `offset_seconds`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.offset_seconds) for more information.
   * 
   *  Additional information: [1](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html)
   */
  void clear_gmt_offset();

  /**
   * Returns the value of the `gmt_offset` field as offset seconds.
   * 
   * Errors if the `gmt_offset` field is empty.
   * 
   * See the [Rust documentation for `offset_seconds`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.offset_seconds) for more information.
   * 
   *  Additional information: [1](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html)
   */
  diplomat::result<int32_t, std::monostate> gmt_offset_seconds() const;

  /**
   * Returns whether the `gmt_offset` field is positive.
   * 
   * Errors if the `gmt_offset` field is empty.
   * 
   * See the [Rust documentation for `is_positive`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.is_positive) for more information.
   */
  diplomat::result<bool, std::monostate> is_gmt_offset_positive() const;

  /**
   * Returns whether the `gmt_offset` field is zero.
   * 
   * Errors if the `gmt_offset` field is empty (which is not the same as zero).
   * 
   * See the [Rust documentation for `is_zero`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.is_zero) for more information.
   */
  diplomat::result<bool, std::monostate> is_gmt_offset_zero() const;

  /**
   * Returns whether the `gmt_offset` field has nonzero minutes.
   * 
   * Errors if the `gmt_offset` field is empty.
   * 
   * See the [Rust documentation for `has_minutes`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.has_minutes) for more information.
   */
  diplomat::result<bool, std::monostate> gmt_offset_has_minutes() const;

  /**
   * Returns whether the `gmt_offset` field has nonzero seconds.
   * 
   * Errors if the `gmt_offset` field is empty.
   * 
   * See the [Rust documentation for `has_seconds`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.has_seconds) for more information.
   */
  diplomat::result<bool, std::monostate> gmt_offset_has_seconds() const;

  /**
   * Sets the `time_zone_id` field from a string.
   * 
   * Errors if the string is not a valid BCP-47 time zone ID.
   * 
   * See the [Rust documentation for `time_zone_id`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id) for more information.
   * 
   *  Additional information: [1](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.TimeZoneBcp47Id.html)
   */
  diplomat::result<std::monostate, ICU4XError> try_set_time_zone_id(const std::string_view id);

  /**
   * Clears the `time_zone_id` field.
   * 
   * See the [Rust documentation for `time_zone_id`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id) for more information.
   * 
   *  Additional information: [1](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.TimeZoneBcp47Id.html)
   */
  void clear_time_zone_id();

  /**
   * Returns the value of the `time_zone_id` field as a string.
   * 
   * Errors if the `time_zone_id` field is empty.
   * 
   * See the [Rust documentation for `time_zone_id`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id) for more information.
   * 
   *  Additional information: [1](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.TimeZoneBcp47Id.html)
   */
  diplomat::result<const std::string_view, std::monostate> time_zone_id() const;

  /**
   * Sets the `meta_zone_id` field from a string.
   * 
   * Errors if the string is not a valid BCP-47 meta zone ID.
   * 
   * See the [Rust documentation for `meta_zone_id`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.meta_zone_id) for more information.
   * 
   *  Additional information: [1](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneId.html)
   */
  diplomat::result<std::monostate, ICU4XError> try_set_meta_zone_id(const std::string_view id);

  /**
   * Clears the `meta_zone_id` field.
   * 
   * See the [Rust documentation for `meta_zone_id`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.meta_zone_id) for more information.
   * 
   *  Additional information: [1](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneId.html)
   */
  void clear_meta_zone_id();

  /**
   * Returns the value of the `meta_zone_id` field as a string.
   * 
   * Errors if the `meta_zone_id` field is empty.
   * 
   * See the [Rust documentation for `meta_zone_id`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.meta_zone_id) for more information.
   * 
   *  Additional information: [1](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneId.html)
   */
  diplomat::result<const std::string_view, std::monostate> meta_zone_id() const;

  /**
   * Sets the `zone_variant` field from a string.
   * 
   * Errors if the string is not a valid zone variant.
   * 
   * See the [Rust documentation for `zone_variant`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant) for more information.
   * 
   *  Additional information: [1](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html)
   */
  diplomat::result<std::monostate, ICU4XError> try_set_zone_variant(const std::string_view id);

  /**
   * Clears the `zone_variant` field.
   * 
   * See the [Rust documentation for `zone_variant`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant) for more information.
   * 
   *  Additional information: [1](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html)
   */
  void clear_zone_variant();

  /**
   * Returns the value of the `zone_variant` field as a string.
   * 
   * Errors if the `zone_variant` field is empty.
   * 
   * See the [Rust documentation for `zone_variant`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant) for more information.
   * 
   *  Additional information: [1](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html)
   */
  diplomat::result<const std::string_view, std::monostate> zone_variant() const;

  /**
   * Sets the `zone_variant` field to standard time.
   * 
   * See the [Rust documentation for `standard`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.standard) for more information.
   * 
   *  Additional information: [1](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant)
   */
  void set_standard_time();

  /**
   * Sets the `zone_variant` field to daylight time.
   * 
   * See the [Rust documentation for `daylight`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.daylight) for more information.
   * 
   *  Additional information: [1](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant)
   */
  void set_daylight_time();

  /**
   * Returns whether the `zone_variant` field is standard time.
   * 
   * Errors if the `zone_variant` field is empty.
   * 
   * See the [Rust documentation for `standard`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.standard) for more information.
   * 
   *  Additional information: [1](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant)
   */
  diplomat::result<bool, std::monostate> is_standard_time() const;

  /**
   * Returns whether the `zone_variant` field is daylight time.
   * 
   * Errors if the `zone_variant` field is empty.
   * 
   * See the [Rust documentation for `daylight`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.daylight) for more information.
   * 
   *  Additional information: [1](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant)
   */
  diplomat::result<bool, std::monostate> is_daylight_time() const;

  /**
   * Sets the meta zone based on the time zone and the local timestamp.
   * 
   * See the [Rust documentation for `maybe_set_meta_zone`](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#method.maybe_set_meta_zone) for more information.
   * 
   *  Additional information: [1](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneCalculator.html#method.compute_metazone_from_timezone)
   */
  void maybe_set_meta_zone(const ICU4XIsoDateTime& local_datetime, const ICU4XMetaZoneCalculator& metazone_calculator);
  inline const capi::ICU4XCustomTimeZone* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XCustomTimeZone* AsFFIMut() { return this->inner.get(); }
  inline ICU4XCustomTimeZone(capi::ICU4XCustomTimeZone* i) : inner(i) {}
  ICU4XCustomTimeZone() = default;
  ICU4XCustomTimeZone(ICU4XCustomTimeZone&&) noexcept = default;
  ICU4XCustomTimeZone& operator=(ICU4XCustomTimeZone&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XCustomTimeZone, ICU4XCustomTimeZoneDeleter> inner;
};

#include "ICU4XIsoDateTime.hpp"
#include "ICU4XMetaZoneCalculator.hpp"

inline diplomat::result<ICU4XCustomTimeZone, ICU4XError> ICU4XCustomTimeZone::create_from_str(const std::string_view s) {
  auto diplomat_result_raw_out_value = capi::ICU4XCustomTimeZone_create_from_str(s.data(), s.size());
  diplomat::result<ICU4XCustomTimeZone, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCustomTimeZone>(std::move(ICU4XCustomTimeZone(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::monostate, ICU4XError> ICU4XCustomTimeZone::try_set_gmt_offset_seconds(int32_t offset_seconds) {
  auto diplomat_result_raw_out_value = capi::ICU4XCustomTimeZone_try_set_gmt_offset_seconds(this->inner.get(), offset_seconds);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline void ICU4XCustomTimeZone::clear_gmt_offset() {
  capi::ICU4XCustomTimeZone_clear_gmt_offset(this->inner.get());
}
inline diplomat::result<int32_t, std::monostate> ICU4XCustomTimeZone::gmt_offset_seconds() const {
  auto diplomat_result_raw_out_value = capi::ICU4XCustomTimeZone_gmt_offset_seconds(this->inner.get());
  diplomat::result<int32_t, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<int32_t>(std::move(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err(std::monostate());
  }
  return diplomat_result_out_value;
}
inline diplomat::result<bool, std::monostate> ICU4XCustomTimeZone::is_gmt_offset_positive() const {
  auto diplomat_result_raw_out_value = capi::ICU4XCustomTimeZone_is_gmt_offset_positive(this->inner.get());
  diplomat::result<bool, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<bool>(std::move(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err(std::monostate());
  }
  return diplomat_result_out_value;
}
inline diplomat::result<bool, std::monostate> ICU4XCustomTimeZone::is_gmt_offset_zero() const {
  auto diplomat_result_raw_out_value = capi::ICU4XCustomTimeZone_is_gmt_offset_zero(this->inner.get());
  diplomat::result<bool, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<bool>(std::move(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err(std::monostate());
  }
  return diplomat_result_out_value;
}
inline diplomat::result<bool, std::monostate> ICU4XCustomTimeZone::gmt_offset_has_minutes() const {
  auto diplomat_result_raw_out_value = capi::ICU4XCustomTimeZone_gmt_offset_has_minutes(this->inner.get());
  diplomat::result<bool, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<bool>(std::move(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err(std::monostate());
  }
  return diplomat_result_out_value;
}
inline diplomat::result<bool, std::monostate> ICU4XCustomTimeZone::gmt_offset_has_seconds() const {
  auto diplomat_result_raw_out_value = capi::ICU4XCustomTimeZone_gmt_offset_has_seconds(this->inner.get());
  diplomat::result<bool, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<bool>(std::move(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err(std::monostate());
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::monostate, ICU4XError> ICU4XCustomTimeZone::try_set_time_zone_id(const std::string_view id) {
  auto diplomat_result_raw_out_value = capi::ICU4XCustomTimeZone_try_set_time_zone_id(this->inner.get(), id.data(), id.size());
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline void ICU4XCustomTimeZone::clear_time_zone_id() {
  capi::ICU4XCustomTimeZone_clear_time_zone_id(this->inner.get());
}
inline diplomat::result<const std::string_view, std::monostate> ICU4XCustomTimeZone::time_zone_id() const {
  auto diplomat_result_raw_out_value = capi::ICU4XCustomTimeZone_time_zone_id(this->inner.get());
  diplomat::result<const std::string_view, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
  capi::DiplomatStringView diplomat_str_raw_out_value = diplomat_result_raw_out_value.ok;
  std::string_view str(diplomat_str_raw_out_value.data, diplomat_str_raw_out_value.len);
    diplomat_result_out_value = diplomat::Ok<const std::string_view>(std::move(str));
  } else {
    diplomat_result_out_value = diplomat::Err(std::monostate());
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::monostate, ICU4XError> ICU4XCustomTimeZone::try_set_meta_zone_id(const std::string_view id) {
  auto diplomat_result_raw_out_value = capi::ICU4XCustomTimeZone_try_set_meta_zone_id(this->inner.get(), id.data(), id.size());
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline void ICU4XCustomTimeZone::clear_meta_zone_id() {
  capi::ICU4XCustomTimeZone_clear_meta_zone_id(this->inner.get());
}
inline diplomat::result<const std::string_view, std::monostate> ICU4XCustomTimeZone::meta_zone_id() const {
  auto diplomat_result_raw_out_value = capi::ICU4XCustomTimeZone_meta_zone_id(this->inner.get());
  diplomat::result<const std::string_view, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
  capi::DiplomatStringView diplomat_str_raw_out_value = diplomat_result_raw_out_value.ok;
  std::string_view str(diplomat_str_raw_out_value.data, diplomat_str_raw_out_value.len);
    diplomat_result_out_value = diplomat::Ok<const std::string_view>(std::move(str));
  } else {
    diplomat_result_out_value = diplomat::Err(std::monostate());
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::monostate, ICU4XError> ICU4XCustomTimeZone::try_set_zone_variant(const std::string_view id) {
  auto diplomat_result_raw_out_value = capi::ICU4XCustomTimeZone_try_set_zone_variant(this->inner.get(), id.data(), id.size());
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline void ICU4XCustomTimeZone::clear_zone_variant() {
  capi::ICU4XCustomTimeZone_clear_zone_variant(this->inner.get());
}
inline diplomat::result<const std::string_view, std::monostate> ICU4XCustomTimeZone::zone_variant() const {
  auto diplomat_result_raw_out_value = capi::ICU4XCustomTimeZone_zone_variant(this->inner.get());
  diplomat::result<const std::string_view, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
  capi::DiplomatStringView diplomat_str_raw_out_value = diplomat_result_raw_out_value.ok;
  std::string_view str(diplomat_str_raw_out_value.data, diplomat_str_raw_out_value.len);
    diplomat_result_out_value = diplomat::Ok<const std::string_view>(std::move(str));
  } else {
    diplomat_result_out_value = diplomat::Err(std::monostate());
  }
  return diplomat_result_out_value;
}
inline void ICU4XCustomTimeZone::set_standard_time() {
  capi::ICU4XCustomTimeZone_set_standard_time(this->inner.get());
}
inline void ICU4XCustomTimeZone::set_daylight_time() {
  capi::ICU4XCustomTimeZone_set_daylight_time(this->inner.get());
}
inline diplomat::result<bool, std::monostate> ICU4XCustomTimeZone::is_standard_time() const {
  auto diplomat_result_raw_out_value = capi::ICU4XCustomTimeZone_is_standard_time(this->inner.get());
  diplomat::result<bool, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<bool>(std::move(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err(std::monostate());
  }
  return diplomat_result_out_value;
}
inline diplomat::result<bool, std::monostate> ICU4XCustomTimeZone::is_daylight_time() const {
  auto diplomat_result_raw_out_value = capi::ICU4XCustomTimeZone_is_daylight_time(this->inner.get());
  diplomat::result<bool, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<bool>(std::move(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err(std::monostate());
  }
  return diplomat_result_out_value;
}
inline void ICU4XCustomTimeZone::maybe_set_meta_zone(const ICU4XIsoDateTime& local_datetime, const ICU4XMetaZoneCalculator& metazone_calculator) {
  capi::ICU4XCustomTimeZone_maybe_set_meta_zone(this->inner.get(), local_datetime.AsFFI(), metazone_calculator.AsFFI());
}
#endif
