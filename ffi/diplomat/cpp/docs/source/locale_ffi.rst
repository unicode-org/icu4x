``locale::ffi``
===============

.. cpp:class:: ICU4XLocale

    An ICU4X Locale, capable of representing strings like ``"en-US"``.

    See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XLocale, ICU4XError> create(const std::string_view name)

        Construct an :cpp:class:`ICU4XLocale` from an locale identifier.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.from_bytes>`__ for more information.


    .. cpp:function:: static ICU4XLocale create_en()

        Construct an :cpp:class:`ICU4XLocale` for the English language.


    .. cpp:function:: static ICU4XLocale create_bn()

        Construct an :cpp:class:`ICU4XLocale` for the Bangla language.


    .. cpp:function:: static ICU4XLocale und()

        Construct a default undefined :cpp:class:`ICU4XLocale` "und".


    .. cpp:function:: ICU4XLocale clone() const

        Clones the :cpp:class:`ICU4XLocale`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> basename_to_writeable(W& write) const

        Write a string representation of the ``LanguageIdentifier`` part of :cpp:class:`ICU4XLocale` to ``write``.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> basename() const

        Write a string representation of the ``LanguageIdentifier`` part of :cpp:class:`ICU4XLocale` to ``write``.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> get_unicode_extension_to_writeable(const std::string_view bytes, W& write) const

        Write a string representation of the unicode extension to ``write``

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.extensions>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> get_unicode_extension(const std::string_view bytes) const

        Write a string representation of the unicode extension to ``write``

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.extensions>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> language_to_writeable(W& write) const

        Write a string representation of :cpp:class:`ICU4XLocale` language to ``write``

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> language() const

        Write a string representation of :cpp:class:`ICU4XLocale` language to ``write``

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.


    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> set_language(const std::string_view bytes)

        Set the language part of the :cpp:class:`ICU4XLocale`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.from_bytes>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> region_to_writeable(W& write) const

        Write a string representation of :cpp:class:`ICU4XLocale` region to ``write``

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> region() const

        Write a string representation of :cpp:class:`ICU4XLocale` region to ``write``

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.


    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> set_region(const std::string_view bytes)

        Set the region part of the :cpp:class:`ICU4XLocale`.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.from_bytes>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> script_to_writeable(W& write) const

        Write a string representation of :cpp:class:`ICU4XLocale` script to ``write``

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> script() const

        Write a string representation of :cpp:class:`ICU4XLocale` script to ``write``

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.


    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> set_script(const std::string_view bytes)

        Set the script part of the :cpp:class:`ICU4XLocale`. Pass an empty string to remove the script.

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.from_bytes>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> tostring_to_writeable(W& write) const

        Write a string representation of :cpp:class:`ICU4XLocale` to ``write``

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> tostring() const

        Write a string representation of :cpp:class:`ICU4XLocale` to ``write``

        See the `Rust documentation <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html>`__ for more information.

