``locale::ffi``
===============

.. js:class:: ICU4XLocale

    An ICU4X Locale, capable of representing strings like ``"en-US"``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html>`__ for more information.

    .. js:staticfunction:: create(name)

        Construct an :js:class:`ICU4XLocale` from an locale identifier. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.from_bytes>`__ for more information.

    .. js:staticfunction:: create_en()

        Construct an :js:class:`ICU4XLocale` for the English language.

    .. js:staticfunction:: create_bn()

        Construct an :js:class:`ICU4XLocale` for the Bangla language.

    .. js:function:: clone()

        Clones the :js:class:`ICU4XLocale`. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html>`__ for more information.

    .. js:function:: basename()

        Write a string representation of the ``LanguageIdentifier`` part of :js:class:`ICU4XLocale` to ``write``. See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.

    .. js:function:: get_unicode_extension(bytes)

        Write a string representation of the unicode extension to ``write`` See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.extensions>`__ for more information.

    .. js:function:: language()

        Write a string representation of :js:class:`ICU4XLocale` language to ``write`` See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.

    .. js:function:: region()

        Write a string representation of :js:class:`ICU4XLocale` region to ``write`` See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.

    .. js:function:: script()

        Write a string representation of :js:class:`ICU4XLocale` script to ``write`` See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.

    .. js:function:: tostring()

        Write a string representation of :js:class:`ICU4XLocale` to ``write`` See `the Rust docs <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html>`__ for more information.

.. js:class:: ICU4XLocaleError
