#ifndef ICU4XCaseMapCloser_D_HPP
#define ICU4XCaseMapCloser_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.d.hpp"

class ICU4XCodePointSetBuilder;
class ICU4XDataProvider;
class ICU4XDataError;


namespace capi {
    typedef struct ICU4XCaseMapCloser ICU4XCaseMapCloser;
}

class ICU4XCaseMapCloser {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XCaseMapCloser>, ICU4XDataError> create(const ICU4XDataProvider& provider);

  inline void add_case_closure_to(char32_t c, ICU4XCodePointSetBuilder& builder) const;

  inline bool add_string_case_closure_to(std::string_view s, ICU4XCodePointSetBuilder& builder) const;

  inline const capi::ICU4XCaseMapCloser* AsFFI() const;
  inline capi::ICU4XCaseMapCloser* AsFFI();
  inline static const ICU4XCaseMapCloser* FromFFI(const capi::ICU4XCaseMapCloser* ptr);
  inline static ICU4XCaseMapCloser* FromFFI(capi::ICU4XCaseMapCloser* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XCaseMapCloser() = delete;
  ICU4XCaseMapCloser(const ICU4XCaseMapCloser&) = delete;
  ICU4XCaseMapCloser(ICU4XCaseMapCloser&&) noexcept = delete;
  ICU4XCaseMapCloser operator=(const ICU4XCaseMapCloser&) = delete;
  ICU4XCaseMapCloser operator=(ICU4XCaseMapCloser&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XCaseMapCloser_D_HPP
