#ifndef ICU4XLogger_HPP
#define ICU4XLogger_HPP

#include "ICU4XLogger.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLogger.h"


inline bool ICU4XLogger::init_simple_logger() {
  auto result = capi::ICU4XLogger_init_simple_logger();
  return result;
}

inline const capi::ICU4XLogger* ICU4XLogger::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XLogger*>(this);
}

inline capi::ICU4XLogger* ICU4XLogger::AsFFI() {
  return reinterpret_cast<capi::ICU4XLogger*>(this);
}

inline const ICU4XLogger* ICU4XLogger::FromFFI(const capi::ICU4XLogger* ptr) {
  return reinterpret_cast<const ICU4XLogger*>(ptr);
}

inline ICU4XLogger* ICU4XLogger::FromFFI(capi::ICU4XLogger* ptr) {
  return reinterpret_cast<ICU4XLogger*>(ptr);
}

inline void ICU4XLogger::operator delete(void* ptr) {
  capi::ICU4XLogger_destroy(reinterpret_cast<capi::ICU4XLogger*>(ptr));
}


#endif // ICU4XLogger_HPP
