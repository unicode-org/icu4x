``timezone::ffi``
=================

.. js:class:: ICU4XCustomTimeZone

    See the `Rust documentation for CustomTimeZone <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html>`__ for more information.


    .. js:function:: create_from_string(s)

        Creates a time zone from an offset string.

        See the `Rust documentation for from_str <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#method.from_str>`__ for more information.


    .. js:function:: create_empty()

        Creates a time zone with no information.

        See the `Rust documentation for new_empty <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#method.new_empty>`__ for more information.


    .. js:function:: create_utc()

        Creates a time zone for UTC.

        See the `Rust documentation for utc <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#method.utc>`__ for more information.


    .. js:function:: try_set_gmt_offset_seconds(offset_seconds)

        Sets the ``gmt_offset`` field from offset seconds.

        Errors if the offset seconds are out of range.

        See the `Rust documentation for try_from_offset_seconds <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.try_from_offset_seconds>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html>`__


    .. js:function:: clear_gmt_offset()

        Clears the ``gmt_offset`` field.

        See the `Rust documentation for offset_seconds <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.offset_seconds>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html>`__


    .. js:function:: gmt_offset_seconds()

        Returns the value of the ``gmt_offset`` field as offset seconds.

        Errors if the ``gmt_offset`` field is empty.

        See the `Rust documentation for offset_seconds <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.offset_seconds>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html>`__


    .. js:function:: is_gmt_offset_positive()

        Returns whether the ``gmt_offset`` field is positive.

        Errors if the ``gmt_offset`` field is empty.

        See the `Rust documentation for is_positive <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.is_positive>`__ for more information.


    .. js:function:: is_gmt_offset_zero()

        Returns whether the ``gmt_offset`` field is zero.

        Errors if the ``gmt_offset`` field is empty (which is not the same as zero).

        See the `Rust documentation for is_zero <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.is_zero>`__ for more information.


    .. js:function:: gmt_offset_has_minutes()

        Returns whether the ``gmt_offset`` field has nonzero minutes.

        Errors if the ``gmt_offset`` field is empty.

        See the `Rust documentation for has_minutes <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.has_minutes>`__ for more information.


    .. js:function:: gmt_offset_has_seconds()

        Returns whether the ``gmt_offset`` field has nonzero seconds.

        Errors if the ``gmt_offset`` field is empty.

        See the `Rust documentation for has_seconds <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.has_seconds>`__ for more information.


    .. js:function:: try_set_time_zone_id(id)

        Sets the ``time_zone_id`` field from a string.

        Errors if the string is not a valid BCP-47 time zone ID.

        See the `Rust documentation for time_zone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.TimeZoneBcp47Id.html>`__


    .. js:function:: clear_time_zone_id()

        Clears the ``time_zone_id`` field.

        See the `Rust documentation for time_zone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.TimeZoneBcp47Id.html>`__


    .. js:function:: time_zone_id()

        Writes the value of the ``time_zone_id`` field as a string.

        Errors if the ``time_zone_id`` field is empty.

        See the `Rust documentation for time_zone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.TimeZoneBcp47Id.html>`__


    .. js:function:: try_set_metazone_id(id)

        Sets the ``metazone_id`` field from a string.

        Errors if the string is not a valid BCP-47 metazone ID.

        See the `Rust documentation for metazone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.metazone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetazoneId.html>`__


    .. js:function:: clear_metazone_id()

        Clears the ``metazone_id`` field.

        See the `Rust documentation for metazone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.metazone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetazoneId.html>`__


    .. js:function:: metazone_id()

        Writes the value of the ``metazone_id`` field as a string.

        Errors if the ``metazone_id`` field is empty.

        See the `Rust documentation for metazone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.metazone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetazoneId.html>`__


    .. js:function:: try_set_zone_variant(id)

        Sets the ``zone_variant`` field from a string.

        Errors if the string is not a valid zone variant.

        See the `Rust documentation for zone_variant <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html>`__


    .. js:function:: clear_zone_variant()

        Clears the ``zone_variant`` field.

        See the `Rust documentation for zone_variant <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html>`__


    .. js:function:: zone_variant()

        Writes the value of the ``zone_variant`` field as a string.

        Errors if the ``zone_variant`` field is empty.

        See the `Rust documentation for zone_variant <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html>`__


    .. js:function:: set_standard_time()

        Sets the ``zone_variant`` field to standard time.

        See the `Rust documentation for standard <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.standard>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__


    .. js:function:: set_daylight_time()

        Sets the ``zone_variant`` field to daylight time.

        See the `Rust documentation for daylight <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.daylight>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__


    .. js:function:: is_standard_time()

        Returns whether the ``zone_variant`` field is standard time.

        Errors if the ``zone_variant`` field is empty.

        See the `Rust documentation for standard <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.standard>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__


    .. js:function:: is_daylight_time()

        Returns whether the ``zone_variant`` field is daylight time.

        Errors if the ``zone_variant`` field is empty.

        See the `Rust documentation for daylight <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.daylight>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__


    .. js:function:: maybe_calculate_metazone(metazone_calculator, local_datetime)

        Sets the metazone based on the time zone and the local timestamp.

        See the `Rust documentation for maybe_calculate_metazone <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#method.maybe_calculate_metazone>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetazoneCalculator.html#method.compute_metazone_from_time_zone>`__


.. js:class:: ICU4XMetazoneCalculator

    See the `Rust documentation for MetazoneCalculator <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetazoneCalculator.html>`__ for more information.


    .. js:function:: create(provider)

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetazoneCalculator.html#method.try_new_unstable>`__ for more information.

