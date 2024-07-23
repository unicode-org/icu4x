#ifndef UnitsConverterFactory_HPP
#define UnitsConverterFactory_HPP

#include "UnitsConverterFactory.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "MeasureUnit.hpp"
#include "MeasureUnitParser.hpp"
#include "UnitsConverter.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct icu4x_UnitsConverterFactory_create_mv1_result {union {diplomat::capi::UnitsConverterFactory* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_UnitsConverterFactory_create_mv1_result;
    icu4x_UnitsConverterFactory_create_mv1_result icu4x_UnitsConverterFactory_create_mv1(const diplomat::capi::DataProvider* provider);
    
    diplomat::capi::UnitsConverter* icu4x_UnitsConverterFactory_converter_mv1(const diplomat::capi::UnitsConverterFactory* self, const diplomat::capi::MeasureUnit* from, const diplomat::capi::MeasureUnit* to);
    
    diplomat::capi::MeasureUnitParser* icu4x_UnitsConverterFactory_parser_mv1(const diplomat::capi::UnitsConverterFactory* self);
    
    
    void icu4x_UnitsConverterFactory_destroy_mv1(UnitsConverterFactory* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<UnitsConverterFactory>, DataError> UnitsConverterFactory::create(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_UnitsConverterFactory_create_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<UnitsConverterFactory>, DataError>(diplomat::Ok<std::unique_ptr<UnitsConverterFactory>>(std::unique_ptr<UnitsConverterFactory>(UnitsConverterFactory::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<UnitsConverterFactory>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::unique_ptr<UnitsConverter> UnitsConverterFactory::converter(const MeasureUnit& from, const MeasureUnit& to) const {
  auto result = diplomat::capi::icu4x_UnitsConverterFactory_converter_mv1(this->AsFFI(),
    from.AsFFI(),
    to.AsFFI());
  return std::unique_ptr<UnitsConverter>(UnitsConverter::FromFFI(result));
}

inline std::unique_ptr<MeasureUnitParser> UnitsConverterFactory::parser() const {
  auto result = diplomat::capi::icu4x_UnitsConverterFactory_parser_mv1(this->AsFFI());
  return std::unique_ptr<MeasureUnitParser>(MeasureUnitParser::FromFFI(result));
}

inline const diplomat::capi::UnitsConverterFactory* UnitsConverterFactory::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::UnitsConverterFactory*>(this);
}

inline diplomat::capi::UnitsConverterFactory* UnitsConverterFactory::AsFFI() {
  return reinterpret_cast<diplomat::capi::UnitsConverterFactory*>(this);
}

inline const UnitsConverterFactory* UnitsConverterFactory::FromFFI(const diplomat::capi::UnitsConverterFactory* ptr) {
  return reinterpret_cast<const UnitsConverterFactory*>(ptr);
}

inline UnitsConverterFactory* UnitsConverterFactory::FromFFI(diplomat::capi::UnitsConverterFactory* ptr) {
  return reinterpret_cast<UnitsConverterFactory*>(ptr);
}

inline void UnitsConverterFactory::operator delete(void* ptr) {
  diplomat::capi::icu4x_UnitsConverterFactory_destroy_mv1(reinterpret_cast<diplomat::capi::UnitsConverterFactory*>(ptr));
}


#endif // UnitsConverterFactory_HPP
