``displaynames::ffi``
=====================

.. js:class:: ICU4XDisplayNamesOptions

    See the `Rust documentation for DisplayNamesOptions <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/options/struct.DisplayNamesOptions.html>`__ for more information.


    .. js:attribute:: style

        The optional formatting style to use for display name.


    .. js:attribute:: fallback

        The fallback return when the system does not have the requested display name, defaults to "code".


    .. js:attribute:: language_display

        The language display kind, defaults to "dialect".


.. js:class:: ICU4XFallback

    See the `Rust documentation for Fallback <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/options/enum.Fallback.html>`__ for more information.


.. js:class:: ICU4XLanguageDisplay

    See the `Rust documentation for LanguageDisplay <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/options/enum.LanguageDisplay.html>`__ for more information.


.. js:class:: ICU4XLanguageDisplayNames

    See the `Rust documentation for LanguageDisplayNames <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/struct.LanguageDisplayNames.html>`__ for more information.


    .. js:function:: try_new_unstable(provider, locale, options)

        Creates a new ``LanguageDisplayNames`` from locale data and an options bag.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/struct.LanguageDisplayNames.html#method.try_new_unstable>`__ for more information.


    .. js:method:: of(code)

        See the `Rust documentation for of <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/struct.LanguageDisplayNames.html#method.of>`__ for more information.


.. js:class:: ICU4XRegionDisplayNames

    See the `Rust documentation for RegionDisplayNames <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/struct.RegionDisplayNames.html>`__ for more information.


    .. js:function:: try_new_unstable(provider, locale)

        Creates a new ``RegionDisplayNames`` from locale data and an options bag.

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/struct.RegionDisplayNames.html#method.try_new_unstable>`__ for more information.


    .. js:method:: of(code)

        See the `Rust documentation for of <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/struct.RegionDisplayNames.html#method.of>`__ for more information.


.. js:class:: ICU4XStyle

    See the `Rust documentation for Style <https://unicode-org.github.io/icu4x-docs/doc/icu/displaynames/options/enum.Style.html>`__ for more information.

