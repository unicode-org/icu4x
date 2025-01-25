export * as lib from "icu4x";
import * as CaseMapperDemo from "./CaseMapper.mjs";
export * as CaseMapperDemo from "./CaseMapper.mjs";
import * as TitlecaseMapperDemo from "./TitlecaseMapper.mjs";
export * as TitlecaseMapperDemo from "./TitlecaseMapper.mjs";
import * as DateDemo from "./Date.mjs";
export * as DateDemo from "./Date.mjs";
import * as DateFormatterDemo from "./DateFormatter.mjs";
export * as DateFormatterDemo from "./DateFormatter.mjs";
import * as DateTimeFormatterDemo from "./DateTimeFormatter.mjs";
export * as DateTimeFormatterDemo from "./DateTimeFormatter.mjs";
import * as GregorianDateFormatterDemo from "./GregorianDateFormatter.mjs";
export * as GregorianDateFormatterDemo from "./GregorianDateFormatter.mjs";
import * as GregorianDateTimeFormatterDemo from "./GregorianDateTimeFormatter.mjs";
export * as GregorianDateTimeFormatterDemo from "./GregorianDateTimeFormatter.mjs";
import * as TimeFormatterDemo from "./TimeFormatter.mjs";
export * as TimeFormatterDemo from "./TimeFormatter.mjs";
import * as FixedDecimalFormatterDemo from "./FixedDecimalFormatter.mjs";
export * as FixedDecimalFormatterDemo from "./FixedDecimalFormatter.mjs";
import * as SignedFixedDecimalDemo from "./SignedFixedDecimal.mjs";
export * as SignedFixedDecimalDemo from "./SignedFixedDecimal.mjs";
import * as ListFormatterDemo from "./ListFormatter.mjs";
export * as ListFormatterDemo from "./ListFormatter.mjs";
import * as LocaleDemo from "./Locale.mjs";
export * as LocaleDemo from "./Locale.mjs";
import * as NeoDateTimeFormatterDemo from "./NeoDateTimeFormatter.mjs";
export * as NeoDateTimeFormatterDemo from "./NeoDateTimeFormatter.mjs";
import * as ComposingNormalizerDemo from "./ComposingNormalizer.mjs";
export * as ComposingNormalizerDemo from "./ComposingNormalizer.mjs";
import * as DecomposingNormalizerDemo from "./DecomposingNormalizer.mjs";
export * as DecomposingNormalizerDemo from "./DecomposingNormalizer.mjs";
import * as TimeZoneInfoDemo from "./TimeZoneInfo.mjs";
export * as TimeZoneInfoDemo from "./TimeZoneInfo.mjs";
import * as TimeZoneIdMapperDemo from "./TimeZoneIdMapper.mjs";
export * as TimeZoneIdMapperDemo from "./TimeZoneIdMapper.mjs";
import * as TimeZoneIdMapperWithFastCanonicalizationDemo from "./TimeZoneIdMapperWithFastCanonicalization.mjs";
export * as TimeZoneIdMapperWithFastCanonicalizationDemo from "./TimeZoneIdMapperWithFastCanonicalization.mjs";
import * as GregorianZonedDateTimeFormatterDemo from "./GregorianZonedDateTimeFormatter.mjs";
export * as GregorianZonedDateTimeFormatterDemo from "./GregorianZonedDateTimeFormatter.mjs";
import * as ZonedDateTimeFormatterDemo from "./ZonedDateTimeFormatter.mjs";
export * as ZonedDateTimeFormatterDemo from "./ZonedDateTimeFormatter.mjs";
import * as AnyCalendarKindDemo from "./AnyCalendarKind.mjs";
export * as AnyCalendarKindDemo from "./AnyCalendarKind.mjs";

import RenderTerminiWordSegmenter from "./WordSegmenter.mjs";


let termini = Object.assign({
    "CaseMapper.lowercase": {
        func: CaseMapperDemo.lowercase,
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.lowercase",
        parameters: [
            
            {
                name: "S",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Locale:Name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "CaseMapper.uppercase": {
        func: CaseMapperDemo.uppercase,
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.uppercase",
        parameters: [
            
            {
                name: "S",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Locale:Name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "CaseMapper.titlecaseSegmentWithOnlyCaseData": {
        func: CaseMapperDemo.titlecaseSegmentWithOnlyCaseData,
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.titlecaseSegmentWithOnlyCaseData",
        parameters: [
            
            {
                name: "S",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Locale:Name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Options:LeadingAdjustment",
                type: "LeadingAdjustment",
                typeUse: "enumerator"
            },
            
            {
                name: "Options:TrailingCase",
                type: "TrailingCase",
                typeUse: "enumerator"
            }
            
        ]
    },

    "CaseMapper.fold": {
        func: CaseMapperDemo.fold,
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.fold",
        parameters: [
            
            {
                name: "S",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "CaseMapper.foldTurkic": {
        func: CaseMapperDemo.foldTurkic,
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.foldTurkic",
        parameters: [
            
            {
                name: "S",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "TitlecaseMapper.titlecaseSegment": {
        func: TitlecaseMapperDemo.titlecaseSegment,
        // For avoiding webpacking minifying issues:
        funcName: "TitlecaseMapper.titlecaseSegment",
        parameters: [
            
            {
                name: "S",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Locale:Name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Options:LeadingAdjustment",
                type: "LeadingAdjustment",
                typeUse: "enumerator"
            },
            
            {
                name: "Options:TrailingCase",
                type: "TrailingCase",
                typeUse: "enumerator"
            }
            
        ]
    },

    "Date.monthCode": {
        func: DateDemo.monthCode,
        // For avoiding webpacking minifying issues:
        funcName: "Date.monthCode",
        parameters: [
            
            {
                name: "Self:Year",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Self:Month",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Self:Day",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Self:Calendar:Locale:Name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Date.era": {
        func: DateDemo.era,
        // For avoiding webpacking minifying issues:
        funcName: "Date.era",
        parameters: [
            
            {
                name: "Self:Year",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Self:Month",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Self:Day",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Self:Calendar:Locale:Name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "DateFormatter.format": {
        func: DateFormatterDemo.format,
        // For avoiding webpacking minifying issues:
        funcName: "DateFormatter.format",
        parameters: [
            
            {
                name: "Self:Locale:Name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Self:Length",
                type: "DateTimeLength",
                typeUse: "enumerator"
            },
            
            {
                name: "Value:Year",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Value:Month",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Value:Day",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Value:Calendar:Locale:Name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "DateFormatter.formatIso": {
        func: DateFormatterDemo.formatIso,
        // For avoiding webpacking minifying issues:
        funcName: "DateFormatter.formatIso",
        parameters: [
            
            {
                name: "Self:Locale:Name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Self:Length",
                type: "DateTimeLength",
                typeUse: "enumerator"
            },
            
            {
                name: "Value:Year",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Value:Month",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Value:Day",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "DateTimeFormatter.format": {
        func: DateTimeFormatterDemo.format,
        // For avoiding webpacking minifying issues:
        funcName: "DateTimeFormatter.format",
        parameters: [
            
            {
                name: "Self:Locale:Name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Self:Length",
                type: "DateTimeLength",
                typeUse: "enumerator"
            },
            
            {
                name: "Date:Year",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Date:Month",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Date:Day",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Date:Calendar:Locale:Name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Time:Hour",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Minute",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Second",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Nanosecond",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "DateTimeFormatter.formatIso": {
        func: DateTimeFormatterDemo.formatIso,
        // For avoiding webpacking minifying issues:
        funcName: "DateTimeFormatter.formatIso",
        parameters: [
            
            {
                name: "Self:Locale:Name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Self:Length",
                type: "DateTimeLength",
                typeUse: "enumerator"
            },
            
            {
                name: "Date:Year",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Date:Month",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Date:Day",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Hour",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Minute",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Second",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Nanosecond",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "GregorianDateFormatter.formatIso": {
        func: GregorianDateFormatterDemo.formatIso,
        // For avoiding webpacking minifying issues:
        funcName: "GregorianDateFormatter.formatIso",
        parameters: [
            
            {
                name: "Self:Locale:Name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Self:Length",
                type: "DateTimeLength",
                typeUse: "enumerator"
            },
            
            {
                name: "Value:Year",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Value:Month",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Value:Day",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "GregorianDateTimeFormatter.formatIso": {
        func: GregorianDateTimeFormatterDemo.formatIso,
        // For avoiding webpacking minifying issues:
        funcName: "GregorianDateTimeFormatter.formatIso",
        parameters: [
            
            {
                name: "Self:Locale:Name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Self:Length",
                type: "DateTimeLength",
                typeUse: "enumerator"
            },
            
            {
                name: "Date:Year",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Date:Month",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Date:Day",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Hour",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Minute",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Second",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Nanosecond",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "TimeFormatter.format": {
        func: TimeFormatterDemo.format,
        // For avoiding webpacking minifying issues:
        funcName: "TimeFormatter.format",
        parameters: [
            
            {
                name: "Self:Locale:Name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Self:Length",
                type: "DateTimeLength",
                typeUse: "enumerator"
            },
            
            {
                name: "Value:Hour",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Value:Minute",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Value:Second",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Value:Nanosecond",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "FixedDecimalFormatter.format": {
        func: FixedDecimalFormatterDemo.format,
        // For avoiding webpacking minifying issues:
        funcName: "FixedDecimalFormatter.format",
        parameters: [
            
            {
                name: "Self:Locale:Name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Self:GroupingStrategy",
                type: "FixedDecimalGroupingStrategy",
                typeUse: "enumerator"
            },
            
            {
                name: "Value:F",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Value:Magnitude",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "SignedFixedDecimal.toString": {
        func: SignedFixedDecimalDemo.toString,
        // For avoiding webpacking minifying issues:
        funcName: "SignedFixedDecimal.toString",
        parameters: [
            
            {
                name: "Self:F",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Self:Magnitude",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "ListFormatter.format": {
        func: ListFormatterDemo.format,
        // For avoiding webpacking minifying issues:
        funcName: "ListFormatter.format",
        parameters: [
            
            {
                name: "Self:Locale:Name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Self:Length",
                type: "ListLength",
                typeUse: "enumerator"
            },
            
            {
                name: "List",
                type: "Array<string>",
                typeUse: "Array<string>"
            }
            
        ]
    },

    "Locale.basename": {
        func: LocaleDemo.basename,
        // For avoiding webpacking minifying issues:
        funcName: "Locale.basename",
        parameters: [
            
            {
                name: "Self:Name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Locale.getUnicodeExtension": {
        func: LocaleDemo.getUnicodeExtension,
        // For avoiding webpacking minifying issues:
        funcName: "Locale.getUnicodeExtension",
        parameters: [
            
            {
                name: "Self:Name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "S",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Locale.language": {
        func: LocaleDemo.language,
        // For avoiding webpacking minifying issues:
        funcName: "Locale.language",
        parameters: [
            
            {
                name: "Self:Name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Locale.region": {
        func: LocaleDemo.region,
        // For avoiding webpacking minifying issues:
        funcName: "Locale.region",
        parameters: [
            
            {
                name: "Self:Name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Locale.script": {
        func: LocaleDemo.script,
        // For avoiding webpacking minifying issues:
        funcName: "Locale.script",
        parameters: [
            
            {
                name: "Self:Name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Locale.normalize": {
        func: LocaleDemo.normalize,
        // For avoiding webpacking minifying issues:
        funcName: "Locale.normalize",
        parameters: [
            
            {
                name: "S",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Locale.toString": {
        func: LocaleDemo.toString,
        // For avoiding webpacking minifying issues:
        funcName: "Locale.toString",
        parameters: [
            
            {
                name: "Self:Name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "NeoDateTimeFormatter.formatIso": {
        func: NeoDateTimeFormatterDemo.formatIso,
        // For avoiding webpacking minifying issues:
        funcName: "NeoDateTimeFormatter.formatIso",
        parameters: [
            
            {
                name: "Self:Locale:Name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Self:Builder:Length",
                type: "NeoDateTimeLength",
                typeUse: "enumerator"
            },
            
            {
                name: "Self:Builder:DateFields",
                type: "DateFields",
                typeUse: "enumerator"
            },
            
            {
                name: "Self:Builder:TimePrecision",
                type: "TimePrecision",
                typeUse: "enumerator"
            },
            
            {
                name: "Self:Builder:ZoneStyle",
                type: "ZoneStyle",
                typeUse: "enumerator"
            },
            
            {
                name: "Self:Builder:Alignment",
                type: "DateTimeAlignment",
                typeUse: "enumerator"
            },
            
            {
                name: "Self:Builder:YearStyle",
                type: "YearStyle",
                typeUse: "enumerator"
            },
            
            {
                name: "Date:Year",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Date:Month",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Date:Day",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Hour",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Minute",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Second",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Nanosecond",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "ComposingNormalizer.normalize": {
        func: ComposingNormalizerDemo.normalize,
        // For avoiding webpacking minifying issues:
        funcName: "ComposingNormalizer.normalize",
        parameters: [
            
            {
                name: "S",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "DecomposingNormalizer.normalize": {
        func: DecomposingNormalizerDemo.normalize,
        // For avoiding webpacking minifying issues:
        funcName: "DecomposingNormalizer.normalize",
        parameters: [
            
            {
                name: "S",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "TimeZoneInfo.timeZoneId": {
        func: TimeZoneInfoDemo.timeZoneId,
        // For avoiding webpacking minifying issues:
        funcName: "TimeZoneInfo.timeZoneId",
        parameters: [
            
            {
                name: "Self:Bcp47Id",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Self:OffsetSeconds",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Self:Dst",
                type: "boolean",
                typeUse: "boolean"
            }
            
        ]
    },

    "TimeZoneIdMapper.ianaToBcp47": {
        func: TimeZoneIdMapperDemo.ianaToBcp47,
        // For avoiding webpacking minifying issues:
        funcName: "TimeZoneIdMapper.ianaToBcp47",
        parameters: [
            
            {
                name: "Value",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "TimeZoneIdMapper.normalizeIana": {
        func: TimeZoneIdMapperDemo.normalizeIana,
        // For avoiding webpacking minifying issues:
        funcName: "TimeZoneIdMapper.normalizeIana",
        parameters: [
            
            {
                name: "Value",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "TimeZoneIdMapper.canonicalizeIana": {
        func: TimeZoneIdMapperDemo.canonicalizeIana,
        // For avoiding webpacking minifying issues:
        funcName: "TimeZoneIdMapper.canonicalizeIana",
        parameters: [
            
            {
                name: "Value",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "TimeZoneIdMapper.findCanonicalIanaFromBcp47": {
        func: TimeZoneIdMapperDemo.findCanonicalIanaFromBcp47,
        // For avoiding webpacking minifying issues:
        funcName: "TimeZoneIdMapper.findCanonicalIanaFromBcp47",
        parameters: [
            
            {
                name: "Value",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "TimeZoneIdMapperWithFastCanonicalization.canonicalizeIana": {
        func: TimeZoneIdMapperWithFastCanonicalizationDemo.canonicalizeIana,
        // For avoiding webpacking minifying issues:
        funcName: "TimeZoneIdMapperWithFastCanonicalization.canonicalizeIana",
        parameters: [
            
            {
                name: "Value",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "TimeZoneIdMapperWithFastCanonicalization.canonicalIanaFromBcp47": {
        func: TimeZoneIdMapperWithFastCanonicalizationDemo.canonicalIanaFromBcp47,
        // For avoiding webpacking minifying issues:
        funcName: "TimeZoneIdMapperWithFastCanonicalization.canonicalIanaFromBcp47",
        parameters: [
            
            {
                name: "Value",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "GregorianZonedDateTimeFormatter.formatIso": {
        func: GregorianZonedDateTimeFormatterDemo.formatIso,
        // For avoiding webpacking minifying issues:
        funcName: "GregorianZonedDateTimeFormatter.formatIso",
        parameters: [
            
            {
                name: "Self:Locale:Name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Self:Length",
                type: "DateTimeLength",
                typeUse: "enumerator"
            },
            
            {
                name: "Date:Year",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Date:Month",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Date:Day",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Hour",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Minute",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Second",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Nanosecond",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Zone:Bcp47Id",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Zone:OffsetSeconds",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Zone:Dst",
                type: "boolean",
                typeUse: "boolean"
            }
            
        ]
    },

    "ZonedDateTimeFormatter.format": {
        func: ZonedDateTimeFormatterDemo.format,
        // For avoiding webpacking minifying issues:
        funcName: "ZonedDateTimeFormatter.format",
        parameters: [
            
            {
                name: "Self:Locale:Name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Self:Length",
                type: "DateTimeLength",
                typeUse: "enumerator"
            },
            
            {
                name: "Date:Year",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Date:Month",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Date:Day",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Date:Calendar:Locale:Name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Time:Hour",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Minute",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Second",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Nanosecond",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Zone:Bcp47Id",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Zone:OffsetSeconds",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Zone:Dst",
                type: "boolean",
                typeUse: "boolean"
            }
            
        ]
    },

    "ZonedDateTimeFormatter.formatIso": {
        func: ZonedDateTimeFormatterDemo.formatIso,
        // For avoiding webpacking minifying issues:
        funcName: "ZonedDateTimeFormatter.formatIso",
        parameters: [
            
            {
                name: "Self:Locale:Name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Self:Length",
                type: "DateTimeLength",
                typeUse: "enumerator"
            },
            
            {
                name: "Date:Year",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Date:Month",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Date:Day",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Hour",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Minute",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Second",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Time:Nanosecond",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Zone:Bcp47Id",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "Zone:OffsetSeconds",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "Zone:Dst",
                type: "boolean",
                typeUse: "boolean"
            }
            
        ]
    },

    "AnyCalendarKind.bcp47": {
        func: AnyCalendarKindDemo.bcp47,
        // For avoiding webpacking minifying issues:
        funcName: "AnyCalendarKind.bcp47",
        parameters: [
            
            {
                name: "Self",
                type: "AnyCalendarKind",
                typeUse: "enumerator"
            }
            
        ]
    }
}, RenderTerminiWordSegmenter);

export const RenderInfo = {
    "termini": termini
};