#ifndef MeasureUnitParser_HPP
#define MeasureUnitParser_HPP

#include "MeasureUnitParser.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "MeasureUnit.hpp"


namespace capi {
    extern "C" {
    
    MeasureUnit* ICU4XMeasureUnitParser_parse(const MeasureUnitParser* self, const char* unit_id_data, size_t unit_id_len);
    
    
    void ICU4XMeasureUnitParser_destroy(MeasureUnitParser* self);
    
    } // extern "C"
}

inline std::unique_ptr<MeasureUnit> MeasureUnitParser::parse(std::string_view unit_id) const {
  auto result = capi::ICU4XMeasureUnitParser_parse(this->AsFFI(),
    unit_id.data(),
    unit_id.size());
  return std::unique_ptr<MeasureUnit>(MeasureUnit::FromFFI(result));
}

inline const capi::MeasureUnitParser* MeasureUnitParser::AsFFI() const {
  return reinterpret_cast<const capi::MeasureUnitParser*>(this);
}

inline capi::MeasureUnitParser* MeasureUnitParser::AsFFI() {
  return reinterpret_cast<capi::MeasureUnitParser*>(this);
}

inline const MeasureUnitParser* MeasureUnitParser::FromFFI(const capi::MeasureUnitParser* ptr) {
  return reinterpret_cast<const MeasureUnitParser*>(ptr);
}

inline MeasureUnitParser* MeasureUnitParser::FromFFI(capi::MeasureUnitParser* ptr) {
  return reinterpret_cast<MeasureUnitParser*>(ptr);
}

inline void MeasureUnitParser::operator delete(void* ptr) {
  capi::ICU4XMeasureUnitParser_destroy(reinterpret_cast<capi::MeasureUnitParser*>(ptr));
}


#endif // MeasureUnitParser_HPP
