#ifndef ScriptWithExtensionsBorrowed_HPP
#define ScriptWithExtensionsBorrowed_HPP

#include "ScriptWithExtensionsBorrowed.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CodePointSetData.hpp"
#include "ScriptExtensionsSet.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    uint16_t icu4x_ScriptWithExtensionsBorrowed_get_script_val_mv1(const diplomat::capi::ScriptWithExtensionsBorrowed* self, uint32_t code_point);
    
    diplomat::capi::ScriptExtensionsSet* icu4x_ScriptWithExtensionsBorrowed_get_script_extensions_val_mv1(const diplomat::capi::ScriptWithExtensionsBorrowed* self, uint32_t code_point);
    
    bool icu4x_ScriptWithExtensionsBorrowed_has_script_mv1(const diplomat::capi::ScriptWithExtensionsBorrowed* self, uint32_t code_point, uint16_t script);
    
    diplomat::capi::CodePointSetData* icu4x_ScriptWithExtensionsBorrowed_get_script_extensions_set_mv1(const diplomat::capi::ScriptWithExtensionsBorrowed* self, uint16_t script);
    
    
    void icu4x_ScriptWithExtensionsBorrowed_destroy_mv1(ScriptWithExtensionsBorrowed* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline uint16_t ScriptWithExtensionsBorrowed::get_script_val(uint32_t code_point) const {
  auto result = diplomat::capi::icu4x_ScriptWithExtensionsBorrowed_get_script_val_mv1(this->AsFFI(),
    code_point);
  return result;
}

inline std::unique_ptr<ScriptExtensionsSet> ScriptWithExtensionsBorrowed::get_script_extensions_val(uint32_t code_point) const {
  auto result = diplomat::capi::icu4x_ScriptWithExtensionsBorrowed_get_script_extensions_val_mv1(this->AsFFI(),
    code_point);
  return std::unique_ptr<ScriptExtensionsSet>(ScriptExtensionsSet::FromFFI(result));
}

inline bool ScriptWithExtensionsBorrowed::has_script(uint32_t code_point, uint16_t script) const {
  auto result = diplomat::capi::icu4x_ScriptWithExtensionsBorrowed_has_script_mv1(this->AsFFI(),
    code_point,
    script);
  return result;
}

inline std::unique_ptr<CodePointSetData> ScriptWithExtensionsBorrowed::get_script_extensions_set(uint16_t script) const {
  auto result = diplomat::capi::icu4x_ScriptWithExtensionsBorrowed_get_script_extensions_set_mv1(this->AsFFI(),
    script);
  return std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result));
}

inline const diplomat::capi::ScriptWithExtensionsBorrowed* ScriptWithExtensionsBorrowed::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::ScriptWithExtensionsBorrowed*>(this);
}

inline diplomat::capi::ScriptWithExtensionsBorrowed* ScriptWithExtensionsBorrowed::AsFFI() {
  return reinterpret_cast<diplomat::capi::ScriptWithExtensionsBorrowed*>(this);
}

inline const ScriptWithExtensionsBorrowed* ScriptWithExtensionsBorrowed::FromFFI(const diplomat::capi::ScriptWithExtensionsBorrowed* ptr) {
  return reinterpret_cast<const ScriptWithExtensionsBorrowed*>(ptr);
}

inline ScriptWithExtensionsBorrowed* ScriptWithExtensionsBorrowed::FromFFI(diplomat::capi::ScriptWithExtensionsBorrowed* ptr) {
  return reinterpret_cast<ScriptWithExtensionsBorrowed*>(ptr);
}

inline void ScriptWithExtensionsBorrowed::operator delete(void* ptr) {
  diplomat::capi::icu4x_ScriptWithExtensionsBorrowed_destroy_mv1(reinterpret_cast<diplomat::capi::ScriptWithExtensionsBorrowed*>(ptr));
}


#endif // ScriptWithExtensionsBorrowed_HPP
