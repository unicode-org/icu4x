#ifndef ICU4XFixedDecimal_HPP
#define ICU4XFixedDecimal_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <optional>
#include <variant>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XFixedDecimal.h"
}

class ICU4XFixedDecimal;
struct ICU4XCreateFixedDecimalResult;

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
  template<typename W> void to_string_to_writeable(W& to) const;
  std::string to_string() const;
  inline const capi::ICU4XFixedDecimal* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XFixedDecimal* AsFFIMut() { return this->inner.get(); }
  inline ICU4XFixedDecimal(capi::ICU4XFixedDecimal* i) : inner(i) {}
 private:
  std::unique_ptr<capi::ICU4XFixedDecimal, ICU4XFixedDecimalDeleter> inner;
};

#include "ICU4XCreateFixedDecimalResult.hpp"

inline ICU4XFixedDecimal ICU4XFixedDecimal::create(int32_t v) {
  return ICU4XFixedDecimal(capi::ICU4XFixedDecimal_create(v));
}
inline ICU4XCreateFixedDecimalResult ICU4XFixedDecimal::create_fromstr(const std::string_view v) {
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
inline bool ICU4XFixedDecimal::multiply_pow10(int16_t power) {
  return capi::ICU4XFixedDecimal_multiply_pow10(this->inner.get(), power);
}
inline void ICU4XFixedDecimal::negate() {
  capi::ICU4XFixedDecimal_negate(this->inner.get());
}
template<typename W> inline void ICU4XFixedDecimal::to_string_to_writeable(W& to) const {
  capi::DiplomatWriteable to_writer = diplomat::WriteableTrait<W>::Construct(to);
  capi::ICU4XFixedDecimal_to_string(this->inner.get(), &to_writer);
}
inline std::string ICU4XFixedDecimal::to_string() const {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  capi::ICU4XFixedDecimal_to_string(this->inner.get(), &diplomat_writeable_out);
  return diplomat_writeable_string;
}
#endif
