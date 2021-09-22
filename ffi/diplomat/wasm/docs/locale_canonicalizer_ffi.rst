``locale_canonicalizer::ffi``
=============================

.. js:class:: ICU4XCanonicalizationResult

    FFI version of ``CanonicalizationResult``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/enum.CanonicalizationResult.html>`__ for more details.

.. js:class:: ICU4XLocaleCanonicalizer

    A locale canonicalizer. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html>`__ for more details.

    .. js:staticfunction:: create(provider)

        Create a new :js:class:`ICU4XLocaleCanonicalizer`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.new>`__ for more details.

    .. js:function:: canonicalize(locale)

        FFI version of ``LocaleCanonicalizer::canonicalize()``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.canonicalize>`__ for more details.

    .. js:function:: maximize(locale)

        FFI version of ``LocaleCanonicalizer::maximize()``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.maximize>`__ for more details.

    .. js:function:: minimize(locale)

        FFI version of ``LocaleCanonicalizer::minimize()``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.minimize>`__ for more details.
