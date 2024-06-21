#ifndef ICU4XFixedDecimalFormatter_HPP
#define ICU4XFixedDecimalFormatter_HPP

#include "ICU4XFixedDecimalFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XDataStruct.hpp"
#include "ICU4XFixedDecimal.hpp"
#include "ICU4XFixedDecimalGroupingStrategy.hpp"
#include "ICU4XLocale.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XFixedDecimalFormatter_create_with_grouping_strategy_result {union {ICU4XFixedDecimalFormatter* ok; ICU4XDataError err;}; bool is_ok;} ICU4XFixedDecimalFormatter_create_with_grouping_strategy_result;
    ICU4XFixedDecimalFormatter_create_with_grouping_strategy_result ICU4XFixedDecimalFormatter_create_with_grouping_strategy(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XFixedDecimalGroupingStrategy grouping_strategy);
    
    typedef struct ICU4XFixedDecimalFormatter_create_with_decimal_symbols_v1_result {union {ICU4XFixedDecimalFormatter* ok; ICU4XDataError err;}; bool is_ok;} ICU4XFixedDecimalFormatter_create_with_decimal_symbols_v1_result;
    ICU4XFixedDecimalFormatter_create_with_decimal_symbols_v1_result ICU4XFixedDecimalFormatter_create_with_decimal_symbols_v1(const ICU4XDataStruct* data_struct, ICU4XFixedDecimalGroupingStrategy grouping_strategy);
    
    void ICU4XFixedDecimalFormatter_format(const ICU4XFixedDecimalFormatter* self, const ICU4XFixedDecimal* value, DiplomatWrite* write);
    
    
    void ICU4XFixedDecimalFormatter_destroy(ICU4XFixedDecimalFormatter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XFixedDecimalFormatter>, ICU4XDataError> ICU4XFixedDecimalFormatter::create_with_grouping_strategy(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XFixedDecimalGroupingStrategy grouping_strategy) {
  auto result = capi::ICU4XFixedDecimalFormatter_create_with_grouping_strategy(provider.AsFFI(),
    locale.AsFFI(),
    grouping_strategy.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XFixedDecimalFormatter>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XFixedDecimalFormatter>>(std::unique_ptr<ICU4XFixedDecimalFormatter>(ICU4XFixedDecimalFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XFixedDecimalFormatter>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XFixedDecimalFormatter>, ICU4XDataError> ICU4XFixedDecimalFormatter::create_with_decimal_symbols_v1(const ICU4XDataStruct& data_struct, ICU4XFixedDecimalGroupingStrategy grouping_strategy) {
  auto result = capi::ICU4XFixedDecimalFormatter_create_with_decimal_symbols_v1(data_struct.AsFFI(),
    grouping_strategy.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XFixedDecimalFormatter>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XFixedDecimalFormatter>>(std::unique_ptr<ICU4XFixedDecimalFormatter>(ICU4XFixedDecimalFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XFixedDecimalFormatter>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline std::string ICU4XFixedDecimalFormatter::format(const ICU4XFixedDecimal& value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XFixedDecimalFormatter_format(this->AsFFI(),
    value.AsFFI(),
    &write);
  return output;
}

inline const capi::ICU4XFixedDecimalFormatter* ICU4XFixedDecimalFormatter::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XFixedDecimalFormatter*>(this);
}

inline capi::ICU4XFixedDecimalFormatter* ICU4XFixedDecimalFormatter::AsFFI() {
  return reinterpret_cast<capi::ICU4XFixedDecimalFormatter*>(this);
}

inline const ICU4XFixedDecimalFormatter* ICU4XFixedDecimalFormatter::FromFFI(const capi::ICU4XFixedDecimalFormatter* ptr) {
  return reinterpret_cast<const ICU4XFixedDecimalFormatter*>(ptr);
}

inline ICU4XFixedDecimalFormatter* ICU4XFixedDecimalFormatter::FromFFI(capi::ICU4XFixedDecimalFormatter* ptr) {
  return reinterpret_cast<ICU4XFixedDecimalFormatter*>(ptr);
}

inline void ICU4XFixedDecimalFormatter::operator delete(void* ptr) {
  capi::ICU4XFixedDecimalFormatter_destroy(reinterpret_cast<capi::ICU4XFixedDecimalFormatter*>(ptr));
}


#endif // ICU4XFixedDecimalFormatter_HPP
