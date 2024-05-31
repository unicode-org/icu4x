#ifndef ICU4XTimeZoneFormatter_D_HPP
#define ICU4XTimeZoneFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XError.d.hpp"
#include "ICU4XIsoTimeZoneOptions.d.hpp"
#include "ICU4XTimeZoneFormatter.d.h"

class ICU4XCustomTimeZone;
class ICU4XDataProvider;
class ICU4XLocale;
struct ICU4XIsoTimeZoneOptions;
class ICU4XError;


class ICU4XTimeZoneFormatter {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XTimeZoneFormatter>, ICU4XError> create_with_localized_gmt_fallback(const ICU4XDataProvider& provider, const ICU4XLocale& locale);

  inline static diplomat::result<std::unique_ptr<ICU4XTimeZoneFormatter>, ICU4XError> create_with_iso_8601_fallback(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XIsoTimeZoneOptions options);

  inline diplomat::result<std::monostate, ICU4XError> load_generic_non_location_long(const ICU4XDataProvider& provider);

  inline diplomat::result<std::monostate, ICU4XError> load_generic_non_location_short(const ICU4XDataProvider& provider);

  inline diplomat::result<std::monostate, ICU4XError> load_specific_non_location_long(const ICU4XDataProvider& provider);

  inline diplomat::result<std::monostate, ICU4XError> load_specific_non_location_short(const ICU4XDataProvider& provider);

  inline diplomat::result<std::monostate, ICU4XError> load_generic_location_format(const ICU4XDataProvider& provider);

  inline diplomat::result<std::monostate, ICU4XError> include_localized_gmt_format();

  inline diplomat::result<std::monostate, ICU4XError> load_iso_8601_format(ICU4XIsoTimeZoneOptions options);

  inline std::string format_custom_time_zone(const ICU4XCustomTimeZone& value) const;

  inline diplomat::result<std::string, ICU4XError> format_custom_time_zone_no_fallback(const ICU4XCustomTimeZone& value) const;

  inline const capi::ICU4XTimeZoneFormatter* AsFFI() const;
  inline capi::ICU4XTimeZoneFormatter* AsFFI();
  inline static const ICU4XTimeZoneFormatter* FromFFI(const capi::ICU4XTimeZoneFormatter* ptr);
  inline static ICU4XTimeZoneFormatter* FromFFI(capi::ICU4XTimeZoneFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XTimeZoneFormatter() = delete;
  ICU4XTimeZoneFormatter(const ICU4XTimeZoneFormatter&) = delete;
  ICU4XTimeZoneFormatter(ICU4XTimeZoneFormatter&&) noexcept = delete;
  ICU4XTimeZoneFormatter operator=(const ICU4XTimeZoneFormatter&) = delete;
  ICU4XTimeZoneFormatter operator=(ICU4XTimeZoneFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XTimeZoneFormatter_D_HPP
