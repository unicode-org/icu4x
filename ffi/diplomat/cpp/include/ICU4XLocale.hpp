#ifndef ICU4XLocale_HPP
#define ICU4XLocale_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XLocale.h"

class ICU4XLocale;
#include "ICU4XError.hpp"

/**
 * A destruction policy for using ICU4XLocale with std::unique_ptr.
 */
struct ICU4XLocaleDeleter {
  void operator()(capi::ICU4XLocale* l) const noexcept {
    capi::ICU4XLocale_destroy(l);
  }
};

/**
 * An ICU4X Locale, capable of representing strings like `"en-US"`.
 * 
 * See the [Rust documentation for `Locale`](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html) for more information.
 */
class ICU4XLocale {
 public:

  /**
   * Construct an [`ICU4XLocale`] from an locale identifier.
   * 
   * See the [Rust documentation for `from_bytes`](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.from_bytes) for more information.
   */
  static diplomat::result<ICU4XLocale, ICU4XError> create(const std::string_view name);

  /**
   * Construct an [`ICU4XLocale`] for the English language.
   */
  static ICU4XLocale create_en();

  /**
   * Construct an [`ICU4XLocale`] for the Bangla language.
   */
  static ICU4XLocale create_bn();

  /**
   * Construct a default undefined [`ICU4XLocale`] "und".
   */
  static ICU4XLocale und();

  /**
   * Clones the [`ICU4XLocale`].
   * 
   * See the [Rust documentation for `Locale`](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html) for more information.
   */
  ICU4XLocale clone() const;

  /**
   * Write a string representation of the `LanguageIdentifier` part of
   * [`ICU4XLocale`] to `write`.
   * 
   * See the [Rust documentation for `id`](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id) for more information.
   */
  template<typename W> diplomat::result<std::monostate, ICU4XError> basename_to_writeable(W& write) const;

  /**
   * Write a string representation of the `LanguageIdentifier` part of
   * [`ICU4XLocale`] to `write`.
   * 
   * See the [Rust documentation for `id`](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id) for more information.
   */
  diplomat::result<std::string, ICU4XError> basename() const;

  /**
   * Write a string representation of the unicode extension to `write`
   * 
   * See the [Rust documentation for `extensions`](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.extensions) for more information.
   */
  template<typename W> diplomat::result<std::monostate, ICU4XError> get_unicode_extension_to_writeable(const std::string_view bytes, W& write) const;

  /**
   * Write a string representation of the unicode extension to `write`
   * 
   * See the [Rust documentation for `extensions`](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.extensions) for more information.
   */
  diplomat::result<std::string, ICU4XError> get_unicode_extension(const std::string_view bytes) const;

  /**
   * Write a string representation of [`ICU4XLocale`] language to `write`
   * 
   * See the [Rust documentation for `id`](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id) for more information.
   */
  template<typename W> diplomat::result<std::monostate, ICU4XError> language_to_writeable(W& write) const;

  /**
   * Write a string representation of [`ICU4XLocale`] language to `write`
   * 
   * See the [Rust documentation for `id`](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id) for more information.
   */
  diplomat::result<std::string, ICU4XError> language() const;

  /**
   * Set the language part of the [`ICU4XLocale`].
   * 
   * See the [Rust documentation for `from_bytes`](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.from_bytes) for more information.
   */
  diplomat::result<std::monostate, ICU4XError> set_language(const std::string_view bytes);

  /**
   * Write a string representation of [`ICU4XLocale`] region to `write`
   * 
   * See the [Rust documentation for `id`](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id) for more information.
   */
  template<typename W> diplomat::result<std::monostate, ICU4XError> region_to_writeable(W& write) const;

  /**
   * Write a string representation of [`ICU4XLocale`] region to `write`
   * 
   * See the [Rust documentation for `id`](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id) for more information.
   */
  diplomat::result<std::string, ICU4XError> region() const;

  /**
   * Set the region part of the [`ICU4XLocale`].
   * 
   * See the [Rust documentation for `from_bytes`](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.from_bytes) for more information.
   */
  diplomat::result<std::monostate, ICU4XError> set_region(const std::string_view bytes);

  /**
   * Write a string representation of [`ICU4XLocale`] script to `write`
   * 
   * See the [Rust documentation for `id`](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id) for more information.
   */
  template<typename W> diplomat::result<std::monostate, ICU4XError> script_to_writeable(W& write) const;

  /**
   * Write a string representation of [`ICU4XLocale`] script to `write`
   * 
   * See the [Rust documentation for `id`](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id) for more information.
   */
  diplomat::result<std::string, ICU4XError> script() const;

  /**
   * Set the script part of the [`ICU4XLocale`]. Pass an empty string to remove the script.
   * 
   * See the [Rust documentation for `from_bytes`](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.from_bytes) for more information.
   */
  diplomat::result<std::monostate, ICU4XError> set_script(const std::string_view bytes);

  /**
   * Write a string representation of [`ICU4XLocale`] to `write`
   * 
   * See the [Rust documentation for `write_to`](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.write_to) for more information.
   */
  template<typename W> diplomat::result<std::monostate, ICU4XError> to_string_to_writeable(W& write) const;

  /**
   * Write a string representation of [`ICU4XLocale`] to `write`
   * 
   * See the [Rust documentation for `write_to`](https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.write_to) for more information.
   */
  diplomat::result<std::string, ICU4XError> to_string() const;
  inline const capi::ICU4XLocale* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XLocale* AsFFIMut() { return this->inner.get(); }
  inline ICU4XLocale(capi::ICU4XLocale* i) : inner(i) {}
  ICU4XLocale() = default;
  ICU4XLocale(ICU4XLocale&&) noexcept = default;
  ICU4XLocale& operator=(ICU4XLocale&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XLocale, ICU4XLocaleDeleter> inner;
};


inline diplomat::result<ICU4XLocale, ICU4XError> ICU4XLocale::create(const std::string_view name) {
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_create(name.data(), name.size());
  diplomat::result<ICU4XLocale, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XLocale>(std::move(ICU4XLocale(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline ICU4XLocale ICU4XLocale::create_en() {
  return ICU4XLocale(capi::ICU4XLocale_create_en());
}
inline ICU4XLocale ICU4XLocale::create_bn() {
  return ICU4XLocale(capi::ICU4XLocale_create_bn());
}
inline ICU4XLocale ICU4XLocale::und() {
  return ICU4XLocale(capi::ICU4XLocale_und());
}
inline ICU4XLocale ICU4XLocale::clone() const {
  return ICU4XLocale(capi::ICU4XLocale_clone(this->inner.get()));
}
template<typename W> inline diplomat::result<std::monostate, ICU4XError> ICU4XLocale::basename_to_writeable(W& write) const {
  capi::DiplomatWriteable write_writer = diplomat::WriteableTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_basename(this->inner.get(), &write_writer);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, ICU4XError> ICU4XLocale::basename() const {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_basename(this->inner.get(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_writeable_string));
}
template<typename W> inline diplomat::result<std::monostate, ICU4XError> ICU4XLocale::get_unicode_extension_to_writeable(const std::string_view bytes, W& write) const {
  capi::DiplomatWriteable write_writer = diplomat::WriteableTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_get_unicode_extension(this->inner.get(), bytes.data(), bytes.size(), &write_writer);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, ICU4XError> ICU4XLocale::get_unicode_extension(const std::string_view bytes) const {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_get_unicode_extension(this->inner.get(), bytes.data(), bytes.size(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_writeable_string));
}
template<typename W> inline diplomat::result<std::monostate, ICU4XError> ICU4XLocale::language_to_writeable(W& write) const {
  capi::DiplomatWriteable write_writer = diplomat::WriteableTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_language(this->inner.get(), &write_writer);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, ICU4XError> ICU4XLocale::language() const {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_language(this->inner.get(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_writeable_string));
}
inline diplomat::result<std::monostate, ICU4XError> ICU4XLocale::set_language(const std::string_view bytes) {
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_set_language(this->inner.get(), bytes.data(), bytes.size());
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
template<typename W> inline diplomat::result<std::monostate, ICU4XError> ICU4XLocale::region_to_writeable(W& write) const {
  capi::DiplomatWriteable write_writer = diplomat::WriteableTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_region(this->inner.get(), &write_writer);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, ICU4XError> ICU4XLocale::region() const {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_region(this->inner.get(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_writeable_string));
}
inline diplomat::result<std::monostate, ICU4XError> ICU4XLocale::set_region(const std::string_view bytes) {
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_set_region(this->inner.get(), bytes.data(), bytes.size());
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
template<typename W> inline diplomat::result<std::monostate, ICU4XError> ICU4XLocale::script_to_writeable(W& write) const {
  capi::DiplomatWriteable write_writer = diplomat::WriteableTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_script(this->inner.get(), &write_writer);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, ICU4XError> ICU4XLocale::script() const {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_script(this->inner.get(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_writeable_string));
}
inline diplomat::result<std::monostate, ICU4XError> ICU4XLocale::set_script(const std::string_view bytes) {
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_set_script(this->inner.get(), bytes.data(), bytes.size());
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
template<typename W> inline diplomat::result<std::monostate, ICU4XError> ICU4XLocale::to_string_to_writeable(W& write) const {
  capi::DiplomatWriteable write_writer = diplomat::WriteableTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_to_string(this->inner.get(), &write_writer);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, ICU4XError> ICU4XLocale::to_string() const {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_to_string(this->inner.get(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_writeable_string));
}
#endif
