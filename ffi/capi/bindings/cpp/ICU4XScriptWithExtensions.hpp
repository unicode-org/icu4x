#ifndef ICU4XScriptWithExtensions_HPP
#define ICU4XScriptWithExtensions_HPP

#include "ICU4XScriptWithExtensions.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCodePointRangeIterator.hpp"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XScriptWithExtensionsBorrowed.hpp"


namespace capi {
    extern "C" {
    
    struct ICU4XScriptWithExtensions_create_result {union {ICU4XScriptWithExtensions* ok; ICU4XDataError err;}; bool is_ok;};
    struct ICU4XScriptWithExtensions_create_result ICU4XScriptWithExtensions_create(const ICU4XDataProvider* provider);
    
    uint16_t ICU4XScriptWithExtensions_get_script_val(const ICU4XScriptWithExtensions* self, uint32_t code_point);
    
    bool ICU4XScriptWithExtensions_has_script(const ICU4XScriptWithExtensions* self, uint32_t code_point, uint16_t script);
    
    ICU4XScriptWithExtensionsBorrowed* ICU4XScriptWithExtensions_as_borrowed(const ICU4XScriptWithExtensions* self);
    
    ICU4XCodePointRangeIterator* ICU4XScriptWithExtensions_iter_ranges_for_script(const ICU4XScriptWithExtensions* self, uint16_t script);
    
    
    void ICU4XScriptWithExtensions_destroy(ICU4XScriptWithExtensions* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XScriptWithExtensions>, ICU4XDataError> ICU4XScriptWithExtensions::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XScriptWithExtensions_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XScriptWithExtensions>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XScriptWithExtensions>>(std::unique_ptr<ICU4XScriptWithExtensions>(ICU4XScriptWithExtensions::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XScriptWithExtensions>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline uint16_t ICU4XScriptWithExtensions::get_script_val(uint32_t code_point) const {
  auto result = capi::ICU4XScriptWithExtensions_get_script_val(this->AsFFI(),
    code_point);
  return result;
}

inline bool ICU4XScriptWithExtensions::has_script(uint32_t code_point, uint16_t script) const {
  auto result = capi::ICU4XScriptWithExtensions_has_script(this->AsFFI(),
    code_point,
    script);
  return result;
}

inline std::unique_ptr<ICU4XScriptWithExtensionsBorrowed> ICU4XScriptWithExtensions::as_borrowed() const {
  auto result = capi::ICU4XScriptWithExtensions_as_borrowed(this->AsFFI());
  return std::unique_ptr<ICU4XScriptWithExtensionsBorrowed>(ICU4XScriptWithExtensionsBorrowed::FromFFI(result));
}

inline std::unique_ptr<ICU4XCodePointRangeIterator> ICU4XScriptWithExtensions::iter_ranges_for_script(uint16_t script) const {
  auto result = capi::ICU4XScriptWithExtensions_iter_ranges_for_script(this->AsFFI(),
    script);
  return std::unique_ptr<ICU4XCodePointRangeIterator>(ICU4XCodePointRangeIterator::FromFFI(result));
}

inline const capi::ICU4XScriptWithExtensions* ICU4XScriptWithExtensions::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XScriptWithExtensions*>(this);
}

inline capi::ICU4XScriptWithExtensions* ICU4XScriptWithExtensions::AsFFI() {
  return reinterpret_cast<capi::ICU4XScriptWithExtensions*>(this);
}

inline const ICU4XScriptWithExtensions* ICU4XScriptWithExtensions::FromFFI(const capi::ICU4XScriptWithExtensions* ptr) {
  return reinterpret_cast<const ICU4XScriptWithExtensions*>(ptr);
}

inline ICU4XScriptWithExtensions* ICU4XScriptWithExtensions::FromFFI(capi::ICU4XScriptWithExtensions* ptr) {
  return reinterpret_cast<ICU4XScriptWithExtensions*>(ptr);
}

inline void ICU4XScriptWithExtensions::operator delete(void* ptr) {
  capi::ICU4XScriptWithExtensions_destroy(reinterpret_cast<capi::ICU4XScriptWithExtensions*>(ptr));
}


#endif // ICU4XScriptWithExtensions_HPP
