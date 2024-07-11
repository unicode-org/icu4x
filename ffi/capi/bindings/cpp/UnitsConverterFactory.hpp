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


namespace capi {
    extern "C" {
    
    typedef struct ICU4XUnitsConverterFactory_create_result {union {UnitsConverterFactory* ok; DataError err;}; bool is_ok;} ICU4XUnitsConverterFactory_create_result;
    ICU4XUnitsConverterFactory_create_result ICU4XUnitsConverterFactory_create(const DataProvider* provider);
    
    UnitsConverter* ICU4XUnitsConverterFactory_converter(const UnitsConverterFactory* self, const MeasureUnit* from, const MeasureUnit* to);
    
    MeasureUnitParser* ICU4XUnitsConverterFactory_parser(const UnitsConverterFactory* self);
    
    
    void ICU4XUnitsConverterFactory_destroy(UnitsConverterFactory* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<UnitsConverterFactory>, DataError> UnitsConverterFactory::create(const DataProvider& provider) {
  auto result = capi::ICU4XUnitsConverterFactory_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<UnitsConverterFactory>, DataError>(diplomat::Ok<std::unique_ptr<UnitsConverterFactory>>(std::unique_ptr<UnitsConverterFactory>(UnitsConverterFactory::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<UnitsConverterFactory>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline std::unique_ptr<UnitsConverter> UnitsConverterFactory::converter(const MeasureUnit& from, const MeasureUnit& to) const {
  auto result = capi::ICU4XUnitsConverterFactory_converter(this->AsFFI(),
    from.AsFFI(),
    to.AsFFI());
  return std::unique_ptr<UnitsConverter>(UnitsConverter::FromFFI(result));
}

inline std::unique_ptr<MeasureUnitParser> UnitsConverterFactory::parser() const {
  auto result = capi::ICU4XUnitsConverterFactory_parser(this->AsFFI());
  return std::unique_ptr<MeasureUnitParser>(MeasureUnitParser::FromFFI(result));
}

inline const capi::UnitsConverterFactory* UnitsConverterFactory::AsFFI() const {
  return reinterpret_cast<const capi::UnitsConverterFactory*>(this);
}

inline capi::UnitsConverterFactory* UnitsConverterFactory::AsFFI() {
  return reinterpret_cast<capi::UnitsConverterFactory*>(this);
}

inline const UnitsConverterFactory* UnitsConverterFactory::FromFFI(const capi::UnitsConverterFactory* ptr) {
  return reinterpret_cast<const UnitsConverterFactory*>(ptr);
}

inline UnitsConverterFactory* UnitsConverterFactory::FromFFI(capi::UnitsConverterFactory* ptr) {
  return reinterpret_cast<UnitsConverterFactory*>(ptr);
}

inline void UnitsConverterFactory::operator delete(void* ptr) {
  capi::ICU4XUnitsConverterFactory_destroy(reinterpret_cast<capi::UnitsConverterFactory*>(ptr));
}


#endif // UnitsConverterFactory_HPP
