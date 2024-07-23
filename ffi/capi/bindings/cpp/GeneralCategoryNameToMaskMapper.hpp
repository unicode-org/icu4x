#ifndef GeneralCategoryNameToMaskMapper_HPP
#define GeneralCategoryNameToMaskMapper_HPP

#include "GeneralCategoryNameToMaskMapper.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    uint32_t icu4x_GeneralCategoryNameToMaskMapper_get_strict_mv1(const diplomat::capi::GeneralCategoryNameToMaskMapper* self, const char* name_data, size_t name_len);
    
    uint32_t icu4x_GeneralCategoryNameToMaskMapper_get_loose_mv1(const diplomat::capi::GeneralCategoryNameToMaskMapper* self, const char* name_data, size_t name_len);
    
    typedef struct icu4x_GeneralCategoryNameToMaskMapper_load_mv1_result {union {diplomat::capi::GeneralCategoryNameToMaskMapper* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_GeneralCategoryNameToMaskMapper_load_mv1_result;
    icu4x_GeneralCategoryNameToMaskMapper_load_mv1_result icu4x_GeneralCategoryNameToMaskMapper_load_mv1(const diplomat::capi::DataProvider* provider);
    
    
    void icu4x_GeneralCategoryNameToMaskMapper_destroy_mv1(GeneralCategoryNameToMaskMapper* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline uint32_t GeneralCategoryNameToMaskMapper::get_strict(std::string_view name) const {
  auto result = diplomat::capi::icu4x_GeneralCategoryNameToMaskMapper_get_strict_mv1(this->AsFFI(),
    name.data(),
    name.size());
  return result;
}

inline uint32_t GeneralCategoryNameToMaskMapper::get_loose(std::string_view name) const {
  auto result = diplomat::capi::icu4x_GeneralCategoryNameToMaskMapper_get_loose_mv1(this->AsFFI(),
    name.data(),
    name.size());
  return result;
}

inline diplomat::result<std::unique_ptr<GeneralCategoryNameToMaskMapper>, DataError> GeneralCategoryNameToMaskMapper::load(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_GeneralCategoryNameToMaskMapper_load_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<GeneralCategoryNameToMaskMapper>, DataError>(diplomat::Ok<std::unique_ptr<GeneralCategoryNameToMaskMapper>>(std::unique_ptr<GeneralCategoryNameToMaskMapper>(GeneralCategoryNameToMaskMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<GeneralCategoryNameToMaskMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline const diplomat::capi::GeneralCategoryNameToMaskMapper* GeneralCategoryNameToMaskMapper::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::GeneralCategoryNameToMaskMapper*>(this);
}

inline diplomat::capi::GeneralCategoryNameToMaskMapper* GeneralCategoryNameToMaskMapper::AsFFI() {
  return reinterpret_cast<diplomat::capi::GeneralCategoryNameToMaskMapper*>(this);
}

inline const GeneralCategoryNameToMaskMapper* GeneralCategoryNameToMaskMapper::FromFFI(const diplomat::capi::GeneralCategoryNameToMaskMapper* ptr) {
  return reinterpret_cast<const GeneralCategoryNameToMaskMapper*>(ptr);
}

inline GeneralCategoryNameToMaskMapper* GeneralCategoryNameToMaskMapper::FromFFI(diplomat::capi::GeneralCategoryNameToMaskMapper* ptr) {
  return reinterpret_cast<GeneralCategoryNameToMaskMapper*>(ptr);
}

inline void GeneralCategoryNameToMaskMapper::operator delete(void* ptr) {
  diplomat::capi::icu4x_GeneralCategoryNameToMaskMapper_destroy_mv1(reinterpret_cast<diplomat::capi::GeneralCategoryNameToMaskMapper*>(ptr));
}


#endif // GeneralCategoryNameToMaskMapper_HPP
