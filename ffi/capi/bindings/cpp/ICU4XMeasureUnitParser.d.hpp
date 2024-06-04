#ifndef ICU4XMeasureUnitParser_D_HPP
#define ICU4XMeasureUnitParser_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XMeasureUnitParser.d.h"

class ICU4XMeasureUnit;


class ICU4XMeasureUnitParser {
public:

  inline std::unique_ptr<ICU4XMeasureUnit> parse(std::string_view unit_id) const;

  inline const capi::ICU4XMeasureUnitParser* AsFFI() const;
  inline capi::ICU4XMeasureUnitParser* AsFFI();
  inline static const ICU4XMeasureUnitParser* FromFFI(const capi::ICU4XMeasureUnitParser* ptr);
  inline static ICU4XMeasureUnitParser* FromFFI(capi::ICU4XMeasureUnitParser* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XMeasureUnitParser() = delete;
  ICU4XMeasureUnitParser(const ICU4XMeasureUnitParser&) = delete;
  ICU4XMeasureUnitParser(ICU4XMeasureUnitParser&&) noexcept = delete;
  ICU4XMeasureUnitParser operator=(const ICU4XMeasureUnitParser&) = delete;
  ICU4XMeasureUnitParser operator=(ICU4XMeasureUnitParser&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XMeasureUnitParser_D_HPP
