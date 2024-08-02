import { DataProvider } from "./DataProvider.mjs"
import { Locale } from "./Locale.mjs"
import { TitlecaseMapper } from "./TitlecaseMapper.mjs"
import { TitlecaseOptions } from "./js/TitlecaseOptions.mjs"
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
                let out = new TitlecaseOptions();
                
                out.leadingAdjustment = args[0];
                
                out.trailingCase = args[1];
                
                return out;
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
