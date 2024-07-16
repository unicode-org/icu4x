#ifndef CaseMapCloser_D_HPP
#define CaseMapCloser_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct CodePointSetBuilder; }
class CodePointSetBuilder;
namespace diplomat::capi { struct DataProvider; }
class DataProvider;
class DataError;


namespace diplomat {
namespace capi {
    struct CaseMapCloser;
} // namespace capi
} // namespace

class CaseMapCloser {
public:

  inline static diplomat::result<std::unique_ptr<CaseMapCloser>, DataError> create(const DataProvider& provider);

  inline void add_case_closure_to(char32_t c, CodePointSetBuilder& builder) const;

  inline bool add_string_case_closure_to(std::string_view s, CodePointSetBuilder& builder) const;

  inline const diplomat::capi::CaseMapCloser* AsFFI() const;
  inline diplomat::capi::CaseMapCloser* AsFFI();
  inline static const CaseMapCloser* FromFFI(const diplomat::capi::CaseMapCloser* ptr);
  inline static CaseMapCloser* FromFFI(diplomat::capi::CaseMapCloser* ptr);
  inline static void operator delete(void* ptr);
private:
  CaseMapCloser() = delete;
  CaseMapCloser(const CaseMapCloser&) = delete;
  CaseMapCloser(CaseMapCloser&&) noexcept = delete;
  CaseMapCloser operator=(const CaseMapCloser&) = delete;
  CaseMapCloser operator=(CaseMapCloser&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // CaseMapCloser_D_HPP
