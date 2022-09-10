``collator::ffi``
=================

.. cpp:class:: ICU4XCollator

    See the `Rust documentation for Collator <https://unicode-org.github.io/icu4x-docs/doc/icu/collator/struct.Collator.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCollator, ICU4XError> try_new(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XCollatorOptions options)

        Construct a new Collator instance.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/collator/struct.Collator.html#method.try_new_unstable>`__ for more information.


    .. cpp:function:: ICU4XOrdering compare(const std::string_view left, const std::string_view right) const

        Compare potentially ill-formed UTF-8 strings.

        Ill-formed input is compared as if errors had been replaced with REPLACEMENT CHARACTERs according to the WHATWG Encoding Standard.

        See the `Rust documentation for compare_utf8 <https://unicode-org.github.io/icu4x-docs/doc/icu/collator/struct.Collator.html#method.compare_utf8>`__ for more information.


    .. cpp:function:: ICU4XOrdering compare_valid_utf8(const std::string_view left, const std::string_view right) const

        Compare guaranteed well-formed UTF-8 strings.

        Note: In C++, passing ill-formed UTF-8 strings is undefined behavior (and may be memory-unsafe to do so, too).

        See the `Rust documentation for compare <https://unicode-org.github.io/icu4x-docs/doc/icu/collator/struct.Collator.html#method.compare>`__ for more information.


    .. cpp:function:: ICU4XOrdering compare_utf16(const diplomat::span<uint16_t> left, const diplomat::span<uint16_t> right) const

        Compare potentially ill-formed UTF-16 strings, with unpaired surrogates compared as REPLACEMENT CHARACTER.

        See the `Rust documentation for compare_utf16 <https://unicode-org.github.io/icu4x-docs/doc/icu/collator/struct.Collator.html#method.compare_utf16>`__ for more information.


.. cpp:enum-struct:: ICU4XCollatorAlternateHandling

    See the `Rust documentation for AlternateHandling <https://unicode-org.github.io/icu4x-docs/doc/icu/collator/enum.AlternateHandling.html>`__ for more information.


    .. cpp:enumerator:: Auto

    .. cpp:enumerator:: NonIgnorable

    .. cpp:enumerator:: Shifted

.. cpp:enum-struct:: ICU4XCollatorBackwardSecondLevel

    See the `Rust documentation for BackwardSecondLevel <https://unicode-org.github.io/icu4x-docs/doc/icu/collator/enum.BackwardSecondLevel.html>`__ for more information.


    .. cpp:enumerator:: Auto

    .. cpp:enumerator:: Off

    .. cpp:enumerator:: On

.. cpp:enum-struct:: ICU4XCollatorCaseFirst

    See the `Rust documentation for CaseFirst <https://unicode-org.github.io/icu4x-docs/doc/icu/collator/enum.CaseFirst.html>`__ for more information.


    .. cpp:enumerator:: Auto

    .. cpp:enumerator:: Off

    .. cpp:enumerator:: LowerFirst

    .. cpp:enumerator:: UpperFirst

.. cpp:enum-struct:: ICU4XCollatorCaseLevel

    See the `Rust documentation for CaseLevel <https://unicode-org.github.io/icu4x-docs/doc/icu/collator/enum.CaseLevel.html>`__ for more information.


    .. cpp:enumerator:: Auto

    .. cpp:enumerator:: Off

    .. cpp:enumerator:: On

.. cpp:enum-struct:: ICU4XCollatorMaxVariable

    See the `Rust documentation for MaxVariable <https://unicode-org.github.io/icu4x-docs/doc/icu/collator/enum.MaxVariable.html>`__ for more information.


    .. cpp:enumerator:: Auto

    .. cpp:enumerator:: Space

    .. cpp:enumerator:: Punctuation

    .. cpp:enumerator:: Symbol

    .. cpp:enumerator:: Currency

.. cpp:enum-struct:: ICU4XCollatorNumeric

    See the `Rust documentation for Numeric <https://unicode-org.github.io/icu4x-docs/doc/icu/collator/enum.Numeric.html>`__ for more information.


    .. cpp:enumerator:: Auto

    .. cpp:enumerator:: Off

    .. cpp:enumerator:: On

.. cpp:struct:: ICU4XCollatorOptions

    See the `Rust documentation for CollatorOptions <https://unicode-org.github.io/icu4x-docs/doc/icu/collator/struct.CollatorOptions.html>`__ for more information.


    .. cpp:member:: ICU4XCollatorStrength strength

    .. cpp:member:: ICU4XCollatorAlternateHandling alternate_handling

    .. cpp:member:: ICU4XCollatorCaseFirst case_first

    .. cpp:member:: ICU4XCollatorMaxVariable max_variable

    .. cpp:member:: ICU4XCollatorCaseLevel case_level

    .. cpp:member:: ICU4XCollatorNumeric numeric

    .. cpp:member:: ICU4XCollatorBackwardSecondLevel backward_second_level

.. cpp:enum-struct:: ICU4XCollatorStrength

    See the `Rust documentation for Strength <https://unicode-org.github.io/icu4x-docs/doc/icu/collator/enum.Strength.html>`__ for more information.


    .. cpp:enumerator:: Auto

    .. cpp:enumerator:: Primary

    .. cpp:enumerator:: Secondary

    .. cpp:enumerator:: Tertiary

    .. cpp:enumerator:: Quaternary

    .. cpp:enumerator:: Identical

.. cpp:enum-struct:: ICU4XOrdering

    See the `Rust documentation for Ordering <https://unicode-org.github.io/icu4x-docs/doc/core/cmp/enum.Ordering.html>`__ for more information.


    .. cpp:enumerator:: Less

    .. cpp:enumerator:: Equal

    .. cpp:enumerator:: Greater
