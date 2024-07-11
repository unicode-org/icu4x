#ifndef CaseMapCloser_HPP
#define CaseMapCloser_HPP

#include "CaseMapCloser.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CodePointSetBuilder.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XCaseMapCloser_create_result {union {CaseMapCloser* ok; DataError err;}; bool is_ok;} ICU4XCaseMapCloser_create_result;
    ICU4XCaseMapCloser_create_result ICU4XCaseMapCloser_create(const DataProvider* provider);
    
    void ICU4XCaseMapCloser_add_case_closure_to(const CaseMapCloser* self, char32_t c, CodePointSetBuilder* builder);
    
    bool ICU4XCaseMapCloser_add_string_case_closure_to(const CaseMapCloser* self, const char* s_data, size_t s_len, CodePointSetBuilder* builder);
    
    
    void ICU4XCaseMapCloser_destroy(CaseMapCloser* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<CaseMapCloser>, DataError> CaseMapCloser::create(const DataProvider& provider) {
  auto result = capi::ICU4XCaseMapCloser_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CaseMapCloser>, DataError>(diplomat::Ok<std::unique_ptr<CaseMapCloser>>(std::unique_ptr<CaseMapCloser>(CaseMapCloser::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CaseMapCloser>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline void CaseMapCloser::add_case_closure_to(char32_t c, CodePointSetBuilder& builder) const {
  capi::ICU4XCaseMapCloser_add_case_closure_to(this->AsFFI(),
    c,
    builder.AsFFI());
}

inline bool CaseMapCloser::add_string_case_closure_to(std::string_view s, CodePointSetBuilder& builder) const {
  auto result = capi::ICU4XCaseMapCloser_add_string_case_closure_to(this->AsFFI(),
    s.data(),
    s.size(),
    builder.AsFFI());
  return result;
}

inline const capi::CaseMapCloser* CaseMapCloser::AsFFI() const {
  return reinterpret_cast<const capi::CaseMapCloser*>(this);
}

inline capi::CaseMapCloser* CaseMapCloser::AsFFI() {
  return reinterpret_cast<capi::CaseMapCloser*>(this);
}

inline const CaseMapCloser* CaseMapCloser::FromFFI(const capi::CaseMapCloser* ptr) {
  return reinterpret_cast<const CaseMapCloser*>(ptr);
}

inline CaseMapCloser* CaseMapCloser::FromFFI(capi::CaseMapCloser* ptr) {
  return reinterpret_cast<CaseMapCloser*>(ptr);
}

inline void CaseMapCloser::operator delete(void* ptr) {
  capi::ICU4XCaseMapCloser_destroy(reinterpret_cast<capi::CaseMapCloser*>(ptr));
}


#endif // CaseMapCloser_HPP
