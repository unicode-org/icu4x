#ifndef CaseMapCloser_HPP
#define CaseMapCloser_HPP

#include "CaseMapCloser.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "CodePointSetBuilder.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_CaseMapCloser_create_mv1_result {union {diplomat::capi::CaseMapCloser* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CaseMapCloser_create_mv1_result;
    icu4x_CaseMapCloser_create_mv1_result icu4x_CaseMapCloser_create_mv1(const diplomat::capi::DataProvider* provider);
    
    void icu4x_CaseMapCloser_add_case_closure_to_mv1(const diplomat::capi::CaseMapCloser* self, char32_t c, diplomat::capi::CodePointSetBuilder* builder);
    
    bool icu4x_CaseMapCloser_add_string_case_closure_to_mv1(const diplomat::capi::CaseMapCloser* self, const char* s_data, size_t s_len, diplomat::capi::CodePointSetBuilder* builder);
    
    
    void icu4x_CaseMapCloser_destroy_mv1(CaseMapCloser* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<CaseMapCloser>, DataError> CaseMapCloser::create(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CaseMapCloser_create_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CaseMapCloser>, DataError>(diplomat::Ok<std::unique_ptr<CaseMapCloser>>(std::unique_ptr<CaseMapCloser>(CaseMapCloser::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CaseMapCloser>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline void CaseMapCloser::add_case_closure_to(char32_t c, CodePointSetBuilder& builder) const {
  diplomat::capi::icu4x_CaseMapCloser_add_case_closure_to_mv1(this->AsFFI(),
    c,
    builder.AsFFI());
}

inline bool CaseMapCloser::add_string_case_closure_to(std::string_view s, CodePointSetBuilder& builder) const {
  auto result = diplomat::capi::icu4x_CaseMapCloser_add_string_case_closure_to_mv1(this->AsFFI(),
    s.data(),
    s.size(),
    builder.AsFFI());
  return result;
}

inline const diplomat::capi::CaseMapCloser* CaseMapCloser::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::CaseMapCloser*>(this);
}

inline diplomat::capi::CaseMapCloser* CaseMapCloser::AsFFI() {
  return reinterpret_cast<diplomat::capi::CaseMapCloser*>(this);
}

inline const CaseMapCloser* CaseMapCloser::FromFFI(const diplomat::capi::CaseMapCloser* ptr) {
  return reinterpret_cast<const CaseMapCloser*>(ptr);
}

inline CaseMapCloser* CaseMapCloser::FromFFI(diplomat::capi::CaseMapCloser* ptr) {
  return reinterpret_cast<CaseMapCloser*>(ptr);
}

inline void CaseMapCloser::operator delete(void* ptr) {
  diplomat::capi::icu4x_CaseMapCloser_destroy_mv1(reinterpret_cast<diplomat::capi::CaseMapCloser*>(ptr));
}


#endif // CaseMapCloser_HPP
