``displaynames::ffi``
=====================

.. js:class:: ICU4XDisplayNamesFallback

    See the `Rust documentation for Fallback <https://docs.rs/icu/latest/icu/displaynames/options/enum.Fallback.html>`__ for more information.


.. js:class:: ICU4XDisplayNamesOptionsV1

    See the `Rust documentation for DisplayNamesOptions <https://docs.rs/icu/latest/icu/displaynames/options/struct.DisplayNamesOptions.html>`__ for more information.


    .. js:attribute:: style

        The optional formatting style to use for display name.


    .. js:attribute:: fallback

        The fallback return when the system does not have the requested display name, defaults to "code".


    .. js:attribute:: language_display

        The language display kind, defaults to "dialect".


.. js:class:: ICU4XDisplayNamesStyle

    See the `Rust documentation for Style <https://docs.rs/icu/latest/icu/displaynames/options/enum.Style.html>`__ for more information.


.. js:class:: ICU4XLanguageDisplay

    See the `Rust documentation for LanguageDisplay <https://docs.rs/icu/latest/icu/displaynames/options/enum.LanguageDisplay.html>`__ for more information.


.. js:class:: ICU4XLanguageDisplayNames

    See the `Rust documentation for LanguageDisplayNames <https://docs.rs/icu/latest/icu/displaynames/struct.LanguageDisplayNames.html>`__ for more information.


    .. js:function:: try_new_unstable(provider, locale, options)

        Creates a new ``LanguageDisplayNames`` from locale data and an options bag.

        See the `Rust documentation for try_new_unstable <https://docs.rs/icu/latest/icu/displaynames/struct.LanguageDisplayNames.html#method.try_new_unstable>`__ for more information.


    .. js:method:: of(code)

        Returns the locale specific display name of a language for a given string. Note that the funtion returns an empty string in case the display name for a given language code is not found.

        See the `Rust documentation for of <https://docs.rs/icu/latest/icu/displaynames/struct.LanguageDisplayNames.html#method.of>`__ for more information.


.. js:class:: ICU4XRegionDisplayNames

    See the `Rust documentation for RegionDisplayNames <https://docs.rs/icu/latest/icu/displaynames/struct.RegionDisplayNames.html>`__ for more information.


    .. js:function:: try_new_unstable(provider, locale)

        Creates a new ``RegionDisplayNames`` from locale data and an options bag.

        See the `Rust documentation for try_new_unstable <https://docs.rs/icu/latest/icu/displaynames/struct.RegionDisplayNames.html#method.try_new_unstable>`__ for more information.


    .. js:method:: of(code)

        Returns the locale specific display name of a region for a given string. Note that the funtion returns an empty string in case the display name for a given region code is not found.

        See the `Rust documentation for of <https://docs.rs/icu/latest/icu/displaynames/struct.RegionDisplayNames.html#method.of>`__ for more information.

