#ifndef CodePointMapData16_D_HPP
#define CodePointMapData16_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct CodePointRangeIterator; }
class CodePointRangeIterator;
namespace diplomat::capi { struct CodePointSetData; }
class CodePointSetData;
namespace diplomat::capi { struct DataProvider; }
class DataProvider;
class DataError;


namespace diplomat {
namespace capi {
    struct CodePointMapData16;
} // namespace capi
} // namespace

class CodePointMapData16 {
public:

  inline uint16_t get(char32_t cp) const;

  inline std::unique_ptr<CodePointRangeIterator> iter_ranges_for_value(uint16_t value) const;

  inline std::unique_ptr<CodePointRangeIterator> iter_ranges_for_value_complemented(uint16_t value) const;

  inline std::unique_ptr<CodePointSetData> get_set_for_value(uint16_t value) const;

  inline static diplomat::result<std::unique_ptr<CodePointMapData16>, DataError> load_script(const DataProvider& provider);

  inline const diplomat::capi::CodePointMapData16* AsFFI() const;
  inline diplomat::capi::CodePointMapData16* AsFFI();
  inline static const CodePointMapData16* FromFFI(const diplomat::capi::CodePointMapData16* ptr);
  inline static CodePointMapData16* FromFFI(diplomat::capi::CodePointMapData16* ptr);
  inline static void operator delete(void* ptr);
private:
  CodePointMapData16() = delete;
  CodePointMapData16(const CodePointMapData16&) = delete;
  CodePointMapData16(CodePointMapData16&&) noexcept = delete;
  CodePointMapData16 operator=(const CodePointMapData16&) = delete;
  CodePointMapData16 operator=(CodePointMapData16&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // CodePointMapData16_D_HPP
