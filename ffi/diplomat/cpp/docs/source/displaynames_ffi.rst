``displaynames::ffi``
=====================

.. cpp:enum-struct:: ICU4XDisplayNamesFallback

    See the `Rust documentation for Fallback <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/options/enum.Fallback.html>`__ for more information.


    .. cpp:enumerator:: Code

    .. cpp:enumerator:: None

.. cpp:struct:: ICU4XDisplayNamesOptions

    See the `Rust documentation for DisplayNamesOptions <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/options/struct.DisplayNamesOptions.html>`__ for more information.


    .. cpp:member:: ICU4XDisplayNamesStyle style

        The optional formatting style to use for display name.


    .. cpp:member:: ICU4XDisplayNamesFallback fallback

        The fallback return when the system does not have the requested display name, defaults to "code".


    .. cpp:member:: ICU4XLanguageDisplay language_display

        The language display kind, defaults to "dialect".


.. cpp:enum-struct:: ICU4XDisplayNamesStyle

    See the `Rust documentation for Style <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/options/enum.Style.html>`__ for more information.


    .. cpp:enumerator:: Auto

    .. cpp:enumerator:: Narrow

    .. cpp:enumerator:: Short

    .. cpp:enumerator:: Long

    .. cpp:enumerator:: Menu

.. cpp:enum-struct:: ICU4XLanguageDisplay

    See the `Rust documentation for LanguageDisplay <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/options/enum.LanguageDisplay.html>`__ for more information.


    .. cpp:enumerator:: Dialect

    .. cpp:enumerator:: Standard

.. cpp:class:: ICU4XLanguageDisplayNames

    See the `Rust documentation for LanguageDisplayNames <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/struct.LanguageDisplayNames.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XLanguageDisplayNames, ICU4XError> try_new_unstable(const ICU4XDataProvider& provider, const ICU4XLocale& locale, ICU4XDisplayNamesOptions options)

        Creates a new ``LanguageDisplayNames`` from locale data and an options bag.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/struct.LanguageDisplayNames.html#method.try_new_unstable>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> of_to_writeable(const std::string_view code, W& write) const

        See the `Rust documentation for of <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/struct.LanguageDisplayNames.html#method.of>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> of(const std::string_view code) const

        See the `Rust documentation for of <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/struct.LanguageDisplayNames.html#method.of>`__ for more information.


.. cpp:class:: ICU4XRegionDisplayNames

    See the `Rust documentation for RegionDisplayNames <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/struct.RegionDisplayNames.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XRegionDisplayNames, ICU4XError> try_new_unstable(const ICU4XDataProvider& provider, const ICU4XLocale& locale)

        Creates a new ``RegionDisplayNames`` from locale data and an options bag.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/struct.RegionDisplayNames.html#method.try_new_unstable>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> of_to_writeable(const std::string_view code, W& write) const

        See the `Rust documentation for of <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/struct.RegionDisplayNames.html#method.of>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> of(const std::string_view code) const

        See the `Rust documentation for of <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/struct.RegionDisplayNames.html#method.of>`__ for more information.

