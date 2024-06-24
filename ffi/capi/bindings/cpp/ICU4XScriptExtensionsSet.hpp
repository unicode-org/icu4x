#ifndef ICU4XScriptExtensionsSet_HPP
#define ICU4XScriptExtensionsSet_HPP

#include "ICU4XScriptExtensionsSet.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    extern "C" {
    
    bool ICU4XScriptExtensionsSet_contains(const ICU4XScriptExtensionsSet* self, uint16_t script);
    
    size_t ICU4XScriptExtensionsSet_count(const ICU4XScriptExtensionsSet* self);
    
    typedef struct ICU4XScriptExtensionsSet_script_at_result {union {uint16_t ok; }; bool is_ok;} ICU4XScriptExtensionsSet_script_at_result;
    ICU4XScriptExtensionsSet_script_at_result ICU4XScriptExtensionsSet_script_at(const ICU4XScriptExtensionsSet* self, size_t index);
    
    
    void ICU4XScriptExtensionsSet_destroy(ICU4XScriptExtensionsSet* self);
    
    } // extern "C"
}

inline bool ICU4XScriptExtensionsSet::contains(uint16_t script) const {
  auto result = capi::ICU4XScriptExtensionsSet_contains(this->AsFFI(),
    script);
  return result;
}

inline size_t ICU4XScriptExtensionsSet::count() const {
  auto result = capi::ICU4XScriptExtensionsSet_count(this->AsFFI());
  return result;
}

inline std::optional<uint16_t> ICU4XScriptExtensionsSet::script_at(size_t index) const {
  auto result = capi::ICU4XScriptExtensionsSet_script_at(this->AsFFI(),
    index);
  return result.is_ok ? std::optional<uint16_t>(result.ok) : std::nullopt;
}

inline const capi::ICU4XScriptExtensionsSet* ICU4XScriptExtensionsSet::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XScriptExtensionsSet*>(this);
}

inline capi::ICU4XScriptExtensionsSet* ICU4XScriptExtensionsSet::AsFFI() {
  return reinterpret_cast<capi::ICU4XScriptExtensionsSet*>(this);
}

inline const ICU4XScriptExtensionsSet* ICU4XScriptExtensionsSet::FromFFI(const capi::ICU4XScriptExtensionsSet* ptr) {
  return reinterpret_cast<const ICU4XScriptExtensionsSet*>(ptr);
}

inline ICU4XScriptExtensionsSet* ICU4XScriptExtensionsSet::FromFFI(capi::ICU4XScriptExtensionsSet* ptr) {
  return reinterpret_cast<ICU4XScriptExtensionsSet*>(ptr);
}

inline void ICU4XScriptExtensionsSet::operator delete(void* ptr) {
  capi::ICU4XScriptExtensionsSet_destroy(reinterpret_cast<capi::ICU4XScriptExtensionsSet*>(ptr));
}


#endif // ICU4XScriptExtensionsSet_HPP
