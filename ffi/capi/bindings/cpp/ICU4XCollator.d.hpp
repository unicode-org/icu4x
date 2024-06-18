#ifndef ICU4XCollator_D_HPP
#define ICU4XCollator_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCollator.d.h"
#include "ICU4XCollatorOptionsV1.d.hpp"
#include "ICU4XCollatorResolvedOptionsV1.d.hpp"
#include "ICU4XDataError.d.hpp"

class ICU4XDataProvider;
class ICU4XLocale;
struct ICU4XCollatorOptionsV1;
struct ICU4XCollatorResolvedOptionsV1;
class ICU4XDataError;


class ICU4XCollator {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XCollator>, ICU4XDataError> create_v1(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XCollatorOptionsV1 options);

  inline int8_t compare16(std::u16string_view left, std::u16string_view right) const;

  inline int8_t compare(std::string_view left, std::string_view right) const;

  inline ICU4XCollatorResolvedOptionsV1 resolved_options() const;

  inline const capi::ICU4XCollator* AsFFI() const;
  inline capi::ICU4XCollator* AsFFI();
  inline static const ICU4XCollator* FromFFI(const capi::ICU4XCollator* ptr);
  inline static ICU4XCollator* FromFFI(capi::ICU4XCollator* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XCollator() = delete;
  ICU4XCollator(const ICU4XCollator&) = delete;
  ICU4XCollator(ICU4XCollator&&) noexcept = delete;
  ICU4XCollator operator=(const ICU4XCollator&) = delete;
  ICU4XCollator operator=(ICU4XCollator&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XCollator_D_HPP
