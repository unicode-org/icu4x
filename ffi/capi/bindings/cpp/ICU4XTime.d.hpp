#ifndef ICU4XTime_D_HPP
#define ICU4XTime_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCalendarError.d.hpp"
#include "ICU4XTime.d.h"

class ICU4XCalendarError;


class ICU4XTime {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XTime>, ICU4XCalendarError> create(uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond);

  inline static diplomat::result<std::unique_ptr<ICU4XTime>, ICU4XCalendarError> create_midnight();

  inline uint8_t hour() const;

  inline uint8_t minute() const;

  inline uint8_t second() const;

  inline uint32_t nanosecond() const;

  inline const capi::ICU4XTime* AsFFI() const;
  inline capi::ICU4XTime* AsFFI();
  inline static const ICU4XTime* FromFFI(const capi::ICU4XTime* ptr);
  inline static ICU4XTime* FromFFI(capi::ICU4XTime* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XTime() = delete;
  ICU4XTime(const ICU4XTime&) = delete;
  ICU4XTime(ICU4XTime&&) noexcept = delete;
  ICU4XTime operator=(const ICU4XTime&) = delete;
  ICU4XTime operator=(ICU4XTime&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XTime_D_HPP
