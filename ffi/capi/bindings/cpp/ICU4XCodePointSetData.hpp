#ifndef ICU4XCodePointSetData_HPP
#define ICU4XCodePointSetData_HPP

#include "ICU4XCodePointSetData.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCodePointRangeIterator.hpp"
#include "ICU4XCodePointSetData.h"
#include "ICU4XDataProvider.hpp"
#include "ICU4XError.hpp"


inline bool ICU4XCodePointSetData::contains(char32_t cp) const {
  auto result = capi::ICU4XCodePointSetData_contains(this->AsFFI(),
    cp);
  return result;
}

inline bool ICU4XCodePointSetData::contains32(uint32_t cp) const {
  auto result = capi::ICU4XCodePointSetData_contains32(this->AsFFI(),
    cp);
  return result;
}

inline std::unique_ptr<ICU4XCodePointRangeIterator> ICU4XCodePointSetData::iter_ranges() const {
  auto result = capi::ICU4XCodePointSetData_iter_ranges(this->AsFFI());
  return std::unique_ptr<ICU4XCodePointRangeIterator>(ICU4XCodePointRangeIterator::FromFFI(result));
}

inline std::unique_ptr<ICU4XCodePointRangeIterator> ICU4XCodePointSetData::iter_ranges_complemented() const {
  auto result = capi::ICU4XCodePointSetData_iter_ranges_complemented(this->AsFFI());
  return std::unique_ptr<ICU4XCodePointRangeIterator>(ICU4XCodePointRangeIterator::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_for_general_category_group(const ICU4XDataProvider& provider, uint32_t group) {
  auto result = capi::ICU4XCodePointSetData_load_for_general_category_group(provider.AsFFI(),
    group);
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_ascii_hex_digit(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_ascii_hex_digit(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_alnum(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_alnum(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_alphabetic(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_alphabetic(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_bidi_control(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_bidi_control(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_bidi_mirrored(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_bidi_mirrored(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_blank(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_blank(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_cased(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_cased(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_case_ignorable(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_case_ignorable(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_full_composition_exclusion(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_full_composition_exclusion(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_changes_when_casefolded(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_changes_when_casefolded(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_changes_when_casemapped(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_changes_when_casemapped(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_changes_when_nfkc_casefolded(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_changes_when_nfkc_casefolded(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_changes_when_lowercased(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_changes_when_lowercased(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_changes_when_titlecased(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_changes_when_titlecased(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_changes_when_uppercased(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_changes_when_uppercased(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_dash(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_dash(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_deprecated(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_deprecated(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_default_ignorable_code_point(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_default_ignorable_code_point(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_diacritic(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_diacritic(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_emoji_modifier_base(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_emoji_modifier_base(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_emoji_component(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_emoji_component(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_emoji_modifier(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_emoji_modifier(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_emoji(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_emoji(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_emoji_presentation(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_emoji_presentation(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_extender(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_extender(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_extended_pictographic(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_extended_pictographic(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_graph(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_graph(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_grapheme_base(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_grapheme_base(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_grapheme_extend(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_grapheme_extend(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_grapheme_link(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_grapheme_link(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_hex_digit(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_hex_digit(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_hyphen(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_hyphen(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_id_continue(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_id_continue(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_ideographic(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_ideographic(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_id_start(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_id_start(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_ids_binary_operator(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_ids_binary_operator(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_ids_trinary_operator(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_ids_trinary_operator(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_join_control(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_join_control(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_logical_order_exception(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_logical_order_exception(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_lowercase(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_lowercase(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_math(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_math(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_noncharacter_code_point(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_noncharacter_code_point(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_nfc_inert(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_nfc_inert(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_nfd_inert(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_nfd_inert(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_nfkc_inert(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_nfkc_inert(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_nfkd_inert(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_nfkd_inert(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_pattern_syntax(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_pattern_syntax(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_pattern_white_space(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_pattern_white_space(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_prepended_concatenation_mark(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_prepended_concatenation_mark(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_print(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_print(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_quotation_mark(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_quotation_mark(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_radical(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_radical(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_regional_indicator(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_regional_indicator(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_soft_dotted(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_soft_dotted(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_segment_starter(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_segment_starter(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_case_sensitive(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_case_sensitive(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_sentence_terminal(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_sentence_terminal(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_terminal_punctuation(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_terminal_punctuation(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_unified_ideograph(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_unified_ideograph(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_uppercase(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_uppercase(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_variation_selector(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_variation_selector(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_white_space(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_white_space(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_xdigit(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_xdigit(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_xid_continue(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_xid_continue(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError> ICU4XCodePointSetData::load_xid_start(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_xid_start(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>, diplomat::Utf8Error> ICU4XCodePointSetData::load_for_ecma262(const ICU4XDataProvider& provider, std::string_view property_name) {
  if (!capi::diplomat_is_str(property_name.data(), property_name.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  auto result = capi::ICU4XCodePointSetData_load_for_ecma262(provider.AsFFI(),
    property_name.data(),
    property_name.size());
  return diplomat::Ok<diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>>(std::move(result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointSetData>>(std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)))));
}

inline const capi::ICU4XCodePointSetData* ICU4XCodePointSetData::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XCodePointSetData*>(this);
}

inline capi::ICU4XCodePointSetData* ICU4XCodePointSetData::AsFFI() {
  return reinterpret_cast<capi::ICU4XCodePointSetData*>(this);
}

inline const ICU4XCodePointSetData* ICU4XCodePointSetData::FromFFI(const capi::ICU4XCodePointSetData* ptr) {
  return reinterpret_cast<const ICU4XCodePointSetData*>(ptr);
}

inline ICU4XCodePointSetData* ICU4XCodePointSetData::FromFFI(capi::ICU4XCodePointSetData* ptr) {
  return reinterpret_cast<ICU4XCodePointSetData*>(ptr);
}

inline void ICU4XCodePointSetData::operator delete(void* ptr) {
  capi::ICU4XCodePointSetData_destroy(reinterpret_cast<capi::ICU4XCodePointSetData*>(ptr));
}


#endif // ICU4XCodePointSetData_HPP
