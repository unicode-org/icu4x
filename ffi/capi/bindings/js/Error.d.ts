// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";

// Base enumerator definition
/** Legacy error
*
*Additional information: [1](https://docs.rs/icu/latest/icu/datetime/enum.DateTimeError.html), [2](https://docs.rs/icu/latest/icu/provider/struct.DataError.html), [3](https://docs.rs/icu/latest/icu/provider/enum.DataErrorKind.html)
*/
export class Error {
    constructor(value : Error | string);

    get value() : string;

    get ffiValue() : number;

    static UnknownError : Error;
    static DataMissingDataMarkerError : Error;
    static DataMissingLocaleError : Error;
    static DataNeedsLocaleError : Error;
    static DataExtraneousLocaleError : Error;
    static DataFilteredResourceError : Error;
    static DataMismatchedTypeError : Error;
    static DataCustomError : Error;
    static DataIoError : Error;
    static DataUnavailableBufferFormatError : Error;
    static DateTimePatternError : Error;
    static DateTimeMissingInputFieldError : Error;
    static DateTimeSkeletonError : Error;
    static DateTimeUnsupportedFieldError : Error;
    static DateTimeUnsupportedOptionsError : Error;
    static DateTimeMissingWeekdaySymbolError : Error;
    static DateTimeMissingMonthSymbolError : Error;
    static DateTimeFixedDecimalError : Error;
    static DateTimeMismatchedCalendarError : Error;
    static DateTimeDuplicateFieldError : Error;
    static DateTimeTooNarrowError : Error;
    static DateTimeMissingNamesError : Error;
    static DateTimeZoneInfoMissingFieldsError : Error;
}