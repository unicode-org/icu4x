#ifndef icu4x_WeekOfYear_D_HPP
#define icu4x_WeekOfYear_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    struct WeekOfYear {
      uint8_t week_number;
      int32_t iso_year;
    };
    
    typedef struct WeekOfYear_option {union { WeekOfYear ok; }; bool is_ok; } WeekOfYear_option;
} // namespace capi
} // namespace


namespace icu4x {
struct WeekOfYear {
  uint8_t week_number;
  int32_t iso_year;

  inline icu4x::capi::WeekOfYear AsFFI() const;
  inline static icu4x::WeekOfYear FromFFI(icu4x::capi::WeekOfYear c_struct);
};

} // namespace
#endif // icu4x_WeekOfYear_D_HPP
