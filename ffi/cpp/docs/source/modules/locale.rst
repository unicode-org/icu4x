..
    This file is part of ICU4X. For terms of use, please see the file
    called LICENSE at the top level of the ICU4X source tree
    (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

Locale
======

This file details the locale-handling utilities exposed by ``<locale.hpp>``.

.. cpp:class:: icu4x::Locale

    An ICU4X Locale, capable of representing strings like ``"en-US"``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html>`_ for more information.

    .. cpp:function:: Locale::Locale(const std::string_view& value)

        Construct a locale from a string. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.from_bytes>`_ for more information

    .. cpp:function:: std::optional<std::string> Locale::ToString() const

        Convert the locale to a string, returning an error if string conversion failed.