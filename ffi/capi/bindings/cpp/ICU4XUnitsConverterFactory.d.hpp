#ifndef ICU4XUnitsConverterFactory_D_HPP
#define ICU4XUnitsConverterFactory_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.d.hpp"
#include "ICU4XUnitsConverterFactory.d.h"

class ICU4XDataProvider;
class ICU4XMeasureUnit;
class ICU4XMeasureUnitParser;
class ICU4XUnitsConverter;
class ICU4XDataError;


class ICU4XUnitsConverterFactory {
public:

  inline static diplomat::result<std::unique_ptr<ICU4XUnitsConverterFactory>, ICU4XDataError> create(const ICU4XDataProvider& provider);

  inline std::unique_ptr<ICU4XUnitsConverter> converter(const ICU4XMeasureUnit& from, const ICU4XMeasureUnit& to) const;

  inline std::unique_ptr<ICU4XMeasureUnitParser> parser() const;

  inline const capi::ICU4XUnitsConverterFactory* AsFFI() const;
  inline capi::ICU4XUnitsConverterFactory* AsFFI();
  inline static const ICU4XUnitsConverterFactory* FromFFI(const capi::ICU4XUnitsConverterFactory* ptr);
  inline static ICU4XUnitsConverterFactory* FromFFI(capi::ICU4XUnitsConverterFactory* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XUnitsConverterFactory() = delete;
  ICU4XUnitsConverterFactory(const ICU4XUnitsConverterFactory&) = delete;
  ICU4XUnitsConverterFactory(ICU4XUnitsConverterFactory&&) noexcept = delete;
  ICU4XUnitsConverterFactory operator=(const ICU4XUnitsConverterFactory&) = delete;
  ICU4XUnitsConverterFactory operator=(ICU4XUnitsConverterFactory&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XUnitsConverterFactory_D_HPP
