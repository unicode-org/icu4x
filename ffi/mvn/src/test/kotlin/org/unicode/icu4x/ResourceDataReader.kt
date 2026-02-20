package org.unicode.icu4x

import java.io.BufferedReader
import java.io.InputStreamReader
import java.nio.charset.StandardCharsets
import java.util.stream.Collectors

class ResourceDataReader {

    companion object {
        val LINES = readTestLines()

        fun readTestLines() : List<String> {
            ResourceDataReader.javaClass.getResourceAsStream("/pg2600.txt").use {
                InputStreamReader(it, StandardCharsets.UTF_8).use {
                    BufferedReader(it).use {
                        return it.lines().collect(Collectors.toList())
                    }
                }
            }
        }

        /*
        fun testDumpLines() {
            System.out.println("==================")
            for (line in LINES.orEmpty()) {
                System.out.println(line)
            }
            System.out.println("==================")
        }
        */
    }
}
