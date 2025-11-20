package org.unicode.icu4x

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class ExemplarCharsTest {
    @Test
    fun testFormatter() {
        val locale = Locale.fromString("bn").getOrThrow()
        val exemplar = ExemplarCharacters.createMain(locale).getOrThrow()
        assert(exemplar.contains("ক"))
    }
}
