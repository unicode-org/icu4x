``provider::ffi``
=================

.. cpp:struct:: ICU4XCreateDataProviderResult

    A result type for ``ICU4XDataProvider::create``.


    .. cpp:member:: std::optional<ICU4XDataProvider> provider

        Will be ``None`` if ``success`` is ``false``, do not use in that case.


    .. cpp:member:: bool success

.. cpp:class:: ICU4XDataProvider

    An ICU4X data provider, capable of loading ICU4X data keys from some source.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_provider/index.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XDataProvider, ICU4XError> create_fs(const std::string_view path)

        Constructs an ``FsDataProvider`` and returns it as an :cpp:class:`ICU4XDataProvider`. Requires the ``provider_fs`` feature. Not supported in WASM.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_fs/struct.FsDataProvider.html>`__ for more information.


    .. cpp:function:: static ICU4XDataProvider create_test()

        Constructs a testdata provider and returns it as an :cpp:class:`ICU4XDataProvider`. Requires the ``provider_test`` feature.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_testdata/index.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XDataProvider, ICU4XError> create_from_byte_slice(const diplomat::span<uint8_t> blob)

        Constructs a ``BlobDataProvider`` and returns it as an :cpp:class:`ICU4XDataProvider`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.BlobDataProvider.html>`__ for more information.


    .. cpp:function:: static ICU4XDataProvider create_empty()

        Constructs an empty ``StaticDataProvider`` and returns it as an :cpp:class:`ICU4XDataProvider`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.StaticDataProvider.html>`__ for more information.

