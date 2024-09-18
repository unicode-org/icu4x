#ifndef icu4x_Error_HPP
#define icu4x_Error_HPP

#include "Error.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::capi::Error icu4x::Error::AsFFI() const {
  return static_cast<icu4x::capi::Error>(value);
}

inline icu4x::Error icu4x::Error::FromFFI(icu4x::capi::Error c_enum) {
  switch (c_enum) {
    case icu4x::capi::Error_UnknownError:
    case icu4x::capi::Error_DataMissingDataMarkerError:
    case icu4x::capi::Error_DataMissingLocaleError:
    case icu4x::capi::Error_DataNeedsLocaleError:
    case icu4x::capi::Error_DataExtraneousLocaleError:
    case icu4x::capi::Error_DataFilteredResourceError:
    case icu4x::capi::Error_DataMismatchedTypeError:
    case icu4x::capi::Error_DataCustomError:
    case icu4x::capi::Error_DataIoError:
    case icu4x::capi::Error_DataUnavailableBufferFormatError:
    case icu4x::capi::Error_PropertyUnexpectedPropertyNameError:
    case icu4x::capi::Error_DateTimePatternError:
    case icu4x::capi::Error_DateTimeMissingInputFieldError:
    case icu4x::capi::Error_DateTimeSkeletonError:
    case icu4x::capi::Error_DateTimeUnsupportedFieldError:
    case icu4x::capi::Error_DateTimeUnsupportedOptionsError:
    case icu4x::capi::Error_DateTimeMissingWeekdaySymbolError:
    case icu4x::capi::Error_DateTimeMissingMonthSymbolError:
    case icu4x::capi::Error_DateTimeFixedDecimalError:
    case icu4x::capi::Error_DateTimeMismatchedCalendarError:
    case icu4x::capi::Error_DateTimeDuplicateFieldError:
    case icu4x::capi::Error_DateTimeTooNarrowError:
    case icu4x::capi::Error_DateTimeMissingNamesError:
      return static_cast<icu4x::Error::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_Error_HPP
