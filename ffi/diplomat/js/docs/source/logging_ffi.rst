``logging::ffi``
================

.. js:class:: ICU4XLogger

    An object allowing control over the logging used


    .. js:function:: init_simple_logger()

        Initialize the logger using ``simple_logger``, or console.log/warn in WASM.

        Returns ``false`` if there was already a logger set, or if logging has not been compiled into the platform

