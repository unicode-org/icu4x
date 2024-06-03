#ifndef ICU4XLocaleDirectionality_D_HPP
#define ICU4XLocaleDirectionality_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XError.d.hpp"
#include "ICU4XLocaleDirection.d.hpp"
#include "ICU4XLocaleDirectionality.d.h"

class ICU4XDataProvider;
class ICU4XLocale;
class ICU4XLocaleExpander;
class ICU4XError;
class ICU4XLocaleDirection;


class ICU4XLocaleDirectionality {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XLocaleDirectionality>, ICU4XError> create(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XLocaleDirectionality>, ICU4XError> create_with_expander(const ICU4XDataProvider& provider, const ICU4XLocaleExpander& expander);

  inline ICU4XLocaleDirection get(const ICU4XLocale& locale) const;

  inline bool is_left_to_right(const ICU4XLocale& locale) const;

  inline bool is_right_to_left(const ICU4XLocale& locale) const;

  inline const capi::ICU4XLocaleDirectionality* AsFFI() const;
  inline capi::ICU4XLocaleDirectionality* AsFFI();
  inline static const ICU4XLocaleDirectionality* FromFFI(const capi::ICU4XLocaleDirectionality* ptr);
  inline static ICU4XLocaleDirectionality* FromFFI(capi::ICU4XLocaleDirectionality* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XLocaleDirectionality() = delete;
  ICU4XLocaleDirectionality(const ICU4XLocaleDirectionality&) = delete;
  ICU4XLocaleDirectionality(ICU4XLocaleDirectionality&&) noexcept = delete;
  ICU4XLocaleDirectionality operator=(const ICU4XLocaleDirectionality&) = delete;
  ICU4XLocaleDirectionality operator=(ICU4XLocaleDirectionality&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XLocaleDirectionality_D_HPP
