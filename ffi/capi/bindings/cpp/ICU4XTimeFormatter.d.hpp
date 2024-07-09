#ifndef ICU4XTimeFormatter_D_HPP
#define ICU4XTimeFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XError.d.hpp"
#include "ICU4XTimeLength.d.hpp"

class ICU4XDataProvider;
class ICU4XDateTime;
class ICU4XIsoDateTime;
class ICU4XLocale;
class ICU4XTime;
class ICU4XError;
class ICU4XTimeLength;


namespace capi {
    typedef struct ICU4XTimeFormatter ICU4XTimeFormatter;
}

class ICU4XTimeFormatter {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XTimeFormatter>, ICU4XError> create_with_length(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XTimeLength length);

  inline std::string format_time(const ICU4XTime& value) const;

  inline std::string format_datetime(const ICU4XDateTime& value) const;

  inline std::string format_iso_datetime(const ICU4XIsoDateTime& value) const;

  inline const capi::ICU4XTimeFormatter* AsFFI() const;
  inline capi::ICU4XTimeFormatter* AsFFI();
  inline static const ICU4XTimeFormatter* FromFFI(const capi::ICU4XTimeFormatter* ptr);
  inline static ICU4XTimeFormatter* FromFFI(capi::ICU4XTimeFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XTimeFormatter() = delete;
  ICU4XTimeFormatter(const ICU4XTimeFormatter&) = delete;
  ICU4XTimeFormatter(ICU4XTimeFormatter&&) noexcept = delete;
  ICU4XTimeFormatter operator=(const ICU4XTimeFormatter&) = delete;
  ICU4XTimeFormatter operator=(ICU4XTimeFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XTimeFormatter_D_HPP
