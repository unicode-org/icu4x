``provider::ffi``
=================

.. js:class:: ICU4XCreateDataProviderResult

    A result type for ``ICU4XDataProvider::create``.

    .. js:attribute:: provider

        Will be ``None`` if ``success`` is ``false``, do not use in that case.

    .. js:attribute:: success

.. js:class:: ICU4XCreateStaticDataProviderResult

    A result type for ``ICU4XStaticDataProvider::create``.

    .. js:attribute:: provider

        Will be ``None`` if ``success`` is ``false``, do not use in that case.

    .. js:attribute:: success

.. js:class:: ICU4XDataProvider

    An ICU4X data provider, capable of loading ICU4X data keys from some source. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_provider/prelude/trait.DataProvider.html>`__ for more information.

    .. js:staticfunction:: create_fs(path)

        Constructs an ``FsDataProvider`` and returns it as an :js:class:`ICU4XDataProvider`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_fs/struct.FsDataProvider.html>`__ for more details.

    .. js:staticfunction:: create_static()

        Constructs an ``StaticDataProvider`` and returns it as an :js:class:`ICU4XDataProvider`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.StaticDataProvider.html>`__ for more details.

.. js:class:: ICU4XStaticDataProvider

    An ICU4X data provider backed by static data. This is a specialization of :js:class:`ICU4XDataProvider` intended to reduce code size.
    See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.StaticDataProvider.html>`__ for more information.

    .. js:staticfunction:: create()

        Constructs an ``StaticDataProvider`` and returns it as an :js:class:`ICU4XStaticDataProvider`.
        See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.StaticDataProvider.html>`__ for more details.
