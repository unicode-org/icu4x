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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::Error Error::AsFFI() const {
  return static_cast<capi::Error>(value);
}

inline Error Error::FromFFI(capi::Error c_enum) {
  switch (c_enum) {
    case capi::Error_UnknownError:
    case capi::Error_DataMissingDataMarkerError:
    case capi::Error_DataMissingLocaleError:
    case capi::Error_DataNeedsLocaleError:
    case capi::Error_DataExtraneousLocaleError:
    case capi::Error_DataFilteredResourceError:
    case capi::Error_DataMismatchedTypeError:
    case capi::Error_DataCustomError:
    case capi::Error_DataIoError:
    case capi::Error_DataUnavailableBufferFormatError:
    case capi::Error_PropertyUnexpectedPropertyNameError:
    case capi::Error_DateTimePatternError:
    case capi::Error_DateTimeMissingInputFieldError:
    case capi::Error_DateTimeSkeletonError:
    case capi::Error_DateTimeUnsupportedFieldError:
    case capi::Error_DateTimeUnsupportedOptionsError:
    case capi::Error_DateTimeMissingWeekdaySymbolError:
    case capi::Error_DateTimeMissingMonthSymbolError:
    case capi::Error_DateTimeFixedDecimalError:
    case capi::Error_DateTimeMismatchedCalendarError:
      return static_cast<Error::Value>(c_enum);
    default:
      abort();
  }
}
#endif // Error_HPP
