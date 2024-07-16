#ifndef ScriptExtensionsSet_HPP
#define ScriptExtensionsSet_HPP

#include "ScriptExtensionsSet.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    bool ICU4XScriptExtensionsSet_contains(const diplomat::capi::ScriptExtensionsSet* self, uint16_t script);
    
    size_t ICU4XScriptExtensionsSet_count(const diplomat::capi::ScriptExtensionsSet* self);
    
    typedef struct ICU4XScriptExtensionsSet_script_at_result {union {uint16_t ok; }; bool is_ok;} ICU4XScriptExtensionsSet_script_at_result;
    ICU4XScriptExtensionsSet_script_at_result ICU4XScriptExtensionsSet_script_at(const diplomat::capi::ScriptExtensionsSet* self, size_t index);
    
    
    void ICU4XScriptExtensionsSet_destroy(ScriptExtensionsSet* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline bool ScriptExtensionsSet::contains(uint16_t script) const {
  auto result = diplomat::capi::ICU4XScriptExtensionsSet_contains(this->AsFFI(),
    script);
  return result;
}

inline size_t ScriptExtensionsSet::count() const {
  auto result = diplomat::capi::ICU4XScriptExtensionsSet_count(this->AsFFI());
  return result;
}

inline std::optional<uint16_t> ScriptExtensionsSet::script_at(size_t index) const {
  auto result = diplomat::capi::ICU4XScriptExtensionsSet_script_at(this->AsFFI(),
    index);
  return result.is_ok ? std::optional<uint16_t>(result.ok) : std::nullopt;
}

inline const diplomat::capi::ScriptExtensionsSet* ScriptExtensionsSet::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::ScriptExtensionsSet*>(this);
}

inline diplomat::capi::ScriptExtensionsSet* ScriptExtensionsSet::AsFFI() {
  return reinterpret_cast<diplomat::capi::ScriptExtensionsSet*>(this);
}

inline const ScriptExtensionsSet* ScriptExtensionsSet::FromFFI(const diplomat::capi::ScriptExtensionsSet* ptr) {
  return reinterpret_cast<const ScriptExtensionsSet*>(ptr);
}

inline ScriptExtensionsSet* ScriptExtensionsSet::FromFFI(diplomat::capi::ScriptExtensionsSet* ptr) {
  return reinterpret_cast<ScriptExtensionsSet*>(ptr);
}

inline void ScriptExtensionsSet::operator delete(void* ptr) {
  diplomat::capi::ICU4XScriptExtensionsSet_destroy(reinterpret_cast<diplomat::capi::ScriptExtensionsSet*>(ptr));
}


#endif // ScriptExtensionsSet_HPP
