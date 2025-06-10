#ifndef icu4x_ErasedMeasureUnitParser_D_HPP
#define icu4x_ErasedMeasureUnitParser_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"

namespace icu4x {
namespace capi { struct DataProvider; }
class DataProvider;
namespace capi { struct ErasedMeasureUnit; }
class ErasedMeasureUnit;
namespace capi { struct ErasedMeasureUnitParser; }
class ErasedMeasureUnitParser;
class DataError;
}


namespace icu4x {
namespace capi {
    struct ErasedMeasureUnitParser;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * An ICU4X Erased Measure Unit Parser object, capable of parsing the CLDR unit identifier (e.g. `meter-per-square-second`) and get the {@link ErasedMeasureUnit}.
 *
 * See the [Rust documentation for `ErasedMeasureUnitParser`](https://docs.rs/icu/2.0.0/icu/experimental/measure/parser/struct.ErasedMeasureUnitParser.html) for more information.
 */
class ErasedMeasureUnitParser {
public:

  /**
   * Construct a new {@link ErasedMeasureUnitParser} instance using compiled data.
   *
   * See the [Rust documentation for `new`](https://docs.rs/icu/2.0.0/icu/experimental/measure/parser/struct.ErasedMeasureUnitParser.html#method.new) for more information.
   */
  inline static std::unique_ptr<icu4x::ErasedMeasureUnitParser> create();

  /**
   * Construct a new {@link ErasedMeasureUnitParser} instance using a particular data source.
   *
   * See the [Rust documentation for `new`](https://docs.rs/icu/2.0.0/icu/experimental/measure/parser/struct.ErasedMeasureUnitParser.html#method.new) for more information.
   */
  inline static diplomat::result<std::unique_ptr<icu4x::ErasedMeasureUnitParser>, icu4x::DataError> create_with_provider(const icu4x::DataProvider& provider);

  /**
   * See the [Rust documentation for `parse`](https://docs.rs/icu/2.0.0/icu/experimental/measure/parser/struct.ErasedMeasureUnitParser.html#method.parse) for more information.
   */
  inline std::unique_ptr<icu4x::ErasedMeasureUnit> parse(std::string_view unit_id) const;

  inline const icu4x::capi::ErasedMeasureUnitParser* AsFFI() const;
  inline icu4x::capi::ErasedMeasureUnitParser* AsFFI();
  inline static const icu4x::ErasedMeasureUnitParser* FromFFI(const icu4x::capi::ErasedMeasureUnitParser* ptr);
  inline static icu4x::ErasedMeasureUnitParser* FromFFI(icu4x::capi::ErasedMeasureUnitParser* ptr);
  inline static void operator delete(void* ptr);
private:
  ErasedMeasureUnitParser() = delete;
  ErasedMeasureUnitParser(const icu4x::ErasedMeasureUnitParser&) = delete;
  ErasedMeasureUnitParser(icu4x::ErasedMeasureUnitParser&&) noexcept = delete;
  ErasedMeasureUnitParser operator=(const icu4x::ErasedMeasureUnitParser&) = delete;
  ErasedMeasureUnitParser operator=(icu4x::ErasedMeasureUnitParser&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_ErasedMeasureUnitParser_D_HPP
