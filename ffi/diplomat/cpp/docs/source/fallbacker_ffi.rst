``fallbacker::ffi``
===================

.. cpp:struct:: ICU4XLocaleFallbackConfig

    See the `Rust documentation for LocaleFallbackConfig <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackConfig.html>`__ for more information.


    .. cpp:member:: ICU4XLocaleFallbackPriority priority

    .. cpp:member:: std::string_view extension_key

        An empty string is considered ``None``.


.. cpp:class:: ICU4XLocaleFallbackIterator

    See the `Rust documentation for LocaleFallbackIterator <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackIterator.html>`__ for more information.


    .. cpp:function:: ICU4XLocale get() const

        See the `Rust documentation for get <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackIterator.html#method.get>`__ for more information.


    .. cpp:function:: void step()

        See the `Rust documentation for step <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackIterator.html#method.step>`__ for more information.


.. cpp:enum-struct:: ICU4XLocaleFallbackPriority

    See the `Rust documentation for FallbackPriority <https://unicode-org.github.io/icu4x-docs/doc/icu_provider/enum.FallbackPriority.html>`__ for more information.


    .. cpp:enumerator:: Language

    .. cpp:enumerator:: Region

    .. cpp:enumerator:: Collation

.. cpp:class:: ICU4XLocaleFallbacker

    See the `Rust documentation for LocaleFallbacker <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbacker.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XLocaleFallbacker, ICU4XError> create(const ICU4XDataProvider& provider)

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbacker.html#method.try_new_unstable>`__ for more information.


    .. cpp:function:: static ICU4XLocaleFallbacker create_without_data()

        See the `Rust documentation for new_without_data <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbacker.html#method.new_without_data>`__ for more information.


    .. cpp:function:: diplomat::result<ICU4XLocaleFallbackerWithConfig, ICU4XError> for_config(ICU4XLocaleFallbackConfig config) const

        See the `Rust documentation for for_config <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbacker.html#method.for_config>`__ for more information.


        Lifetimes: ``this`` must live at least as long as the output.

.. cpp:class:: ICU4XLocaleFallbackerWithConfig

    See the `Rust documentation for LocaleFallbackerWithConfig <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackerWithConfig.html>`__ for more information.


    .. cpp:function:: ICU4XLocaleFallbackIterator fallback_for_locale(const ICU4XLocale& locale) const

        See the `Rust documentation for fallback_for <https://unicode-org.github.io/icu4x-docs/doc/icu_provider_adapters/fallback/struct.LocaleFallbackerWithConfig.html#method.fallback_for>`__ for more information.


        Lifetimes: ``this`` must live at least as long as the output.
