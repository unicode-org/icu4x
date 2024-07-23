#ifndef Error_HPP
#define Error_HPP

#include "Error.d.hpp"

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
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::Error Error::AsFFI() const {
  return static_cast<diplomat::capi::Error>(value);
}

inline Error Error::FromFFI(diplomat::capi::Error c_enum) {
  switch (c_enum) {
    case diplomat::capi::Error_UnknownError:
    case diplomat::capi::Error_DataMissingDataMarkerError:
    case diplomat::capi::Error_DataMissingLocaleError:
    case diplomat::capi::Error_DataNeedsLocaleError:
    case diplomat::capi::Error_DataExtraneousLocaleError:
    case diplomat::capi::Error_DataFilteredResourceError:
    case diplomat::capi::Error_DataMismatchedTypeError:
    case diplomat::capi::Error_DataCustomError:
    case diplomat::capi::Error_DataIoError:
    case diplomat::capi::Error_DataUnavailableBufferFormatError:
    case diplomat::capi::Error_PropertyUnexpectedPropertyNameError:
    case diplomat::capi::Error_DateTimePatternError:
    case diplomat::capi::Error_DateTimeMissingInputFieldError:
    case diplomat::capi::Error_DateTimeSkeletonError:
    case diplomat::capi::Error_DateTimeUnsupportedFieldError:
    case diplomat::capi::Error_DateTimeUnsupportedOptionsError:
    case diplomat::capi::Error_DateTimeMissingWeekdaySymbolError:
    case diplomat::capi::Error_DateTimeMissingMonthSymbolError:
    case diplomat::capi::Error_DateTimeFixedDecimalError:
    case diplomat::capi::Error_DateTimeMismatchedCalendarError:
      return static_cast<Error::Value>(c_enum);
    default:
      abort();
  }
}
#endif // Error_HPP
