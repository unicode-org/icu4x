#ifndef ICU4XGeneralCategoryNameToMaskMapper_HPP
#define ICU4XGeneralCategoryNameToMaskMapper_HPP

#include "ICU4XGeneralCategoryNameToMaskMapper.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"


namespace capi {
    extern "C" {
    
    uint32_t ICU4XGeneralCategoryNameToMaskMapper_get_strict(const ICU4XGeneralCategoryNameToMaskMapper* self, const char* name_data, size_t name_len);
    
    uint32_t ICU4XGeneralCategoryNameToMaskMapper_get_loose(const ICU4XGeneralCategoryNameToMaskMapper* self, const char* name_data, size_t name_len);
    
    struct ICU4XGeneralCategoryNameToMaskMapper_load_result {union {ICU4XGeneralCategoryNameToMaskMapper* ok; ICU4XDataError err;}; bool is_ok;};
    struct ICU4XGeneralCategoryNameToMaskMapper_load_result ICU4XGeneralCategoryNameToMaskMapper_load(const ICU4XDataProvider* provider);
    
    
    void ICU4XGeneralCategoryNameToMaskMapper_destroy(ICU4XGeneralCategoryNameToMaskMapper* self);
    
    } // extern "C"
}

inline uint32_t ICU4XGeneralCategoryNameToMaskMapper::get_strict(std::string_view name) const {
  auto result = capi::ICU4XGeneralCategoryNameToMaskMapper_get_strict(this->AsFFI(),
    name.data(),
    name.size());
  return result;
}

inline uint32_t ICU4XGeneralCategoryNameToMaskMapper::get_loose(std::string_view name) const {
  auto result = capi::ICU4XGeneralCategoryNameToMaskMapper_get_loose(this->AsFFI(),
    name.data(),
    name.size());
  return result;
}

inline diplomat::result<std::unique_ptr<ICU4XGeneralCategoryNameToMaskMapper>, ICU4XDataError> ICU4XGeneralCategoryNameToMaskMapper::load(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XGeneralCategoryNameToMaskMapper_load(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XGeneralCategoryNameToMaskMapper>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XGeneralCategoryNameToMaskMapper>>(std::unique_ptr<ICU4XGeneralCategoryNameToMaskMapper>(ICU4XGeneralCategoryNameToMaskMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XGeneralCategoryNameToMaskMapper>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline const capi::ICU4XGeneralCategoryNameToMaskMapper* ICU4XGeneralCategoryNameToMaskMapper::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XGeneralCategoryNameToMaskMapper*>(this);
}

inline capi::ICU4XGeneralCategoryNameToMaskMapper* ICU4XGeneralCategoryNameToMaskMapper::AsFFI() {
  return reinterpret_cast<capi::ICU4XGeneralCategoryNameToMaskMapper*>(this);
}

inline const ICU4XGeneralCategoryNameToMaskMapper* ICU4XGeneralCategoryNameToMaskMapper::FromFFI(const capi::ICU4XGeneralCategoryNameToMaskMapper* ptr) {
  return reinterpret_cast<const ICU4XGeneralCategoryNameToMaskMapper*>(ptr);
}

inline ICU4XGeneralCategoryNameToMaskMapper* ICU4XGeneralCategoryNameToMaskMapper::FromFFI(capi::ICU4XGeneralCategoryNameToMaskMapper* ptr) {
  return reinterpret_cast<ICU4XGeneralCategoryNameToMaskMapper*>(ptr);
}

inline void ICU4XGeneralCategoryNameToMaskMapper::operator delete(void* ptr) {
  capi::ICU4XGeneralCategoryNameToMaskMapper_destroy(reinterpret_cast<capi::ICU4XGeneralCategoryNameToMaskMapper*>(ptr));
}


#endif // ICU4XGeneralCategoryNameToMaskMapper_HPP
