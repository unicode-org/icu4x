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


namespace diplomat {
namespace capi {
    extern "C" {
    
    bool icu4x_Logger_init_simple_logger_mv1(void);
    
    
    void icu4x_Logger_destroy_mv1(Logger* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline bool Logger::init_simple_logger() {
  auto result = diplomat::capi::icu4x_Logger_init_simple_logger_mv1();
  return result;
}

inline const diplomat::capi::Logger* Logger::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::Logger*>(this);
}

inline diplomat::capi::Logger* Logger::AsFFI() {
  return reinterpret_cast<diplomat::capi::Logger*>(this);
}

inline const Logger* Logger::FromFFI(const diplomat::capi::Logger* ptr) {
  return reinterpret_cast<const Logger*>(ptr);
}

inline Logger* Logger::FromFFI(diplomat::capi::Logger* ptr) {
  return reinterpret_cast<Logger*>(ptr);
}

inline void Logger::operator delete(void* ptr) {
  diplomat::capi::icu4x_Logger_destroy_mv1(reinterpret_cast<diplomat::capi::Logger*>(ptr));
}


#endif // Logger_HPP
