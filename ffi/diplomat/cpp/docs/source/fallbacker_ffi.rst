``fallbacker::ffi``
===================

.. cpp:struct:: ICU4XLocaleFallbackConfig

    Collection of configurations for the ICU4X fallback algorithm.

    See the `Rust documentation for LocaleFallbackConfig <https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fallback/struct.LocaleFallbackConfig.html>`__ for more information.


    .. cpp:member:: ICU4XLocaleFallbackPriority priority

        Choice of priority mode.


    .. cpp:member:: std::string_view extension_key

        An empty string is considered ``None``.


.. cpp:class:: ICU4XLocaleFallbackIterator

    An iterator over the locale under fallback.

    See the `Rust documentation for LocaleFallbackIterator <https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fallback/struct.LocaleFallbackIterator.html>`__ for more information.


    .. cpp:function:: ICU4XLocale get() const

        Gets a snapshot of the current state of the locale.

        See the `Rust documentation for get <https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fallback/struct.LocaleFallbackIterator.html#method.get>`__ for more information.


    .. cpp:function:: void step()

        Performs one step of the fallback algorithm, mutating the locale.

        See the `Rust documentation for step <https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fallback/struct.LocaleFallbackIterator.html#method.step>`__ for more information.


.. cpp:enum-struct:: ICU4XLocaleFallbackPriority

    Priority mode for the ICU4X fallback algorithm.

    See the `Rust documentation for FallbackPriority <https://docs.rs/icu_provider/latest/icu_provider/enum.FallbackPriority.html>`__ for more information.


    .. cpp:enumerator:: Language

    .. cpp:enumerator:: Region

    .. cpp:enumerator:: Collation

.. cpp:class:: ICU4XLocaleFallbacker

    An object that runs the ICU4X locale fallback algorithm.

    See the `Rust documentation for LocaleFallbacker <https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fallback/struct.LocaleFallbacker.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XLocaleFallbacker, ICU4XError> create(const ICU4XDataProvider& provider)

        Creates a new ``ICU4XLocaleFallbacker`` from a data provider.

        See the `Rust documentation for try_new_unstable <https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fallback/struct.LocaleFallbacker.html#method.try_new_unstable>`__ for more information.


    .. cpp:function:: static ICU4XLocaleFallbacker create_without_data()

        Creates a new ``ICU4XLocaleFallbacker`` without data for limited functionality.

        See the `Rust documentation for new_without_data <https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fallback/struct.LocaleFallbacker.html#method.new_without_data>`__ for more information.


    .. cpp:function:: diplomat::result<ICU4XLocaleFallbackerWithConfig, ICU4XError> for_config(ICU4XLocaleFallbackConfig config) const

        Associates this ``ICU4XLocaleFallbacker`` with configuration options.

        See the `Rust documentation for for_config <https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fallback/struct.LocaleFallbacker.html#method.for_config>`__ for more information.

        Lifetimes: ``this`` must live at least as long as the output.


.. cpp:class:: ICU4XLocaleFallbackerWithConfig

    An object that runs the ICU4X locale fallback algorithm with specific configurations.

    See the `Rust documentation for LocaleFallbackerWithConfig <https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fallback/struct.LocaleFallbackerWithConfig.html>`__ for more information.


    .. cpp:function:: ICU4XLocaleFallbackIterator fallback_for_locale(const ICU4XLocale& locale) const

        Creates an iterator from a locale with each step of fallback.

        See the `Rust documentation for fallback_for <https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fallback/struct.LocaleFallbackerWithConfig.html#method.fallback_for>`__ for more information.

        Lifetimes: ``this`` must live at least as long as the output.

