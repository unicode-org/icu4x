#ifndef icu4x_DateTimeFieldSetBuilder_D_HPP
#define icu4x_DateTimeFieldSetBuilder_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "DateFields.d.hpp"
#include "DateTimeAlignment.d.hpp"
#include "NeoDateTimeLength.d.hpp"
#include "TimePrecision.d.hpp"
#include "YearStyle.d.hpp"
#include "ZoneStyle.d.hpp"

namespace icu4x {
class DateFields;
class DateTimeAlignment;
class NeoDateTimeLength;
class TimePrecision;
class YearStyle;
class ZoneStyle;
}


namespace icu4x {
namespace capi {
    struct DateTimeFieldSetBuilder {
      icu4x::capi::NeoDateTimeLength_option length;
      icu4x::capi::DateFields_option date_fields;
      icu4x::capi::TimePrecision_option time_precision;
      icu4x::capi::ZoneStyle_option zone_style;
      icu4x::capi::DateTimeAlignment_option alignment;
      icu4x::capi::YearStyle_option year_style;
    };
    
    typedef struct DateTimeFieldSetBuilder_option {union { DateTimeFieldSetBuilder ok; }; bool is_ok; } DateTimeFieldSetBuilder_option;
} // namespace capi
} // namespace


namespace icu4x {
struct DateTimeFieldSetBuilder {
  std::optional<icu4x::NeoDateTimeLength> length;
  std::optional<icu4x::DateFields> date_fields;
  std::optional<icu4x::TimePrecision> time_precision;
  std::optional<icu4x::ZoneStyle> zone_style;
  std::optional<icu4x::DateTimeAlignment> alignment;
  std::optional<icu4x::YearStyle> year_style;

  inline icu4x::capi::DateTimeFieldSetBuilder AsFFI() const;
  inline static icu4x::DateTimeFieldSetBuilder FromFFI(icu4x::capi::DateTimeFieldSetBuilder c_struct);
};

} // namespace
#endif // icu4x_DateTimeFieldSetBuilder_D_HPP
