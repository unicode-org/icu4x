#ifndef ICU4XCodePointMapData8_HPP
#define ICU4XCodePointMapData8_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XCodePointMapData8.h"

class ICU4XDataProvider;
class ICU4XCodePointMapData8;
#include "ICU4XError.hpp"

/**
 * A destruction policy for using ICU4XCodePointMapData8 with std::unique_ptr.
 */
struct ICU4XCodePointMapData8Deleter {
  void operator()(capi::ICU4XCodePointMapData8* l) const noexcept {
    capi::ICU4XCodePointMapData8_destroy(l);
  }
};

/**
 * An ICU4X Unicode Map Property object, capable of querying whether a code point (key) to obtain the Unicode property value, for a specific Unicode property.
 * 
 * For properties whose values fit into 8 bits.
 * 
 * See the [Rust documentation for `properties`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/index.html) for more information.
 * 
 * See the [Rust documentation for `CodePointMapData`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapData.html) for more information.
 * 
 * See the [Rust documentation for `CodePointMapDataBorrowed`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html) for more information.
 */
class ICU4XCodePointMapData8 {
 public:

  /**
   * Gets a map for Unicode property General_Category from a [`ICU4XDataProvider`].
   * 
   * See the [Rust documentation for `load_general_category`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_general_category.html) for more information.
   */
  static diplomat::result<ICU4XCodePointMapData8, ICU4XError> try_get_general_category(const ICU4XDataProvider& provider);

  /**
   * Gets a map for Unicode property Bidi_Class from a [`ICU4XDataProvider`].
   * 
   * See the [Rust documentation for `load_bidi_class`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_bidi_class.html) for more information.
   */
  static diplomat::result<ICU4XCodePointMapData8, ICU4XError> try_get_bidi_class(const ICU4XDataProvider& provider);

  /**
   * Gets a map for Unicode property East_Asian_Width from a [`ICU4XDataProvider`].
   * 
   * See the [Rust documentation for `load_east_asian_width`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_east_asian_width.html) for more information.
   */
  static diplomat::result<ICU4XCodePointMapData8, ICU4XError> try_get_east_asian_width(const ICU4XDataProvider& provider);

  /**
   * Gets a map for Unicode property Line_Break from a [`ICU4XDataProvider`].
   * 
   * See the [Rust documentation for `load_line_break`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_line_break.html) for more information.
   */
  static diplomat::result<ICU4XCodePointMapData8, ICU4XError> try_get_line_break(const ICU4XDataProvider& provider);

  /**
   * Gets a map for Unicode property Grapheme_Cluster_Break from a [`ICU4XDataProvider`].
   * 
   * See the [Rust documentation for `load_grapheme_cluster_break`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_grapheme_cluster_break.html) for more information.
   */
  static diplomat::result<ICU4XCodePointMapData8, ICU4XError> try_grapheme_cluster_break(const ICU4XDataProvider& provider);

  /**
   * Gets a map for Unicode property Word_Break from a [`ICU4XDataProvider`].
   * 
   * See the [Rust documentation for `load_word_break`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_word_break.html) for more information.
   */
  static diplomat::result<ICU4XCodePointMapData8, ICU4XError> try_get_word_break(const ICU4XDataProvider& provider);

  /**
   * Gets a map for Unicode property Sentence_Break from a [`ICU4XDataProvider`].
   * 
   * See the [Rust documentation for `load_sentence_break`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/fn.load_sentence_break.html) for more information.
   */
  static diplomat::result<ICU4XCodePointMapData8, ICU4XError> try_get_sentence_break(const ICU4XDataProvider& provider);

  /**
   * Gets the value for a code point.
   * 
   * See the [Rust documentation for `get`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get) for more information.
   */
  uint8_t get(char32_t cp) const;
  inline const capi::ICU4XCodePointMapData8* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XCodePointMapData8* AsFFIMut() { return this->inner.get(); }
  inline ICU4XCodePointMapData8(capi::ICU4XCodePointMapData8* i) : inner(i) {}
  ICU4XCodePointMapData8() = default;
  ICU4XCodePointMapData8(ICU4XCodePointMapData8&&) noexcept = default;
  ICU4XCodePointMapData8& operator=(ICU4XCodePointMapData8&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XCodePointMapData8, ICU4XCodePointMapData8Deleter> inner;
};

#include "ICU4XDataProvider.hpp"

inline diplomat::result<ICU4XCodePointMapData8, ICU4XError> ICU4XCodePointMapData8::try_get_general_category(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointMapData8_try_get_general_category(provider.AsFFI());
  diplomat::result<ICU4XCodePointMapData8, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointMapData8>(std::move(ICU4XCodePointMapData8(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointMapData8, ICU4XError> ICU4XCodePointMapData8::try_get_bidi_class(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointMapData8_try_get_bidi_class(provider.AsFFI());
  diplomat::result<ICU4XCodePointMapData8, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointMapData8>(std::move(ICU4XCodePointMapData8(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointMapData8, ICU4XError> ICU4XCodePointMapData8::try_get_east_asian_width(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointMapData8_try_get_east_asian_width(provider.AsFFI());
  diplomat::result<ICU4XCodePointMapData8, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointMapData8>(std::move(ICU4XCodePointMapData8(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointMapData8, ICU4XError> ICU4XCodePointMapData8::try_get_line_break(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointMapData8_try_get_line_break(provider.AsFFI());
  diplomat::result<ICU4XCodePointMapData8, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointMapData8>(std::move(ICU4XCodePointMapData8(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointMapData8, ICU4XError> ICU4XCodePointMapData8::try_grapheme_cluster_break(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointMapData8_try_grapheme_cluster_break(provider.AsFFI());
  diplomat::result<ICU4XCodePointMapData8, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointMapData8>(std::move(ICU4XCodePointMapData8(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointMapData8, ICU4XError> ICU4XCodePointMapData8::try_get_word_break(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointMapData8_try_get_word_break(provider.AsFFI());
  diplomat::result<ICU4XCodePointMapData8, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointMapData8>(std::move(ICU4XCodePointMapData8(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointMapData8, ICU4XError> ICU4XCodePointMapData8::try_get_sentence_break(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointMapData8_try_get_sentence_break(provider.AsFFI());
  diplomat::result<ICU4XCodePointMapData8, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointMapData8>(std::move(ICU4XCodePointMapData8(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline uint8_t ICU4XCodePointMapData8::get(char32_t cp) const {
  return capi::ICU4XCodePointMapData8_get(this->inner.get(), cp);
}
#endif
