``provider::ffi``
=================

.. cpp:struct:: ICU4XCreateDataProviderResult

    A result type for ``ICU4XDataProvider::create``.

    .. cpp:member:: std::optional<ICU4XDataProvider> provider

        Will be ``None`` if ``success`` is ``false``, do not use in that case.

    .. cpp:member:: bool success

.. cpp:struct:: ICU4XCreateStaticDataProviderResult

    A result type for ``ICU4XStaticDataProvider::create``.

    .. cpp:member:: std::optional<ICU4XStaticDataProvider> provider

        Will be ``None`` if ``success`` is ``false``, do not use in that case.

    .. cpp:member:: bool success

.. cpp:class:: ICU4XDataProvider

    An ICU4X data provider, capable of loading ICU4X data keys from some source. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_provider/prelude/trait.DataProvider.html>`__ for more information.

    .. cpp:function:: static ICU4XCreateDataProviderResult create_fs(const std::string_view path)

        Constructs an ``FsDataProvider`` and returns it as an :cpp:class:`ICU4XDataProvider`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_fs/struct.FsDataProvider.html>`__ for more details.

    .. cpp:function:: static ICU4XCreateDataProviderResult create_static()

        Constructs an ``StaticDataProvider`` and returns it as an :cpp:class:`ICU4XDataProvider`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.StaticDataProvider.html>`__ for more details.

    .. cpp:function:: static ICU4XCreateDataProviderResult create_from_byte_slice(const std::span<uint8_t> blob)

        Constructs a ``BlobDataProvider`` and returns it as an :cpp:class:`ICU4XDataProvider`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.BlobDataProvider.html>`__ for more details.

.. cpp:class:: ICU4XStaticDataProvider

    An ICU4X data provider backed by static data. This is a specialization of :cpp:class:`ICU4XDataProvider` intended to reduce code size.
    See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.StaticDataProvider.html>`__ for more information.

    .. cpp:function:: static ICU4XCreateStaticDataProviderResult create()

        Constructs an ``StaticDataProvider`` and returns it as an :cpp:class:`ICU4XStaticDataProvider`.
        See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.StaticDataProvider.html>`__ for more details.
