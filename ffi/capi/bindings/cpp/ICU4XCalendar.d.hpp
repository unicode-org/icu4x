#ifndef ICU4XCalendar_D_HPP
#define ICU4XCalendar_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XAnyCalendarKind.d.hpp"
#include "ICU4XDataError.d.hpp"

class ICU4XDataProvider;
class ICU4XLocale;
class ICU4XAnyCalendarKind;
class ICU4XDataError;


namespace capi {
    typedef struct ICU4XCalendar ICU4XCalendar;
}

class ICU4XCalendar {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XCalendar>, ICU4XDataError> create_for_locale(const ICU4XDataProvider& provider, const ICU4XLocale& locale);

  inline static diplomat::result<std::unique_ptr<ICU4XCalendar>, ICU4XDataError> create_for_kind(const ICU4XDataProvider& provider, ICU4XAnyCalendarKind kind);

  inline ICU4XAnyCalendarKind kind() const;

  inline const capi::ICU4XCalendar* AsFFI() const;
  inline capi::ICU4XCalendar* AsFFI();
  inline static const ICU4XCalendar* FromFFI(const capi::ICU4XCalendar* ptr);
  inline static ICU4XCalendar* FromFFI(capi::ICU4XCalendar* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XCalendar() = delete;
  ICU4XCalendar(const ICU4XCalendar&) = delete;
  ICU4XCalendar(ICU4XCalendar&&) noexcept = delete;
  ICU4XCalendar operator=(const ICU4XCalendar&) = delete;
  ICU4XCalendar operator=(ICU4XCalendar&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XCalendar_D_HPP
