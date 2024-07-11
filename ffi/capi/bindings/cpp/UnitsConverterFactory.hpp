#ifndef UnitsConverterFactory_HPP
#define UnitsConverterFactory_HPP

#include "UnitsConverterFactory.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "MeasureUnit.hpp"
#include "MeasureUnitParser.hpp"
#include "UnitsConverter.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    typedef struct ICU4XUnitsConverterFactory_create_result {union {diplomat::capi::UnitsConverterFactory* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XUnitsConverterFactory_create_result;
    ICU4XUnitsConverterFactory_create_result ICU4XUnitsConverterFactory_create(const diplomat::capi::DataProvider* provider);
    
    diplomat::capi::UnitsConverter* ICU4XUnitsConverterFactory_converter(const diplomat::capi::UnitsConverterFactory* self, const diplomat::capi::MeasureUnit* from, const diplomat::capi::MeasureUnit* to);
    
    diplomat::capi::MeasureUnitParser* ICU4XUnitsConverterFactory_parser(const diplomat::capi::UnitsConverterFactory* self);
    
    
    void ICU4XUnitsConverterFactory_destroy(UnitsConverterFactory* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::result<std::unique_ptr<UnitsConverterFactory>, DataError> UnitsConverterFactory::create(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XUnitsConverterFactory_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<UnitsConverterFactory>, DataError>(diplomat::Ok<std::unique_ptr<UnitsConverterFactory>>(std::unique_ptr<UnitsConverterFactory>(UnitsConverterFactory::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<UnitsConverterFactory>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::unique_ptr<UnitsConverter> UnitsConverterFactory::converter(const MeasureUnit& from, const MeasureUnit& to) const {
  auto result = diplomat::capi::ICU4XUnitsConverterFactory_converter(this->AsFFI(),
    from.AsFFI(),
    to.AsFFI());
  return std::unique_ptr<UnitsConverter>(UnitsConverter::FromFFI(result));
}

inline std::unique_ptr<MeasureUnitParser> UnitsConverterFactory::parser() const {
  auto result = diplomat::capi::ICU4XUnitsConverterFactory_parser(this->AsFFI());
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
  diplomat::capi::ICU4XUnitsConverterFactory_destroy(reinterpret_cast<diplomat::capi::UnitsConverterFactory*>(ptr));
}


#endif // UnitsConverterFactory_HPP
