``casemap::ffi``
================

.. js:class:: ICU4XCaseMapper

    See the `Rust documentation for CaseMapper <https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html>`__ for more information.


    .. js:function:: create(provider)

        Construct a new ICU4XCaseMapper instance for NFC

        See the `Rust documentation for try_new_nfc_unstable <https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.try_new_nfc_unstable>`__ for more information.


    .. js:method:: lowercase(s, locale)

        Returns the full lowercase mapping of the given string

        See the `Rust documentation for lowercase <https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.lowercase>`__ for more information.


    .. js:method:: uppercase(s, locale)

        Returns the full uppercase mapping of the given string

        See the `Rust documentation for uppercase <https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.uppercase>`__ for more information.


    .. js:method:: titlecase_segment(s, locale)

        Returns the full titlecase mapping of the given string

        See the `Rust documentation for titlecase_segment <https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.titlecase_segment>`__ for more information.


    .. js:method:: fold(s)

        Case-folds the characters in the given string

        See the `Rust documentation for fold <https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.fold>`__ for more information.


    .. js:method:: fold_turkic(s)

        Case-folds the characters in the given string using Turkic (T) mappings for dotted/dotless I.

        See the `Rust documentation for fold_turkic <https://docs.rs/icu/latest/icu/casemap/struct.ComposingNormalizer.html#method.fold_turkic>`__ for more information.

