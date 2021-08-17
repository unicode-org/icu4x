#ifndef ICU4XLocale_HPP
#define ICU4XLocale_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XLocale.h"
}

class ICU4XLocale;
#include "ICU4XLocaleError.hpp"

struct ICU4XLocaleDeleter {
  void operator()(capi::ICU4XLocale* l) const noexcept {
    capi::ICU4XLocale_destroy(l);
  }
};
class ICU4XLocale {
 public:
  static std::optional<ICU4XLocale> create(const std::string_view name);
  ICU4XLocale clone();
  template<typename W> diplomat::result<std::monostate, ICU4XLocaleError> basename_to_writeable(W& write);
  diplomat::result<std::string, ICU4XLocaleError> basename();
  template<typename W> diplomat::result<std::monostate, ICU4XLocaleError> get_unicode_extension_to_writeable(const std::string_view bytes, W& write);
  diplomat::result<std::string, ICU4XLocaleError> get_unicode_extension(const std::string_view bytes);
  template<typename W> diplomat::result<std::monostate, ICU4XLocaleError> language_to_writeable(W& write);
  diplomat::result<std::string, ICU4XLocaleError> language();
  template<typename W> diplomat::result<std::monostate, ICU4XLocaleError> region_to_writeable(W& write);
  diplomat::result<std::string, ICU4XLocaleError> region();
  template<typename W> diplomat::result<std::monostate, ICU4XLocaleError> script_to_writeable(W& write);
  diplomat::result<std::string, ICU4XLocaleError> script();
  template<typename W> diplomat::result<std::monostate, ICU4XLocaleError> tostring_to_writeable(W& write);
  diplomat::result<std::string, ICU4XLocaleError> tostring();
  inline const capi::ICU4XLocale* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XLocale* AsFFIMut() { return this->inner.get(); }
  inline ICU4XLocale(capi::ICU4XLocale* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XLocale, ICU4XLocaleDeleter> inner;
};


inline std::optional<ICU4XLocale> ICU4XLocale::create(const std::string_view name) {
  auto diplomat_optional_raw_out_value = capi::ICU4XLocale_create(name.data(), name.length());
  std::optional<ICU4XLocale> diplomat_optional_out_value;
  if (diplomat_optional_raw_out_value != nullptr) {
    diplomat_optional_out_value = ICU4XLocale(diplomat_optional_raw_out_value);
  } else {
    diplomat_optional_out_value = std::nullopt;
  }
  return diplomat_optional_out_value;
}
inline ICU4XLocale ICU4XLocale::clone() {
  return ICU4XLocale(capi::ICU4XLocale_clone(this->inner.get()));
}
template<typename W> inline diplomat::result<std::monostate, ICU4XLocaleError> ICU4XLocale::basename_to_writeable(W& write) {
  capi::DiplomatWriteable write_writer = diplomat::WriteableTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_basename(this->inner.get(), &write_writer);
  diplomat::result<std::monostate, ICU4XLocaleError> diplomat_result_out_value(diplomat_result_raw_out_value.is_ok);
  if (diplomat_result_raw_out_value.is_ok) {
  } else {
    diplomat_result_out_value.set_err((std::move(ICU4XLocaleError{ diplomat_result_raw_out_value.err })));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, ICU4XLocaleError> ICU4XLocale::basename() {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_basename(this->inner.get(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XLocaleError> diplomat_result_out_value(diplomat_result_raw_out_value.is_ok);
  if (diplomat_result_raw_out_value.is_ok) {
  } else {
    diplomat_result_out_value.set_err((std::move(ICU4XLocaleError{ diplomat_result_raw_out_value.err })));
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_writeable_string));
}
template<typename W> inline diplomat::result<std::monostate, ICU4XLocaleError> ICU4XLocale::get_unicode_extension_to_writeable(const std::string_view bytes, W& write) {
  capi::DiplomatWriteable write_writer = diplomat::WriteableTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_get_unicode_extension(this->inner.get(), bytes.data(), bytes.length(), &write_writer);
  diplomat::result<std::monostate, ICU4XLocaleError> diplomat_result_out_value(diplomat_result_raw_out_value.is_ok);
  if (diplomat_result_raw_out_value.is_ok) {
  } else {
    diplomat_result_out_value.set_err((std::move(ICU4XLocaleError{ diplomat_result_raw_out_value.err })));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, ICU4XLocaleError> ICU4XLocale::get_unicode_extension(const std::string_view bytes) {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_get_unicode_extension(this->inner.get(), bytes.data(), bytes.length(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XLocaleError> diplomat_result_out_value(diplomat_result_raw_out_value.is_ok);
  if (diplomat_result_raw_out_value.is_ok) {
  } else {
    diplomat_result_out_value.set_err((std::move(ICU4XLocaleError{ diplomat_result_raw_out_value.err })));
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_writeable_string));
}
template<typename W> inline diplomat::result<std::monostate, ICU4XLocaleError> ICU4XLocale::language_to_writeable(W& write) {
  capi::DiplomatWriteable write_writer = diplomat::WriteableTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_language(this->inner.get(), &write_writer);
  diplomat::result<std::monostate, ICU4XLocaleError> diplomat_result_out_value(diplomat_result_raw_out_value.is_ok);
  if (diplomat_result_raw_out_value.is_ok) {
  } else {
    diplomat_result_out_value.set_err((std::move(ICU4XLocaleError{ diplomat_result_raw_out_value.err })));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, ICU4XLocaleError> ICU4XLocale::language() {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_language(this->inner.get(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XLocaleError> diplomat_result_out_value(diplomat_result_raw_out_value.is_ok);
  if (diplomat_result_raw_out_value.is_ok) {
  } else {
    diplomat_result_out_value.set_err((std::move(ICU4XLocaleError{ diplomat_result_raw_out_value.err })));
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_writeable_string));
}
template<typename W> inline diplomat::result<std::monostate, ICU4XLocaleError> ICU4XLocale::region_to_writeable(W& write) {
  capi::DiplomatWriteable write_writer = diplomat::WriteableTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_region(this->inner.get(), &write_writer);
  diplomat::result<std::monostate, ICU4XLocaleError> diplomat_result_out_value(diplomat_result_raw_out_value.is_ok);
  if (diplomat_result_raw_out_value.is_ok) {
  } else {
    diplomat_result_out_value.set_err((std::move(ICU4XLocaleError{ diplomat_result_raw_out_value.err })));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, ICU4XLocaleError> ICU4XLocale::region() {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_region(this->inner.get(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XLocaleError> diplomat_result_out_value(diplomat_result_raw_out_value.is_ok);
  if (diplomat_result_raw_out_value.is_ok) {
  } else {
    diplomat_result_out_value.set_err((std::move(ICU4XLocaleError{ diplomat_result_raw_out_value.err })));
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_writeable_string));
}
template<typename W> inline diplomat::result<std::monostate, ICU4XLocaleError> ICU4XLocale::script_to_writeable(W& write) {
  capi::DiplomatWriteable write_writer = diplomat::WriteableTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_script(this->inner.get(), &write_writer);
  diplomat::result<std::monostate, ICU4XLocaleError> diplomat_result_out_value(diplomat_result_raw_out_value.is_ok);
  if (diplomat_result_raw_out_value.is_ok) {
  } else {
    diplomat_result_out_value.set_err((std::move(ICU4XLocaleError{ diplomat_result_raw_out_value.err })));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, ICU4XLocaleError> ICU4XLocale::script() {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_script(this->inner.get(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XLocaleError> diplomat_result_out_value(diplomat_result_raw_out_value.is_ok);
  if (diplomat_result_raw_out_value.is_ok) {
  } else {
    diplomat_result_out_value.set_err((std::move(ICU4XLocaleError{ diplomat_result_raw_out_value.err })));
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_writeable_string));
}
template<typename W> inline diplomat::result<std::monostate, ICU4XLocaleError> ICU4XLocale::tostring_to_writeable(W& write) {
  capi::DiplomatWriteable write_writer = diplomat::WriteableTrait<W>::Construct(write);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_tostring(this->inner.get(), &write_writer);
  diplomat::result<std::monostate, ICU4XLocaleError> diplomat_result_out_value(diplomat_result_raw_out_value.is_ok);
  if (diplomat_result_raw_out_value.is_ok) {
  } else {
    diplomat_result_out_value.set_err((std::move(ICU4XLocaleError{ diplomat_result_raw_out_value.err })));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, ICU4XLocaleError> ICU4XLocale::tostring() {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_tostring(this->inner.get(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XLocaleError> diplomat_result_out_value(diplomat_result_raw_out_value.is_ok);
  if (diplomat_result_raw_out_value.is_ok) {
  } else {
    diplomat_result_out_value.set_err((std::move(ICU4XLocaleError{ diplomat_result_raw_out_value.err })));
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_writeable_string));
}
#endif
