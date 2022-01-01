``provider::ffi``
=================

.. js:class:: ICU4XCreateDataProviderResult

    A result type for ``ICU4XDataProvider::create``.

    .. js:attribute:: provider

        Will be ``None`` if ``success`` is ``false``, do not use in that case.

    .. js:attribute:: success

.. js:class:: ICU4XDataProvider

    An ICU4X data provider, capable of loading ICU4X data keys from some source. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_provider/prelude/trait.DataProvider.html>`__ for more information.

    .. js:staticfunction:: create_fs(path)

        Constructs an ``FsDataProvider`` and returns it as an :js:class:`ICU4XDataProvider`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_fs/struct.FsDataProvider.html>`__ for more details.

    .. js:staticfunction:: create_static()

        Constructs an ``StaticDataProvider`` and returns it as an :js:class:`ICU4XDataProvider`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.StaticDataProvider.html>`__ for more details.

    .. js:staticfunction:: create_from_byte_slice(blob)

        Constructs a ``BlobDataProvider`` and returns it as an :js:class:`ICU4XDataProvider`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.BlobDataProvider.html>`__ for more details.

        - Note: ``blob`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.

    .. js:staticfunction:: create_empty()

        Constructs an empty ``StaticDataProvider`` and returns it as an :js:class:`ICU4XDataProvider`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.StaticDataProvider.html>`__ for more details.
