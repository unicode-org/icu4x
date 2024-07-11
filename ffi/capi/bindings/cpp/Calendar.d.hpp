#ifndef Calendar_D_HPP
#define Calendar_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {typedef struct DataProvider DataProvider; }
class DataProvider;
namespace capi {typedef struct Locale Locale; }
class Locale;
class AnyCalendarKind;
class DataError;


namespace diplomat {
namespace capi {
    typedef struct Calendar Calendar;
} // namespace capi
} // namespace

class Calendar {
public:

  inline static diplomat::result<std::unique_ptr<Calendar>, DataError> create_for_locale(const DataProvider& provider, const Locale& locale);

  inline static diplomat::result<std::unique_ptr<Calendar>, DataError> create_for_kind(const DataProvider& provider, AnyCalendarKind kind);

  inline AnyCalendarKind kind() const;

  inline const diplomat::capi::Calendar* AsFFI() const;
  inline diplomat::capi::Calendar* AsFFI();
  inline static const Calendar* FromFFI(const diplomat::capi::Calendar* ptr);
  inline static Calendar* FromFFI(diplomat::capi::Calendar* ptr);
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
