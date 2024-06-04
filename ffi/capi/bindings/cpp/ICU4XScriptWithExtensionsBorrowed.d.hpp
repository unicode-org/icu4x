#ifndef ICU4XScriptWithExtensionsBorrowed_D_HPP
#define ICU4XScriptWithExtensionsBorrowed_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XScriptWithExtensionsBorrowed.d.h"

class ICU4XCodePointSetData;
class ICU4XScriptExtensionsSet;


class ICU4XScriptWithExtensionsBorrowed {
public:

  inline uint16_t get_script_val(uint32_t code_point) const;

  inline std::unique_ptr<ICU4XScriptExtensionsSet> get_script_extensions_val(uint32_t code_point) const;

  inline bool has_script(uint32_t code_point, uint16_t script) const;

  inline std::unique_ptr<ICU4XCodePointSetData> get_script_extensions_set(uint16_t script) const;

  inline const capi::ICU4XScriptWithExtensionsBorrowed* AsFFI() const;
  inline capi::ICU4XScriptWithExtensionsBorrowed* AsFFI();
  inline static const ICU4XScriptWithExtensionsBorrowed* FromFFI(const capi::ICU4XScriptWithExtensionsBorrowed* ptr);
  inline static ICU4XScriptWithExtensionsBorrowed* FromFFI(capi::ICU4XScriptWithExtensionsBorrowed* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XScriptWithExtensionsBorrowed() = delete;
  ICU4XScriptWithExtensionsBorrowed(const ICU4XScriptWithExtensionsBorrowed&) = delete;
  ICU4XScriptWithExtensionsBorrowed(ICU4XScriptWithExtensionsBorrowed&&) noexcept = delete;
  ICU4XScriptWithExtensionsBorrowed operator=(const ICU4XScriptWithExtensionsBorrowed&) = delete;
  ICU4XScriptWithExtensionsBorrowed operator=(ICU4XScriptWithExtensionsBorrowed&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XScriptWithExtensionsBorrowed_D_HPP
