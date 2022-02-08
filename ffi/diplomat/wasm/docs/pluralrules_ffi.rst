``pluralrules::ffi``
====================

.. js:class:: ICU4XCreatePluralOperandsResult

    This is the result returned by ``ICU4XPluralOperands::create()`` See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralOperands.html>`__ for more details.

    .. js:attribute:: operands

    .. js:attribute:: success

.. js:class:: ICU4XCreatePluralRulesResult

    .. js:attribute:: rules

    .. js:attribute:: success

.. js:class:: ICU4XPluralCategories

    FFI version of ``PluralRules::categories()`` data.

    .. js:attribute:: zero

    .. js:attribute:: one

    .. js:attribute:: two

    .. js:attribute:: few

    .. js:attribute:: many

    .. js:attribute:: other

.. js:class:: ICU4XPluralCategory

    FFI version of ``PluralCategory``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/enum.PluralCategory.html>`__ for more details.

.. js:class:: ICU4XPluralOperands

    FFI version of ``PluralOperands``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralOperands.html>`__ for more details.

    .. js:attribute:: i

    .. js:attribute:: v

    .. js:attribute:: w

    .. js:attribute:: f

    .. js:attribute:: t

    .. js:attribute:: c

    .. js:staticfunction:: create(s)

        FFI version of ``PluralOperands::from_str()``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralOperands.html#method.from_str>`__ for more details.

.. js:class:: ICU4XPluralRules

    FFI version of ``PluralRules``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html>`__ for more details.

    .. js:staticfunction:: try_new_cardinal(locale, provider)

        FFI version of ``PluralRules::try_new_cardinal()``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.try_new>`__ for more details.

    .. js:staticfunction:: try_new_ordinal(locale, provider)

        FFI version of ``PluralRules::try_new_ordinal()``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.try_new>`__ for more details.

    .. js:function:: select(op)

        FFI version of ``PluralRules::select()``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.select>`__ for more details.

    .. js:function:: categories()

        FFI version of ``PluralRules::categories()``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.categories>`__ for more details.
