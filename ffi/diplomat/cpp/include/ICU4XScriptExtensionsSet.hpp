#ifndef ICU4XScriptExtensionsSet_HPP
#define ICU4XScriptExtensionsSet_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XScriptExtensionsSet.h"


/**
 * A destruction policy for using ICU4XScriptExtensionsSet with std::unique_ptr.
 */
struct ICU4XScriptExtensionsSetDeleter {
  void operator()(capi::ICU4XScriptExtensionsSet* l) const noexcept {
    capi::ICU4XScriptExtensionsSet_destroy(l);
  }
};

/**
 * An object that represents the Script_Extensions property for a single character
 * 
 * See the [Rust documentation for `ScriptExtensionsSet`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptExtensionsSet.html) for more information.
 */
class ICU4XScriptExtensionsSet {
 public:

  /**
   * Check if the Script_Extensions property of the given code point covers the given script
   * 
   * See the [Rust documentation for `contains`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptExtensionsSet.html#method.contains) for more information.
   */
  bool contains(uint16_t script) const;
  inline const capi::ICU4XScriptExtensionsSet* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XScriptExtensionsSet* AsFFIMut() { return this->inner.get(); }
  inline ICU4XScriptExtensionsSet(capi::ICU4XScriptExtensionsSet* i) : inner(i) {}
  ICU4XScriptExtensionsSet() = default;
  ICU4XScriptExtensionsSet(ICU4XScriptExtensionsSet&&) noexcept = default;
  ICU4XScriptExtensionsSet& operator=(ICU4XScriptExtensionsSet&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XScriptExtensionsSet, ICU4XScriptExtensionsSetDeleter> inner;
};


inline bool ICU4XScriptExtensionsSet::contains(uint16_t script) const {
  return capi::ICU4XScriptExtensionsSet_contains(this->inner.get(), script);
}
#endif
