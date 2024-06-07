#ifndef ICU4XCaseMapCloser_HPP
#define ICU4XCaseMapCloser_HPP

#include "ICU4XCaseMapCloser.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCaseMapCloser.h"
#include "ICU4XCodePointSetBuilder.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XError.hpp"


inline diplomat::result<std::unique_ptr<ICU4XCaseMapCloser>, ICU4XError> ICU4XCaseMapCloser::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCaseMapCloser_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCaseMapCloser>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCaseMapCloser>>(std::unique_ptr<ICU4XCaseMapCloser>(ICU4XCaseMapCloser::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCaseMapCloser>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline void ICU4XCaseMapCloser::add_case_closure_to(char32_t c, ICU4XCodePointSetBuilder& builder) const {
  capi::ICU4XCaseMapCloser_add_case_closure_to(this->AsFFI(),
    c,
    builder.AsFFI());
}

inline bool ICU4XCaseMapCloser::add_string_case_closure_to(std::string_view s, ICU4XCodePointSetBuilder& builder) const {
  auto result = capi::ICU4XCaseMapCloser_add_string_case_closure_to(this->AsFFI(),
    s.data(),
    s.size(),
    builder.AsFFI());
  return result;
}

inline const capi::ICU4XCaseMapCloser* ICU4XCaseMapCloser::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XCaseMapCloser*>(this);
}

inline capi::ICU4XCaseMapCloser* ICU4XCaseMapCloser::AsFFI() {
  return reinterpret_cast<capi::ICU4XCaseMapCloser*>(this);
}

inline const ICU4XCaseMapCloser* ICU4XCaseMapCloser::FromFFI(const capi::ICU4XCaseMapCloser* ptr) {
  return reinterpret_cast<const ICU4XCaseMapCloser*>(ptr);
}

inline ICU4XCaseMapCloser* ICU4XCaseMapCloser::FromFFI(capi::ICU4XCaseMapCloser* ptr) {
  return reinterpret_cast<ICU4XCaseMapCloser*>(ptr);
}

inline void ICU4XCaseMapCloser::operator delete(void* ptr) {
  capi::ICU4XCaseMapCloser_destroy(reinterpret_cast<capi::ICU4XCaseMapCloser*>(ptr));
}


#endif // ICU4XCaseMapCloser_HPP
