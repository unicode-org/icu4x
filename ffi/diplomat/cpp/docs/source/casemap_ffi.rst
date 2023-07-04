``casemap::ffi``
================

.. cpp:class:: ICU4XCaseMapper

    See the `Rust documentation for CaseMapper <https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCaseMapper, ICU4XError> create(const ICU4XDataProvider& provider)

        Construct a new ICU4XCaseMapper instance for NFC

        See the `Rust documentation for try_new_nfc_unstable <https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.try_new_nfc_unstable>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> lowercase_to_writeable(const std::string_view s, const ICU4XLocale& locale, W& write) const

        Returns the full lowercase mapping of the given string

        See the `Rust documentation for lowercase <https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.lowercase>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> lowercase(const std::string_view s, const ICU4XLocale& locale) const

        Returns the full lowercase mapping of the given string

        See the `Rust documentation for lowercase <https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.lowercase>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> uppercase_to_writeable(const std::string_view s, const ICU4XLocale& locale, W& write) const

        Returns the full uppercase mapping of the given string

        See the `Rust documentation for uppercase <https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.uppercase>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> uppercase(const std::string_view s, const ICU4XLocale& locale) const

        Returns the full uppercase mapping of the given string

        See the `Rust documentation for uppercase <https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.uppercase>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> titlecase_segment_to_writeable(const std::string_view s, const ICU4XLocale& locale, W& write) const

        Returns the full titlecase mapping of the given string

        See the `Rust documentation for titlecase_segment <https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.titlecase_segment>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> titlecase_segment(const std::string_view s, const ICU4XLocale& locale) const

        Returns the full titlecase mapping of the given string

        See the `Rust documentation for titlecase_segment <https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.titlecase_segment>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> fold_to_writeable(const std::string_view s, W& write) const

        Case-folds the characters in the given string

        See the `Rust documentation for fold <https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.fold>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> fold(const std::string_view s) const

        Case-folds the characters in the given string

        See the `Rust documentation for fold <https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.fold>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> fold_turkic_to_writeable(const std::string_view s, W& write) const

        Case-folds the characters in the given string using Turkic (T) mappings for dotted/dotless I.

        See the `Rust documentation for fold_turkic <https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.fold_turkic>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> fold_turkic(const std::string_view s) const

        Case-folds the characters in the given string using Turkic (T) mappings for dotted/dotless I.

        See the `Rust documentation for fold_turkic <https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.fold_turkic>`__ for more information.

