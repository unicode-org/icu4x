``script::ffi``
===============

.. js:class:: ICU4XScriptExtensionsSet

    An object that represents the Script_Extensions property for a single character

    See the `Rust documentation for ScriptExtensionsSet <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptExtensionsSet.html>`__ for more information.


    .. js:function:: contains(script)

        Check if the Script_Extensions property of the given code point covers the given script

        See the `Rust documentation for contains <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptExtensionsSet.html#method.contains>`__ for more information.


    .. js:function:: count()

        Get the number of scripts contained in here

        See the `Rust documentation for iter <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptExtensionsSet.html#method.iter>`__ for more information.


    .. js:function:: script_at(index)

        Get script at index, returning an error if out of bounds

        See the `Rust documentation for iter <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptExtensionsSet.html#method.iter>`__ for more information.


.. js:class:: ICU4XScriptWithExtensions

    An ICU4X ScriptWithExtensions map object, capable of holding a map of codepoints to scriptextensions values

    See the `Rust documentation for ScriptWithExtensions <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptWithExtensions.html>`__ for more information.


    .. js:staticfunction:: create(provider)

        See the `Rust documentation for load_script_with_extensions_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/fn.load_script_with_extensions_unstable.html>`__ for more information.


    .. js:function:: get_script_val(code_point)

        Get the Script property value for a code point

        See the `Rust documentation for get_script_val <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.get_script_val>`__ for more information.


    .. js:function:: has_script(code_point, script)

        Check if the Script_Extensions property of the given code point covers the given script

        See the `Rust documentation for has_script <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.has_script>`__ for more information.


    .. js:function:: as_borrowed()

        Borrow this object for a slightly faster variant with more operations

        See the `Rust documentation for as_borrowed <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptWithExtensions.html#method.as_borrowed>`__ for more information.


.. js:class:: ICU4XScriptWithExtensionsBorrowed

    A slightly faster ICU4XScriptWithExtensions object

    See the `Rust documentation for ScriptWithExtensionsBorrowed <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html>`__ for more information.


    .. js:function:: get_script_val(code_point)

        Get the Script property value for a code point

        See the `Rust documentation for get_script_val <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.get_script_val>`__ for more information.


    .. js:function:: get_script_extensions_val(code_point)

        Get the Script property value for a code point

        See the `Rust documentation for get_script_extensions_val <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.get_script_extensions_val>`__ for more information.


    .. js:function:: has_script(code_point, script)

        Check if the Script_Extensions property of the given code point covers the given script

        See the `Rust documentation for has_script <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.has_script>`__ for more information.

