import { CaseMapper } from "icu4x"
import { DataProvider } from "icu4x"
import { Locale } from "icu4x"
import { TitlecaseOptions } from "icu4x"
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
