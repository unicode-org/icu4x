#ifndef ICU4XMeasureUnitParser_HPP
#define ICU4XMeasureUnitParser_HPP

#include "ICU4XMeasureUnitParser.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XMeasureUnit.hpp"


namespace capi {
    extern "C" {
    
    ICU4XMeasureUnit* ICU4XMeasureUnitParser_parse(const ICU4XMeasureUnitParser* self, const char* unit_id_data, size_t unit_id_len);
    
    
    void ICU4XMeasureUnitParser_destroy(ICU4XMeasureUnitParser* self);
    
    } // extern "C"
}

inline std::unique_ptr<ICU4XMeasureUnit> ICU4XMeasureUnitParser::parse(std::string_view unit_id) const {
  auto result = capi::ICU4XMeasureUnitParser_parse(this->AsFFI(),
    unit_id.data(),
    unit_id.size());
  return std::unique_ptr<ICU4XMeasureUnit>(ICU4XMeasureUnit::FromFFI(result));
}

inline const capi::ICU4XMeasureUnitParser* ICU4XMeasureUnitParser::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XMeasureUnitParser*>(this);
}

inline capi::ICU4XMeasureUnitParser* ICU4XMeasureUnitParser::AsFFI() {
  return reinterpret_cast<capi::ICU4XMeasureUnitParser*>(this);
}

inline const ICU4XMeasureUnitParser* ICU4XMeasureUnitParser::FromFFI(const capi::ICU4XMeasureUnitParser* ptr) {
  return reinterpret_cast<const ICU4XMeasureUnitParser*>(ptr);
}

inline ICU4XMeasureUnitParser* ICU4XMeasureUnitParser::FromFFI(capi::ICU4XMeasureUnitParser* ptr) {
  return reinterpret_cast<ICU4XMeasureUnitParser*>(ptr);
}

inline void ICU4XMeasureUnitParser::operator delete(void* ptr) {
  capi::ICU4XMeasureUnitParser_destroy(reinterpret_cast<capi::ICU4XMeasureUnitParser*>(ptr));
}


#endif // ICU4XMeasureUnitParser_HPP
