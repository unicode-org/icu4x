#ifndef ICU4X_RelativeTimeFormatter_D_HPP
#define ICU4X_RelativeTimeFormatter_D_HPP

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
namespace capi { struct Decimal; }
class Decimal;
namespace capi { struct Locale; }
class Locale;
namespace capi { struct RelativeTimeFormatter; }
class RelativeTimeFormatter;
struct RelativeTimeFormatterOptionsV1;
class DataError;
} // namespace icu4x



namespace icu4x {
namespace capi {
    struct RelativeTimeFormatter;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * An ICU4X `RelativeTimeFormatter` object capable of formatting a {@link Decimal} into a
 * locale-sensitive relative time string.
 *
 * See the [Rust documentation for `RelativeTimeFormatter`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class RelativeTimeFormatter {
public:

  /**
   * Construct a new {@link RelativeTimeFormatter} for long seconds from compiled data.
   *
   * See the [Rust documentation for `try_new_long_second`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_second) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_long_second(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for long seconds using a particular data source.
   *
   * See the [Rust documentation for `try_new_long_second`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_second) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_long_second_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for long minutes from compiled data.
   *
   * See the [Rust documentation for `try_new_long_minute`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_minute) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_long_minute(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for long minutes using a particular data source.
   *
   * See the [Rust documentation for `try_new_long_minute`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_minute) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_long_minute_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for long hours from compiled data.
   *
   * See the [Rust documentation for `try_new_long_hour`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_hour) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_long_hour(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for long hours using a particular data source.
   *
   * See the [Rust documentation for `try_new_long_hour`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_hour) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_long_hour_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for long days from compiled data.
   *
   * See the [Rust documentation for `try_new_long_day`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_day) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_long_day(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for long days using a particular data source.
   *
   * See the [Rust documentation for `try_new_long_day`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_day) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_long_day_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for long weeks from compiled data.
   *
   * See the [Rust documentation for `try_new_long_week`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_week) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_long_week(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for long weeks using a particular data source.
   *
   * See the [Rust documentation for `try_new_long_week`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_week) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_long_week_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for long months from compiled data.
   *
   * See the [Rust documentation for `try_new_long_month`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_month) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_long_month(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for long months using a particular data source.
   *
   * See the [Rust documentation for `try_new_long_month`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_month) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_long_month_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for long quarters from compiled data.
   *
   * See the [Rust documentation for `try_new_long_quarter`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_quarter) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_long_quarter(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for long quarters using a particular data source.
   *
   * See the [Rust documentation for `try_new_long_quarter`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_quarter) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_long_quarter_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for long years from compiled data.
   *
   * See the [Rust documentation for `try_new_long_year`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_year) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_long_year(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for long years using a particular data source.
   *
   * See the [Rust documentation for `try_new_long_year`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_long_year) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_long_year_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for short seconds from compiled data.
   *
   * See the [Rust documentation for `try_new_short_second`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_second) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_short_second(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for short seconds using a particular data source.
   *
   * See the [Rust documentation for `try_new_short_second`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_second) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_short_second_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for short minutes from compiled data.
   *
   * See the [Rust documentation for `try_new_short_minute`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_minute) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_short_minute(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for short minutes using a particular data source.
   *
   * See the [Rust documentation for `try_new_short_minute`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_minute) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_short_minute_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for short hours from compiled data.
   *
   * See the [Rust documentation for `try_new_short_hour`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_hour) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_short_hour(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for short hours using a particular data source.
   *
   * See the [Rust documentation for `try_new_short_hour`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_hour) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_short_hour_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for short days from compiled data.
   *
   * See the [Rust documentation for `try_new_short_day`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_day) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_short_day(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for short days using a particular data source.
   *
   * See the [Rust documentation for `try_new_short_day`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_day) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_short_day_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for short weeks from compiled data.
   *
   * See the [Rust documentation for `try_new_short_week`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_week) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_short_week(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for short weeks using a particular data source.
   *
   * See the [Rust documentation for `try_new_short_week`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_week) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_short_week_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for short months from compiled data.
   *
   * See the [Rust documentation for `try_new_short_month`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_month) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_short_month(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for short months using a particular data source.
   *
   * See the [Rust documentation for `try_new_short_month`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_month) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_short_month_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for short quarters from compiled data.
   *
   * See the [Rust documentation for `try_new_short_quarter`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_quarter) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_short_quarter(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for short quarters using a particular data source.
   *
   * See the [Rust documentation for `try_new_short_quarter`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_quarter) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_short_quarter_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for short years from compiled data.
   *
   * See the [Rust documentation for `try_new_short_year`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_year) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_short_year(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for short years using a particular data source.
   *
   * See the [Rust documentation for `try_new_short_year`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_short_year) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_short_year_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for narrow seconds from compiled data.
   *
   * See the [Rust documentation for `try_new_narrow_second`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_second) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_narrow_second(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for narrow seconds using a particular data source.
   *
   * See the [Rust documentation for `try_new_narrow_second`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_second) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_narrow_second_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for narrow minutes from compiled data.
   *
   * See the [Rust documentation for `try_new_narrow_minute`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_minute) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_narrow_minute(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for narrow minutes using a particular data source.
   *
   * See the [Rust documentation for `try_new_narrow_minute`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_minute) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_narrow_minute_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for narrow hours from compiled data.
   *
   * See the [Rust documentation for `try_new_narrow_hour`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_hour) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_narrow_hour(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for narrow hours using a particular data source.
   *
   * See the [Rust documentation for `try_new_narrow_hour`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_hour) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_narrow_hour_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for narrow days from compiled data.
   *
   * See the [Rust documentation for `try_new_narrow_day`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_day) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_narrow_day(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for narrow days using a particular data source.
   *
   * See the [Rust documentation for `try_new_narrow_day`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_day) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_narrow_day_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for narrow weeks from compiled data.
   *
   * See the [Rust documentation for `try_new_narrow_week`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_week) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_narrow_week(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for narrow weeks using a particular data source.
   *
   * See the [Rust documentation for `try_new_narrow_week`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_week) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_narrow_week_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for narrow months from compiled data.
   *
   * See the [Rust documentation for `try_new_narrow_month`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_month) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_narrow_month(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for narrow months using a particular data source.
   *
   * See the [Rust documentation for `try_new_narrow_month`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_month) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_narrow_month_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for narrow quarters from compiled data.
   *
   * See the [Rust documentation for `try_new_narrow_quarter`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_quarter) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_narrow_quarter(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for narrow quarters using a particular data source.
   *
   * See the [Rust documentation for `try_new_narrow_quarter`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_quarter) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_narrow_quarter_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for narrow years from compiled data.
   *
   * See the [Rust documentation for `try_new_narrow_year`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_year) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_narrow_year(const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Construct a new {@link RelativeTimeFormatter} for narrow years using a particular data source.
   *
   * See the [Rust documentation for `try_new_narrow_year`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.try_new_narrow_year) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline static icu4x::diplomat::result<std::unique_ptr<icu4x::RelativeTimeFormatter>, icu4x::DataError> create_narrow_year_with_provider(const icu4x::DataProvider& provider, const icu4x::Locale& locale, icu4x::RelativeTimeFormatterOptionsV1 options);

  /**
   * Formats a {@link Decimal} to a string.
   *
   * See the [Rust documentation for `format`](https://docs.rs/icu/2.4.0/icu/experimental/relativetime/struct.RelativeTimeFormatter.html#method.format) for more information.
   *
   * 🚧 This API is unstable and may experience breaking changes outside major releases.
   */
  inline std::string format(const icu4x::Decimal& value) const;
  template<typename W>
  inline void format_write(const icu4x::Decimal& value, W& writeable_output) const;

    inline const icu4x::capi::RelativeTimeFormatter* AsFFI() const;
    inline icu4x::capi::RelativeTimeFormatter* AsFFI();
    inline static const icu4x::RelativeTimeFormatter* FromFFI(const icu4x::capi::RelativeTimeFormatter* ptr);
    inline static icu4x::RelativeTimeFormatter* FromFFI(icu4x::capi::RelativeTimeFormatter* ptr);
    inline static void operator delete(void* ptr);
private:
    RelativeTimeFormatter() = delete;
    RelativeTimeFormatter(const icu4x::RelativeTimeFormatter&) = delete;
    RelativeTimeFormatter(icu4x::RelativeTimeFormatter&&) noexcept = delete;
    RelativeTimeFormatter operator=(const icu4x::RelativeTimeFormatter&) = delete;
    RelativeTimeFormatter operator=(icu4x::RelativeTimeFormatter&&) noexcept = delete;
    static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // ICU4X_RelativeTimeFormatter_D_HPP
