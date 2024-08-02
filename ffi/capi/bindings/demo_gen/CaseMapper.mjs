import { CaseMapper } from "./CaseMapper.mjs"
import { DataProvider } from "./DataProvider.mjs"
import { Locale } from "./Locale.mjs"
import { TitlecaseOptions } from "./js/TitlecaseOptions.mjs"
export function lowercase() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].lowercase(...args.slice(1)) }).apply(
        null,
        [
            CaseMapper.create.apply(
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
            )
        ]
    );
}
export function uppercase() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].uppercase(...args.slice(1)) }).apply(
        null,
        [
            CaseMapper.create.apply(
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
            )
        ]
    );
}
export function titlecaseSegmentWithOnlyCaseData() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].titlecaseSegmentWithOnlyCaseData(...args.slice(1)) }).apply(
        null,
        [
            CaseMapper.create.apply(
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
export function fold() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].fold(...args.slice(1)) }).apply(
        null,
        [
            CaseMapper.create.apply(
                null,
                [
                    DataProvider.compiled.apply(
                        null,
                        [
                        ]
                    )
                ]
            ),
            terminusArgs[0]
        ]
    );
}
export function foldTurkic() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].foldTurkic(...args.slice(1)) }).apply(
        null,
        [
            CaseMapper.create.apply(
                null,
                [
                    DataProvider.compiled.apply(
                        null,
                        [
                        ]
                    )
                ]
            ),
            terminusArgs[0]
        ]
    );
}
