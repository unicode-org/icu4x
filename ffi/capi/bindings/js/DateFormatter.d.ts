// generated by diplomat-tool
import type { Calendar } from "./Calendar"
import type { DataProvider } from "./DataProvider"
import type { Date } from "./Date"
import type { DateTimeFormatterLoadError } from "./DateTimeFormatterLoadError"
import type { DateTimeLength } from "./DateTimeLength"
import type { IsoDate } from "./IsoDate"
import type { Locale } from "./Locale"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/** An ICU4X DateFormatter object capable of formatting a [`Date`] as a string,
*using some calendar specified at runtime in the locale.
*
*See the [Rust documentation for `DateTimeFormatter`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html) for more information.
*
*Additional information: [1](https://docs.rs/icu/latest/icu/datetime/fieldsets/struct.YMD.html)
*/


export class DateFormatter {
    
    get ffiValue(): pointer;

    static createWithLength(locale: Locale, length: DateTimeLength): DateFormatter;

    static createWithLengthAndProvider(provider: DataProvider, locale: Locale, length: DateTimeLength): DateFormatter;

    format(value: Date): string;

    formatIso(value: IsoDate): string;

    calendar(): Calendar;
}