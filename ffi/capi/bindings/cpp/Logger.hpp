#ifndef Logger_HPP
#define Logger_HPP

#include "Logger.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    extern "C" {
    
    bool ICU4XLogger_init_simple_logger();
    
    
    void ICU4XLogger_destroy(Logger* self);
    
    } // extern "C"
}

inline bool Logger::init_simple_logger() {
  auto result = capi::ICU4XLogger_init_simple_logger();
  return result;
}

inline const capi::Logger* Logger::AsFFI() const {
  return reinterpret_cast<const capi::Logger*>(this);
}

inline capi::Logger* Logger::AsFFI() {
  return reinterpret_cast<capi::Logger*>(this);
}

inline const Logger* Logger::FromFFI(const capi::Logger* ptr) {
  return reinterpret_cast<const Logger*>(ptr);
}

inline Logger* Logger::FromFFI(capi::Logger* ptr) {
  return reinterpret_cast<Logger*>(ptr);
}

inline void Logger::operator delete(void* ptr) {
  capi::ICU4XLogger_destroy(reinterpret_cast<capi::Logger*>(ptr));
}


#endif // Logger_HPP
