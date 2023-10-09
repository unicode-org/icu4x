``logging::ffi``
================

.. cpp:class:: ICU4XLogger

    An object allowing control over the logging used


    .. cpp:function:: static bool init_simple_logger()

        Initialize the logger using ``simple_logger``

        Requires the ``simple_logger`` Cargo feature.

        Returns ``false`` if there was already a logger set.


    .. cpp:function:: static bool init_console_logger()

        Initialize the logger to use the WASM console.

        Only available on ``wasm32`` targets.

        Returns ``false`` if there was already a logger set.

