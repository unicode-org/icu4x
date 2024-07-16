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


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::MeasureUnit* ICU4XMeasureUnitParser_parse(const diplomat::capi::MeasureUnitParser* self, const char* unit_id_data, size_t unit_id_len);
    
    
    void ICU4XMeasureUnitParser_destroy(MeasureUnitParser* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<MeasureUnit> MeasureUnitParser::parse(std::string_view unit_id) const {
  auto result = diplomat::capi::ICU4XMeasureUnitParser_parse(this->AsFFI(),
    unit_id.data(),
    unit_id.size());
  return std::unique_ptr<MeasureUnit>(MeasureUnit::FromFFI(result));
}

inline const diplomat::capi::MeasureUnitParser* MeasureUnitParser::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::MeasureUnitParser*>(this);
}

inline diplomat::capi::MeasureUnitParser* MeasureUnitParser::AsFFI() {
  return reinterpret_cast<diplomat::capi::MeasureUnitParser*>(this);
}

inline const MeasureUnitParser* MeasureUnitParser::FromFFI(const diplomat::capi::MeasureUnitParser* ptr) {
  return reinterpret_cast<const MeasureUnitParser*>(ptr);
}

inline MeasureUnitParser* MeasureUnitParser::FromFFI(diplomat::capi::MeasureUnitParser* ptr) {
  return reinterpret_cast<MeasureUnitParser*>(ptr);
}

inline void MeasureUnitParser::operator delete(void* ptr) {
  diplomat::capi::ICU4XMeasureUnitParser_destroy(reinterpret_cast<diplomat::capi::MeasureUnitParser*>(ptr));
}


#endif // MeasureUnitParser_HPP
