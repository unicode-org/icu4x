#ifndef ScriptWithExtensions_HPP
#define ScriptWithExtensions_HPP

#include "ScriptWithExtensions.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "CodePointRangeIterator.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "ScriptWithExtensionsBorrowed.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_ScriptWithExtensions_create_mv1_result {union {diplomat::capi::ScriptWithExtensions* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_ScriptWithExtensions_create_mv1_result;
    icu4x_ScriptWithExtensions_create_mv1_result icu4x_ScriptWithExtensions_create_mv1(const diplomat::capi::DataProvider* provider);
    
    uint16_t icu4x_ScriptWithExtensions_get_script_val_mv1(const diplomat::capi::ScriptWithExtensions* self, uint32_t code_point);
    
    bool icu4x_ScriptWithExtensions_has_script_mv1(const diplomat::capi::ScriptWithExtensions* self, uint32_t code_point, uint16_t script);
    
    diplomat::capi::ScriptWithExtensionsBorrowed* icu4x_ScriptWithExtensions_as_borrowed_mv1(const diplomat::capi::ScriptWithExtensions* self);
    
    diplomat::capi::CodePointRangeIterator* icu4x_ScriptWithExtensions_iter_ranges_for_script_mv1(const diplomat::capi::ScriptWithExtensions* self, uint16_t script);
    
    
    void icu4x_ScriptWithExtensions_destroy_mv1(ScriptWithExtensions* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<ScriptWithExtensions>, DataError> ScriptWithExtensions::create(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_ScriptWithExtensions_create_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ScriptWithExtensions>, DataError>(diplomat::Ok<std::unique_ptr<ScriptWithExtensions>>(std::unique_ptr<ScriptWithExtensions>(ScriptWithExtensions::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ScriptWithExtensions>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline uint16_t ScriptWithExtensions::get_script_val(uint32_t code_point) const {
  auto result = diplomat::capi::icu4x_ScriptWithExtensions_get_script_val_mv1(this->AsFFI(),
    code_point);
  return result;
}

inline bool ScriptWithExtensions::has_script(uint32_t code_point, uint16_t script) const {
  auto result = diplomat::capi::icu4x_ScriptWithExtensions_has_script_mv1(this->AsFFI(),
    code_point,
    script);
  return result;
}

inline std::unique_ptr<ScriptWithExtensionsBorrowed> ScriptWithExtensions::as_borrowed() const {
  auto result = diplomat::capi::icu4x_ScriptWithExtensions_as_borrowed_mv1(this->AsFFI());
  return std::unique_ptr<ScriptWithExtensionsBorrowed>(ScriptWithExtensionsBorrowed::FromFFI(result));
}

inline std::unique_ptr<CodePointRangeIterator> ScriptWithExtensions::iter_ranges_for_script(uint16_t script) const {
  auto result = diplomat::capi::icu4x_ScriptWithExtensions_iter_ranges_for_script_mv1(this->AsFFI(),
    script);
  return std::unique_ptr<CodePointRangeIterator>(CodePointRangeIterator::FromFFI(result));
}

inline const diplomat::capi::ScriptWithExtensions* ScriptWithExtensions::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::ScriptWithExtensions*>(this);
}

inline diplomat::capi::ScriptWithExtensions* ScriptWithExtensions::AsFFI() {
  return reinterpret_cast<diplomat::capi::ScriptWithExtensions*>(this);
}

inline const ScriptWithExtensions* ScriptWithExtensions::FromFFI(const diplomat::capi::ScriptWithExtensions* ptr) {
  return reinterpret_cast<const ScriptWithExtensions*>(ptr);
}

inline ScriptWithExtensions* ScriptWithExtensions::FromFFI(diplomat::capi::ScriptWithExtensions* ptr) {
  return reinterpret_cast<ScriptWithExtensions*>(ptr);
}

inline void ScriptWithExtensions::operator delete(void* ptr) {
  diplomat::capi::icu4x_ScriptWithExtensions_destroy_mv1(reinterpret_cast<diplomat::capi::ScriptWithExtensions*>(ptr));
}


#endif // ScriptWithExtensions_HPP
