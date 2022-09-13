``fallbacker::ffi``
===================

.. js:class:: ICU4XLocaleFallbackConfig

    See the `Rust documentation for LocaleFallbackConfig <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackConfig.html>`__ for more information.


    .. js:attribute:: priority

    .. js:attribute:: extension_key

        An empty string is considered ``None``.


.. js:class:: ICU4XLocaleFallbackIterator

    See the `Rust documentation for LocaleFallbackIterator <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackIterator.html>`__ for more information.


    .. js:function:: get()

        See the `Rust documentation for get <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackIterator.html#method.get>`__ for more information.


    .. js:function:: step()

        See the `Rust documentation for step <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackIterator.html#method.step>`__ for more information.


.. js:class:: ICU4XLocaleFallbackPriority

    See the `Rust documentation for FallbackPriority <https://unicode-org.github.io/icu4x-docs/doc/icu_provider/enum.FallbackPriority.html>`__ for more information.


.. js:class:: ICU4XLocaleFallbacker

    See the `Rust documentation for LocaleFallbacker <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbacker.html>`__ for more information.


    .. js:staticfunction:: create(provider)

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbacker.html#method.try_new_unstable>`__ for more information.


    .. js:staticfunction:: create_without_data()

        See the `Rust documentation for new_without_data <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbacker.html#method.new_without_data>`__ for more information.


    .. js:function:: for_config(config)

        See the `Rust documentation for for_config <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbacker.html#method.for_config>`__ for more information.


.. js:class:: ICU4XLocaleFallbackerWithConfig

    See the `Rust documentation for LocaleFallbackerWithConfig <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackerWithConfig.html>`__ for more information.


    .. js:function:: fallback_for_locale(locale)

        See the `Rust documentation for fallback_for <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackerWithConfig.html#method.fallback_for>`__ for more information.

