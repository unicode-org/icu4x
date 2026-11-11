#ifndef ICU4X_DurationFormatter_D_HPP
#define ICU4X_DurationFormatter_D_HPP

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
namespace capi { struct DataProvider; }
class DataProvider;
namespace capi { struct DurationFormatter; }
class DurationFormatter;
namespace capi { struct Locale; }
class Locale;
namespace capi { struct Parts; }
class Parts;
struct Duration;
struct DurationFormatterOptions;
class DurationFormatterError;
} // namespace icu4x



namespace icu4x {
namespace capi {
    struct DurationFormatter;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * See the [Rust documentation for `DurationFormatter`](https://docs.rs/icu/2.4.0/icu/experimental/duration/struct.DurationFormatter.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class DurationFormatter {
public:

  /**
   * Creates a new {@link DurationFormatter} from locale data and an options bag using compiled data.
   *
   * See the [Rust documentation for `try_new`](https://docs.rs/icu/2.4.0/icu/experimental/duration/struct.DurationFormatter.html#method.try_new) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::DurationFormatter>, icu4x::DurationFormatterError> create(const icu4x::Locale& locale, icu4x::DurationFormatterOptions options);

  /**
   * Creates a new {@link DurationFormatter} from locale data and an options bag using a particular data source.
   *
   * See the [Rust documentation for `try_new`](https://docs.rs/icu/2.4.0/icu/experimental/duration/struct.DurationFormatter.html#method.try_new) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::DurationFormatter>, icu4x::DurationFormatterError> create_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::DurationFormatterOptions options);

  /**
   * Formats a {@link Duration} to a string.
   *
   * See the [Rust documentation for `format`](https://docs.rs/icu/2.4.0/icu/experimental/duration/struct.DurationFormatter.html#method.format) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline std::string format(icu4x::Duration duration) const;
  template<typename W>
  inline void format_write(icu4x::Duration duration, W& writeable_output) const;

  /**
   * Formats a {@link Duration} to a UTF-8 string while recording part spans into `parts`.
   *
   * See the [Rust documentation for `format`](https://docs.rs/icu/2.4.0/icu/experimental/duration/struct.DurationFormatter.html#method.format) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline std::string format_to_parts(icu4x::Duration duration, icu4x::Parts& parts) const;
  template<typename W>
  inline void format_to_parts_write(icu4x::Duration duration, icu4x::Parts& parts, W& writeable_output) const;

  /**
   * Formats a {@link Duration} to a UTF-16 string while recording UTF-16 code-unit part spans into `parts`.
   *
   * See the [Rust documentation for `format`](https://docs.rs/icu/2.4.0/icu/experimental/duration/struct.DurationFormatter.html#method.format) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline std::u16string format_to_parts16(icu4x::Duration duration, icu4x::Parts& parts) const;
  template<typename W>
  inline void format_to_parts16_write(icu4x::Duration duration, icu4x::Parts& parts, W& writeable_output) const;

    inline const icu4x::capi::DurationFormatter* AsFFI() const;
    inline icu4x::capi::DurationFormatter* AsFFI();
    inline static const icu4x::DurationFormatter* FromFFI(const icu4x::capi::DurationFormatter* ptr);
    inline static icu4x::DurationFormatter* FromFFI(icu4x::capi::DurationFormatter* ptr);
    inline static void operator delete(void* ptr);
private:
    DurationFormatter() = delete;
    DurationFormatter(const icu4x::DurationFormatter&) = delete;
    DurationFormatter(icu4x::DurationFormatter&&) noexcept = delete;
    DurationFormatter operator=(const icu4x::DurationFormatter&) = delete;
    DurationFormatter operator=(icu4x::DurationFormatter&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ICU4X_DurationFormatter_D_HPP
