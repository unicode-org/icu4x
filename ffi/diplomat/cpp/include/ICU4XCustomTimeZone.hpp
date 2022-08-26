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
 * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html) for more information.
 */
class ICU4XCustomTimeZone {
 public:
  static diplomat::result<ICU4XCustomTimeZone, ICU4XError> create_from_str(const std::string_view s);

  /**
   * 
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.try_from_offset_seconds) for more information.
   */
  diplomat::result<std::monostate, ICU4XError> try_set_gmt_offset_seconds(int32_t offset_seconds);

  /**
   * 
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.offset_seconds) for more information.
   */
  diplomat::result<int32_t, std::monostate> gmt_offset_seconds() const;

  /**
   * 
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.is_positive) for more information.
   */
  diplomat::result<bool, std::monostate> is_gmt_offset_positive() const;

  /**
   * 
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.is_zero) for more information.
   */
  diplomat::result<bool, std::monostate> is_gmt_offset_zero() const;

  /**
   * 
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.has_minutes) for more information.
   */
  diplomat::result<bool, std::monostate> gmt_offset_has_minutes() const;

  /**
   * 
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.has_seconds) for more information.
   */
  diplomat::result<bool, std::monostate> gmt_offset_has_seconds() const;

  /**
   * 
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id) for more information.
   */
  diplomat::result<std::monostate, ICU4XError> try_set_time_zone_id(const std::string_view id);

  /**
   * 
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id) for more information.
   */
  diplomat::result<const std::string_view, std::monostate> time_zone_id() const;

  /**
   * 
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.meta_zone_id) for more information.
   */
  diplomat::result<std::monostate, ICU4XError> try_set_meta_zone_id(const std::string_view id);

  /**
   * 
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.meta_zone_id) for more information.
   */
  diplomat::result<const std::string_view, std::monostate> meta_zone_id() const;

  /**
   * 
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant) for more information.
   */
  diplomat::result<std::monostate, ICU4XError> try_set_zone_variant(const std::string_view id);

  /**
   * 
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant) for more information.
   */
  diplomat::result<const std::string_view, std::monostate> zone_variant() const;

  /**
   * 
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.standard) for more information.
   */
  void set_standard_time();

  /**
   * 
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.daylight) for more information.
   */
  void set_daylight_time();

  /**
   * 
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.standard) for more information.
   */
  diplomat::result<bool, std::monostate> is_standard_time() const;

  /**
   * 
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.daylight) for more information.
   */
  diplomat::result<bool, std::monostate> is_daylight_time() const;

  /**
   * 
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#method.maybe_set_meta_zone) for more information.
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
