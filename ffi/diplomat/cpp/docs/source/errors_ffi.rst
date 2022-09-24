``errors::ffi``
===============

.. cpp:enum-struct:: ICU4XError

    A common enum for errors that ICU4X may return, organized by API

    The error names are stable and can be checked against as strings in the JS API

    Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/enum.Error.html>`__, `2 <https://unicode-org.github.io/icu4x-docs/doc/icu/calendar/enum.DateTimeError.html>`__, `3 <https://unicode-org.github.io/icu4x-docs/doc/icu/datetime/enum.DateTimeFormatterError.html>`__, `4 <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/enum.ParserError.html>`__, `5 <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/enum.PropertiesError.html>`__, `6 <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/enum.PluralRulesError.html>`__, `7 <https://unicode-org.github.io/icu4x-docs/doc/icu/provider/struct.DataError.html>`__, `8 <https://unicode-org.github.io/icu4x-docs/doc/icu/provider/enum.DataErrorKind.html>`__, `9 <https://unicode-org.github.io/icu4x-docs/doc/icu/normalizer/enum.NormalizerError.html>`__, `10 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/enum.TimeZoneError.html>`__, `11 <https://unicode-org.github.io/icu4x-docs/doc/icu/collator/enum.CollatorError.html>`__, `12 <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/enum.FixedDecimalFormatterError.html>`__


    .. cpp:enumerator:: UnknownError

        The error is not currently categorized as ICU4XError. Please file a bug


    .. cpp:enumerator:: WriteableError

        An error arising from writing to a string Typically found when not enough space is allocated Most APIs that return a string may return this error


    .. cpp:enumerator:: OutOfBoundsError

    .. cpp:enumerator:: DataMissingDataKeyError

    .. cpp:enumerator:: DataMissingVariantError

    .. cpp:enumerator:: DataMissingLocaleError

    .. cpp:enumerator:: DataNeedsVariantError

    .. cpp:enumerator:: DataNeedsLocaleError

    .. cpp:enumerator:: DataExtraneousLocaleError

    .. cpp:enumerator:: DataFilteredResourceError

    .. cpp:enumerator:: DataMismatchedTypeError

    .. cpp:enumerator:: DataMissingPayloadError

    .. cpp:enumerator:: DataInvalidStateError

    .. cpp:enumerator:: DataCustomError

    .. cpp:enumerator:: DataIoError

    .. cpp:enumerator:: DataUnavailableBufferFormatError

    .. cpp:enumerator:: DataMismatchedAnyBufferError

    .. cpp:enumerator:: LocaleUndefinedSubtagError

        The subtag being requested was not set


    .. cpp:enumerator:: LocaleParserLanguageError

        The locale or subtag string failed to parse


    .. cpp:enumerator:: LocaleParserSubtagError

    .. cpp:enumerator:: LocaleParserExtensionError

    .. cpp:enumerator:: DataStructValidityError

        Attempted to construct an invalid data struct


    .. cpp:enumerator:: PropertyUnknownScriptIdError

    .. cpp:enumerator:: PropertyUnknownGeneralCategoryGroupError

    .. cpp:enumerator:: FixedDecimalLimitError

    .. cpp:enumerator:: FixedDecimalSyntaxError

    .. cpp:enumerator:: PluralParserError

    .. cpp:enumerator:: DateTimeParseError

    .. cpp:enumerator:: DateTimeOverflowError

    .. cpp:enumerator:: DateTimeUnderflowError

    .. cpp:enumerator:: DateTimeOutOfRangeError

    .. cpp:enumerator:: DateTimeUnknownEraError

    .. cpp:enumerator:: DateTimeUnknownMonthCodeError

    .. cpp:enumerator:: DateTimeMissingInputError

    .. cpp:enumerator:: DateTimeUnknownAnyCalendarKindError

    .. cpp:enumerator:: DateTimeFormatPatternError

    .. cpp:enumerator:: DateTimeFormatMissingInputFieldError

    .. cpp:enumerator:: DateTimeFormatSkeletonError

    .. cpp:enumerator:: DateTimeFormatUnsupportedFieldError

    .. cpp:enumerator:: DateTimeFormatUnsupportedOptionsError

    .. cpp:enumerator:: DateTimeFormatMissingWeekdaySymbolError

    .. cpp:enumerator:: DateTimeFormatMissingMonthSymbolError

    .. cpp:enumerator:: DateTimeFormatFixedDecimalError

    .. cpp:enumerator:: DateTimeFormatMismatchedAnyCalendarError

    .. cpp:enumerator:: TinyStrTooLargeError

    .. cpp:enumerator:: TinyStrContainsNullError

    .. cpp:enumerator:: TinyStrNonAsciiError

    .. cpp:enumerator:: TimeZoneOffsetOutOfBoundsError

    .. cpp:enumerator:: TimeZoneInvalidOffsetError

    .. cpp:enumerator:: TimeZoneMissingInputError

    .. cpp:enumerator:: NormalizerFutureExtensionError

    .. cpp:enumerator:: NormalizerValidationError
