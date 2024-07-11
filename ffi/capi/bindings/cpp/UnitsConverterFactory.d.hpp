#ifndef UnitsConverterFactory_D_HPP
#define UnitsConverterFactory_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "DataError.d.hpp"

class DataProvider;
class MeasureUnit;
class MeasureUnitParser;
class UnitsConverter;
class DataError;


namespace capi {
    typedef struct UnitsConverterFactory UnitsConverterFactory;
}

class UnitsConverterFactory {
public:

  inline static diplomat::result<std::unique_ptr<UnitsConverterFactory>, DataError> create(const DataProvider& provider);

  inline std::unique_ptr<UnitsConverter> converter(const MeasureUnit& from, const MeasureUnit& to) const;

  inline std::unique_ptr<MeasureUnitParser> parser() const;

  inline const capi::UnitsConverterFactory* AsFFI() const;
  inline capi::UnitsConverterFactory* AsFFI();
  inline static const UnitsConverterFactory* FromFFI(const capi::UnitsConverterFactory* ptr);
  inline static UnitsConverterFactory* FromFFI(capi::UnitsConverterFactory* ptr);
  inline static void operator delete(void* ptr);
private:
  UnitsConverterFactory() = delete;
  UnitsConverterFactory(const UnitsConverterFactory&) = delete;
  UnitsConverterFactory(UnitsConverterFactory&&) noexcept = delete;
  UnitsConverterFactory operator=(const UnitsConverterFactory&) = delete;
  UnitsConverterFactory operator=(UnitsConverterFactory&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // UnitsConverterFactory_D_HPP
