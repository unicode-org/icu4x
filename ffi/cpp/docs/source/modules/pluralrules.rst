..
    This file is part of ICU4X. For terms of use, please see the file
    called LICENSE at the top level of the ICU4X source tree
    (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

Plural Rules
============

This file details the plural rules functionality exposed by ``<pluralrules.hpp>``.

.. cpp:class:: icu4x::PluralRules

    An ICU4X Plural Rules object, capable of retrieving the appropriate :cpp:enum:`PluralRuleType` for a number represented as :cpp:struct:`PluralOperands`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/struct.PluralRules.html>`_ for more information.

    .. cpp:function:: static std::optional<PluralRules> PluralRules::Create(const Locale& locale, const DataProvider& provider, PluralRuleType ty)

        Construct a :cpp:class:`PluralRules`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/struct.PluralRules.html#method.try_new>`_ for more information.

    .. cpp:function:: PluralCategory PluralRules::Select(PluralOperands& op)

        Retrieve the appropriate :cpp:enum:`PluralRuleType` for a number represented as :cpp:struct:`PluralOperands`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/struct.PluralRules.html#method.select>`_ for more information.

.. cpp:enum-class:: icu4x::PluralRuleType

    The type of plural rules being requested. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/enum.PluralRuleType.html>`_ for more information.

    .. cpp:enumerator:: icu4x::PluralRuleType::Cardinal
    .. cpp:enumerator:: icu4x::PluralRuleType::Ordinal

.. cpp:enum-class:: icu4x::PluralCategory

    The type of plural rules being requested. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/enum.PluralCategory.html>`_ for more information.

    .. cpp:enumerator:: icu4x::PluralCategory::Zero
    .. cpp:enumerator:: icu4x::PluralCategory::One
    .. cpp:enumerator:: icu4x::PluralCategory::Two
    .. cpp:enumerator:: icu4x::PluralCategory::Few
    .. cpp:enumerator:: icu4x::PluralCategory::Many
    .. cpp:enumerator:: icu4x::PluralCategory::Other

.. cpp:struct:: icu4x::PluralOperands

    A full plural operands representation of a number. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/plurals/struct.PluralOperands.html>`_ for more information.

    .. cpp:member:: uint64_t i
    .. cpp:member:: size_t v
    .. cpp:member:: size_t w
    .. cpp:member:: uint64_t f
    .. cpp:member:: uint64_t t
    .. cpp:member:: size_t c
