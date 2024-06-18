#ifndef ICU4XFixedDecimalFormatter_D_HPP
#define ICU4XFixedDecimalFormatter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.d.hpp"
#include "ICU4XFixedDecimalFormatter.d.h"
#include "ICU4XFixedDecimalGroupingStrategy.d.hpp"

class ICU4XDataProvider;
class ICU4XDataStruct;
class ICU4XFixedDecimal;
class ICU4XLocale;
class ICU4XDataError;
class ICU4XFixedDecimalGroupingStrategy;


class ICU4XFixedDecimalFormatter {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XFixedDecimalFormatter>, ICU4XDataError> create_with_grouping_strategy(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XFixedDecimalGroupingStrategy grouping_strategy);

  inline static diplomat::result<std::unique_ptr<ICU4XFixedDecimalFormatter>, ICU4XDataError> create_with_decimal_symbols_v1(const ICU4XDataStruct& data_struct, ICU4XFixedDecimalGroupingStrategy grouping_strategy);

  inline std::string format(const ICU4XFixedDecimal& value) const;

  inline const capi::ICU4XFixedDecimalFormatter* AsFFI() const;
  inline capi::ICU4XFixedDecimalFormatter* AsFFI();
  inline static const ICU4XFixedDecimalFormatter* FromFFI(const capi::ICU4XFixedDecimalFormatter* ptr);
  inline static ICU4XFixedDecimalFormatter* FromFFI(capi::ICU4XFixedDecimalFormatter* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XFixedDecimalFormatter() = delete;
  ICU4XFixedDecimalFormatter(const ICU4XFixedDecimalFormatter&) = delete;
  ICU4XFixedDecimalFormatter(ICU4XFixedDecimalFormatter&&) noexcept = delete;
  ICU4XFixedDecimalFormatter operator=(const ICU4XFixedDecimalFormatter&) = delete;
  ICU4XFixedDecimalFormatter operator=(ICU4XFixedDecimalFormatter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XFixedDecimalFormatter_D_HPP
