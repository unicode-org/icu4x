``normalizer::ffi``
===================

.. js:class:: ICU4XComposingNormalizer

    See the `Rust documentation for ComposingNormalizer <https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html>`__ for more information.


    .. js:function:: create_nfc(provider)

        Construct a new ICU4XComposingNormalizer instance for NFC

        See the `Rust documentation for try_new_nfc_unstable <https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html#method.try_new_nfc_unstable>`__ for more information.


    .. js:function:: create_nfkc(provider)

        Construct a new ICU4XComposingNormalizer instance for NFKC

        See the `Rust documentation for try_new_nfkc_unstable <https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html#method.try_new_nfkc_unstable>`__ for more information.


    .. js:method:: normalize(s)

        Normalize a (potentially ill-formed) UTF8 string

        Errors are mapped to REPLACEMENT CHARACTER

        See the `Rust documentation for normalize_utf8 <https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html#method.normalize_utf8>`__ for more information.


    .. js:method:: is_normalized(s)

        Check if a (potentially ill-formed) UTF8 string is normalized

        Errors are mapped to REPLACEMENT CHARACTER

        See the `Rust documentation for is_normalized_utf8 <https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html#method.is_normalized_utf8>`__ for more information.


.. js:class:: ICU4XDecomposingNormalizer

    See the `Rust documentation for DecomposingNormalizer <https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html>`__ for more information.


    .. js:function:: create_nfd(provider)

        Construct a new ICU4XDecomposingNormalizer instance for NFC

        See the `Rust documentation for try_new_nfd_unstable <https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.try_new_nfd_unstable>`__ for more information.


    .. js:function:: create_nfkd(provider)

        Construct a new ICU4XDecomposingNormalizer instance for NFKC

        See the `Rust documentation for try_new_nfkd_unstable <https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.try_new_nfkd_unstable>`__ for more information.


    .. js:method:: normalize(s)

        Normalize a (potentially ill-formed) UTF8 string

        Errors are mapped to REPLACEMENT CHARACTER

        See the `Rust documentation for normalize_utf8 <https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.normalize_utf8>`__ for more information.


    .. js:method:: is_normalized(s)

        Check if a (potentially ill-formed) UTF8 string is normalized

        Errors are mapped to REPLACEMENT CHARACTER

        See the `Rust documentation for is_normalized_utf8 <https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.is_normalized_utf8>`__ for more information.

