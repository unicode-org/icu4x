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
#include "ICU4XCodePointSetBuilder.hpp"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XCaseMapCloser_create_result {union {ICU4XCaseMapCloser* ok; ICU4XDataError err;}; bool is_ok;} ICU4XCaseMapCloser_create_result;
    ICU4XCaseMapCloser_create_result ICU4XCaseMapCloser_create(const ICU4XDataProvider* provider);
    
    void ICU4XCaseMapCloser_add_case_closure_to(const ICU4XCaseMapCloser* self, char32_t c, ICU4XCodePointSetBuilder* builder);
    
    bool ICU4XCaseMapCloser_add_string_case_closure_to(const ICU4XCaseMapCloser* self, const char* s_data, size_t s_len, ICU4XCodePointSetBuilder* builder);
    
    
    void ICU4XCaseMapCloser_destroy(ICU4XCaseMapCloser* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XCaseMapCloser>, ICU4XDataError> ICU4XCaseMapCloser::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCaseMapCloser_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCaseMapCloser>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XCaseMapCloser>>(std::unique_ptr<ICU4XCaseMapCloser>(ICU4XCaseMapCloser::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCaseMapCloser>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
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
