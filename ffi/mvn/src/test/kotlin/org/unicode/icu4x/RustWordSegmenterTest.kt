package org.unicode.icu4x

import org.junit.jupiter.api.Test

class RustWordSegmenterTest {
    @Test
    fun testRustWordSegmenter() {
        val locale = Locale.fromString("en").getOrThrow()
        val ws = WordSegmenter.createAutoWithContentLocale(locale).getOrThrow()
        var lineCount = 0;
        var wordCount = 0;
        val start = System.nanoTime()
        for (line in ResourceDataReader.LINES.orEmpty()) {
            lineCount++
            val seg = ws.segment(line)
            while (true) {
                val pos = seg.next()
                if (pos == -1) {
                    break
                }
                wordCount++
            }
        }
        val end = System.nanoTime()
        System.out.printf("    Rust (%,d :: %,d) : %,d ns%n", lineCount, wordCount, end - start)
    }
}
