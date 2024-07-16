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
    
    uint16_t ICU4XCodePointMapData16_get(const diplomat::capi::CodePointMapData16* self, char32_t cp);
    
    uint16_t ICU4XCodePointMapData16_get32(const diplomat::capi::CodePointMapData16* self, uint32_t cp);
    
    diplomat::capi::CodePointRangeIterator* ICU4XCodePointMapData16_iter_ranges_for_value(const diplomat::capi::CodePointMapData16* self, uint16_t value);
    
    diplomat::capi::CodePointRangeIterator* ICU4XCodePointMapData16_iter_ranges_for_value_complemented(const diplomat::capi::CodePointMapData16* self, uint16_t value);
    
    diplomat::capi::CodePointSetData* ICU4XCodePointMapData16_get_set_for_value(const diplomat::capi::CodePointMapData16* self, uint16_t value);
    
    typedef struct ICU4XCodePointMapData16_load_script_result {union {diplomat::capi::CodePointMapData16* ok; diplomat::capi::DataError err;}; bool is_ok;} ICU4XCodePointMapData16_load_script_result;
    ICU4XCodePointMapData16_load_script_result ICU4XCodePointMapData16_load_script(const diplomat::capi::DataProvider* provider);
    
    
    void ICU4XCodePointMapData16_destroy(CodePointMapData16* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline uint16_t CodePointMapData16::get(char32_t cp) const {
  auto result = diplomat::capi::ICU4XCodePointMapData16_get(this->AsFFI(),
    cp);
  return result;
}

inline uint16_t CodePointMapData16::get32(uint32_t cp) const {
  auto result = diplomat::capi::ICU4XCodePointMapData16_get32(this->AsFFI(),
    cp);
  return result;
}

inline std::unique_ptr<CodePointRangeIterator> CodePointMapData16::iter_ranges_for_value(uint16_t value) const {
  auto result = diplomat::capi::ICU4XCodePointMapData16_iter_ranges_for_value(this->AsFFI(),
    value);
  return std::unique_ptr<CodePointRangeIterator>(CodePointRangeIterator::FromFFI(result));
}

inline std::unique_ptr<CodePointRangeIterator> CodePointMapData16::iter_ranges_for_value_complemented(uint16_t value) const {
  auto result = diplomat::capi::ICU4XCodePointMapData16_iter_ranges_for_value_complemented(this->AsFFI(),
    value);
  return std::unique_ptr<CodePointRangeIterator>(CodePointRangeIterator::FromFFI(result));
}

inline std::unique_ptr<CodePointSetData> CodePointMapData16::get_set_for_value(uint16_t value) const {
  auto result = diplomat::capi::ICU4XCodePointMapData16_get_set_for_value(this->AsFFI(),
    value);
  return std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<CodePointMapData16>, DataError> CodePointMapData16::load_script(const DataProvider& provider) {
  auto result = diplomat::capi::ICU4XCodePointMapData16_load_script(provider.AsFFI());
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
  diplomat::capi::ICU4XCodePointMapData16_destroy(reinterpret_cast<diplomat::capi::CodePointMapData16*>(ptr));
}


#endif // CodePointMapData16_HPP
