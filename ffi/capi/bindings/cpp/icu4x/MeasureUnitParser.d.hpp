#ifndef icu4x_MeasureUnitParser_D_HPP
#define icu4x_MeasureUnitParser_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct MeasureUnit; }
class MeasureUnit;
}


namespace icu4x {
namespace capi {
    struct MeasureUnitParser;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * An ICU4X Measurement Unit parser object which is capable of parsing the CLDR unit identifier
 * (e.g. `meter-per-square-second`) and get the [`MeasureUnit`].
 *
 * See the [Rust documentation for `MeasureUnitParser`](https://docs.rs/icu/latest/icu/experimental/measure/parser/struct.MeasureUnitParser.html) for more information.
 */
class MeasureUnitParser {
public:

  /**
   * Parses the CLDR unit identifier (e.g. `meter-per-square-second`) and returns the corresponding [`MeasureUnit`],
   * if the identifier is valid.
   *
   * See the [Rust documentation for `parse`](https://docs.rs/icu/latest/icu/experimental/measure/parser/struct.MeasureUnitParser.html#method.parse) for more information.
   */
  inline std::unique_ptr<icu4x::MeasureUnit> parse(std::string_view unit_id) const;

  inline const icu4x::capi::MeasureUnitParser* AsFFI() const;
  inline icu4x::capi::MeasureUnitParser* AsFFI();
  inline static const icu4x::MeasureUnitParser* FromFFI(const icu4x::capi::MeasureUnitParser* ptr);
  inline static icu4x::MeasureUnitParser* FromFFI(icu4x::capi::MeasureUnitParser* ptr);
  inline static void operator delete(void* ptr);
private:
  MeasureUnitParser() = delete;
  MeasureUnitParser(const icu4x::MeasureUnitParser&) = delete;
  MeasureUnitParser(icu4x::MeasureUnitParser&&) noexcept = delete;
  MeasureUnitParser operator=(const icu4x::MeasureUnitParser&) = delete;
  MeasureUnitParser operator=(icu4x::MeasureUnitParser&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_MeasureUnitParser_D_HPP
