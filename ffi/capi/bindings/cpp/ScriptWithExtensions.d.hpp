#ifndef ScriptWithExtensions_D_HPP
#define ScriptWithExtensions_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.d.hpp"

class CodePointRangeIterator;
class DataProvider;
class ScriptWithExtensionsBorrowed;
class DataError;


namespace capi {
    typedef struct ScriptWithExtensions ScriptWithExtensions;
}

class ScriptWithExtensions {
public:

  inline static diplomat::result<std::unique_ptr<ScriptWithExtensions>, DataError> create(const DataProvider& provider);

  inline uint16_t get_script_val(uint32_t code_point) const;

  inline bool has_script(uint32_t code_point, uint16_t script) const;

  inline std::unique_ptr<ScriptWithExtensionsBorrowed> as_borrowed() const;

  inline std::unique_ptr<CodePointRangeIterator> iter_ranges_for_script(uint16_t script) const;

  inline const capi::ScriptWithExtensions* AsFFI() const;
  inline capi::ScriptWithExtensions* AsFFI();
  inline static const ScriptWithExtensions* FromFFI(const capi::ScriptWithExtensions* ptr);
  inline static ScriptWithExtensions* FromFFI(capi::ScriptWithExtensions* ptr);
  inline static void operator delete(void* ptr);
private:
  ScriptWithExtensions() = delete;
  ScriptWithExtensions(const ScriptWithExtensions&) = delete;
  ScriptWithExtensions(ScriptWithExtensions&&) noexcept = delete;
  ScriptWithExtensions operator=(const ScriptWithExtensions&) = delete;
  ScriptWithExtensions operator=(ScriptWithExtensions&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ScriptWithExtensions_D_HPP
