``locale::ffi``
===============

.. cpp:class:: ICU4XLocale

    An ICU4X Locale, capable of representing strings like ``"en-US"``.

    See the `Rust documentation for Locale <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XLocale, ICU4XError> create_from_string(const std::string_view name)

        Construct an :cpp:class:`ICU4XLocale` from an locale identifier.

        This will run the complete locale parsing algorithm. If code size and performance are critical and the locale is of a known shape (such as ``aa-BB``) use ``create_und``, ``set_language``, ``set_script``, and ``set_region``.

        See the `Rust documentation for try_from_bytes <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.try_from_bytes>`__ for more information.


    .. cpp:function:: static ICU4XLocale create_und()

        Construct a default undefined :cpp:class:`ICU4XLocale` "und".

        See the `Rust documentation for UND <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#associatedconstant.UND>`__ for more information.


    .. cpp:function:: ICU4XLocale clone() const

        Clones the :cpp:class:`ICU4XLocale`.

        See the `Rust documentation for Locale <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> basename_to_writeable(W& write) const

        Write a string representation of the ``LanguageIdentifier`` part of :cpp:class:`ICU4XLocale` to ``write``.

        See the `Rust documentation for id <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> basename() const

        Write a string representation of the ``LanguageIdentifier`` part of :cpp:class:`ICU4XLocale` to ``write``.

        See the `Rust documentation for id <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> get_unicode_extension_to_writeable(const std::string_view bytes, W& write) const

        Write a string representation of the unicode extension to ``write``

        See the `Rust documentation for extensions <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.extensions>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> get_unicode_extension(const std::string_view bytes) const

        Write a string representation of the unicode extension to ``write``

        See the `Rust documentation for extensions <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.extensions>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> language_to_writeable(W& write) const

        Write a string representation of :cpp:class:`ICU4XLocale` language to ``write``

        See the `Rust documentation for id <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> language() const

        Write a string representation of :cpp:class:`ICU4XLocale` language to ``write``

        See the `Rust documentation for id <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.


    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> set_language(const std::string_view bytes)

        Set the language part of the :cpp:class:`ICU4XLocale`.

        See the `Rust documentation for try_from_bytes <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.try_from_bytes>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> region_to_writeable(W& write) const

        Write a string representation of :cpp:class:`ICU4XLocale` region to ``write``

        See the `Rust documentation for id <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> region() const

        Write a string representation of :cpp:class:`ICU4XLocale` region to ``write``

        See the `Rust documentation for id <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.


    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> set_region(const std::string_view bytes)

        Set the region part of the :cpp:class:`ICU4XLocale`.

        See the `Rust documentation for try_from_bytes <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.try_from_bytes>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> script_to_writeable(W& write) const

        Write a string representation of :cpp:class:`ICU4XLocale` script to ``write``

        See the `Rust documentation for id <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> script() const

        Write a string representation of :cpp:class:`ICU4XLocale` script to ``write``

        See the `Rust documentation for id <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#structfield.id>`__ for more information.


    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> set_script(const std::string_view bytes)

        Set the script part of the :cpp:class:`ICU4XLocale`. Pass an empty string to remove the script.

        See the `Rust documentation for try_from_bytes <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.try_from_bytes>`__ for more information.


    .. cpp:function:: template<typename W> static diplomat::result<std::monostate, ICU4XError> canonicalize_to_writeable(const std::string_view bytes, W& write)

        Best effort locale canonicalizer that doesn't need any data

        Use ICU4XLocaleCanonicalizer for better control and functionality

        See the `Rust documentation for canonicalize <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.canonicalize>`__ for more information.


    .. cpp:function:: static diplomat::result<std::string, ICU4XError> canonicalize(const std::string_view bytes)

        Best effort locale canonicalizer that doesn't need any data

        Use ICU4XLocaleCanonicalizer for better control and functionality

        See the `Rust documentation for canonicalize <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.canonicalize>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> to_string_to_writeable(W& write) const

        Write a string representation of :cpp:class:`ICU4XLocale` to ``write``

        See the `Rust documentation for write_to <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.write_to>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> to_string() const

        Write a string representation of :cpp:class:`ICU4XLocale` to ``write``

        See the `Rust documentation for write_to <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.write_to>`__ for more information.


    .. cpp:function:: bool normalizing_eq(const std::string_view other) const

        See the `Rust documentation for normalizing_eq <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.normalizing_eq>`__ for more information.


    .. cpp:function:: ICU4XOrdering strict_cmp(const std::string_view other) const

        See the `Rust documentation for strict_cmp <https://unicode-org.github.io/icu4x-docs/doc/icu/locid/struct.Locale.html#method.strict_cmp>`__ for more information.


    .. cpp:function:: static ICU4XLocale create_en()

        Construct an :cpp:class:`ICU4XLocale` for the English language.

        This convenience constructor is intended for testing only and requires the ``provider_test`` feature.


    .. cpp:function:: static ICU4XLocale create_bn()

        Construct an :cpp:class:`ICU4XLocale` for the Bangla language.

        This convenience constructor is intended for testing only and requires the ``provider_test`` feature.

