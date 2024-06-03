#ifndef ICU4XError_HPP
#define ICU4XError_HPP

#include "ICU4XError.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XError.h"


inline capi::ICU4XError ICU4XError::AsFFI() const {
  return static_cast<capi::ICU4XError>(value);
}

inline ICU4XError ICU4XError::FromFFI(capi::ICU4XError c_enum) {
  switch (c_enum) {
    case capi::ICU4XError_UnknownError:
    case capi::ICU4XError_DataMissingDataMarkerError:
    case capi::ICU4XError_DataMissingVariantError:
    case capi::ICU4XError_DataMissingLocaleError:
    case capi::ICU4XError_DataNeedsVariantError:
    case capi::ICU4XError_DataNeedsLocaleError:
    case capi::ICU4XError_DataExtraneousLocaleError:
    case capi::ICU4XError_DataFilteredResourceError:
    case capi::ICU4XError_DataMismatchedTypeError:
    case capi::ICU4XError_DataMissingPayloadError:
    case capi::ICU4XError_DataInvalidStateError:
    case capi::ICU4XError_DataCustomError:
    case capi::ICU4XError_DataIoError:
    case capi::ICU4XError_DataUnavailableBufferFormatError:
    case capi::ICU4XError_DataMismatchedAnyBufferError:
    case capi::ICU4XError_LocaleParserLanguageError:
    case capi::ICU4XError_LocaleParserSubtagError:
    case capi::ICU4XError_LocaleParserExtensionError:
    case capi::ICU4XError_DataStructValidityError:
    case capi::ICU4XError_PropertyUnknownScriptIdError:
    case capi::ICU4XError_PropertyUnknownGeneralCategoryGroupError:
    case capi::ICU4XError_PropertyUnexpectedPropertyNameError:
    case capi::ICU4XError_FixedDecimalLimitError:
    case capi::ICU4XError_FixedDecimalSyntaxError:
    case capi::ICU4XError_PluralsParserError:
    case capi::ICU4XError_CalendarParseError:
    case capi::ICU4XError_CalendarOverflowError:
    case capi::ICU4XError_CalendarUnderflowError:
    case capi::ICU4XError_CalendarOutOfRangeError:
    case capi::ICU4XError_CalendarUnknownEraError:
    case capi::ICU4XError_CalendarUnknownMonthCodeError:
    case capi::ICU4XError_CalendarMissingInputError:
    case capi::ICU4XError_CalendarUnknownKindError:
    case capi::ICU4XError_CalendarMissingError:
    case capi::ICU4XError_DateTimePatternError:
    case capi::ICU4XError_DateTimeMissingInputFieldError:
    case capi::ICU4XError_DateTimeSkeletonError:
    case capi::ICU4XError_DateTimeUnsupportedFieldError:
    case capi::ICU4XError_DateTimeUnsupportedOptionsError:
    case capi::ICU4XError_DateTimeMissingWeekdaySymbolError:
    case capi::ICU4XError_DateTimeMissingMonthSymbolError:
    case capi::ICU4XError_DateTimeFixedDecimalError:
    case capi::ICU4XError_DateTimeMismatchedCalendarError:
    case capi::ICU4XError_TinyStrTooLargeError:
    case capi::ICU4XError_TinyStrContainsNullError:
    case capi::ICU4XError_TinyStrNonAsciiError:
    case capi::ICU4XError_TimeZoneOffsetOutOfBoundsError:
    case capi::ICU4XError_TimeZoneInvalidOffsetError:
    case capi::ICU4XError_TimeZoneInvalidIdError:
    case capi::ICU4XError_NormalizerFutureExtensionError:
    case capi::ICU4XError_NormalizerValidationError:
      return static_cast<ICU4XError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XError_HPP
