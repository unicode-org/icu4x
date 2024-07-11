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


namespace capi {
    extern "C" {
    
    uint16_t ICU4XScriptWithExtensionsBorrowed_get_script_val(const ScriptWithExtensionsBorrowed* self, uint32_t code_point);
    
    ScriptExtensionsSet* ICU4XScriptWithExtensionsBorrowed_get_script_extensions_val(const ScriptWithExtensionsBorrowed* self, uint32_t code_point);
    
    bool ICU4XScriptWithExtensionsBorrowed_has_script(const ScriptWithExtensionsBorrowed* self, uint32_t code_point, uint16_t script);
    
    CodePointSetData* ICU4XScriptWithExtensionsBorrowed_get_script_extensions_set(const ScriptWithExtensionsBorrowed* self, uint16_t script);
    
    
    void ICU4XScriptWithExtensionsBorrowed_destroy(ScriptWithExtensionsBorrowed* self);
    
    } // extern "C"
}

inline uint16_t ScriptWithExtensionsBorrowed::get_script_val(uint32_t code_point) const {
  auto result = capi::ICU4XScriptWithExtensionsBorrowed_get_script_val(this->AsFFI(),
    code_point);
  return result;
}

inline std::unique_ptr<ScriptExtensionsSet> ScriptWithExtensionsBorrowed::get_script_extensions_val(uint32_t code_point) const {
  auto result = capi::ICU4XScriptWithExtensionsBorrowed_get_script_extensions_val(this->AsFFI(),
    code_point);
  return std::unique_ptr<ScriptExtensionsSet>(ScriptExtensionsSet::FromFFI(result));
}

inline bool ScriptWithExtensionsBorrowed::has_script(uint32_t code_point, uint16_t script) const {
  auto result = capi::ICU4XScriptWithExtensionsBorrowed_has_script(this->AsFFI(),
    code_point,
    script);
  return result;
}

inline std::unique_ptr<CodePointSetData> ScriptWithExtensionsBorrowed::get_script_extensions_set(uint16_t script) const {
  auto result = capi::ICU4XScriptWithExtensionsBorrowed_get_script_extensions_set(this->AsFFI(),
    script);
  return std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result));
}

inline const capi::ScriptWithExtensionsBorrowed* ScriptWithExtensionsBorrowed::AsFFI() const {
  return reinterpret_cast<const capi::ScriptWithExtensionsBorrowed*>(this);
}

inline capi::ScriptWithExtensionsBorrowed* ScriptWithExtensionsBorrowed::AsFFI() {
  return reinterpret_cast<capi::ScriptWithExtensionsBorrowed*>(this);
}

inline const ScriptWithExtensionsBorrowed* ScriptWithExtensionsBorrowed::FromFFI(const capi::ScriptWithExtensionsBorrowed* ptr) {
  return reinterpret_cast<const ScriptWithExtensionsBorrowed*>(ptr);
}

inline ScriptWithExtensionsBorrowed* ScriptWithExtensionsBorrowed::FromFFI(capi::ScriptWithExtensionsBorrowed* ptr) {
  return reinterpret_cast<ScriptWithExtensionsBorrowed*>(ptr);
}

inline void ScriptWithExtensionsBorrowed::operator delete(void* ptr) {
  capi::ICU4XScriptWithExtensionsBorrowed_destroy(reinterpret_cast<capi::ScriptWithExtensionsBorrowed*>(ptr));
}


#endif // ScriptWithExtensionsBorrowed_HPP
