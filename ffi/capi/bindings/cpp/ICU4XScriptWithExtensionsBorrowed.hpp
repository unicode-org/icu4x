#ifndef ICU4XScriptWithExtensionsBorrowed_HPP
#define ICU4XScriptWithExtensionsBorrowed_HPP

#include "ICU4XScriptWithExtensionsBorrowed.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCodePointSetData.hpp"
#include "ICU4XScriptExtensionsSet.hpp"


namespace capi {
    extern "C" {
    
    uint16_t ICU4XScriptWithExtensionsBorrowed_get_script_val(const ICU4XScriptWithExtensionsBorrowed* self, uint32_t code_point);
    
    ICU4XScriptExtensionsSet* ICU4XScriptWithExtensionsBorrowed_get_script_extensions_val(const ICU4XScriptWithExtensionsBorrowed* self, uint32_t code_point);
    
    bool ICU4XScriptWithExtensionsBorrowed_has_script(const ICU4XScriptWithExtensionsBorrowed* self, uint32_t code_point, uint16_t script);
    
    ICU4XCodePointSetData* ICU4XScriptWithExtensionsBorrowed_get_script_extensions_set(const ICU4XScriptWithExtensionsBorrowed* self, uint16_t script);
    
    
    void ICU4XScriptWithExtensionsBorrowed_destroy(ICU4XScriptWithExtensionsBorrowed* self);
    
    } // extern "C"
}

inline uint16_t ICU4XScriptWithExtensionsBorrowed::get_script_val(uint32_t code_point) const {
  auto result = capi::ICU4XScriptWithExtensionsBorrowed_get_script_val(this->AsFFI(),
    code_point);
  return result;
}

inline std::unique_ptr<ICU4XScriptExtensionsSet> ICU4XScriptWithExtensionsBorrowed::get_script_extensions_val(uint32_t code_point) const {
  auto result = capi::ICU4XScriptWithExtensionsBorrowed_get_script_extensions_val(this->AsFFI(),
    code_point);
  return std::unique_ptr<ICU4XScriptExtensionsSet>(ICU4XScriptExtensionsSet::FromFFI(result));
}

inline bool ICU4XScriptWithExtensionsBorrowed::has_script(uint32_t code_point, uint16_t script) const {
  auto result = capi::ICU4XScriptWithExtensionsBorrowed_has_script(this->AsFFI(),
    code_point,
    script);
  return result;
}

inline std::unique_ptr<ICU4XCodePointSetData> ICU4XScriptWithExtensionsBorrowed::get_script_extensions_set(uint16_t script) const {
  auto result = capi::ICU4XScriptWithExtensionsBorrowed_get_script_extensions_set(this->AsFFI(),
    script);
  return std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result));
}

inline const capi::ICU4XScriptWithExtensionsBorrowed* ICU4XScriptWithExtensionsBorrowed::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XScriptWithExtensionsBorrowed*>(this);
}

inline capi::ICU4XScriptWithExtensionsBorrowed* ICU4XScriptWithExtensionsBorrowed::AsFFI() {
  return reinterpret_cast<capi::ICU4XScriptWithExtensionsBorrowed*>(this);
}

inline const ICU4XScriptWithExtensionsBorrowed* ICU4XScriptWithExtensionsBorrowed::FromFFI(const capi::ICU4XScriptWithExtensionsBorrowed* ptr) {
  return reinterpret_cast<const ICU4XScriptWithExtensionsBorrowed*>(ptr);
}

inline ICU4XScriptWithExtensionsBorrowed* ICU4XScriptWithExtensionsBorrowed::FromFFI(capi::ICU4XScriptWithExtensionsBorrowed* ptr) {
  return reinterpret_cast<ICU4XScriptWithExtensionsBorrowed*>(ptr);
}

inline void ICU4XScriptWithExtensionsBorrowed::operator delete(void* ptr) {
  capi::ICU4XScriptWithExtensionsBorrowed_destroy(reinterpret_cast<capi::ICU4XScriptWithExtensionsBorrowed*>(ptr));
}


#endif // ICU4XScriptWithExtensionsBorrowed_HPP
