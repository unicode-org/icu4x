#ifndef GeneralCategoryNameToMaskMapper_HPP
#define GeneralCategoryNameToMaskMapper_HPP

#include "GeneralCategoryNameToMaskMapper.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"


namespace capi {
    extern "C" {
    
    uint32_t ICU4XGeneralCategoryNameToMaskMapper_get_strict(const GeneralCategoryNameToMaskMapper* self, const char* name_data, size_t name_len);
    
    uint32_t ICU4XGeneralCategoryNameToMaskMapper_get_loose(const GeneralCategoryNameToMaskMapper* self, const char* name_data, size_t name_len);
    
    typedef struct ICU4XGeneralCategoryNameToMaskMapper_load_result {union {GeneralCategoryNameToMaskMapper* ok; DataError err;}; bool is_ok;} ICU4XGeneralCategoryNameToMaskMapper_load_result;
    ICU4XGeneralCategoryNameToMaskMapper_load_result ICU4XGeneralCategoryNameToMaskMapper_load(const DataProvider* provider);
    
    
    void ICU4XGeneralCategoryNameToMaskMapper_destroy(GeneralCategoryNameToMaskMapper* self);
    
    } // extern "C"
}

inline uint32_t GeneralCategoryNameToMaskMapper::get_strict(std::string_view name) const {
  auto result = capi::ICU4XGeneralCategoryNameToMaskMapper_get_strict(this->AsFFI(),
    name.data(),
    name.size());
  return result;
}

inline uint32_t GeneralCategoryNameToMaskMapper::get_loose(std::string_view name) const {
  auto result = capi::ICU4XGeneralCategoryNameToMaskMapper_get_loose(this->AsFFI(),
    name.data(),
    name.size());
  return result;
}

inline diplomat::result<std::unique_ptr<GeneralCategoryNameToMaskMapper>, DataError> GeneralCategoryNameToMaskMapper::load(const DataProvider& provider) {
  auto result = capi::ICU4XGeneralCategoryNameToMaskMapper_load(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<GeneralCategoryNameToMaskMapper>, DataError>(diplomat::Ok<std::unique_ptr<GeneralCategoryNameToMaskMapper>>(std::unique_ptr<GeneralCategoryNameToMaskMapper>(GeneralCategoryNameToMaskMapper::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<GeneralCategoryNameToMaskMapper>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline const capi::GeneralCategoryNameToMaskMapper* GeneralCategoryNameToMaskMapper::AsFFI() const {
  return reinterpret_cast<const capi::GeneralCategoryNameToMaskMapper*>(this);
}

inline capi::GeneralCategoryNameToMaskMapper* GeneralCategoryNameToMaskMapper::AsFFI() {
  return reinterpret_cast<capi::GeneralCategoryNameToMaskMapper*>(this);
}

inline const GeneralCategoryNameToMaskMapper* GeneralCategoryNameToMaskMapper::FromFFI(const capi::GeneralCategoryNameToMaskMapper* ptr) {
  return reinterpret_cast<const GeneralCategoryNameToMaskMapper*>(ptr);
}

inline GeneralCategoryNameToMaskMapper* GeneralCategoryNameToMaskMapper::FromFFI(capi::GeneralCategoryNameToMaskMapper* ptr) {
  return reinterpret_cast<GeneralCategoryNameToMaskMapper*>(ptr);
}

inline void GeneralCategoryNameToMaskMapper::operator delete(void* ptr) {
  capi::ICU4XGeneralCategoryNameToMaskMapper_destroy(reinterpret_cast<capi::GeneralCategoryNameToMaskMapper*>(ptr));
}


#endif // GeneralCategoryNameToMaskMapper_HPP
