``locale_canonicalizer::ffi``
=============================

.. js:class:: ICU4XCanonicalizationResult

    FFI version of ``CanonicalizationResult``.
    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/enum.CanonicalizationResult.html>`__ for more information.

.. js:class:: ICU4XLocaleCanonicalizer

    A locale canonicalizer.
    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html>`__ for more information.

    .. js:staticfunction:: create(provider)

        Create a new :js:class:`ICU4XLocaleCanonicalizer`.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.new>`__ for more information.

    .. js:function:: canonicalize(locale)

        FFI version of ``LocaleCanonicalizer::canonicalize()``.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.canonicalize>`__ for more information.

    .. js:function:: maximize(locale)

        FFI version of ``LocaleCanonicalizer::maximize()``.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.maximize>`__ for more information.

    .. js:function:: minimize(locale)

        FFI version of ``LocaleCanonicalizer::minimize()``.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.minimize>`__ for more information.
