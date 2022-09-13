``script::ffi``
===============

.. js:class:: ICU4XScriptWithExtensionsSet

    An ICU4X ScriptWithExtensions map object, capable of holding a map of codepoints to scriptextensions values

    See the `Rust documentation for ScriptWithExtensions <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptWithExtensions.html>`__ for more information.


    .. js:staticfunction:: load(provider)

        See the `Rust documentation for load_script_with_extensions_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/fn.load_script_with_extensions_unstable.html>`__ for more information.


    .. js:function:: get_script_val(code_point)

        Get the Script property value for a code point

        See the `Rust documentation for get_script_val <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptWithExtensions.html#method.get_script_val>`__ for more information.


    .. js:function:: has_script(code_point, script)

        Check if the Script_Extensions property of the given code point covers the given script

        See the `Rust documentation for has_script <https://unicode-org.github.io/icu4x-docs/doc/icu/properties/script/struct.ScriptWithExtensions.html#method.has_script>`__ for more information.

