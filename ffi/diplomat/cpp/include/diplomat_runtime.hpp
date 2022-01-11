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

template<class T> struct Ok {
  T inner;
  Ok(T&& i): inner(std::move(i)) {}
  Ok() = default;
  Ok(Ok&&) noexcept = default;
  Ok(const Ok &) = default;
  Ok& operator=(const Ok&) = default;
  Ok& operator=(Ok&&) noexcept = default;
};

template<class T> struct Err {
  T inner;
  Err(T&& i): inner(std::move(i)) {}
  Err() = default;
  Err(Err&&) noexcept = default;
  Err(const Err &) = default;
  Err& operator=(const Err&) = default;
  Err& operator=(Err&&) noexcept = default;
};

template<class T, class E>
class result {
private:
    std::variant<Ok<T>, Err<E>> val;
public:
  result(Ok<T>&& v): val(std::move(v)) {}
  result(Err<E>&& v): val(std::move(v)) {}
  result() = default;
  result(const result &) = default;
  result& operator=(const result&) = default;
  result& operator=(result&&) noexcept = default;
  result(result &&) noexcept = default;
  ~result() = default;
  bool is_ok() const {
    return std::holds_alternative<Ok<T>>(this->val);
  };
  bool is_err() const {
    return std::holds_alternative<Err<E>>(this->val);
  };

  std::optional<T> ok() && {
    if (!this->is_ok()) {
      return std::nullopt;
    }
    return std::make_optional(std::move(std::get<Ok<T>>(std::move(this->val)).inner));
  };
  std::optional<E> err() && {
    if (!this->is_err()) {
      return std::nullopt;
    }
    return std::make_optional(std::move(std::get<Err<E>>(std::move(this->val)).inner));
  }

  void set_ok(T&& t) {
    this->val = Ok<T>(std::move(t));
  }

  void set_err(E&& e) {
    this->val = Err<E>(std::move(e));
  }

  template<typename T2>
  result<T2, E> replace_ok(T2&& t) {
    if (this->is_err()) {
      return result<T2, E>(Err<E>(std::get<Err<E>>(std::move(this->val))));
    } else {
      return result<T2, E>(Ok<T2>(std::move(t)));
    }
  }
};


// Use custom std::span on C++17, otherwise use std::span
#if __cplusplus >= 202002L

#include<span>
using span = std::span;

#else // __cplusplus >= 202002L

// C++-17-compatible std::span
template<class T>
class span {

public:
  constexpr span(T* data, size_t size)
    : data_(data), size_(size) {}
  template<size_t N>
  constexpr span(std::array<T, N>& arr)
    : data_(arr.data()), size_(N) {}
  constexpr T* data() const noexcept {
    return this->data_;
  }
  constexpr size_t size() const noexcept {
    return this->size_;
  }
private:
  T* data_;
  size_t size_;
};

#endif // __cplusplus >= 202002L

}

#endif
