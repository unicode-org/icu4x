#ifndef ICU4XListFormatter_D_HPP
#define ICU4XListFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.d.hpp"
#include "ICU4XListLength.d.hpp"

class ICU4XDataProvider;
class ICU4XLocale;
class ICU4XDataError;
class ICU4XListLength;


namespace capi {
    typedef struct ICU4XListFormatter ICU4XListFormatter;
}

class ICU4XListFormatter {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XListFormatter>, ICU4XDataError> create_and_with_length(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XListLength length);

  inline static diplomat::result<std::unique_ptr<ICU4XListFormatter>, ICU4XDataError> create_or_with_length(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XListLength length);

  inline static diplomat::result<std::unique_ptr<ICU4XListFormatter>, ICU4XDataError> create_unit_with_length(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XListLength length);

  inline std::string format_valid_utf8(diplomat::span<const std::string_view> list) const;

  inline std::string format_utf8(diplomat::span<const std::string_view> list) const;

  inline std::string format_utf16(diplomat::span<const std::u16string_view> list) const;

  inline const capi::ICU4XListFormatter* AsFFI() const;
  inline capi::ICU4XListFormatter* AsFFI();
  inline static const ICU4XListFormatter* FromFFI(const capi::ICU4XListFormatter* ptr);
  inline static ICU4XListFormatter* FromFFI(capi::ICU4XListFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XListFormatter() = delete;
  ICU4XListFormatter(const ICU4XListFormatter&) = delete;
  ICU4XListFormatter(ICU4XListFormatter&&) noexcept = delete;
  ICU4XListFormatter operator=(const ICU4XListFormatter&) = delete;
  ICU4XListFormatter operator=(ICU4XListFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XListFormatter_D_HPP
