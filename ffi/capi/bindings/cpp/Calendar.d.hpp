#ifndef Calendar_D_HPP
#define Calendar_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "AnyCalendarKind.d.hpp"
#include "DataError.d.hpp"

class DataProvider;
class Locale;
class AnyCalendarKind;
class DataError;


namespace capi {
    typedef struct Calendar Calendar;
}

class Calendar {
public:

  inline static diplomat::result<std::unique_ptr<Calendar>, DataError> create_for_locale(const DataProvider& provider, const Locale& locale);

  inline static diplomat::result<std::unique_ptr<Calendar>, DataError> create_for_kind(const DataProvider& provider, AnyCalendarKind kind);

  inline AnyCalendarKind kind() const;

  inline const capi::Calendar* AsFFI() const;
  inline capi::Calendar* AsFFI();
  inline static const Calendar* FromFFI(const capi::Calendar* ptr);
  inline static Calendar* FromFFI(capi::Calendar* ptr);
  inline static void operator delete(void* ptr);
private:
  Calendar() = delete;
  Calendar(const Calendar&) = delete;
  Calendar(Calendar&&) noexcept = delete;
  Calendar operator=(const Calendar&) = delete;
  Calendar operator=(Calendar&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // Calendar_D_HPP
