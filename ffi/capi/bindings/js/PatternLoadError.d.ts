// generated by diplomat-tool
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";

// Base enumerator definition
/** Additional information: [1](https://docs.rs/icu/latest/icu/datetime/enum.PatternLoadError.html), [2](https://docs.rs/icu/latest/icu/provider/struct.DataError.html), [3](https://docs.rs/icu/latest/icu/provider/enum.DataErrorKind.html)
*/
export class PatternLoadError {
    constructor(value : PatternLoadError | string);

    get value() : string;

    get ffiValue() : number;

    static Unknown : PatternLoadError;
    static UnsupportedField : PatternLoadError;
    static DuplicateField : PatternLoadError;
    static TypeTooSpecific : PatternLoadError;
    static DataMarkerNotFound : PatternLoadError;
    static DataIdentifierNotFound : PatternLoadError;
    static DataInvalidRequest : PatternLoadError;
    static DataInconsistentData : PatternLoadError;
    static DataDowncast : PatternLoadError;
    static DataDeserialize : PatternLoadError;
    static DataCustom : PatternLoadError;
    static DataIo : PatternLoadError;
}