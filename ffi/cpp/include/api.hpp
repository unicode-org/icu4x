#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


enum struct ICU4XCanonicalizationResult : ssize_t {
  Modified = 0,
  Unmodified = 1,
};

struct ICU4XCreateDataProviderResult;

struct ICU4XCreateFixedDecimalResult;

struct ICU4XCreatePluralOperandsResult;

struct ICU4XCreatePluralRulesResult;

class ICU4XDataProvider;

class ICU4XFixedDecimal;

class ICU4XFixedDecimalFormat;

struct ICU4XFixedDecimalFormatOptions;

struct ICU4XFixedDecimalFormatResult;

enum struct ICU4XFixedDecimalGroupingStrategy : ssize_t {
  Auto = 0,
  Never = 1,
  Always = 2,
  Min2 = 3,
};

enum struct ICU4XFixedDecimalSignDisplay : ssize_t {
  Auto = 0,
  Never = 1,
  Always = 2,
  ExceptZero = 3,
  Negative = 4,
};

class ICU4XLocale;

class ICU4XLocaleCanonicalizer;

enum struct ICU4XLocaleResult : ssize_t {
  Ok = 0,
  Undefined = 1,
  Error = 2,
};

struct ICU4XPluralCategories;

enum struct ICU4XPluralCategory : ssize_t {
  Zero = 0,
  One = 1,
  Two = 2,
  Few = 3,
  Many = 4,
  Other = 5,
};

struct ICU4XPluralOperands;

enum struct ICU4XPluralRuleType : ssize_t {
  Cardinal = 0,
  Ordinal = 1,
};

class ICU4XPluralRules;

struct ICU4XDataProviderDeleter {
  void operator()(capi::ICU4XDataProvider* l) const noexcept {
    capi::ICU4XDataProvider_destroy(l);
  }
};
class ICU4XDataProvider {
 public:
  static ICU4XCreateDataProviderResult create_fs(const std::string_view path);
  static ICU4XCreateDataProviderResult create_static();
  inline const capi::ICU4XDataProvider* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XDataProvider* AsFFIMut() { return this->inner.get(); }
  ICU4XDataProvider(capi::ICU4XDataProvider* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XDataProvider, ICU4XDataProviderDeleter> inner;
};

struct ICU4XFixedDecimalDeleter {
  void operator()(capi::ICU4XFixedDecimal* l) const noexcept {
    capi::ICU4XFixedDecimal_destroy(l);
  }
};
class ICU4XFixedDecimal {
 public:
  static ICU4XFixedDecimal create(int32_t v);
  static ICU4XCreateFixedDecimalResult create_fromstr(const std::string_view v);
  bool multiply_pow10(int16_t power);
  void negate();
  std::string to_string();
  inline const capi::ICU4XFixedDecimal* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XFixedDecimal* AsFFIMut() { return this->inner.get(); }
  ICU4XFixedDecimal(capi::ICU4XFixedDecimal* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XFixedDecimal, ICU4XFixedDecimalDeleter> inner;
};

struct ICU4XFixedDecimalFormatDeleter {
  void operator()(capi::ICU4XFixedDecimalFormat* l) const noexcept {
    capi::ICU4XFixedDecimalFormat_destroy(l);
  }
};
class ICU4XFixedDecimalFormat {
 public:
  static ICU4XFixedDecimalFormatResult try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XFixedDecimalFormatOptions options);
  std::string format_write(const ICU4XFixedDecimal& value);
  inline const capi::ICU4XFixedDecimalFormat* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XFixedDecimalFormat* AsFFIMut() { return this->inner.get(); }
  ICU4XFixedDecimalFormat(capi::ICU4XFixedDecimalFormat* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XFixedDecimalFormat, ICU4XFixedDecimalFormatDeleter> inner;
};

struct ICU4XLocaleDeleter {
  void operator()(capi::ICU4XLocale* l) const noexcept {
    capi::ICU4XLocale_destroy(l);
  }
};
class ICU4XLocale {
 public:
  static std::optional<ICU4XLocale> create(const std::string_view name);
  ICU4XLocale clone();
  std::string basename();
  std::string get_unicode_extension(const std::string_view bytes);
  std::string language();
  std::string region();
  std::string script();
  std::string tostring();
  inline const capi::ICU4XLocale* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XLocale* AsFFIMut() { return this->inner.get(); }
  ICU4XLocale(capi::ICU4XLocale* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XLocale, ICU4XLocaleDeleter> inner;
};

struct ICU4XLocaleCanonicalizerDeleter {
  void operator()(capi::ICU4XLocaleCanonicalizer* l) const noexcept {
    capi::ICU4XLocaleCanonicalizer_destroy(l);
  }
};
class ICU4XLocaleCanonicalizer {
 public:
  static std::optional<ICU4XLocaleCanonicalizer> create(const ICU4XDataProvider& provider);
  ICU4XCanonicalizationResult canonicalize(ICU4XLocale& locale);
  ICU4XCanonicalizationResult maximize(ICU4XLocale& locale);
  ICU4XCanonicalizationResult minimize(ICU4XLocale& locale);
  inline const capi::ICU4XLocaleCanonicalizer* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XLocaleCanonicalizer* AsFFIMut() { return this->inner.get(); }
  ICU4XLocaleCanonicalizer(capi::ICU4XLocaleCanonicalizer* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XLocaleCanonicalizer, ICU4XLocaleCanonicalizerDeleter> inner;
};

struct ICU4XPluralRulesDeleter {
  void operator()(capi::ICU4XPluralRules* l) const noexcept {
    capi::ICU4XPluralRules_destroy(l);
  }
};
class ICU4XPluralRules {
 public:
  static ICU4XCreatePluralRulesResult create(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XPluralRuleType ty);
  ICU4XPluralCategory select(const ICU4XPluralOperands& op);
  ICU4XPluralCategories categories();
  inline const capi::ICU4XPluralRules* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XPluralRules* AsFFIMut() { return this->inner.get(); }
  ICU4XPluralRules(capi::ICU4XPluralRules* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XPluralRules, ICU4XPluralRulesDeleter> inner;
};

struct ICU4XCreateDataProviderResultDeleter {
  void operator()(capi::ICU4XCreateDataProviderResult* l) const noexcept {
    capi::ICU4XCreateDataProviderResult_destroy(l);
  }
};
struct ICU4XCreateDataProviderResult {
 public:
  std::optional<ICU4XDataProvider> provider;
  bool success;
};

struct ICU4XCreateFixedDecimalResultDeleter {
  void operator()(capi::ICU4XCreateFixedDecimalResult* l) const noexcept {
    capi::ICU4XCreateFixedDecimalResult_destroy(l);
  }
};
struct ICU4XCreateFixedDecimalResult {
 public:
  std::optional<ICU4XFixedDecimal> fd;
  bool success;
};

struct ICU4XPluralOperandsDeleter {
  void operator()(capi::ICU4XPluralOperands* l) const noexcept {
    capi::ICU4XPluralOperands_destroy(l);
  }
};
struct ICU4XPluralOperands {
 public:
  uint64_t i;
  size_t v;
  size_t w;
  uint64_t f;
  uint64_t t;
  size_t c;
  static ICU4XCreatePluralOperandsResult create(const std::string_view s);
};

struct ICU4XCreatePluralOperandsResultDeleter {
  void operator()(capi::ICU4XCreatePluralOperandsResult* l) const noexcept {
    capi::ICU4XCreatePluralOperandsResult_destroy(l);
  }
};
struct ICU4XCreatePluralOperandsResult {
 public:
  ICU4XPluralOperands operands;
  bool success;
};

struct ICU4XCreatePluralRulesResultDeleter {
  void operator()(capi::ICU4XCreatePluralRulesResult* l) const noexcept {
    capi::ICU4XCreatePluralRulesResult_destroy(l);
  }
};
struct ICU4XCreatePluralRulesResult {
 public:
  std::optional<ICU4XPluralRules> rules;
  bool success;
};

struct ICU4XFixedDecimalFormatOptionsDeleter {
  void operator()(capi::ICU4XFixedDecimalFormatOptions* l) const noexcept {
    capi::ICU4XFixedDecimalFormatOptions_destroy(l);
  }
};
struct ICU4XFixedDecimalFormatOptions {
 public:
  ICU4XFixedDecimalGroupingStrategy grouping_strategy;
  ICU4XFixedDecimalSignDisplay sign_display;
  static ICU4XFixedDecimalFormatOptions default_();
};

struct ICU4XFixedDecimalFormatResultDeleter {
  void operator()(capi::ICU4XFixedDecimalFormatResult* l) const noexcept {
    capi::ICU4XFixedDecimalFormatResult_destroy(l);
  }
};
struct ICU4XFixedDecimalFormatResult {
 public:
  std::optional<ICU4XFixedDecimalFormat> fdf;
  bool success;
};

struct ICU4XPluralCategoriesDeleter {
  void operator()(capi::ICU4XPluralCategories* l) const noexcept {
    capi::ICU4XPluralCategories_destroy(l);
  }
};
struct ICU4XPluralCategories {
 public:
  bool zero;
  bool one;
  bool two;
  bool few;
  bool many;
  bool other;
};






ICU4XCreateDataProviderResult ICU4XDataProvider::create_fs(const std::string_view path) {
  capi::ICU4XCreateDataProviderResult diplomat_raw_struct_out_value = capi::ICU4XDataProvider_create_fs(path.data(), path.length());
  auto diplomat_optional_raw_out_value_provider = diplomat_raw_struct_out_value.provider;
  std::optional<ICU4XDataProvider> diplomat_optional_out_value_provider;
  if (diplomat_optional_raw_out_value_provider != nullptr) {
    diplomat_optional_out_value_provider = ICU4XDataProvider(diplomat_optional_raw_out_value_provider);
  } else {
    diplomat_optional_out_value_provider = std::nullopt;
  }
  return ICU4XCreateDataProviderResult{ .provider = std::move(diplomat_optional_out_value_provider), .success = std::move(diplomat_raw_struct_out_value.success) };
}
ICU4XCreateDataProviderResult ICU4XDataProvider::create_static() {
  capi::ICU4XCreateDataProviderResult diplomat_raw_struct_out_value = capi::ICU4XDataProvider_create_static();
  auto diplomat_optional_raw_out_value_provider = diplomat_raw_struct_out_value.provider;
  std::optional<ICU4XDataProvider> diplomat_optional_out_value_provider;
  if (diplomat_optional_raw_out_value_provider != nullptr) {
    diplomat_optional_out_value_provider = ICU4XDataProvider(diplomat_optional_raw_out_value_provider);
  } else {
    diplomat_optional_out_value_provider = std::nullopt;
  }
  return ICU4XCreateDataProviderResult{ .provider = std::move(diplomat_optional_out_value_provider), .success = std::move(diplomat_raw_struct_out_value.success) };
}

ICU4XFixedDecimal ICU4XFixedDecimal::create(int32_t v) {
  return ICU4XFixedDecimal(capi::ICU4XFixedDecimal_create(v));
}
ICU4XCreateFixedDecimalResult ICU4XFixedDecimal::create_fromstr(const std::string_view v) {
  capi::ICU4XCreateFixedDecimalResult diplomat_raw_struct_out_value = capi::ICU4XFixedDecimal_create_fromstr(v.data(), v.length());
  auto diplomat_optional_raw_out_value_fd = diplomat_raw_struct_out_value.fd;
  std::optional<ICU4XFixedDecimal> diplomat_optional_out_value_fd;
  if (diplomat_optional_raw_out_value_fd != nullptr) {
    diplomat_optional_out_value_fd = ICU4XFixedDecimal(diplomat_optional_raw_out_value_fd);
  } else {
    diplomat_optional_out_value_fd = std::nullopt;
  }
  return ICU4XCreateFixedDecimalResult{ .fd = std::move(diplomat_optional_out_value_fd), .success = std::move(diplomat_raw_struct_out_value.success) };
}
bool ICU4XFixedDecimal::multiply_pow10(int16_t power) {
  return capi::ICU4XFixedDecimal_multiply_pow10(this->inner.get(), power);
}
void ICU4XFixedDecimal::negate() {
  capi::ICU4XFixedDecimal_negate(this->inner.get());
}
std::string ICU4XFixedDecimal::to_string() {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  capi::ICU4XFixedDecimal_to_string(this->inner.get(), &diplomat_writeable_out);
  return diplomat_writeable_string;
}

ICU4XFixedDecimalFormatResult ICU4XFixedDecimalFormat::try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XFixedDecimalFormatOptions options) {
  ICU4XFixedDecimalFormatOptions diplomat_wrapped_struct_options = options;
  capi::ICU4XFixedDecimalFormatResult diplomat_raw_struct_out_value = capi::ICU4XFixedDecimalFormat_try_new(locale.AsFFI(), provider.AsFFI(), capi::ICU4XFixedDecimalFormatOptions{ .grouping_strategy = static_cast<ssize_t>(diplomat_wrapped_struct_options.grouping_strategy), .sign_display = static_cast<ssize_t>(diplomat_wrapped_struct_options.sign_display) });
  auto diplomat_optional_raw_out_value_fdf = diplomat_raw_struct_out_value.fdf;
  std::optional<ICU4XFixedDecimalFormat> diplomat_optional_out_value_fdf;
  if (diplomat_optional_raw_out_value_fdf != nullptr) {
    diplomat_optional_out_value_fdf = ICU4XFixedDecimalFormat(diplomat_optional_raw_out_value_fdf);
  } else {
    diplomat_optional_out_value_fdf = std::nullopt;
  }
  return ICU4XFixedDecimalFormatResult{ .fdf = std::move(diplomat_optional_out_value_fdf), .success = std::move(diplomat_raw_struct_out_value.success) };
}
std::string ICU4XFixedDecimalFormat::format_write(const ICU4XFixedDecimal& value) {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  capi::ICU4XFixedDecimalFormat_format_write(this->inner.get(), value.AsFFI(), &diplomat_writeable_out);
  return diplomat_writeable_string;
}

ICU4XFixedDecimalFormatOptions ICU4XFixedDecimalFormatOptions::default_() {
  capi::ICU4XFixedDecimalFormatOptions diplomat_raw_struct_out_value = capi::ICU4XFixedDecimalFormatOptions_default();
  return ICU4XFixedDecimalFormatOptions{ .grouping_strategy = std::move(ICU4XFixedDecimalGroupingStrategy{ diplomat_raw_struct_out_value.grouping_strategy }), .sign_display = std::move(ICU4XFixedDecimalSignDisplay{ diplomat_raw_struct_out_value.sign_display }) };
}




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
std::string ICU4XLocale::basename() {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  capi::ICU4XLocale_basename(this->inner.get(), &diplomat_writeable_out);
  return diplomat_writeable_string;
}
std::string ICU4XLocale::get_unicode_extension(const std::string_view bytes) {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  capi::ICU4XLocale_get_unicode_extension(this->inner.get(), bytes.data(), bytes.length(), &diplomat_writeable_out);
  return diplomat_writeable_string;
}
std::string ICU4XLocale::language() {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  capi::ICU4XLocale_language(this->inner.get(), &diplomat_writeable_out);
  return diplomat_writeable_string;
}
std::string ICU4XLocale::region() {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  capi::ICU4XLocale_region(this->inner.get(), &diplomat_writeable_out);
  return diplomat_writeable_string;
}
std::string ICU4XLocale::script() {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  capi::ICU4XLocale_script(this->inner.get(), &diplomat_writeable_out);
  return diplomat_writeable_string;
}
std::string ICU4XLocale::tostring() {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  capi::ICU4XLocale_tostring(this->inner.get(), &diplomat_writeable_out);
  return diplomat_writeable_string;
}

std::optional<ICU4XLocaleCanonicalizer> ICU4XLocaleCanonicalizer::create(const ICU4XDataProvider& provider) {
  auto diplomat_optional_raw_out_value = capi::ICU4XLocaleCanonicalizer_create(provider.AsFFI());
  std::optional<ICU4XLocaleCanonicalizer> diplomat_optional_out_value;
  if (diplomat_optional_raw_out_value != nullptr) {
    diplomat_optional_out_value = ICU4XLocaleCanonicalizer(diplomat_optional_raw_out_value);
  } else {
    diplomat_optional_out_value = std::nullopt;
  }
  return diplomat_optional_out_value;
}
ICU4XCanonicalizationResult ICU4XLocaleCanonicalizer::canonicalize(ICU4XLocale& locale) {
  return ICU4XCanonicalizationResult{ capi::ICU4XLocaleCanonicalizer_canonicalize(this->inner.get(), locale.AsFFIMut()) };
}
ICU4XCanonicalizationResult ICU4XLocaleCanonicalizer::maximize(ICU4XLocale& locale) {
  return ICU4XCanonicalizationResult{ capi::ICU4XLocaleCanonicalizer_maximize(this->inner.get(), locale.AsFFIMut()) };
}
ICU4XCanonicalizationResult ICU4XLocaleCanonicalizer::minimize(ICU4XLocale& locale) {
  return ICU4XCanonicalizationResult{ capi::ICU4XLocaleCanonicalizer_minimize(this->inner.get(), locale.AsFFIMut()) };
}




ICU4XCreatePluralOperandsResult ICU4XPluralOperands::create(const std::string_view s) {
  capi::ICU4XCreatePluralOperandsResult diplomat_raw_struct_out_value = capi::ICU4XPluralOperands_create(s.data(), s.length());
  capi::ICU4XPluralOperands diplomat_raw_struct_out_value_operands = diplomat_raw_struct_out_value.operands;
  return ICU4XCreatePluralOperandsResult{ .operands = std::move(ICU4XPluralOperands{ .i = std::move(diplomat_raw_struct_out_value_operands.i), .v = std::move(diplomat_raw_struct_out_value_operands.v), .w = std::move(diplomat_raw_struct_out_value_operands.w), .f = std::move(diplomat_raw_struct_out_value_operands.f), .t = std::move(diplomat_raw_struct_out_value_operands.t), .c = std::move(diplomat_raw_struct_out_value_operands.c) }), .success = std::move(diplomat_raw_struct_out_value.success) };
}


ICU4XCreatePluralRulesResult ICU4XPluralRules::create(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XPluralRuleType ty) {
  capi::ICU4XCreatePluralRulesResult diplomat_raw_struct_out_value = capi::ICU4XPluralRules_create(locale.AsFFI(), provider.AsFFI(), static_cast<ssize_t>(ty));
  auto diplomat_optional_raw_out_value_rules = diplomat_raw_struct_out_value.rules;
  std::optional<ICU4XPluralRules> diplomat_optional_out_value_rules;
  if (diplomat_optional_raw_out_value_rules != nullptr) {
    diplomat_optional_out_value_rules = ICU4XPluralRules(diplomat_optional_raw_out_value_rules);
  } else {
    diplomat_optional_out_value_rules = std::nullopt;
  }
  return ICU4XCreatePluralRulesResult{ .rules = std::move(diplomat_optional_out_value_rules), .success = std::move(diplomat_raw_struct_out_value.success) };
}
ICU4XPluralCategory ICU4XPluralRules::select(const ICU4XPluralOperands& op) {
  return ICU4XPluralCategory{ capi::ICU4XPluralRules_select(this->inner.get(), (capi::ICU4XPluralOperands*) &op) };
}
ICU4XPluralCategories ICU4XPluralRules::categories() {
  capi::ICU4XPluralCategories diplomat_raw_struct_out_value = capi::ICU4XPluralRules_categories(this->inner.get());
  return ICU4XPluralCategories{ .zero = std::move(diplomat_raw_struct_out_value.zero), .one = std::move(diplomat_raw_struct_out_value.one), .two = std::move(diplomat_raw_struct_out_value.two), .few = std::move(diplomat_raw_struct_out_value.few), .many = std::move(diplomat_raw_struct_out_value.many), .other = std::move(diplomat_raw_struct_out_value.other) };
}
