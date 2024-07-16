#ifndef ScriptExtensionsSet_D_HPP
#define ScriptExtensionsSet_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    struct ScriptExtensionsSet;
} // namespace capi
} // namespace

class ScriptExtensionsSet {
public:

  inline bool contains(uint16_t script) const;

  inline size_t count() const;

  inline std::optional<uint16_t> script_at(size_t index) const;

  inline const diplomat::capi::ScriptExtensionsSet* AsFFI() const;
  inline diplomat::capi::ScriptExtensionsSet* AsFFI();
  inline static const ScriptExtensionsSet* FromFFI(const diplomat::capi::ScriptExtensionsSet* ptr);
  inline static ScriptExtensionsSet* FromFFI(diplomat::capi::ScriptExtensionsSet* ptr);
  inline static void operator delete(void* ptr);
private:
  ScriptExtensionsSet() = delete;
  ScriptExtensionsSet(const ScriptExtensionsSet&) = delete;
  ScriptExtensionsSet(ScriptExtensionsSet&&) noexcept = delete;
  ScriptExtensionsSet operator=(const ScriptExtensionsSet&) = delete;
  ScriptExtensionsSet operator=(ScriptExtensionsSet&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ScriptExtensionsSet_D_HPP
