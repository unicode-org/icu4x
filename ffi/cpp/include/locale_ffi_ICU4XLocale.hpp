#ifndef locale_ffi_ICU4XLocale_HPP
#define locale_ffi_ICU4XLocale_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "locale_ffi_ICU4XLocale.h"
}

class ICU4XLocale;
enum struct ICU4XLocaleError;

struct ICU4XLocaleDeleter {
  void operator()(capi::ICU4XLocale* l) const noexcept {
    capi::ICU4XLocale_destroy(l);
  }
};
class ICU4XLocale {
 public:
  static std::optional<ICU4XLocale> create(const std::string_view name);
  ICU4XLocale clone();
  diplomat::result<std::string, ICU4XLocaleError> basename();
  diplomat::result<std::string, ICU4XLocaleError> get_unicode_extension(const std::string_view bytes);
  diplomat::result<std::string, ICU4XLocaleError> language();
  diplomat::result<std::string, ICU4XLocaleError> region();
  diplomat::result<std::string, ICU4XLocaleError> script();
  diplomat::result<std::string, ICU4XLocaleError> tostring();
  inline const capi::ICU4XLocale* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XLocale* AsFFIMut() { return this->inner.get(); }
  ICU4XLocale(capi::ICU4XLocale* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XLocale, ICU4XLocaleDeleter> inner;
};

#include "locale_ffi_ICU4XLocaleError.hpp"

std::optional<ICU4XLocale> ICU4XLocale::create(const std::string_view name) {
  auto diplomat_optional_raw_out_value = capi::ICU4XLocale_create(name.data(), name.length());
  std::optional<ICU4XLocale> diplomat_optional_out_value;
  if (diplomat_optional_raw_out_value != nullptr) {
    diplomat_optional_out_value = ICU4XLocale(diplomat_optional_raw_out_value);
  } else {
    diplomat_optional_out_value = std::nullopt;
  }
  return diplomat_optional_out_value;
}
ICU4XLocale ICU4XLocale::clone() {
  return ICU4XLocale(capi::ICU4XLocale_clone(this->inner.get()));
}
diplomat::result<std::string, ICU4XLocaleError> ICU4XLocale::basename() {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_basename(this->inner.get(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XLocaleError> diplomat_result_out_value;
  diplomat_result_out_value.is_ok = diplomat_result_raw_out_value.is_ok;
  if (diplomat_result_raw_out_value.is_ok) {
  } else {
    diplomat_result_out_value.err = ICU4XLocaleError{ diplomat_result_raw_out_value.err };
  }
  diplomat::result<std::monostate, ICU4XLocaleError> out_value = diplomat_result_out_value;
  if (out_value.is_ok) {
    return diplomat::result<std::string, ICU4XLocaleError>::new_ok(diplomat_writeable_string);
  } else {
    return diplomat::result<std::string, ICU4XLocaleError>::new_err(out_value.err);
  }
}
diplomat::result<std::string, ICU4XLocaleError> ICU4XLocale::get_unicode_extension(const std::string_view bytes) {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_get_unicode_extension(this->inner.get(), bytes.data(), bytes.length(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XLocaleError> diplomat_result_out_value;
  diplomat_result_out_value.is_ok = diplomat_result_raw_out_value.is_ok;
  if (diplomat_result_raw_out_value.is_ok) {
  } else {
    diplomat_result_out_value.err = ICU4XLocaleError{ diplomat_result_raw_out_value.err };
  }
  diplomat::result<std::monostate, ICU4XLocaleError> out_value = diplomat_result_out_value;
  if (out_value.is_ok) {
    return diplomat::result<std::string, ICU4XLocaleError>::new_ok(diplomat_writeable_string);
  } else {
    return diplomat::result<std::string, ICU4XLocaleError>::new_err(out_value.err);
  }
}
diplomat::result<std::string, ICU4XLocaleError> ICU4XLocale::language() {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_language(this->inner.get(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XLocaleError> diplomat_result_out_value;
  diplomat_result_out_value.is_ok = diplomat_result_raw_out_value.is_ok;
  if (diplomat_result_raw_out_value.is_ok) {
  } else {
    diplomat_result_out_value.err = ICU4XLocaleError{ diplomat_result_raw_out_value.err };
  }
  diplomat::result<std::monostate, ICU4XLocaleError> out_value = diplomat_result_out_value;
  if (out_value.is_ok) {
    return diplomat::result<std::string, ICU4XLocaleError>::new_ok(diplomat_writeable_string);
  } else {
    return diplomat::result<std::string, ICU4XLocaleError>::new_err(out_value.err);
  }
}
diplomat::result<std::string, ICU4XLocaleError> ICU4XLocale::region() {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_region(this->inner.get(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XLocaleError> diplomat_result_out_value;
  diplomat_result_out_value.is_ok = diplomat_result_raw_out_value.is_ok;
  if (diplomat_result_raw_out_value.is_ok) {
  } else {
    diplomat_result_out_value.err = ICU4XLocaleError{ diplomat_result_raw_out_value.err };
  }
  diplomat::result<std::monostate, ICU4XLocaleError> out_value = diplomat_result_out_value;
  if (out_value.is_ok) {
    return diplomat::result<std::string, ICU4XLocaleError>::new_ok(diplomat_writeable_string);
  } else {
    return diplomat::result<std::string, ICU4XLocaleError>::new_err(out_value.err);
  }
}
diplomat::result<std::string, ICU4XLocaleError> ICU4XLocale::script() {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_script(this->inner.get(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XLocaleError> diplomat_result_out_value;
  diplomat_result_out_value.is_ok = diplomat_result_raw_out_value.is_ok;
  if (diplomat_result_raw_out_value.is_ok) {
  } else {
    diplomat_result_out_value.err = ICU4XLocaleError{ diplomat_result_raw_out_value.err };
  }
  diplomat::result<std::monostate, ICU4XLocaleError> out_value = diplomat_result_out_value;
  if (out_value.is_ok) {
    return diplomat::result<std::string, ICU4XLocaleError>::new_ok(diplomat_writeable_string);
  } else {
    return diplomat::result<std::string, ICU4XLocaleError>::new_err(out_value.err);
  }
}
diplomat::result<std::string, ICU4XLocaleError> ICU4XLocale::tostring() {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XLocale_tostring(this->inner.get(), &diplomat_writeable_out);
  diplomat::result<std::monostate, ICU4XLocaleError> diplomat_result_out_value;
  diplomat_result_out_value.is_ok = diplomat_result_raw_out_value.is_ok;
  if (diplomat_result_raw_out_value.is_ok) {
  } else {
    diplomat_result_out_value.err = ICU4XLocaleError{ diplomat_result_raw_out_value.err };
  }
  diplomat::result<std::monostate, ICU4XLocaleError> out_value = diplomat_result_out_value;
  if (out_value.is_ok) {
    return diplomat::result<std::string, ICU4XLocaleError>::new_ok(diplomat_writeable_string);
  } else {
    return diplomat::result<std::string, ICU4XLocaleError>::new_err(out_value.err);
  }
}
#endif
