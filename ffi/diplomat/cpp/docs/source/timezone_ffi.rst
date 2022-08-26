``timezone::ffi``
=================

.. cpp:class:: ICU4XCustomTimeZone

    See the `Rust documentation for CustomTimeZone <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCustomTimeZone, ICU4XError> create_from_str(const std::string_view s)

    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> try_set_gmt_offset_seconds(int32_t offset_seconds)

        See the `Rust documentation for try_from_offset_seconds <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.try_from_offset_seconds>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html>`__


    .. cpp:function:: void clear_gmt_offset()

        See the `Rust documentation for offset_seconds <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.offset_seconds>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html>`__


    .. cpp:function:: diplomat::result<int32_t, std::monostate> gmt_offset_seconds() const

        See the `Rust documentation for offset_seconds <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.offset_seconds>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html>`__


    .. cpp:function:: diplomat::result<bool, std::monostate> is_gmt_offset_positive() const

        See the `Rust documentation for is_positive <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.is_positive>`__ for more information.


    .. cpp:function:: diplomat::result<bool, std::monostate> is_gmt_offset_zero() const

        See the `Rust documentation for is_zero <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.is_zero>`__ for more information.


    .. cpp:function:: diplomat::result<bool, std::monostate> gmt_offset_has_minutes() const

        See the `Rust documentation for has_minutes <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.has_minutes>`__ for more information.


    .. cpp:function:: diplomat::result<bool, std::monostate> gmt_offset_has_seconds() const

        See the `Rust documentation for has_seconds <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.has_seconds>`__ for more information.


    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> try_set_time_zone_id(const std::string_view id)

        See the `Rust documentation for time_zone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.TimeZoneBcp47Id.html>`__


    .. cpp:function:: void clear_time_zone_id()

        See the `Rust documentation for time_zone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.TimeZoneBcp47Id.html>`__


    .. cpp:function:: diplomat::result<const std::string_view, std::monostate> time_zone_id() const

        See the `Rust documentation for time_zone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.TimeZoneBcp47Id.html>`__


        Lifetimes: ``this`` must live at least as long as the output.

    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> try_set_meta_zone_id(const std::string_view id)

        See the `Rust documentation for meta_zone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.meta_zone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneId.html>`__


    .. cpp:function:: void clear_meta_zone_id()

        See the `Rust documentation for meta_zone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.meta_zone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneId.html>`__


    .. cpp:function:: diplomat::result<const std::string_view, std::monostate> meta_zone_id() const

        See the `Rust documentation for meta_zone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.meta_zone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneId.html>`__


        Lifetimes: ``this`` must live at least as long as the output.

    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> try_set_zone_variant(const std::string_view id)

        See the `Rust documentation for zone_variant <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html>`__


    .. cpp:function:: void clear_zone_variant()

        See the `Rust documentation for zone_variant <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html>`__


    .. cpp:function:: diplomat::result<const std::string_view, std::monostate> zone_variant() const

        See the `Rust documentation for zone_variant <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html>`__


        Lifetimes: ``this`` must live at least as long as the output.

    .. cpp:function:: void set_standard_time()

        See the `Rust documentation for standard <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.standard>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__


    .. cpp:function:: void set_daylight_time()

        See the `Rust documentation for daylight <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.daylight>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__


    .. cpp:function:: diplomat::result<bool, std::monostate> is_standard_time() const

        See the `Rust documentation for standard <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.standard>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__


    .. cpp:function:: diplomat::result<bool, std::monostate> is_daylight_time() const

        See the `Rust documentation for daylight <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.daylight>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__


    .. cpp:function:: void maybe_set_meta_zone(const ICU4XIsoDateTime& local_datetime, const ICU4XMetaZoneCalculator& metazone_calculator)

        See the `Rust documentation for maybe_set_meta_zone <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#method.maybe_set_meta_zone>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneCalculator.html#method.compute_metazone_from_timezone>`__


.. cpp:class:: ICU4XMetaZoneCalculator

    See the `Rust documentation for MetaZoneCalculator <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneCalculator.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XMetaZoneCalculator, ICU4XError> try_new(const ICU4XDataProvider& provider)

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneCalculator.html#method.try_new_unstable>`__ for more information.

