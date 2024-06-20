#ifndef ICU4XGeneralCategoryNameToMaskMapper_D_HPP
#define ICU4XGeneralCategoryNameToMaskMapper_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.d.hpp"
#include "ICU4XGeneralCategoryNameToMaskMapper.d.h"

class ICU4XDataProvider;
class ICU4XDataError;


class ICU4XGeneralCategoryNameToMaskMapper {
public:

  inline uint32_t get_strict(std::string_view name) const;

  inline uint32_t get_loose(std::string_view name) const;

  inline static diplomat::result<std::unique_ptr<ICU4XGeneralCategoryNameToMaskMapper>, ICU4XDataError> load(const ICU4XDataProvider& provider);

  inline const capi::ICU4XGeneralCategoryNameToMaskMapper* AsFFI() const;
  inline capi::ICU4XGeneralCategoryNameToMaskMapper* AsFFI();
  inline static const ICU4XGeneralCategoryNameToMaskMapper* FromFFI(const capi::ICU4XGeneralCategoryNameToMaskMapper* ptr);
  inline static ICU4XGeneralCategoryNameToMaskMapper* FromFFI(capi::ICU4XGeneralCategoryNameToMaskMapper* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XGeneralCategoryNameToMaskMapper() = delete;
  ICU4XGeneralCategoryNameToMaskMapper(const ICU4XGeneralCategoryNameToMaskMapper&) = delete;
  ICU4XGeneralCategoryNameToMaskMapper(ICU4XGeneralCategoryNameToMaskMapper&&) noexcept = delete;
  ICU4XGeneralCategoryNameToMaskMapper operator=(const ICU4XGeneralCategoryNameToMaskMapper&) = delete;
  ICU4XGeneralCategoryNameToMaskMapper operator=(ICU4XGeneralCategoryNameToMaskMapper&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XGeneralCategoryNameToMaskMapper_D_HPP
