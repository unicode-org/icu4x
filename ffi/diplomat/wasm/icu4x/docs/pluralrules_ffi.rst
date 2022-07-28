``pluralrules::ffi``
====================

.. js:class:: ICU4XPluralCategories

    FFI version of ``PluralRules::categories()`` data.


    .. js:attribute:: zero

    .. js:attribute:: one

    .. js:attribute:: two

    .. js:attribute:: few

    .. js:attribute:: many

    .. js:attribute:: other

.. js:class:: ICU4XPluralCategory

    FFI version of ``PluralCategory``.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/enum.PluralCategory.html>`__ for more information.


.. js:class:: ICU4XPluralOperands

    FFI version of ``PluralOperands``.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralOperands.html>`__ for more information.


    .. js:attribute:: i

    .. js:attribute:: v

    .. js:attribute:: w

    .. js:attribute:: f

    .. js:attribute:: t

    .. js:attribute:: c

    .. js:staticfunction:: create(s)

        FFI version of ``PluralOperands::from_str()``.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralOperands.html#method.from_str>`__ for more information.


.. js:class:: ICU4XPluralRules

    FFI version of ``PluralRules``.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html>`__ for more information.


    .. js:staticfunction:: try_new_cardinal(provider, locale)

        FFI version of ``PluralRules::try_new_cardinal()``.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.try_new>`__ for more information.


    .. js:staticfunction:: try_new_ordinal(provider, locale)

        FFI version of ``PluralRules::try_new_ordinal()``.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.try_new>`__ for more information.


    .. js:function:: select(op)

        FFI version of ``PluralRules::select()``.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.select>`__ for more information.


    .. js:function:: categories()

        FFI version of ``PluralRules::categories()``.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.categories>`__ for more information.

