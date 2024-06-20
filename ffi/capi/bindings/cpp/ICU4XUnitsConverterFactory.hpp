#ifndef ICU4XUnitsConverterFactory_HPP
#define ICU4XUnitsConverterFactory_HPP

#include "ICU4XUnitsConverterFactory.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XMeasureUnit.hpp"
#include "ICU4XMeasureUnitParser.hpp"
#include "ICU4XUnitsConverter.hpp"


namespace capi {
    extern "C" {
    
    struct ICU4XUnitsConverterFactory_create_result {union {ICU4XUnitsConverterFactory* ok; ICU4XDataError err;}; bool is_ok;};
    struct ICU4XUnitsConverterFactory_create_result ICU4XUnitsConverterFactory_create(const ICU4XDataProvider* provider);
    
    ICU4XUnitsConverter* ICU4XUnitsConverterFactory_converter(const ICU4XUnitsConverterFactory* self, const ICU4XMeasureUnit* from, const ICU4XMeasureUnit* to);
    
    ICU4XMeasureUnitParser* ICU4XUnitsConverterFactory_parser(const ICU4XUnitsConverterFactory* self);
    
    
    void ICU4XUnitsConverterFactory_destroy(ICU4XUnitsConverterFactory* self);
    
    } // extern "C"
}

inline diplomat::result<std::unique_ptr<ICU4XUnitsConverterFactory>, ICU4XDataError> ICU4XUnitsConverterFactory::create(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XUnitsConverterFactory_create(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XUnitsConverterFactory>, ICU4XDataError>(diplomat::Ok<std::unique_ptr<ICU4XUnitsConverterFactory>>(std::unique_ptr<ICU4XUnitsConverterFactory>(ICU4XUnitsConverterFactory::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XUnitsConverterFactory>, ICU4XDataError>(diplomat::Err<ICU4XDataError>(ICU4XDataError::FromFFI(result.err)));
}

inline std::unique_ptr<ICU4XUnitsConverter> ICU4XUnitsConverterFactory::converter(const ICU4XMeasureUnit& from, const ICU4XMeasureUnit& to) const {
  auto result = capi::ICU4XUnitsConverterFactory_converter(this->AsFFI(),
    from.AsFFI(),
    to.AsFFI());
  return std::unique_ptr<ICU4XUnitsConverter>(ICU4XUnitsConverter::FromFFI(result));
}

inline std::unique_ptr<ICU4XMeasureUnitParser> ICU4XUnitsConverterFactory::parser() const {
  auto result = capi::ICU4XUnitsConverterFactory_parser(this->AsFFI());
  return std::unique_ptr<ICU4XMeasureUnitParser>(ICU4XMeasureUnitParser::FromFFI(result));
}

inline const capi::ICU4XUnitsConverterFactory* ICU4XUnitsConverterFactory::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XUnitsConverterFactory*>(this);
}

inline capi::ICU4XUnitsConverterFactory* ICU4XUnitsConverterFactory::AsFFI() {
  return reinterpret_cast<capi::ICU4XUnitsConverterFactory*>(this);
}

inline const ICU4XUnitsConverterFactory* ICU4XUnitsConverterFactory::FromFFI(const capi::ICU4XUnitsConverterFactory* ptr) {
  return reinterpret_cast<const ICU4XUnitsConverterFactory*>(ptr);
}

inline ICU4XUnitsConverterFactory* ICU4XUnitsConverterFactory::FromFFI(capi::ICU4XUnitsConverterFactory* ptr) {
  return reinterpret_cast<ICU4XUnitsConverterFactory*>(ptr);
}

inline void ICU4XUnitsConverterFactory::operator delete(void* ptr) {
  capi::ICU4XUnitsConverterFactory_destroy(reinterpret_cast<capi::ICU4XUnitsConverterFactory*>(ptr));
}


#endif // ICU4XUnitsConverterFactory_HPP
