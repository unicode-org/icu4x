..
    This file is part of ICU4X. For terms of use, please see the file
    called LICENSE at the top level of the ICU4X source tree
    (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

Data providers
==============

This file details the data provider-handling utilities exposed by ``<provider.hpp>``.

.. cpp:class:: icu4x::DataProvider

    An ICU4X data provider, capable of loading ICU4X data keys from some source. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_provider/prelude/trait.DataProvider.html>`_ for more information.

    .. cpp:function:: static std::optional<DataProvider> DataProvider::FsDataProvider(const std::string_view& path)

        Construct `an FsDataProvider <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_fs/struct.FsDataProvider.html>`_ from a source file path.
