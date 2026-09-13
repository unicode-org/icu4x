package org.unicode.icu4x

import com.ibm.icu.segmenter.LocalizedSegmenter
import com.ibm.icu.text.BreakIterator

import org.junit.jupiter.api.Test

class IcuWordSegmenterTest {
    @Test
    fun testIcuWordSegmenterRebuilt() {
        val locale = java.util.Locale.forLanguageTag("en-US")
        var lineCount = 0;
        var wordCount = 0;
        val start = System.nanoTime()
        for (line in ResourceDataReader.LINES.orEmpty()) {
            lineCount++
            val bi = BreakIterator.getWordInstance(locale);
            bi.setText(line);
            while (true) {
                val pos = bi.next()
                if (pos == -1) {
                    break
                }
                wordCount++
            }
        }
        val end = System.nanoTime()
        System.out.printf("    ICU4J(%,d :: %,d) : %,d ns // rebuilt BreakIterator%n", lineCount, wordCount, end - start)
    }

    @Test
    fun testIcuWordSegmenterReused() {
        val locale = java.util.Locale.forLanguageTag("en-US")
        val bi = BreakIterator.getWordInstance(locale);
        var lineCount = 0;
        var wordCount = 0;
        val start = System.nanoTime()
        for (line in ResourceDataReader.LINES.orEmpty()) {
            lineCount++
            bi.setText(line);
            while (true) {
                val pos = bi.next()
                if (pos == -1) {
                    break
                }
                wordCount++
            }
        }
        val end = System.nanoTime()
        System.out.printf("    ICU4J(%,d :: %,d) : %,d ns // reused BreakIterator%n", lineCount, wordCount, end - start)
    }

    @Test
    fun testIcuWordSegmenterNewApi() {
        val locale = java.util.Locale.forLanguageTag("en-US")
        val ws = LocalizedSegmenter.builder()
                .setLocale(locale)
                .setSegmentationType(LocalizedSegmenter.SegmentationType.WORD)
                .build();
        var lineCount = 0;
        var wordCount = 0;
        val start = System.nanoTime()
        for (line in ResourceDataReader.LINES.orEmpty()) {
            lineCount++
            val seg = ws.segment(line)
            seg.boundaries().forEach({ wordCount++ })
        }
        val end = System.nanoTime()
        System.out.printf("    ICU4J(%,d :: %,d) : %,d ns // new Segmenter API%n", lineCount, wordCount, end - start)
    }
}
