#ifndef FixedDecimalFormatter_HPP
#define FixedDecimalFormatter_HPP

#include "FixedDecimalFormatter.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "DataStruct.hpp"
#include "FixedDecimal.hpp"
#include "FixedDecimalGroupingStrategy.hpp"
#include "Locale.hpp"


namespace capi {
    extern "C" {
    
    typedef struct ICU4XFixedDecimalFormatter_create_with_grouping_strategy_result {union {FixedDecimalFormatter* ok; DataError err;}; bool is_ok;} ICU4XFixedDecimalFormatter_create_with_grouping_strategy_result;
    ICU4XFixedDecimalFormatter_create_with_grouping_strategy_result ICU4XFixedDecimalFormatter_create_with_grouping_strategy(const DataProvider* provider, const Locale* locale, FixedDecimalGroupingStrategy grouping_strategy);
    
    typedef struct ICU4XFixedDecimalFormatter_create_with_decimal_symbols_v1_result {union {FixedDecimalFormatter* ok; DataError err;}; bool is_ok;} ICU4XFixedDecimalFormatter_create_with_decimal_symbols_v1_result;
    ICU4XFixedDecimalFormatter_create_with_decimal_symbols_v1_result ICU4XFixedDecimalFormatter_create_with_decimal_symbols_v1(const DataStruct* data_struct, FixedDecimalGroupingStrategy grouping_strategy);
    
    void ICU4XFixedDecimalFormatter_format(const FixedDecimalFormatter* self, const FixedDecimal* value, DiplomatWrite* write);
    
    
    void ICU4XFixedDecimalFormatter_destroy(FixedDecimalFormatter* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<FixedDecimalFormatter>, DataError> FixedDecimalFormatter::create_with_grouping_strategy(const DataProvider& provider, const Locale& locale, FixedDecimalGroupingStrategy grouping_strategy) {
  auto result = capi::ICU4XFixedDecimalFormatter_create_with_grouping_strategy(provider.AsFFI(),
    locale.AsFFI(),
    grouping_strategy.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<FixedDecimalFormatter>, DataError>(diplomat::Ok<std::unique_ptr<FixedDecimalFormatter>>(std::unique_ptr<FixedDecimalFormatter>(FixedDecimalFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<FixedDecimalFormatter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<FixedDecimalFormatter>, DataError> FixedDecimalFormatter::create_with_decimal_symbols_v1(const DataStruct& data_struct, FixedDecimalGroupingStrategy grouping_strategy) {
  auto result = capi::ICU4XFixedDecimalFormatter_create_with_decimal_symbols_v1(data_struct.AsFFI(),
    grouping_strategy.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<FixedDecimalFormatter>, DataError>(diplomat::Ok<std::unique_ptr<FixedDecimalFormatter>>(std::unique_ptr<FixedDecimalFormatter>(FixedDecimalFormatter::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<FixedDecimalFormatter>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::string FixedDecimalFormatter::format(const FixedDecimal& value) const {
  std::string output;
  capi::DiplomatWrite write = diplomat::WriteFromString(output);
  capi::ICU4XFixedDecimalFormatter_format(this->AsFFI(),
    value.AsFFI(),
    &write);
  return output;
}

inline const capi::FixedDecimalFormatter* FixedDecimalFormatter::AsFFI() const {
  return reinterpret_cast<const capi::FixedDecimalFormatter*>(this);
}

inline capi::FixedDecimalFormatter* FixedDecimalFormatter::AsFFI() {
  return reinterpret_cast<capi::FixedDecimalFormatter*>(this);
}

inline const FixedDecimalFormatter* FixedDecimalFormatter::FromFFI(const capi::FixedDecimalFormatter* ptr) {
  return reinterpret_cast<const FixedDecimalFormatter*>(ptr);
}

inline FixedDecimalFormatter* FixedDecimalFormatter::FromFFI(capi::FixedDecimalFormatter* ptr) {
  return reinterpret_cast<FixedDecimalFormatter*>(ptr);
}

inline void FixedDecimalFormatter::operator delete(void* ptr) {
  capi::ICU4XFixedDecimalFormatter_destroy(reinterpret_cast<capi::FixedDecimalFormatter*>(ptr));
}


#endif // FixedDecimalFormatter_HPP
