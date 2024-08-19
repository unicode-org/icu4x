import { DataProvider } from "icu4x"
import { Locale } from "icu4x"
import { TitlecaseMapper } from "icu4x"
import { TitlecaseOptions } from "icu4x"
export function titlecaseSegment() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].titlecaseSegment(...args.slice(1)) }).apply(
        null,
        [
            TitlecaseMapper.create.apply(
                null,
                [
                    DataProvider.compiled.apply(
                        null,
                        [
                        ]
                    )
                ]
            ),
            terminusArgs[0],
            Locale.fromString.apply(
                null,
                [
                    terminusArgs[1]
                ]
            ),
            (function (...args) {
                return new TitlecaseOptions(...args);
            }).apply(
                null,
                [
                    terminusArgs[2],
                    terminusArgs[3]
                ]
            )
        ]
    );
}
