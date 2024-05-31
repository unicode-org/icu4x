#ifndef ICU4XScriptExtensionsSet_D_HPP
#define ICU4XScriptExtensionsSet_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XScriptExtensionsSet.d.h"


class ICU4XScriptExtensionsSet {
public:

  inline bool contains(uint16_t script) const;

  inline size_t count() const;

  inline std::optional<uint16_t> script_at(size_t index) const;

  inline const capi::ICU4XScriptExtensionsSet* AsFFI() const;
  inline capi::ICU4XScriptExtensionsSet* AsFFI();
  inline static const ICU4XScriptExtensionsSet* FromFFI(const capi::ICU4XScriptExtensionsSet* ptr);
  inline static ICU4XScriptExtensionsSet* FromFFI(capi::ICU4XScriptExtensionsSet* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XScriptExtensionsSet() = delete;
  ICU4XScriptExtensionsSet(const ICU4XScriptExtensionsSet&) = delete;
  ICU4XScriptExtensionsSet(ICU4XScriptExtensionsSet&&) noexcept = delete;
  ICU4XScriptExtensionsSet operator=(const ICU4XScriptExtensionsSet&) = delete;
  ICU4XScriptExtensionsSet operator=(ICU4XScriptExtensionsSet&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XScriptExtensionsSet_D_HPP
