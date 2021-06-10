// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_WRITEABLE_UTILS_HPP
#define ICU4X_WRITEABLE_UTILS_HPP

#include <string>

#include "../../capi/include/custom_writeable.h"

namespace icu4x::internal {

extern "C" inline void Flush(ICU4XWriteable* w) {
  std::string* string = reinterpret_cast<std::string*>(w->context);
  string->resize(w->len);
};

extern "C" inline bool Grow(ICU4XWriteable* w, uintptr_t requested) {
  std::string* string = reinterpret_cast<std::string*>(w->context);
  string->resize(requested);
  w->cap = string->length();
  w->buf = string->data();
  return true;
};

inline ICU4XWriteable WriteableFromString(std::string& string) {
  ICU4XWriteable w;
  w.context = &string;
  w.buf = string.data();
  w.len = string.length();
  // Same as length, since C++ strings are not supposed
  // to be written to past their len; you resize *first*
  w.cap = string.length();
  w.flush = Flush;
  w.grow = Grow;
  return w;
};

}  // namespace icu4x::internal

#endif  // ICU4X_WRITEABLE_UTILS_HPP
