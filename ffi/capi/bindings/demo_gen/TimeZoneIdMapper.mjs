import { TimeZoneIdMapper } from "icu4x"
export function ianaToBcp47(value) {
    return (function (...args) { return args[0].ianaToBcp47(...args.slice(1)) }).apply(
        null,
        [
            TimeZoneIdMapper.create.apply(
                null,
                [
                ]
            ),
            value
        ]
    );
}
export function normalizeIana(value) {
    return (function (...args) { return args[0].normalizeIana(...args.slice(1)) }).apply(
        null,
        [
            TimeZoneIdMapper.create.apply(
                null,
                [
                ]
            ),
            value
        ]
    );
}
export function canonicalizeIana(value) {
    return (function (...args) { return args[0].canonicalizeIana(...args.slice(1)) }).apply(
        null,
        [
            TimeZoneIdMapper.create.apply(
                null,
                [
                ]
            ),
            value
        ]
    );
}
export function findCanonicalIanaFromBcp47(value) {
    return (function (...args) { return args[0].findCanonicalIanaFromBcp47(...args.slice(1)) }).apply(
        null,
        [
            TimeZoneIdMapper.create.apply(
                null,
                [
                ]
            ),
            value
        ]
    );
}
