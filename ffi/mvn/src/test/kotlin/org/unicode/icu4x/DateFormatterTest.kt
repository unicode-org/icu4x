package org.unicode.icu4x

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class DateTimeFormatterTest {
    @Test
    fun testFormatter() {
        val locale = Locale.fromString("de-u-ca-islamicc").getOrThrow()


        val zonedDateTimeIso = ZonedIsoDateTime.strictFromString(
          "2025-01-15T14:32:12.34+01[Europe/Zurich]",
          IanaParser.create(),
        ).getOrThrow();

        val dt = DateTimeFormatter.createYmdet(locale, DateTimeLength.Long, null, null, null).getOrThrow();

        assertEquals(
          dt.formatIso(
            zonedDateTimeIso.date,
            zonedDateTimeIso.time,
          ),
          "Mittwoch, 15. Radschab 1446 AH um 14:32:12",
        );

        // The nulls can be removed after https://github.com/rust-diplomat/diplomat/issues/1070
        assertEquals(
          ZonedDateTimeFormatter.createGenericLong(
            locale,
            dt,
          ).getOrThrow().formatIso(
            zonedDateTimeIso.date,
            zonedDateTimeIso.time,
            zonedDateTimeIso.zone,
          ).getOrThrow(),
          "Mittwoch, 15. Radschab 1446 AH um 14:32:12 Mitteleuropäische Zeit",
        );
    }
}
