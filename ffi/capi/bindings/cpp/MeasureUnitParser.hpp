#ifndef MeasureUnitParser_HPP
#define MeasureUnitParser_HPP

#include "MeasureUnitParser.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "MeasureUnit.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::MeasureUnit* icu4x_MeasureUnitParser_parse_mv1(const diplomat::capi::MeasureUnitParser* self, const char* unit_id_data, size_t unit_id_len);
    
    
    void icu4x_MeasureUnitParser_destroy_mv1(MeasureUnitParser* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline std::unique_ptr<MeasureUnit> MeasureUnitParser::parse(std::string_view unit_id) const {
  auto result = diplomat::capi::icu4x_MeasureUnitParser_parse_mv1(this->AsFFI(),
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
  diplomat::capi::icu4x_MeasureUnitParser_destroy_mv1(reinterpret_cast<diplomat::capi::MeasureUnitParser*>(ptr));
}


#endif // MeasureUnitParser_HPP
