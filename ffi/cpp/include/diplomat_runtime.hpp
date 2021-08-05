#ifndef DIPLOMAT_RUNTIME_CPP_H
#define DIPLOMAT_RUNTIME_CPP_H

#include <string>
#include <variant>

namespace capi {
#include "diplomat_runtime.h"
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

template<typename T> struct WriteableTrait {
  // static inline capi::DiplomatWriteable Construct(T& t);
};


template<> struct WriteableTrait<std::string> {
  static inline capi::DiplomatWriteable Construct(std::string& t) {
    return diplomat::WriteableFromString(t);
  }
};

template<class T, class E>
struct result
{
  union {
    T ok;
    E err;
  };
  bool is_ok;

  ~result() {
    if (is_ok) {
      ok.~T();
    } else {
      err.~E();
    }
  }

  static result<T, E> new_ok(T x) {
    return {
      .ok = x,
      .is_ok = true
    };
  }

  static result<std::monostate, E> new_ok_void() {
    return {
      .is_ok = true
    };
  }

  static result<T, E> new_err(E x) {
    return {
      .err = x,
      .is_ok = false
    };
  }

  static result<T, std::monostate> new_err_void() {
    return {
      .is_ok = false
    };
  }
};

}

#endif
