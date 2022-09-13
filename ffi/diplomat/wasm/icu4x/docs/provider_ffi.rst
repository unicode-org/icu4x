``provider::ffi``
=================

.. js:class:: ICU4XCreateDataProviderResult

    A result type for ``ICU4XDataProvider::create``.


    .. js:attribute:: provider

        Will be ``None`` if ``success`` is ``false``, do not use in that case.


    .. js:attribute:: success

.. js:class:: ICU4XDataProvider

    An ICU4X data provider, capable of loading ICU4X data keys from some source.

    See the `Rust documentation for icu_provider <https://unicode-org.github.io/icu4x-docs/doc/icu_provider/index.html>`__ for more information.


    .. js:staticfunction:: create_fs(path)

        Constructs an ``FsDataProvider`` and returns it as an :js:class:`ICU4XDataProvider`. Requires the ``provider_fs`` feature. Not supported in WASM.

        See the `Rust documentation for FsDataProvider <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_fs/struct.FsDataProvider.html>`__ for more information.


    .. js:staticfunction:: create_test()

        Constructs a testdata provider and returns it as an :js:class:`ICU4XDataProvider`. Requires the ``provider_test`` feature.

        See the `Rust documentation for icu_testdata <https://unicode-org.github.io/icu4x-docs/doc/icu_testdata/index.html>`__ for more information.


    .. js:staticfunction:: create_from_byte_slice(blob)

        Constructs a ``BlobDataProvider`` and returns it as an :js:class:`ICU4XDataProvider`.

        See the `Rust documentation for BlobDataProvider <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.BlobDataProvider.html>`__ for more information.


        - Note: ``blob`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.

    .. js:staticfunction:: create_empty()

        Constructs an empty ``StaticDataProvider`` and returns it as an :js:class:`ICU4XDataProvider`.

        See the `Rust documentation for EmptyDataProvider <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/empty/struct.EmptyDataProvider.html>`__ for more information.


    .. js:function:: fork_by_key(other)

        Creates a provider that tries the current provider and then, if the current provider doesn't support the data key, another provider ``other``.

        This takes ownership of the ``other`` provider, leaving an empty provider in its place.

        The providers must be the same type (Any or Buffer). This condition is satisfied if both providers originate from the same constructor, such as ``create_from_byte_slice`` or ``create_fs``. If the condition is not upheld, a runtime error occurs.

        See the `Rust documentation for ForkByKeyProvider <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fork/type.ForkByKeyProvider.html>`__ for more information.


    .. js:function:: enable_locale_fallback()

        Enables locale fallbacking for data requests made to this provider.

        Note that the test provider (from ``create_test``) already has fallbacking enabled.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackProvider.html#method.try_new_unstable>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackProvider.html>`__


    .. js:function:: enable_locale_fallback_with(fallbacker)

        See the `Rust documentation for new_with_fallbacker <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackProvider.html#method.new_with_fallbacker>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackProvider.html>`__

