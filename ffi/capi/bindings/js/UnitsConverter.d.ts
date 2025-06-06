// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";



/**
 * An ICU4X Units Converter object, capable of converting between two {@link MeasureUnit}s.
 *
 * You can create an instance of this object using {@link UnitsConverterFactory} by calling the `converter` method.
 *
 * See the [Rust documentation for `UnitsConverter`](https://docs.rs/icu/2.0.0/icu/experimental/units/converter/struct.UnitsConverter.html) for more information.
 */
export class UnitsConverter {
    /** @internal */
    get ffiValue(): pointer;
    /** @internal */
    constructor();


    /**
     * Converts the input value from the input unit to the output unit (that have been used to create this converter).
     * NOTE:
     * The conversion using floating-point operations is not as accurate as the conversion using ratios.
     *
     * See the [Rust documentation for `convert`](https://docs.rs/icu/2.0.0/icu/experimental/units/converter/struct.UnitsConverter.html#method.convert) for more information.
     */
    convertNumber(value: number): number;

    /**
     * Clones the current {@link UnitsConverter} object.
     *
     * See the [Rust documentation for `clone`](https://docs.rs/icu/2.0.0/icu/experimental/units/converter/struct.UnitsConverter.html#method.clone) for more information.
     */
    clone(): UnitsConverter;
}