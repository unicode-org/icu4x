// generated by diplomat-tool
import type { CalendarError } from "./CalendarError"
import type { Rfc9557ParseError } from "./Rfc9557ParseError"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";



/**
 * An ICU4X Time object representing a time in terms of hour, minute, second, nanosecond
 *
 * See the [Rust documentation for `Time`](https://docs.rs/icu/2.0.0/icu/time/struct.Time.html) for more information.
 */
export class Time {
    /** @internal */
    get ffiValue(): pointer;


    /**
     * Creates a new {@link Time} from an IXDTF string.
     *
     * See the [Rust documentation for `try_from_str`](https://docs.rs/icu/2.0.0/icu/time/struct.Time.html#method.try_from_str) for more information.
     */
    static fromString(v: string): Time;

    /**
     * Creates a new {@link Time} representing the start of the day (00:00:00.000).
     *
     * See the [Rust documentation for `start_of_day`](https://docs.rs/icu/2.0.0/icu/time/struct.Time.html#method.start_of_day) for more information.
     */
    static startOfDay(): Time;

    /**
     * Creates a new {@link Time} representing noon (12:00:00.000).
     *
     * See the [Rust documentation for `noon`](https://docs.rs/icu/2.0.0/icu/time/struct.Time.html#method.noon) for more information.
     */
    static noon(): Time;

    /**
     * Returns the hour in this time
     *
     * See the [Rust documentation for `hour`](https://docs.rs/icu/2.0.0/icu/time/struct.Time.html#structfield.hour) for more information.
     */
    get hour(): number;

    /**
     * Returns the minute in this time
     *
     * See the [Rust documentation for `minute`](https://docs.rs/icu/2.0.0/icu/time/struct.Time.html#structfield.minute) for more information.
     */
    get minute(): number;

    /**
     * Returns the second in this time
     *
     * See the [Rust documentation for `second`](https://docs.rs/icu/2.0.0/icu/time/struct.Time.html#structfield.second) for more information.
     */
    get second(): number;

    /**
     * Returns the subsecond in this time as nanoseconds
     *
     * See the [Rust documentation for `subsecond`](https://docs.rs/icu/2.0.0/icu/time/struct.Time.html#structfield.subsecond) for more information.
     */
    get subsecond(): number;

    /**
     * Creates a new {@link Time} given field values
     *
     * See the [Rust documentation for `try_new`](https://docs.rs/icu/2.0.0/icu/time/struct.Time.html#method.try_new) for more information.
     */
    constructor(hour: number, minute: number, second: number, subsecond: number);
}