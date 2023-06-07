``iana_bcp47_mapper::ffi``
==========================

.. cpp:class:: ICU4XBcp47ToIanaMapper

    An object capable of mapping from a BCP-47 time zone ID to an IANA ID.

    See the `Rust documentation for Bcp47ToIanaMapper <https://docs.rs/icu/latest/icu/timezone/struct.Bcp47ToIanaMapper.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XBcp47ToIanaMapper, ICU4XError> create(const ICU4XDataProvider& provider)

        See the `Rust documentation for try_new_unstable <https://docs.rs/icu/latest/icu/timezone/struct.Bcp47ToIanaMapper.html#method.try_new_unstable>`__ for more information.


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, ICU4XError> get_to_writeable(const std::string_view value, W& write) const

        Writes out the canonical IANA time zone ID corresponding to the given BCP-47 ID.

        See the `Rust documentation for get <https://docs.rs/icu/latest/icu/datetime/time_zone/struct.Bcp47ToIanaMapper.html#method.get>`__ for more information.


    .. cpp:function:: diplomat::result<std::string, ICU4XError> get(const std::string_view value) const

        Writes out the canonical IANA time zone ID corresponding to the given BCP-47 ID.

        See the `Rust documentation for get <https://docs.rs/icu/latest/icu/datetime/time_zone/struct.Bcp47ToIanaMapper.html#method.get>`__ for more information.


.. cpp:class:: ICU4XIanaToBcp47Mapper

    An object capable of mapping from an IANA time zone ID to a BCP-47 ID.

    This can be used via ``try_set_iana_time_zone_id_strict()`` on ```ICU4XCustomTimeZone`` <crate::timezone::ffi::ICU4XCustomTimeZone;>`__.

    See the `Rust documentation for IanaToBcp47Mapper <https://docs.rs/icu/latest/icu/timezone/struct.IanaToBcp47Mapper.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XIanaToBcp47Mapper, ICU4XError> create(const ICU4XDataProvider& provider)

        See the `Rust documentation for try_new_unstable <https://docs.rs/icu/latest/icu/timezone/struct.IanaToBcp47Mapper.html#method.try_new_unstable>`__ for more information.

