#ifndef PropertyValueNameToEnumMapper_D_HPP
#define PropertyValueNameToEnumMapper_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct DataProvider; }
class DataProvider;
class DataError;


namespace diplomat {
namespace capi {
    struct PropertyValueNameToEnumMapper;
} // namespace capi
} // namespace

class PropertyValueNameToEnumMapper {
public:

  inline int16_t get_strict(std::string_view name) const;

  inline int16_t get_loose(std::string_view name) const;

  inline static diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> load_general_category(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> load_hangul_syllable_type(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> load_east_asian_width(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> load_bidi_class(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> load_indic_syllabic_category(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> load_line_break(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> load_grapheme_cluster_break(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> load_word_break(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> load_sentence_break(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<PropertyValueNameToEnumMapper>, DataError> load_script(const DataProvider& provider);

  inline const diplomat::capi::PropertyValueNameToEnumMapper* AsFFI() const;
  inline diplomat::capi::PropertyValueNameToEnumMapper* AsFFI();
  inline static const PropertyValueNameToEnumMapper* FromFFI(const diplomat::capi::PropertyValueNameToEnumMapper* ptr);
  inline static PropertyValueNameToEnumMapper* FromFFI(diplomat::capi::PropertyValueNameToEnumMapper* ptr);
  inline static void operator delete(void* ptr);
private:
  PropertyValueNameToEnumMapper() = delete;
  PropertyValueNameToEnumMapper(const PropertyValueNameToEnumMapper&) = delete;
  PropertyValueNameToEnumMapper(PropertyValueNameToEnumMapper&&) noexcept = delete;
  PropertyValueNameToEnumMapper operator=(const PropertyValueNameToEnumMapper&) = delete;
  PropertyValueNameToEnumMapper operator=(PropertyValueNameToEnumMapper&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // PropertyValueNameToEnumMapper_D_HPP
