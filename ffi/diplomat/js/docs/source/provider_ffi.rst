``provider::ffi``
=================

.. js:class:: ICU4XDataProvider

    An ICU4X data provider, capable of loading ICU4X data keys from some source.

    See the `Rust documentation for icu_provider <https://unicode-org.github.io/icu4x-docs/doc/icu_provider/index.html>`__ for more information.


    .. js:function:: create_fs(path)

        Constructs an ``FsDataProvider`` and returns it as an :js:class:`ICU4XDataProvider`. Requires the ``provider_fs`` Cargo feature. Not supported in WASM.

        See the `Rust documentation for FsDataProvider <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_fs/struct.FsDataProvider.html>`__ for more information.


    .. js:function:: create_test()

        Constructs a testdata provider and returns it as an :js:class:`ICU4XDataProvider`. Requires the ``provider_test`` and one of ``any_provider`` or ``buffer_provider`` Cargo features.

        See the `Rust documentation for icu_testdata <https://unicode-org.github.io/icu4x-docs/doc/icu_testdata/index.html>`__ for more information.


    .. js:function:: create_from_byte_slice(blob)

        Constructs a ``BlobDataProvider`` and returns it as an :js:class:`ICU4XDataProvider`.

        See the `Rust documentation for BlobDataProvider <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.BlobDataProvider.html>`__ for more information.

        - Note: ``blob`` should be an ArrayBuffer or TypedArray corresponding to the slice type expected by Rust.

        - Warning: This method leaks memory. The parameter `blob` will not be freed as it is required to live for the duration of the program.


    .. js:function:: create_empty()

        Constructs an empty :js:class:`ICU4XDataProvider`.

        See the `Rust documentation for EmptyDataProvider <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/empty/struct.EmptyDataProvider.html>`__ for more information.


    .. js:function:: create_baked()

        Constructs a :js:class:`ICU4XDataProvider` containing baked data.

        When compiling the Rust library, set the ``ICU4X_FFI_BAKED_ROOT`` environment variable to the baked data folder.

        If no data is supplied, this behaves like an empty provider.


    .. js:function:: fork_by_key(other)

        Creates a provider that tries the current provider and then, if the current provider doesn't support the data key, another provider ``other``.

        This takes ownership of the ``other`` provider, leaving an empty provider in its place.

        The providers must be the same type (Any or Buffer). This condition is satisfied if both providers originate from the same constructor, such as ``create_from_byte_slice`` or ``create_fs``. If the condition is not upheld, a runtime error occurs.

        See the `Rust documentation for ForkByKeyProvider <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fork/type.ForkByKeyProvider.html>`__ for more information.


    .. js:function:: fork_by_locale(other)

        Same as ``fork_by_key`` but forks by locale instead of key.

        See the `Rust documentation for MissingLocalePredicate <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fork/predicates/struct.MissingLocalePredicate.html>`__ for more information.


    .. js:function:: enable_locale_fallback()

        Enables locale fallbacking for data requests made to this provider.

        Note that the test provider (from ``create_test``) already has fallbacking enabled.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackProvider.html#method.try_new_unstable>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackProvider.html>`__


    .. js:function:: enable_locale_fallback_with(fallbacker)

        See the `Rust documentation for new_with_fallbacker <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackProvider.html#method.new_with_fallbacker>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackProvider.html>`__

