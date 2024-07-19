#ifndef CodePointMapData16_HPP
#define CodePointMapData16_HPP

#include "CodePointMapData16.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CodePointRangeIterator.hpp"
#include "CodePointSetData.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    uint16_t icu4x_CodePointMapData16_get_mv1(const diplomat::capi::CodePointMapData16* self, char32_t cp);
    
    diplomat::capi::CodePointRangeIterator* icu4x_CodePointMapData16_iter_ranges_for_value_mv1(const diplomat::capi::CodePointMapData16* self, uint16_t value);
    
    diplomat::capi::CodePointRangeIterator* icu4x_CodePointMapData16_iter_ranges_for_value_complemented_mv1(const diplomat::capi::CodePointMapData16* self, uint16_t value);
    
    diplomat::capi::CodePointSetData* icu4x_CodePointMapData16_get_set_for_value_mv1(const diplomat::capi::CodePointMapData16* self, uint16_t value);
    
    typedef struct icu4x_CodePointMapData16_load_script_mv1_result {union {diplomat::capi::CodePointMapData16* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointMapData16_load_script_mv1_result;
    icu4x_CodePointMapData16_load_script_mv1_result icu4x_CodePointMapData16_load_script_mv1(const diplomat::capi::DataProvider* provider);
    
    
    void icu4x_CodePointMapData16_destroy_mv1(CodePointMapData16* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline uint16_t CodePointMapData16::get(char32_t cp) const {
  auto result = diplomat::capi::icu4x_CodePointMapData16_get_mv1(this->AsFFI(),
    cp);
  return result;
}

inline std::unique_ptr<CodePointRangeIterator> CodePointMapData16::iter_ranges_for_value(uint16_t value) const {
  auto result = diplomat::capi::icu4x_CodePointMapData16_iter_ranges_for_value_mv1(this->AsFFI(),
    value);
  return std::unique_ptr<CodePointRangeIterator>(CodePointRangeIterator::FromFFI(result));
}

inline std::unique_ptr<CodePointRangeIterator> CodePointMapData16::iter_ranges_for_value_complemented(uint16_t value) const {
  auto result = diplomat::capi::icu4x_CodePointMapData16_iter_ranges_for_value_complemented_mv1(this->AsFFI(),
    value);
  return std::unique_ptr<CodePointRangeIterator>(CodePointRangeIterator::FromFFI(result));
}

inline std::unique_ptr<CodePointSetData> CodePointMapData16::get_set_for_value(uint16_t value) const {
  auto result = diplomat::capi::icu4x_CodePointMapData16_get_set_for_value_mv1(this->AsFFI(),
    value);
  return std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<CodePointMapData16>, DataError> CodePointMapData16::load_script(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointMapData16_load_script_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointMapData16>, DataError>(diplomat::Ok<std::unique_ptr<CodePointMapData16>>(std::unique_ptr<CodePointMapData16>(CodePointMapData16::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointMapData16>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline const diplomat::capi::CodePointMapData16* CodePointMapData16::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::CodePointMapData16*>(this);
}

inline diplomat::capi::CodePointMapData16* CodePointMapData16::AsFFI() {
  return reinterpret_cast<diplomat::capi::CodePointMapData16*>(this);
}

inline const CodePointMapData16* CodePointMapData16::FromFFI(const diplomat::capi::CodePointMapData16* ptr) {
  return reinterpret_cast<const CodePointMapData16*>(ptr);
}

inline CodePointMapData16* CodePointMapData16::FromFFI(diplomat::capi::CodePointMapData16* ptr) {
  return reinterpret_cast<CodePointMapData16*>(ptr);
}

inline void CodePointMapData16::operator delete(void* ptr) {
  diplomat::capi::icu4x_CodePointMapData16_destroy_mv1(reinterpret_cast<diplomat::capi::CodePointMapData16*>(ptr));
}


#endif // CodePointMapData16_HPP
