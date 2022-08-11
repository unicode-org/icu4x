``logging::ffi``
================

.. cpp:class:: ICU4XLogger

    An object allowing control over the logging used


    .. cpp:function:: static bool init_simple_logger()

        Initialize the logger from the ``simple_logger`` crate, which simply logs to stdout. Returns ``false`` if there was already a logger set, or if logging has not been compiled into the platform

