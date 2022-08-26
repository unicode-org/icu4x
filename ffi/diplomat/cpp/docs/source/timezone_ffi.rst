``timezone::ffi``
=================

.. cpp:class:: ICU4XCustomTimeZone

    See the `Rust documentation for CustomTimeZone <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XCustomTimeZone, ICU4XError> create_from_str(const std::string_view s)

    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> try_set_gmt_offset_seconds(int32_t offset_seconds)

        Sets the ``gmt_offset`` field from offset seconds.

        Errors if the offset seconds are out of range.

        See the `Rust documentation for try_from_offset_seconds <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.try_from_offset_seconds>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html>`__


    .. cpp:function:: void clear_gmt_offset()

        Clears the ``gmt_offset`` field.

        See the `Rust documentation for offset_seconds <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.offset_seconds>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html>`__


    .. cpp:function:: diplomat::result<int32_t, std::monostate> gmt_offset_seconds() const

        Returns the value of the ``gmt_offset`` field as offset seconds.

        Errors if the ``gmt_offset`` field is empty.

        See the `Rust documentation for offset_seconds <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.offset_seconds>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html>`__


    .. cpp:function:: diplomat::result<bool, std::monostate> is_gmt_offset_positive() const

        Returns whether the ``gmt_offset`` field is positive.

        Errors if the ``gmt_offset`` field is empty.

        See the `Rust documentation for is_positive <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.is_positive>`__ for more information.


    .. cpp:function:: diplomat::result<bool, std::monostate> is_gmt_offset_zero() const

        Returns whether the ``gmt_offset`` field is zero.

        Errors if the ``gmt_offset`` field is empty (which is not the same as zero).

        See the `Rust documentation for is_zero <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.is_zero>`__ for more information.


    .. cpp:function:: diplomat::result<bool, std::monostate> gmt_offset_has_minutes() const

        Returns whether the ``gmt_offset`` field has nonzero minutes.

        Errors if the ``gmt_offset`` field is empty.

        See the `Rust documentation for has_minutes <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.has_minutes>`__ for more information.


    .. cpp:function:: diplomat::result<bool, std::monostate> gmt_offset_has_seconds() const

        Returns whether the ``gmt_offset`` field has nonzero seconds.

        Errors if the ``gmt_offset`` field is empty.

        See the `Rust documentation for has_seconds <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.GmtOffset.html#method.has_seconds>`__ for more information.


    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> try_set_time_zone_id(const std::string_view id)

        Sets the ``time_zone_id`` field from a string.

        Errors if the string is not a valid BCP-47 time zone ID.

        See the `Rust documentation for time_zone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.TimeZoneBcp47Id.html>`__


    .. cpp:function:: void clear_time_zone_id()

        Clears the ``time_zone_id`` field.

        See the `Rust documentation for time_zone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.TimeZoneBcp47Id.html>`__


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, std::monostate> time_zone_id_to_writeable(W& write) const

        Writes the value of the ``time_zone_id`` field as a string.

        Errors if the ``time_zone_id`` field is empty.

        See the `Rust documentation for time_zone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.TimeZoneBcp47Id.html>`__


    .. cpp:function:: diplomat::result<std::string, std::monostate> time_zone_id() const

        Writes the value of the ``time_zone_id`` field as a string.

        Errors if the ``time_zone_id`` field is empty.

        See the `Rust documentation for time_zone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.TimeZoneBcp47Id.html>`__


    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> try_set_meta_zone_id(const std::string_view id)

        Sets the ``meta_zone_id`` field from a string.

        Errors if the string is not a valid BCP-47 meta zone ID.

        See the `Rust documentation for meta_zone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.meta_zone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneId.html>`__


    .. cpp:function:: void clear_meta_zone_id()

        Clears the ``meta_zone_id`` field.

        See the `Rust documentation for meta_zone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.meta_zone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneId.html>`__


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, std::monostate> meta_zone_id_to_writeable(W& write) const

        Writes the value of the ``meta_zone_id`` field as a string.

        Errors if the ``meta_zone_id`` field is empty.

        See the `Rust documentation for meta_zone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.meta_zone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneId.html>`__


    .. cpp:function:: diplomat::result<std::string, std::monostate> meta_zone_id() const

        Writes the value of the ``meta_zone_id`` field as a string.

        Errors if the ``meta_zone_id`` field is empty.

        See the `Rust documentation for meta_zone_id <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.meta_zone_id>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneId.html>`__


    .. cpp:function:: diplomat::result<std::monostate, ICU4XError> try_set_zone_variant(const std::string_view id)

        Sets the ``zone_variant`` field from a string.

        Errors if the string is not a valid zone variant.

        See the `Rust documentation for zone_variant <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html>`__


    .. cpp:function:: void clear_zone_variant()

        Clears the ``zone_variant`` field.

        See the `Rust documentation for zone_variant <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html>`__


    .. cpp:function:: template<typename W> diplomat::result<std::monostate, std::monostate> zone_variant_to_writeable(W& write) const

        Writes the value of the ``zone_variant`` field as a string.

        Errors if the ``zone_variant`` field is empty.

        See the `Rust documentation for zone_variant <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html>`__


    .. cpp:function:: diplomat::result<std::string, std::monostate> zone_variant() const

        Writes the value of the ``zone_variant`` field as a string.

        Errors if the ``zone_variant`` field is empty.

        See the `Rust documentation for zone_variant <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html>`__


    .. cpp:function:: void set_standard_time()

        Sets the ``zone_variant`` field to standard time.

        See the `Rust documentation for standard <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.standard>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__


    .. cpp:function:: void set_daylight_time()

        Sets the ``zone_variant`` field to daylight time.

        See the `Rust documentation for daylight <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.daylight>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__


    .. cpp:function:: diplomat::result<bool, std::monostate> is_standard_time() const

        Returns whether the ``zone_variant`` field is standard time.

        Errors if the ``zone_variant`` field is empty.

        See the `Rust documentation for standard <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.standard>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__


    .. cpp:function:: diplomat::result<bool, std::monostate> is_daylight_time() const

        Returns whether the ``zone_variant`` field is daylight time.

        Errors if the ``zone_variant`` field is empty.

        See the `Rust documentation for daylight <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.ZoneVariant.html#method.daylight>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant>`__


    .. cpp:function:: void maybe_set_meta_zone(const ICU4XIsoDateTime& local_datetime, const ICU4XMetaZoneCalculator& metazone_calculator)

        Sets the meta zone based on the time zone and the local timestamp.

        See the `Rust documentation for maybe_set_meta_zone <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.CustomTimeZone.html#method.maybe_set_meta_zone>`__ for more information.

        Additional information: `1 <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneCalculator.html#method.compute_metazone_from_timezone>`__


.. cpp:class:: ICU4XMetaZoneCalculator

    See the `Rust documentation for MetaZoneCalculator <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneCalculator.html>`__ for more information.


    .. cpp:function:: static diplomat::result<ICU4XMetaZoneCalculator, ICU4XError> try_new(const ICU4XDataProvider& provider)

        See the `Rust documentation for try_new_unstable <https://unicode-org.github.io/icu4x-docs/doc/icu/timezone/struct.MetaZoneCalculator.html#method.try_new_unstable>`__ for more information.

