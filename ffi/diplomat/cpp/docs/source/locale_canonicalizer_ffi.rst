``locale_canonicalizer::ffi``
=============================

.. cpp:enum-struct:: ICU4XCanonicalizationResult

    FFI version of ``CanonicalizationResult``.
    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/enum.CanonicalizationResult.html>`__ for more information.

    .. cpp:enumerator:: Modified

    .. cpp:enumerator:: Unmodified

.. cpp:class:: ICU4XLocaleCanonicalizer

    A locale canonicalizer.
    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html>`__ for more information.

    .. cpp:function:: static std::optional<ICU4XLocaleCanonicalizer> create(const ICU4XDataProvider& provider)

        Create a new :cpp:class:`ICU4XLocaleCanonicalizer`.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.new>`__ for more information.

    .. cpp:function:: ICU4XCanonicalizationResult canonicalize(ICU4XLocale& locale) const

        FFI version of ``LocaleCanonicalizer::canonicalize()``.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.canonicalize>`__ for more information.

    .. cpp:function:: ICU4XCanonicalizationResult maximize(ICU4XLocale& locale) const

        FFI version of ``LocaleCanonicalizer::maximize()``.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.maximize>`__ for more information.

    .. cpp:function:: ICU4XCanonicalizationResult minimize(ICU4XLocale& locale) const

        FFI version of ``LocaleCanonicalizer::minimize()``.
        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/struct.LocaleCanonicalizer.html#method.minimize>`__ for more information.
