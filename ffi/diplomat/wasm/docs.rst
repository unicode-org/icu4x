..
    This file is part of ICU4X. For terms of use, please see the file
    called LICENSE at the top level of the ICU4X source tree
    (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

.. js:class:: ICU4XCanonicalizationResult


    FFI version of ``CanonicalizationResult``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locale_canonicalizer/enum.CanonicalizationResult.html>`__ for more details.

.. js:class:: ICU4XCreateDataProviderResult


    A result type for ``ICU4XDataProvider::create``.

    .. js:attribute:: provider


        Will be ``None`` if ``success`` is ``false``, do not use in that case.

    .. js:attribute:: success



.. js:class:: ICU4XCreateFixedDecimalFormatDataProviderResult


    A result type for ``ICU4XDataProvider::create``.

    .. js:attribute:: provider


        Will be ``None`` if ``success`` is ``false``, do not use in that case.

    .. js:attribute:: success



.. js:class:: ICU4XCreateFixedDecimalResult



    .. js:attribute:: fd


        Will be None if ``success`` is ``false``

    .. js:attribute:: success


        Currently just a boolean, but we might add a proper error enum as necessary

.. js:class:: ICU4XCreatePluralOperandsResult


    This is the result returned by ``ICU4XPluralOperands::create()`` See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralOperands.html>`__ for more details.

    .. js:attribute:: operands



    .. js:attribute:: success



.. js:class:: ICU4XCreatePluralRulesResult



    .. js:attribute:: rules



    .. js:attribute:: success



.. js:class:: ICU4XDataProvider


    An ICU4X data provider, capable of loading ICU4X data keys from some source. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_provider/prelude/trait.DataProvider.html>`__ for more information.

    .. js:staticfunction:: create_fs(path)

        Constructs an ``FsDataProvider`` and retirns it as an :js:class:`ICU4XDataProvider`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_fs/struct.FsDataProvider.html>`__ for more details.

    .. js:staticfunction:: create_static()

        Constructs an ``StaticDataProvider`` and retirns it as an :js:class:`ICU4XDataProvider`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.StaticDataProvider.html>`__ for more details.

.. js:class:: ICU4XFixedDecimal


    A decimal number. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.

    .. js:staticfunction:: create(v)

        Construct an :js:class:`ICU4XFixedDecimal` from an integer. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.

    .. js:staticfunction:: create_fromstr(v)

        Construct an :js:class:`ICU4XFixedDecimal` from a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html>`__ for more information.

    .. js:function:: multiply_pow10(power)

        Multiply the :js:class:`ICU4XFixedDecimal` by a given power of ten. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.multiply_pow10>`__ for more information.

    .. js:function:: negate()

        Invert the sign of the :js:class:`ICU4XFixedDecimal`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.negate>`__ for more information.

    .. js:function:: to_string()

        Format the :js:class:`ICU4XFixedDecimal` as a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/decimal/struct.FixedDecimal.html#method.write_to>`__ for more information.

.. js:class:: ICU4XFixedDecimalFormat


    An ICU4X Fixed Decimal Format object, capable of formatting a :js:class:`ICU4XFixedDecimal` as a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html>`__ for more information.

    .. js:staticfunction:: try_new(locale, provider, options)

        Creates a new :js:class:`ICU4XFixedDecimalFormat` from locale data. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.try_new>`__ for more information.

    .. js:staticfunction:: try_new_specific(locale, provider, options)

        Creates a new :js:class:`ICU4XFixedDecimalFormat` from a data provider specific to FixedDecimalFormat. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.try_new>`__ for more information.

    .. js:function:: format(value)

        Formats a :js:class:`ICU4XFixedDecimal` to a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/decimal/struct.FixedDecimalFormat.html#method.format>`__ for more information.

.. js:class:: ICU4XFixedDecimalFormatDataProvider


    A DataProvider specific to FixedDecimalFormat.

    .. js:staticfunction:: create_static()

        Create a DataProvider reading from static data specific to FixedDecimalFormat.

.. js:class:: ICU4XFixedDecimalFormatOptions



    .. js:attribute:: grouping_strategy



    .. js:attribute:: sign_display



    .. js:staticfunction:: default()


.. js:class:: ICU4XFixedDecimalFormatResult



    .. js:attribute:: fdf


        The :js:class:`ICU4XFixedDecimalFormat`, exists if creation was successful.

    .. js:attribute:: success


        Whether creating the :js:class:`ICU4XFixedDecimalFormat` was successful.

.. js:class:: ICU4XFixedDecimalGroupingStrategy



.. js:class:: ICU4XFixedDecimalSignDisplay



.. js:class:: ICU4XLocale


    An ICU4X Locale, capable of representing strings like ``"en-US"``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html>`__ for more information.

    .. js:staticfunction:: create(name)

        Construct an :js:class:`ICU4XLocale` from an locale identifier. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.from_bytes>`__ for more information.

    .. js:staticfunction:: create_en()

        Construct an :js:class:`ICU4XLocale` for the English language.

    .. js:staticfunction:: create_bn()

        Construct an :js:class:`ICU4XLocale` for the Bangla language.

    .. js:function:: clone()

        Clones the :js:class:`ICU4XLocale`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html>`__ for more information.

    .. js:function:: basename()

        Write a string representation of the ``LanguageIdentifier`` part of :js:class:`ICU4XLocale` to ``write``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.

    .. js:function:: get_unicode_extension(bytes)

        Write a string representation of the unicode extension to ``write`` See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.extensions>`__ for more information.

    .. js:function:: language()

        Write a string representation of :js:class:`ICU4XLocale` language to ``write`` See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.

    .. js:function:: region()

        Write a string representation of :js:class:`ICU4XLocale` region to ``write`` See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.

    .. js:function:: script()

        Write a string representation of :js:class:`ICU4XLocale` script to ``write`` See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.

    .. js:function:: tostring()

        Write a string representation of :js:class:`ICU4XLocale` to ``write`` See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html>`__ for more information.

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

.. js:class:: ICU4XLocaleError



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

.. js:class:: ICU4XPluralRuleType


    FFI version of ``PluralRuleType``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/enum.PluralRuleType.html>`__ for more details.

.. js:class:: ICU4XPluralRules


    FFI version of ``PluralRules``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html>`__ for more details.

    .. js:staticfunction:: create(locale, provider, ty)

        FFI version of ``PluralRules::try_new()``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.try_new>`__ for more details.

    .. js:function:: select(op)

        FFI version of ``PluralRules::select()``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.select>`__ for more details.

    .. js:function:: categories()

        FFI version of ``PluralRules::categories()``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.categories>`__ for more details.
