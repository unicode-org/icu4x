#ifndef Time_D_HPP
#define Time_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CalendarError.d.hpp"

class CalendarError;


namespace capi {
    typedef struct Time Time;
}

class Time {
public:

  inline static diplomat::result<std::unique_ptr<Time>, CalendarError> create(uint8_t hour, uint8_t minute, uint8_t second, uint32_t nanosecond);

  inline static diplomat::result<std::unique_ptr<Time>, CalendarError> create_midnight();

  inline uint8_t hour() const;

  inline uint8_t minute() const;

  inline uint8_t second() const;

  inline uint32_t nanosecond() const;

  inline const capi::Time* AsFFI() const;
  inline capi::Time* AsFFI();
  inline static const Time* FromFFI(const capi::Time* ptr);
  inline static Time* FromFFI(capi::Time* ptr);
  inline static void operator delete(void* ptr);
private:
  Time() = delete;
  Time(const Time&) = delete;
  Time(Time&&) noexcept = delete;
  Time operator=(const Time&) = delete;
  Time operator=(Time&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Time_D_HPP
