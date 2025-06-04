#ifndef icu4x_ErasedMeasureUnitParser_HPP
#define icu4x_ErasedMeasureUnitParser_HPP

#include "ErasedMeasureUnitParser.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "ErasedMeasureUnit.hpp"


namespace icu4x {
namespace capi {
    extern "C" {

    icu4x::capi::ErasedMeasureUnitParser* icu4x_ErasedMeasureUnitParser_create_mv1(void);

    typedef struct icu4x_ErasedMeasureUnitParser_create_with_provider_mv1_result {union {icu4x::capi::ErasedMeasureUnitParser* ok; icu4x::capi::DataError err;}; bool is_ok;} icu4x_ErasedMeasureUnitParser_create_with_provider_mv1_result;
    icu4x_ErasedMeasureUnitParser_create_with_provider_mv1_result icu4x_ErasedMeasureUnitParser_create_with_provider_mv1(const icu4x::capi::DataProvider* provider);

    icu4x::capi::ErasedMeasureUnit* icu4x_ErasedMeasureUnitParser_parse_mv1(const icu4x::capi::ErasedMeasureUnitParser* self, diplomat::capi::DiplomatStringView unit_id);

    void icu4x_ErasedMeasureUnitParser_destroy_mv1(ErasedMeasureUnitParser* self);

    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<icu4x::ErasedMeasureUnitParser> icu4x::ErasedMeasureUnitParser::create() {
  auto result = icu4x::capi::icu4x_ErasedMeasureUnitParser_create_mv1();
  return std::unique_ptr<icu4x::ErasedMeasureUnitParser>(icu4x::ErasedMeasureUnitParser::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<icu4x::ErasedMeasureUnitParser>, icu4x::DataError> icu4x::ErasedMeasureUnitParser::create_with_provider(const icu4x::DataProvider& provider) {
  auto result = icu4x::capi::icu4x_ErasedMeasureUnitParser_create_with_provider_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<icu4x::ErasedMeasureUnitParser>, icu4x::DataError>(diplomat::Ok<std::unique_ptr<icu4x::ErasedMeasureUnitParser>>(std::unique_ptr<icu4x::ErasedMeasureUnitParser>(icu4x::ErasedMeasureUnitParser::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<icu4x::ErasedMeasureUnitParser>, icu4x::DataError>(diplomat::Err<icu4x::DataError>(icu4x::DataError::FromFFI(result.err)));
}

inline std::unique_ptr<icu4x::ErasedMeasureUnit> icu4x::ErasedMeasureUnitParser::parse(std::string_view unit_id) const {
  auto result = icu4x::capi::icu4x_ErasedMeasureUnitParser_parse_mv1(this->AsFFI(),
    {unit_id.data(), unit_id.size()});
  return std::unique_ptr<icu4x::ErasedMeasureUnit>(icu4x::ErasedMeasureUnit::FromFFI(result));
}

inline const icu4x::capi::ErasedMeasureUnitParser* icu4x::ErasedMeasureUnitParser::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::ErasedMeasureUnitParser*>(this);
}

inline icu4x::capi::ErasedMeasureUnitParser* icu4x::ErasedMeasureUnitParser::AsFFI() {
  return reinterpret_cast<icu4x::capi::ErasedMeasureUnitParser*>(this);
}

inline const icu4x::ErasedMeasureUnitParser* icu4x::ErasedMeasureUnitParser::FromFFI(const icu4x::capi::ErasedMeasureUnitParser* ptr) {
  return reinterpret_cast<const icu4x::ErasedMeasureUnitParser*>(ptr);
}

inline icu4x::ErasedMeasureUnitParser* icu4x::ErasedMeasureUnitParser::FromFFI(icu4x::capi::ErasedMeasureUnitParser* ptr) {
  return reinterpret_cast<icu4x::ErasedMeasureUnitParser*>(ptr);
}

inline void icu4x::ErasedMeasureUnitParser::operator delete(void* ptr) {
  icu4x::capi::icu4x_ErasedMeasureUnitParser_destroy_mv1(reinterpret_cast<icu4x::capi::ErasedMeasureUnitParser*>(ptr));
}


#endif // icu4x_ErasedMeasureUnitParser_HPP
