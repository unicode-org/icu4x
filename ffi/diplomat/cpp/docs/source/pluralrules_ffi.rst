``pluralrules::ffi``
====================

.. cpp:struct:: ICU4XCreatePluralOperandsResult

    This is the result returned by ``ICU4XPluralOperands::create()`` See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralOperands.html>`__ for more details.

    .. cpp:member:: ICU4XPluralOperands operands

    .. cpp:member:: bool success

.. cpp:struct:: ICU4XCreatePluralRulesResult

    .. cpp:member:: std::optional<ICU4XPluralRules> rules

    .. cpp:member:: bool success

.. cpp:struct:: ICU4XPluralCategories

    FFI version of ``PluralRules::categories()`` data.

    .. cpp:member:: bool zero

    .. cpp:member:: bool one

    .. cpp:member:: bool two

    .. cpp:member:: bool few

    .. cpp:member:: bool many

    .. cpp:member:: bool other

.. cpp:enum-struct:: ICU4XPluralCategory

    FFI version of ``PluralCategory``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/enum.PluralCategory.html>`__ for more details.

    .. cpp:enumerator:: Zero

    .. cpp:enumerator:: One

    .. cpp:enumerator:: Two

    .. cpp:enumerator:: Few

    .. cpp:enumerator:: Many

    .. cpp:enumerator:: Other

.. cpp:struct:: ICU4XPluralOperands

    FFI version of ``PluralOperands``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralOperands.html>`__ for more details.

    .. cpp:member:: uint64_t i

    .. cpp:member:: size_t v

    .. cpp:member:: size_t w

    .. cpp:member:: uint64_t f

    .. cpp:member:: uint64_t t

    .. cpp:member:: size_t c

    .. cpp:function:: static ICU4XCreatePluralOperandsResult create(const std::string_view s)

        FFI version of ``PluralOperands::from_str()``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralOperands.html#method.from_str>`__ for more details.

.. cpp:enum-struct:: ICU4XPluralRuleType

    FFI version of ``PluralRuleType``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/enum.PluralRuleType.html>`__ for more details.

    .. cpp:enumerator:: Cardinal

    .. cpp:enumerator:: Ordinal

.. cpp:class:: ICU4XPluralRules

    FFI version of ``PluralRules``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html>`__ for more details.

    .. cpp:function:: static ICU4XCreatePluralRulesResult try_new(const ICU4XLocale& locale, const ICU4XDataProvider& provider, ICU4XPluralRuleType ty)

        FFI version of ``PluralRules::try_new()``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.try_new>`__ for more details.

    .. cpp:function:: ICU4XPluralCategory select(ICU4XPluralOperands op) const

        FFI version of ``PluralRules::select()``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.select>`__ for more details.

    .. cpp:function:: ICU4XPluralCategories categories() const

        FFI version of ``PluralRules::categories()``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/struct.PluralRules.html#method.categories>`__ for more details.
