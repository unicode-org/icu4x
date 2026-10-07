#ifndef ICU4X_CompactDecimalFormatter_D_HPP
#define ICU4X_CompactDecimalFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"
namespace icu4x {
namespace capi { struct CompactDecimalFormatter; }
class CompactDecimalFormatter;
namespace capi { struct DataProvider; }
class DataProvider;
namespace capi { struct Decimal; }
class Decimal;
namespace capi { struct Locale; }
class Locale;
class DataError;
class DecimalGroupingStrategy;
} // namespace icu4x



namespace icu4x {
namespace capi {
    struct CompactDecimalFormatter;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * An ICU4X Compact Decimal Format object, capable of formatting a {@link Decimal} in compact notation
 * and querying locale-specific compact exponents.
 *
 * See the [Rust documentation for `CompactDecimalFormatter`](https://docs.rs/icu/2.3.1/icu/decimal/struct.CompactDecimalFormatter.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class CompactDecimalFormatter {
public:

  /**
   * Creates a new short {@link CompactDecimalFormatter}, using compiled data.
   *
   * See the [Rust documentation for `try_new_short`](https://docs.rs/icu/2.3.1/icu/decimal/struct.CompactDecimalFormatter.html#method.try_new_short) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::CompactDecimalFormatter>, icu4x::DataError> create_short(const icu4x::Locale& locale, std::optional<icu4x::DecimalGroupingStrategy> grouping_strategy);

  /**
   * Creates a new short {@link CompactDecimalFormatter}, using a particular data source.
   *
   * See the [Rust documentation for `try_new_short`](https://docs.rs/icu/2.3.1/icu/decimal/struct.CompactDecimalFormatter.html#method.try_new_short) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::CompactDecimalFormatter>, icu4x::DataError> create_short_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DecimalGroupingStrategy> grouping_strategy);

  /**
   * Creates a new long {@link CompactDecimalFormatter}, using compiled data.
   *
   * See the [Rust documentation for `try_new_long`](https://docs.rs/icu/2.3.1/icu/decimal/struct.CompactDecimalFormatter.html#method.try_new_long) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::CompactDecimalFormatter>, icu4x::DataError> create_long(const icu4x::Locale& locale, std::optional<icu4x::DecimalGroupingStrategy> grouping_strategy);

  /**
   * Creates a new long {@link CompactDecimalFormatter}, using a particular data source.
   *
   * See the [Rust documentation for `try_new_long`](https://docs.rs/icu/2.3.1/icu/decimal/struct.CompactDecimalFormatter.html#method.try_new_long) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::CompactDecimalFormatter>, icu4x::DataError> create_long_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, std::optional<icu4x::DecimalGroupingStrategy> grouping_strategy);

  /**
   * Returns the compact decimal exponent that should be used for a number of
   * the given magnitude when using this formatter.
   *
   * See the [Rust documentation for `compact_exponent_for_magnitude`](https://docs.rs/icu/2.3.1/icu/decimal/struct.CompactDecimalFormatter.html#method.compact_exponent_for_magnitude) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline uint8_t compact_exponent_for_magnitude(int16_t magnitude) const;

  /**
   * Formats a {@link Decimal} in compact notation to a string.
   *
   * See the [Rust documentation for `format`](https://docs.rs/icu/2.3.1/icu/decimal/struct.CompactDecimalFormatter.html#method.format) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline std::string format(const icu4x::Decimal& value) const;
  template<typename W>
  inline void format_write(const icu4x::Decimal& value, W& writeable_output) const;

    inline const icu4x::capi::CompactDecimalFormatter* AsFFI() const;
    inline icu4x::capi::CompactDecimalFormatter* AsFFI();
    inline static const icu4x::CompactDecimalFormatter* FromFFI(const icu4x::capi::CompactDecimalFormatter* ptr);
    inline static icu4x::CompactDecimalFormatter* FromFFI(icu4x::capi::CompactDecimalFormatter* ptr);
    inline static void operator delete(void* ptr);
private:
    CompactDecimalFormatter() = delete;
    CompactDecimalFormatter(const icu4x::CompactDecimalFormatter&) = delete;
    CompactDecimalFormatter(icu4x::CompactDecimalFormatter&&) noexcept = delete;
    CompactDecimalFormatter operator=(const icu4x::CompactDecimalFormatter&) = delete;
    CompactDecimalFormatter operator=(icu4x::CompactDecimalFormatter&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ICU4X_CompactDecimalFormatter_D_HPP
