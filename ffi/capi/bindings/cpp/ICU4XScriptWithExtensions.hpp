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
#include "CodePointRangeIterator.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XError.hpp"
#include "ICU4XScriptWithExtensions.h"
#include "ICU4XScriptWithExtensionsBorrowed.hpp"


inline diplomat::result<std::unique_ptr<ICU4XScriptWithExtensions>, ICU4XError> ICU4XScriptWithExtensions::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XScriptWithExtensions_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XScriptWithExtensions>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XScriptWithExtensions>>(std::unique_ptr<ICU4XScriptWithExtensions>(ICU4XScriptWithExtensions::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XScriptWithExtensions>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
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

inline std::unique_ptr<CodePointRangeIterator> ICU4XScriptWithExtensions::iter_ranges_for_script(uint16_t script) const {
  auto result = capi::ICU4XScriptWithExtensions_iter_ranges_for_script(this->AsFFI(),
    script);
  return std::unique_ptr<CodePointRangeIterator>(CodePointRangeIterator::FromFFI(result));
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
