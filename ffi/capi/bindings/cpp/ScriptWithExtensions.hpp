#ifndef ScriptWithExtensions_HPP
#define ScriptWithExtensions_HPP

#include "ScriptWithExtensions.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CodePointRangeIterator.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "ScriptWithExtensionsBorrowed.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XScriptWithExtensions_create_result {union {ScriptWithExtensions* ok; DataError err;}; bool is_ok;} ICU4XScriptWithExtensions_create_result;
    ICU4XScriptWithExtensions_create_result ICU4XScriptWithExtensions_create(const DataProvider* provider);
    
    uint16_t ICU4XScriptWithExtensions_get_script_val(const ScriptWithExtensions* self, uint32_t code_point);
    
    bool ICU4XScriptWithExtensions_has_script(const ScriptWithExtensions* self, uint32_t code_point, uint16_t script);
    
    ScriptWithExtensionsBorrowed* ICU4XScriptWithExtensions_as_borrowed(const ScriptWithExtensions* self);
    
    CodePointRangeIterator* ICU4XScriptWithExtensions_iter_ranges_for_script(const ScriptWithExtensions* self, uint16_t script);
    
    
    void ICU4XScriptWithExtensions_destroy(ScriptWithExtensions* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ScriptWithExtensions>, DataError> ScriptWithExtensions::create(const DataProvider& provider) {
  auto result = capi::ICU4XScriptWithExtensions_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ScriptWithExtensions>, DataError>(diplomat::Ok<std::unique_ptr<ScriptWithExtensions>>(std::unique_ptr<ScriptWithExtensions>(ScriptWithExtensions::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ScriptWithExtensions>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline uint16_t ScriptWithExtensions::get_script_val(uint32_t code_point) const {
  auto result = capi::ICU4XScriptWithExtensions_get_script_val(this->AsFFI(),
    code_point);
  return result;
}

inline bool ScriptWithExtensions::has_script(uint32_t code_point, uint16_t script) const {
  auto result = capi::ICU4XScriptWithExtensions_has_script(this->AsFFI(),
    code_point,
    script);
  return result;
}

inline std::unique_ptr<ScriptWithExtensionsBorrowed> ScriptWithExtensions::as_borrowed() const {
  auto result = capi::ICU4XScriptWithExtensions_as_borrowed(this->AsFFI());
  return std::unique_ptr<ScriptWithExtensionsBorrowed>(ScriptWithExtensionsBorrowed::FromFFI(result));
}

inline std::unique_ptr<CodePointRangeIterator> ScriptWithExtensions::iter_ranges_for_script(uint16_t script) const {
  auto result = capi::ICU4XScriptWithExtensions_iter_ranges_for_script(this->AsFFI(),
    script);
  return std::unique_ptr<CodePointRangeIterator>(CodePointRangeIterator::FromFFI(result));
}

inline const capi::ScriptWithExtensions* ScriptWithExtensions::AsFFI() const {
  return reinterpret_cast<const capi::ScriptWithExtensions*>(this);
}

inline capi::ScriptWithExtensions* ScriptWithExtensions::AsFFI() {
  return reinterpret_cast<capi::ScriptWithExtensions*>(this);
}

inline const ScriptWithExtensions* ScriptWithExtensions::FromFFI(const capi::ScriptWithExtensions* ptr) {
  return reinterpret_cast<const ScriptWithExtensions*>(ptr);
}

inline ScriptWithExtensions* ScriptWithExtensions::FromFFI(capi::ScriptWithExtensions* ptr) {
  return reinterpret_cast<ScriptWithExtensions*>(ptr);
}

inline void ScriptWithExtensions::operator delete(void* ptr) {
  capi::ICU4XScriptWithExtensions_destroy(reinterpret_cast<capi::ScriptWithExtensions*>(ptr));
}


#endif // ScriptWithExtensions_HPP
