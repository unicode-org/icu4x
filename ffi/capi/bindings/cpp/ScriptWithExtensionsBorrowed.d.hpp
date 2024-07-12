#ifndef ScriptWithExtensionsBorrowed_D_HPP
#define ScriptWithExtensionsBorrowed_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct CodePointSetData; }
class CodePointSetData;
namespace diplomat::capi { struct ScriptExtensionsSet; }
class ScriptExtensionsSet;


namespace diplomat {
namespace capi {
    struct ScriptWithExtensionsBorrowed;
} // namespace capi
} // namespace

class ScriptWithExtensionsBorrowed {
public:

  inline uint16_t get_script_val(uint32_t code_point) const;

  inline std::unique_ptr<ScriptExtensionsSet> get_script_extensions_val(uint32_t code_point) const;

  inline bool has_script(uint32_t code_point, uint16_t script) const;

  inline std::unique_ptr<CodePointSetData> get_script_extensions_set(uint16_t script) const;

  inline const diplomat::capi::ScriptWithExtensionsBorrowed* AsFFI() const;
  inline diplomat::capi::ScriptWithExtensionsBorrowed* AsFFI();
  inline static const ScriptWithExtensionsBorrowed* FromFFI(const diplomat::capi::ScriptWithExtensionsBorrowed* ptr);
  inline static ScriptWithExtensionsBorrowed* FromFFI(diplomat::capi::ScriptWithExtensionsBorrowed* ptr);
  inline static void operator delete(void* ptr);
private:
  ScriptWithExtensionsBorrowed() = delete;
  ScriptWithExtensionsBorrowed(const ScriptWithExtensionsBorrowed&) = delete;
  ScriptWithExtensionsBorrowed(ScriptWithExtensionsBorrowed&&) noexcept = delete;
  ScriptWithExtensionsBorrowed operator=(const ScriptWithExtensionsBorrowed&) = delete;
  ScriptWithExtensionsBorrowed operator=(ScriptWithExtensionsBorrowed&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ScriptWithExtensionsBorrowed_D_HPP
