#ifndef ICU4XScriptWithExtensionsSet_HPP
#define ICU4XScriptWithExtensionsSet_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XScriptWithExtensionsSet.h"

class ICU4XDataProvider;
class ICU4XScriptWithExtensionsSet;
#include "ICU4XError.hpp"

/**
 * A destruction policy for using ICU4XScriptWithExtensionsSet with std::unique_ptr.
 */
struct ICU4XScriptWithExtensionsSetDeleter {
  void operator()(capi::ICU4XScriptWithExtensionsSet* l) const noexcept {
    capi::ICU4XScriptWithExtensionsSet_destroy(l);
  }
};

/**
 * An ICU4X ScriptWithExtensions map object, capable of holding a map of codepoints to scriptextensions values
 * 
 * See the [Rust documentation for `ScriptWithExtensions`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptWithExtensions.html) for more information.
 */
class ICU4XScriptWithExtensionsSet {
 public:

  /**
   * 
   * 
   * See the [Rust documentation for `load_script_with_extensions_unstable`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/fn.load_script_with_extensions_unstable.html) for more information.
   */
  static diplomat::result<ICU4XScriptWithExtensionsSet, ICU4XError> load(const ICU4XDataProvider& provider);

  /**
   * Get the Script property value for a code point
   * 
   * See the [Rust documentation for `get_script_val`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptWithExtensions.html#method.get_script_val) for more information.
   */
  uint16_t get_script_val(uint32_t code_point) const;

  /**
   * Check if the Script_Extensions property of the given code point covers the given script
   * 
   * See the [Rust documentation for `has_script`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptWithExtensions.html#method.has_script) for more information.
   */
  bool has_script(uint32_t code_point, uint16_t script) const;
  inline const capi::ICU4XScriptWithExtensionsSet* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XScriptWithExtensionsSet* AsFFIMut() { return this->inner.get(); }
  inline ICU4XScriptWithExtensionsSet(capi::ICU4XScriptWithExtensionsSet* i) : inner(i) {}
  ICU4XScriptWithExtensionsSet() = default;
  ICU4XScriptWithExtensionsSet(ICU4XScriptWithExtensionsSet&&) noexcept = default;
  ICU4XScriptWithExtensionsSet& operator=(ICU4XScriptWithExtensionsSet&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XScriptWithExtensionsSet, ICU4XScriptWithExtensionsSetDeleter> inner;
};

#include "ICU4XDataProvider.hpp"

inline diplomat::result<ICU4XScriptWithExtensionsSet, ICU4XError> ICU4XScriptWithExtensionsSet::load(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XScriptWithExtensionsSet_load(provider.AsFFI());
  diplomat::result<ICU4XScriptWithExtensionsSet, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XScriptWithExtensionsSet>(std::move(ICU4XScriptWithExtensionsSet(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline uint16_t ICU4XScriptWithExtensionsSet::get_script_val(uint32_t code_point) const {
  return capi::ICU4XScriptWithExtensionsSet_get_script_val(this->inner.get(), code_point);
}
inline bool ICU4XScriptWithExtensionsSet::has_script(uint32_t code_point, uint16_t script) const {
  return capi::ICU4XScriptWithExtensionsSet_has_script(this->inner.get(), code_point, script);
}
#endif
