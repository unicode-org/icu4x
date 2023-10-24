#ifndef ICU4XLogger_HPP
#define ICU4XLogger_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XLogger.h"


/**
 * A destruction policy for using ICU4XLogger with std::unique_ptr.
 */
struct ICU4XLoggerDeleter {
  void operator()(capi::ICU4XLogger* l) const noexcept {
    capi::ICU4XLogger_destroy(l);
  }
};

/**
 * An object allowing control over the logging used
 */
class ICU4XLogger {
 public:

  /**
   * Initialize the logger using `simple_logger`
   * 
   * Requires the `simple_logger` Cargo feature.
   * 
   * Returns `false` if there was already a logger set.
   */
  static bool init_simple_logger();

  /**
   * Deprecated: since ICU4X 1.4, this now happens automatically if the `log` feature is enabled.
   */
  static bool init_console_logger();
  inline const capi::ICU4XLogger* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XLogger* AsFFIMut() { return this->inner.get(); }
  inline ICU4XLogger(capi::ICU4XLogger* i) : inner(i) {}
  ICU4XLogger() = default;
  ICU4XLogger(ICU4XLogger&&) noexcept = default;
  ICU4XLogger& operator=(ICU4XLogger&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XLogger, ICU4XLoggerDeleter> inner;
};


inline bool ICU4XLogger::init_simple_logger() {
  return capi::ICU4XLogger_init_simple_logger();
}
inline bool ICU4XLogger::init_console_logger() {
  return capi::ICU4XLogger_init_console_logger();
}
#endif
