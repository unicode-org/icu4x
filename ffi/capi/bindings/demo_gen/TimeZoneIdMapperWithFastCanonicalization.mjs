import { DataProvider } from "./js/DataProvider.mjs"
import { TimeZoneIdMapperWithFastCanonicalization } from "./js/TimeZoneIdMapperWithFastCanonicalization.mjs"
export function canonicalizeIana() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].canonicalizeIana(...args.slice(1)) }).apply(
        null,
        [
            TimeZoneIdMapperWithFastCanonicalization.create.apply(
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
export function canonicalIanaFromBcp47() {
    var terminusArgs = arguments;
    return (function (...args) { return args[0].canonicalIanaFromBcp47(...args.slice(1)) }).apply(
        null,
        [
            TimeZoneIdMapperWithFastCanonicalization.create.apply(
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
