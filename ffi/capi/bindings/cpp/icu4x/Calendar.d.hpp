#ifndef icu4x_Calendar_D_HPP
#define icu4x_Calendar_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct Calendar; }
class Calendar;
namespace capi { struct DataProvider; }
class DataProvider;
namespace capi { struct Locale; }
class Locale;
class AnyCalendarKind;
class DataError;
}


namespace icu4x {
namespace capi {
    struct Calendar;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * See the [Rust documentation for `AnyCalendar`](https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendar.html) for more information.
 */
class Calendar {
public:

  /**
   * Creates a new [`Calendar`] from the specified date and time, using compiled data.
   *
   * See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendar.html#method.try_new) for more information.
   */
  inline static diplomat::result<std::unique_ptr<icu4x::Calendar>, icu4x::DataError> create_for_locale(const icu4x::Locale& locale);

  /**
   * Creates a new [`Calendar`] from the specified date and time, using compiled data.
   *
   * See the [Rust documentation for `new_for_kind`](https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendar.html#method.new_for_kind) for more information.
   */
  inline static diplomat::result<std::unique_ptr<icu4x::Calendar>, icu4x::DataError> create_for_kind(icu4x::AnyCalendarKind kind);

  /**
   * Creates a new [`Calendar`] from the specified date and time, using a particular data source.
   *
   * See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendar.html#method.try_new) for more information.
   */
  inline static diplomat::result<std::unique_ptr<icu4x::Calendar>, icu4x::DataError> create_for_locale_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale);

  /**
   * Creates a new [`Calendar`] from the specified date and time, using a particular data source.
   *
   * See the [Rust documentation for `new_for_kind`](https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendar.html#method.new_for_kind) for more information.
   */
  inline static diplomat::result<std::unique_ptr<icu4x::Calendar>, icu4x::DataError> create_for_kind_with_provider(const icu4x::DataProvider& provider, icu4x::AnyCalendarKind kind);

  /**
   * Returns the kind of this calendar
   *
   * See the [Rust documentation for `kind`](https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendar.html#method.kind) for more information.
   */
  inline icu4x::AnyCalendarKind kind() const;

  inline const icu4x::capi::Calendar* AsFFI() const;
  inline icu4x::capi::Calendar* AsFFI();
  inline static const icu4x::Calendar* FromFFI(const icu4x::capi::Calendar* ptr);
  inline static icu4x::Calendar* FromFFI(icu4x::capi::Calendar* ptr);
  inline static void operator delete(void* ptr);
private:
  Calendar() = delete;
  Calendar(const icu4x::Calendar&) = delete;
  Calendar(icu4x::Calendar&&) noexcept = delete;
  Calendar operator=(const icu4x::Calendar&) = delete;
  Calendar operator=(icu4x::Calendar&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_Calendar_D_HPP
