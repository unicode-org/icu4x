``fallbacker::ffi``
===================

.. js:class:: ICU4XLocaleFallbackConfig

    Collection of configurations for the ICU4X fallback algorithm.

    See the `Rust documentation for LocaleFallbackConfig <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackConfig.html>`__ for more information.


    .. js:attribute:: priority

        Choice of priority mode.


    .. js:attribute:: extension_key

        An empty string is considered ``None``.


.. js:class:: ICU4XLocaleFallbackIterator

    An iterator over the locale under fallback.

    See the `Rust documentation for LocaleFallbackIterator <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackIterator.html>`__ for more information.


    .. js:function:: get()

        Gets a snapshot of the current state of the locale.

        See the `Rust documentation for get <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackIterator.html#method.get>`__ for more information.


    .. js:function:: step()

        Performs one step of the fallback algorithm, mutating the locale.

        See the `Rust documentation for step <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackIterator.html#method.step>`__ for more information.


.. js:class:: ICU4XLocaleFallbackPriority

    Priority mode for the ICU4X fallback algorithm.

    See the `Rust documentation for FallbackPriority <https://unicode-org.github.io/icu4x-docs/doc/icu_provider/enum.FallbackPriority.html>`__ for more information.


.. js:class:: ICU4XLocaleFallbacker

    An object that runs the ICU4X locale fallback algorithm.

    See the `Rust documentation for LocaleFallbacker <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbacker.html>`__ for more information.


    .. js:staticfunction:: create(provider)

        Creates a new ``ICU4XLocaleFallbacker`` from a data provider.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbacker.html#method.try_new_unstable>`__ for more information.


    .. js:staticfunction:: create_without_data()

        Creates a new ``ICU4XLocaleFallbacker`` without data for limited functionality.

        See the `Rust documentation for new_without_data <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbacker.html#method.new_without_data>`__ for more information.


    .. js:function:: for_config(config)

        Associates this ``ICU4XLocaleFallbacker`` with configuration options.

        See the `Rust documentation for for_config <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbacker.html#method.for_config>`__ for more information.


.. js:class:: ICU4XLocaleFallbackerWithConfig

    An object that runs the ICU4X locale fallback algorithm with specific configurations.

    See the `Rust documentation for LocaleFallbackerWithConfig <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackerWithConfig.html>`__ for more information.


    .. js:function:: fallback_for_locale(locale)

        Creates an iterator from a locale with each step of fallback.

        See the `Rust documentation for fallback_for <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackerWithConfig.html#method.fallback_for>`__ for more information.

