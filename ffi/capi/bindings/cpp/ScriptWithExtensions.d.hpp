#ifndef ScriptWithExtensions_D_HPP
#define ScriptWithExtensions_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct CodePointRangeIterator; }
class CodePointRangeIterator;
namespace diplomat::capi { struct DataProvider; }
class DataProvider;
namespace diplomat::capi { struct ScriptWithExtensionsBorrowed; }
class ScriptWithExtensionsBorrowed;
class DataError;


namespace diplomat {
namespace capi {
    struct ScriptWithExtensions;
} // namespace capi
} // namespace

class ScriptWithExtensions {
public:

  inline static diplomat::result<std::unique_ptr<ScriptWithExtensions>, DataError> create(const DataProvider& provider);

  inline uint16_t get_script_val(uint32_t code_point) const;

  inline bool has_script(uint32_t code_point, uint16_t script) const;

  inline std::unique_ptr<ScriptWithExtensionsBorrowed> as_borrowed() const;

  inline std::unique_ptr<CodePointRangeIterator> iter_ranges_for_script(uint16_t script) const;

  inline const diplomat::capi::ScriptWithExtensions* AsFFI() const;
  inline diplomat::capi::ScriptWithExtensions* AsFFI();
  inline static const ScriptWithExtensions* FromFFI(const diplomat::capi::ScriptWithExtensions* ptr);
  inline static ScriptWithExtensions* FromFFI(diplomat::capi::ScriptWithExtensions* ptr);
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
