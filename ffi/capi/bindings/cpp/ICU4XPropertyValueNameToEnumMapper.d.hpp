#ifndef ICU4XPropertyValueNameToEnumMapper_D_HPP
#define ICU4XPropertyValueNameToEnumMapper_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDataError.d.hpp"

class ICU4XDataProvider;
class ICU4XDataError;


namespace capi {
    typedef struct ICU4XPropertyValueNameToEnumMapper ICU4XPropertyValueNameToEnumMapper;
}

class ICU4XPropertyValueNameToEnumMapper {
public:

  inline int16_t get_strict(std::string_view name) const;

  inline int16_t get_loose(std::string_view name) const;

  inline static diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError> load_general_category(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError> load_hangul_syllable_type(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError> load_east_asian_width(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError> load_bidi_class(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError> load_indic_syllabic_category(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError> load_line_break(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError> load_grapheme_cluster_break(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError> load_word_break(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError> load_sentence_break(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XPropertyValueNameToEnumMapper>, ICU4XDataError> load_script(const ICU4XDataProvider& provider);

  inline const capi::ICU4XPropertyValueNameToEnumMapper* AsFFI() const;
  inline capi::ICU4XPropertyValueNameToEnumMapper* AsFFI();
  inline static const ICU4XPropertyValueNameToEnumMapper* FromFFI(const capi::ICU4XPropertyValueNameToEnumMapper* ptr);
  inline static ICU4XPropertyValueNameToEnumMapper* FromFFI(capi::ICU4XPropertyValueNameToEnumMapper* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XPropertyValueNameToEnumMapper() = delete;
  ICU4XPropertyValueNameToEnumMapper(const ICU4XPropertyValueNameToEnumMapper&) = delete;
  ICU4XPropertyValueNameToEnumMapper(ICU4XPropertyValueNameToEnumMapper&&) noexcept = delete;
  ICU4XPropertyValueNameToEnumMapper operator=(const ICU4XPropertyValueNameToEnumMapper&) = delete;
  ICU4XPropertyValueNameToEnumMapper operator=(ICU4XPropertyValueNameToEnumMapper&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XPropertyValueNameToEnumMapper_D_HPP
