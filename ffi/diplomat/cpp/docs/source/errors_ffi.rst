``errors::ffi``
===============

.. cpp:enum-struct:: ICU4XError

    A common enum for errors that ICU4X may return, organized by API
    The error names are stable and can be checked against as strings in the JS API

    .. cpp:enumerator:: UnknownError

        The error is not currently categorized as ICU4XError. Please file a bug

    .. cpp:enumerator:: WriteableError

        An error arising from writing to a string Typically found when not enough space is allocated

    .. cpp:enumerator:: DataMissingResourceKeyError

    .. cpp:enumerator:: DataMissingVariantError

    .. cpp:enumerator:: DataMissingLocaleError

    .. cpp:enumerator:: DataMissingResourceOptionsError

    .. cpp:enumerator:: DataNeedsVariantError

    .. cpp:enumerator:: DataNeedsLocaleError

    .. cpp:enumerator:: DataExtraneousResourceOptionsError

    .. cpp:enumerator:: DataFilteredResourceError

    .. cpp:enumerator:: DataMismatchedTypeError

    .. cpp:enumerator:: DataMissingPayloadError

    .. cpp:enumerator:: DataInvalidStateError

    .. cpp:enumerator:: DataCustomError

    .. cpp:enumerator:: DataIoError

    .. cpp:enumerator:: DataUnavailableBufferFormatError

    .. cpp:enumerator:: LocaleUndefinedSubtagError

        The subtag being requested was not set

    .. cpp:enumerator:: LocaleParserError

        The locale or subtag string failed to parse

    .. cpp:enumerator:: DataStructValidityError

        Attempted to construct an invalid data struct

    .. cpp:enumerator:: PropertyUnknownScriptIdError

    .. cpp:enumerator:: PropertyUnknownGeneralCategoryGroupError
