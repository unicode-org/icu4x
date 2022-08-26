``timezone::ffi``
=================

.. js:class:: ICU4XCustomTimeZone

    See the `Rust documentation for CustomTimeZone <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html>`__ for more information.


    .. js:staticfunction:: create_from_str(s)

    .. js:function:: try_set_gmt_offset_seconds(offset_seconds)

        See the `Rust documentation for try_from_offset_seconds <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.try_from_offset_seconds>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html>`__


    .. js:function:: gmt_offset_seconds()

        See the `Rust documentation for offset_seconds <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.offset_seconds>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html>`__


    .. js:function:: is_gmt_offset_positive()

        See the `Rust documentation for is_positive <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.is_positive>`__ for more information.


    .. js:function:: is_gmt_offset_zero()

        See the `Rust documentation for is_zero <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.is_zero>`__ for more information.


    .. js:function:: gmt_offset_has_minutes()

        See the `Rust documentation for has_minutes <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.has_minutes>`__ for more information.


    .. js:function:: gmt_offset_has_seconds()

        See the `Rust documentation for has_seconds <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.has_seconds>`__ for more information.


    .. js:function:: try_set_time_zone_id(id)

        See the `Rust documentation for time_zone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.TimeZoneBcp47Id.html>`__


    .. js:function:: time_zone_id()

        See the `Rust documentation for time_zone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.TimeZoneBcp47Id.html>`__


    .. js:function:: try_set_meta_zone_id(id)

        See the `Rust documentation for meta_zone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.meta_zone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneId.html>`__


    .. js:function:: meta_zone_id()

        See the `Rust documentation for meta_zone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.meta_zone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneId.html>`__


    .. js:function:: try_set_zone_variant(id)

        See the `Rust documentation for zone_variant <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html>`__


    .. js:function:: zone_variant()

        See the `Rust documentation for zone_variant <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html>`__


    .. js:function:: set_standard_time()

        See the `Rust documentation for standard <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.standard>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__


    .. js:function:: set_daylight_time()

        See the `Rust documentation for daylight <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.daylight>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__


    .. js:function:: is_standard_time()

        See the `Rust documentation for standard <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.standard>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__


    .. js:function:: is_daylight_time()

        See the `Rust documentation for daylight <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.daylight>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__


    .. js:function:: maybe_set_meta_zone(local_datetime, metazone_calculator)

        See the `Rust documentation for maybe_set_meta_zone <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#method.maybe_set_meta_zone>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneCalculator.html#method.compute_metazone_from_timezone>`__


.. js:class:: ICU4XMetaZoneCalculator

    See the `Rust documentation for MetaZoneCalculator <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneCalculator.html>`__ for more information.


    .. js:staticfunction:: try_new(provider)

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneCalculator.html#method.try_new_unstable>`__ for more information.

