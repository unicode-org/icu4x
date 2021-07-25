#include <string>

namespace capi {
#include "api.h"
}

namespace diplomat {

extern "C" inline void Flush(capi::DiplomatWriteable* w) {
  std::string* string = reinterpret_cast<std::string*>(w->context);
  string->resize(w->len);
};

extern "C" inline bool Grow(capi::DiplomatWriteable* w, uintptr_t requested) {
  std::string* string = reinterpret_cast<std::string*>(w->context);
  string->resize(requested);
  w->cap = string->length();
  w->buf = &(*string)[0];
  return true;
};

inline capi::DiplomatWriteable WriteableFromString(std::string& string) {
  capi::DiplomatWriteable w;
  w.context = &string;
  w.buf = &string[0];
  w.len = string.length();
  // Same as length, since C++ strings are not supposed
  // to be written to past their len; you resize *first*
  w.cap = string.length();
  w.flush = Flush;
  w.grow = Grow;
  return w;
};

}
