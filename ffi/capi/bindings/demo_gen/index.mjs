export * as lib from "icu4x";
import * as CaseMapperDemo from "./CaseMapper.mjs";
export * as CaseMapperDemo from "./CaseMapper.mjs";
import * as TitlecaseMapperDemo from "./TitlecaseMapper.mjs";
export * as TitlecaseMapperDemo from "./TitlecaseMapper.mjs";
import * as DateDemo from "./Date.mjs";
export * as DateDemo from "./Date.mjs";
import * as DateTimeDemo from "./DateTime.mjs";
export * as DateTimeDemo from "./DateTime.mjs";
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
import * as LocaleDisplayNamesFormatterDemo from "./LocaleDisplayNamesFormatter.mjs";
export * as LocaleDisplayNamesFormatterDemo from "./LocaleDisplayNamesFormatter.mjs";
import * as RegionDisplayNamesDemo from "./RegionDisplayNames.mjs";
export * as RegionDisplayNamesDemo from "./RegionDisplayNames.mjs";
import * as FixedDecimalDemo from "./FixedDecimal.mjs";
export * as FixedDecimalDemo from "./FixedDecimal.mjs";
import * as ListFormatterDemo from "./ListFormatter.mjs";
export * as ListFormatterDemo from "./ListFormatter.mjs";
import * as LocaleDemo from "./Locale.mjs";
export * as LocaleDemo from "./Locale.mjs";
import * as ComposingNormalizerDemo from "./ComposingNormalizer.mjs";
export * as ComposingNormalizerDemo from "./ComposingNormalizer.mjs";
import * as DecomposingNormalizerDemo from "./DecomposingNormalizer.mjs";
export * as DecomposingNormalizerDemo from "./DecomposingNormalizer.mjs";
import * as CustomTimeZoneDemo from "./CustomTimeZone.mjs";
export * as CustomTimeZoneDemo from "./CustomTimeZone.mjs";
import * as TimeZoneFormatterDemo from "./TimeZoneFormatter.mjs";
export * as TimeZoneFormatterDemo from "./TimeZoneFormatter.mjs";
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


export const RenderInfo = {
    termini: {
        "CaseMapper.lowercase": {
            func: CaseMapperDemo.lowercase,
            // For avoiding webpacking minifying issues:
            funcName: "CaseMapper.lowercase",
            parameters: [
                
                {
                    name: "S",
                    type: "string"
                },
                
                {
                    name: "Name",
                    type: "string"
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
                    type: "string"
                },
                
                {
                    name: "Name",
                    type: "string"
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
                    type: "string"
                },
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "LeadingAdjustment",
                    type: "LeadingAdjustment"
                },
                
                {
                    name: "TrailingCase",
                    type: "TrailingCase"
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
                    type: "string"
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
                    type: "string"
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
                    type: "string"
                },
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "LeadingAdjustment",
                    type: "LeadingAdjustment"
                },
                
                {
                    name: "TrailingCase",
                    type: "TrailingCase"
                }
                
            ]
        },
        
        "Date.monthCode": {
            func: DateDemo.monthCode,
            // For avoiding webpacking minifying issues:
            funcName: "Date.monthCode",
            parameters: [
                
                {
                    name: "Year",
                    type: "number"
                },
                
                {
                    name: "Month",
                    type: "number"
                },
                
                {
                    name: "Day",
                    type: "number"
                },
                
                {
                    name: "Name",
                    type: "string"
                }
                
            ]
        },
        
        "Date.era": {
            func: DateDemo.era,
            // For avoiding webpacking minifying issues:
            funcName: "Date.era",
            parameters: [
                
                {
                    name: "Year",
                    type: "number"
                },
                
                {
                    name: "Month",
                    type: "number"
                },
                
                {
                    name: "Day",
                    type: "number"
                },
                
                {
                    name: "Name",
                    type: "string"
                }
                
            ]
        },
        
        "DateTime.monthCode": {
            func: DateTimeDemo.monthCode,
            // For avoiding webpacking minifying issues:
            funcName: "DateTime.monthCode",
            parameters: [
                
                {
                    name: "Year",
                    type: "number"
                },
                
                {
                    name: "Month",
                    type: "number"
                },
                
                {
                    name: "Day",
                    type: "number"
                },
                
                {
                    name: "Hour",
                    type: "number"
                },
                
                {
                    name: "Minute",
                    type: "number"
                },
                
                {
                    name: "Second",
                    type: "number"
                },
                
                {
                    name: "Nanosecond",
                    type: "number"
                },
                
                {
                    name: "Name",
                    type: "string"
                }
                
            ]
        },
        
        "DateTime.era": {
            func: DateTimeDemo.era,
            // For avoiding webpacking minifying issues:
            funcName: "DateTime.era",
            parameters: [
                
                {
                    name: "Year",
                    type: "number"
                },
                
                {
                    name: "Month",
                    type: "number"
                },
                
                {
                    name: "Day",
                    type: "number"
                },
                
                {
                    name: "Hour",
                    type: "number"
                },
                
                {
                    name: "Minute",
                    type: "number"
                },
                
                {
                    name: "Second",
                    type: "number"
                },
                
                {
                    name: "Nanosecond",
                    type: "number"
                },
                
                {
                    name: "Name",
                    type: "string"
                }
                
            ]
        },
        
        "DateFormatter.formatDate": {
            func: DateFormatterDemo.formatDate,
            // For avoiding webpacking minifying issues:
            funcName: "DateFormatter.formatDate",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "DateLength",
                    type: "DateLength"
                },
                
                {
                    name: "Year",
                    type: "number"
                },
                
                {
                    name: "Month",
                    type: "number"
                },
                
                {
                    name: "Day",
                    type: "number"
                },
                
                {
                    name: "Name",
                    type: "string"
                }
                
            ]
        },
        
        "DateFormatter.formatIsoDate": {
            func: DateFormatterDemo.formatIsoDate,
            // For avoiding webpacking minifying issues:
            funcName: "DateFormatter.formatIsoDate",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "DateLength",
                    type: "DateLength"
                },
                
                {
                    name: "Year",
                    type: "number"
                },
                
                {
                    name: "Month",
                    type: "number"
                },
                
                {
                    name: "Day",
                    type: "number"
                }
                
            ]
        },
        
        "DateFormatter.formatDatetime": {
            func: DateFormatterDemo.formatDatetime,
            // For avoiding webpacking minifying issues:
            funcName: "DateFormatter.formatDatetime",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "DateLength",
                    type: "DateLength"
                },
                
                {
                    name: "Year",
                    type: "number"
                },
                
                {
                    name: "Month",
                    type: "number"
                },
                
                {
                    name: "Day",
                    type: "number"
                },
                
                {
                    name: "Hour",
                    type: "number"
                },
                
                {
                    name: "Minute",
                    type: "number"
                },
                
                {
                    name: "Second",
                    type: "number"
                },
                
                {
                    name: "Nanosecond",
                    type: "number"
                },
                
                {
                    name: "Name",
                    type: "string"
                }
                
            ]
        },
        
        "DateFormatter.formatIsoDatetime": {
            func: DateFormatterDemo.formatIsoDatetime,
            // For avoiding webpacking minifying issues:
            funcName: "DateFormatter.formatIsoDatetime",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "DateLength",
                    type: "DateLength"
                },
                
                {
                    name: "Year",
                    type: "number"
                },
                
                {
                    name: "Month",
                    type: "number"
                },
                
                {
                    name: "Day",
                    type: "number"
                },
                
                {
                    name: "Hour",
                    type: "number"
                },
                
                {
                    name: "Minute",
                    type: "number"
                },
                
                {
                    name: "Second",
                    type: "number"
                },
                
                {
                    name: "Nanosecond",
                    type: "number"
                }
                
            ]
        },
        
        "DateTimeFormatter.formatDatetime": {
            func: DateTimeFormatterDemo.formatDatetime,
            // For avoiding webpacking minifying issues:
            funcName: "DateTimeFormatter.formatDatetime",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "DateLength",
                    type: "DateLength"
                },
                
                {
                    name: "TimeLength",
                    type: "TimeLength"
                },
                
                {
                    name: "Year",
                    type: "number"
                },
                
                {
                    name: "Month",
                    type: "number"
                },
                
                {
                    name: "Day",
                    type: "number"
                },
                
                {
                    name: "Hour",
                    type: "number"
                },
                
                {
                    name: "Minute",
                    type: "number"
                },
                
                {
                    name: "Second",
                    type: "number"
                },
                
                {
                    name: "Nanosecond",
                    type: "number"
                },
                
                {
                    name: "Name",
                    type: "string"
                }
                
            ]
        },
        
        "DateTimeFormatter.formatIsoDatetime": {
            func: DateTimeFormatterDemo.formatIsoDatetime,
            // For avoiding webpacking minifying issues:
            funcName: "DateTimeFormatter.formatIsoDatetime",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "DateLength",
                    type: "DateLength"
                },
                
                {
                    name: "TimeLength",
                    type: "TimeLength"
                },
                
                {
                    name: "Year",
                    type: "number"
                },
                
                {
                    name: "Month",
                    type: "number"
                },
                
                {
                    name: "Day",
                    type: "number"
                },
                
                {
                    name: "Hour",
                    type: "number"
                },
                
                {
                    name: "Minute",
                    type: "number"
                },
                
                {
                    name: "Second",
                    type: "number"
                },
                
                {
                    name: "Nanosecond",
                    type: "number"
                }
                
            ]
        },
        
        "GregorianDateFormatter.formatIsoDate": {
            func: GregorianDateFormatterDemo.formatIsoDate,
            // For avoiding webpacking minifying issues:
            funcName: "GregorianDateFormatter.formatIsoDate",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "Length",
                    type: "DateLength"
                },
                
                {
                    name: "Year",
                    type: "number"
                },
                
                {
                    name: "Month",
                    type: "number"
                },
                
                {
                    name: "Day",
                    type: "number"
                }
                
            ]
        },
        
        "GregorianDateFormatter.formatIsoDatetime": {
            func: GregorianDateFormatterDemo.formatIsoDatetime,
            // For avoiding webpacking minifying issues:
            funcName: "GregorianDateFormatter.formatIsoDatetime",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "Length",
                    type: "DateLength"
                },
                
                {
                    name: "Year",
                    type: "number"
                },
                
                {
                    name: "Month",
                    type: "number"
                },
                
                {
                    name: "Day",
                    type: "number"
                },
                
                {
                    name: "Hour",
                    type: "number"
                },
                
                {
                    name: "Minute",
                    type: "number"
                },
                
                {
                    name: "Second",
                    type: "number"
                },
                
                {
                    name: "Nanosecond",
                    type: "number"
                }
                
            ]
        },
        
        "GregorianDateTimeFormatter.formatIsoDatetime": {
            func: GregorianDateTimeFormatterDemo.formatIsoDatetime,
            // For avoiding webpacking minifying issues:
            funcName: "GregorianDateTimeFormatter.formatIsoDatetime",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "DateLength",
                    type: "DateLength"
                },
                
                {
                    name: "TimeLength",
                    type: "TimeLength"
                },
                
                {
                    name: "Year",
                    type: "number"
                },
                
                {
                    name: "Month",
                    type: "number"
                },
                
                {
                    name: "Day",
                    type: "number"
                },
                
                {
                    name: "Hour",
                    type: "number"
                },
                
                {
                    name: "Minute",
                    type: "number"
                },
                
                {
                    name: "Second",
                    type: "number"
                },
                
                {
                    name: "Nanosecond",
                    type: "number"
                }
                
            ]
        },
        
        "TimeFormatter.formatTime": {
            func: TimeFormatterDemo.formatTime,
            // For avoiding webpacking minifying issues:
            funcName: "TimeFormatter.formatTime",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "Length",
                    type: "TimeLength"
                },
                
                {
                    name: "Hour",
                    type: "number"
                },
                
                {
                    name: "Minute",
                    type: "number"
                },
                
                {
                    name: "Second",
                    type: "number"
                },
                
                {
                    name: "Nanosecond",
                    type: "number"
                }
                
            ]
        },
        
        "TimeFormatter.formatDatetime": {
            func: TimeFormatterDemo.formatDatetime,
            // For avoiding webpacking minifying issues:
            funcName: "TimeFormatter.formatDatetime",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "Length",
                    type: "TimeLength"
                },
                
                {
                    name: "Year",
                    type: "number"
                },
                
                {
                    name: "Month",
                    type: "number"
                },
                
                {
                    name: "Day",
                    type: "number"
                },
                
                {
                    name: "Hour",
                    type: "number"
                },
                
                {
                    name: "Minute",
                    type: "number"
                },
                
                {
                    name: "Second",
                    type: "number"
                },
                
                {
                    name: "Nanosecond",
                    type: "number"
                },
                
                {
                    name: "Name",
                    type: "string"
                }
                
            ]
        },
        
        "TimeFormatter.formatIsoDatetime": {
            func: TimeFormatterDemo.formatIsoDatetime,
            // For avoiding webpacking minifying issues:
            funcName: "TimeFormatter.formatIsoDatetime",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "Length",
                    type: "TimeLength"
                },
                
                {
                    name: "Year",
                    type: "number"
                },
                
                {
                    name: "Month",
                    type: "number"
                },
                
                {
                    name: "Day",
                    type: "number"
                },
                
                {
                    name: "Hour",
                    type: "number"
                },
                
                {
                    name: "Minute",
                    type: "number"
                },
                
                {
                    name: "Second",
                    type: "number"
                },
                
                {
                    name: "Nanosecond",
                    type: "number"
                }
                
            ]
        },
        
        "FixedDecimalFormatter.format": {
            func: FixedDecimalFormatterDemo.format,
            // For avoiding webpacking minifying issues:
            funcName: "FixedDecimalFormatter.format",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "GroupingStrategy",
                    type: "FixedDecimalGroupingStrategy"
                },
                
                {
                    name: "F",
                    type: "number"
                }
                
            ]
        },
        
        "LocaleDisplayNamesFormatter.of": {
            func: LocaleDisplayNamesFormatterDemo.of,
            // For avoiding webpacking minifying issues:
            funcName: "LocaleDisplayNamesFormatter.of",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "Style",
                    type: "DisplayNamesStyle"
                },
                
                {
                    name: "Fallback",
                    type: "DisplayNamesFallback"
                },
                
                {
                    name: "LanguageDisplay",
                    type: "LanguageDisplay"
                },
                
                {
                    name: "Name",
                    type: "string"
                }
                
            ]
        },
        
        "RegionDisplayNames.of": {
            func: RegionDisplayNamesDemo.of,
            // For avoiding webpacking minifying issues:
            funcName: "RegionDisplayNames.of",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "Region",
                    type: "string"
                }
                
            ]
        },
        
        "FixedDecimal.toString": {
            func: FixedDecimalDemo.toString,
            // For avoiding webpacking minifying issues:
            funcName: "FixedDecimal.toString",
            parameters: [
                
                {
                    name: "F",
                    type: "number"
                }
                
            ]
        },
        
        "ListFormatter.format": {
            func: ListFormatterDemo.format,
            // For avoiding webpacking minifying issues:
            funcName: "ListFormatter.format",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "Length",
                    type: "ListLength"
                },
                
                {
                    name: "List",
                    type: "Array<String>"
                }
                
            ]
        },
        
        "Locale.basename": {
            func: LocaleDemo.basename,
            // For avoiding webpacking minifying issues:
            funcName: "Locale.basename",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                }
                
            ]
        },
        
        "Locale.getUnicodeExtension": {
            func: LocaleDemo.getUnicodeExtension,
            // For avoiding webpacking minifying issues:
            funcName: "Locale.getUnicodeExtension",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "S",
                    type: "string"
                }
                
            ]
        },
        
        "Locale.language": {
            func: LocaleDemo.language,
            // For avoiding webpacking minifying issues:
            funcName: "Locale.language",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                }
                
            ]
        },
        
        "Locale.region": {
            func: LocaleDemo.region,
            // For avoiding webpacking minifying issues:
            funcName: "Locale.region",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                }
                
            ]
        },
        
        "Locale.script": {
            func: LocaleDemo.script,
            // For avoiding webpacking minifying issues:
            funcName: "Locale.script",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                }
                
            ]
        },
        
        "Locale.canonicalize": {
            func: LocaleDemo.canonicalize,
            // For avoiding webpacking minifying issues:
            funcName: "Locale.canonicalize",
            parameters: [
                
                {
                    name: "S",
                    type: "string"
                }
                
            ]
        },
        
        "Locale.toString": {
            func: LocaleDemo.toString,
            // For avoiding webpacking minifying issues:
            funcName: "Locale.toString",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
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
                    type: "string"
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
                    type: "string"
                }
                
            ]
        },
        
        "CustomTimeZone.timeZoneId": {
            func: CustomTimeZoneDemo.timeZoneId,
            // For avoiding webpacking minifying issues:
            funcName: "CustomTimeZone.timeZoneId",
            parameters: [
                
                {
                    name: "S",
                    type: "string"
                }
                
            ]
        },
        
        "CustomTimeZone.metazoneId": {
            func: CustomTimeZoneDemo.metazoneId,
            // For avoiding webpacking minifying issues:
            funcName: "CustomTimeZone.metazoneId",
            parameters: [
                
                {
                    name: "S",
                    type: "string"
                }
                
            ]
        },
        
        "CustomTimeZone.zoneVariant": {
            func: CustomTimeZoneDemo.zoneVariant,
            // For avoiding webpacking minifying issues:
            funcName: "CustomTimeZone.zoneVariant",
            parameters: [
                
                {
                    name: "S",
                    type: "string"
                }
                
            ]
        },
        
        "TimeZoneFormatter.formatCustomTimeZone": {
            func: TimeZoneFormatterDemo.formatCustomTimeZone,
            // For avoiding webpacking minifying issues:
            funcName: "TimeZoneFormatter.formatCustomTimeZone",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "S",
                    type: "string"
                }
                
            ]
        },
        
        "TimeZoneFormatter.formatCustomTimeZoneNoFallback": {
            func: TimeZoneFormatterDemo.formatCustomTimeZoneNoFallback,
            // For avoiding webpacking minifying issues:
            funcName: "TimeZoneFormatter.formatCustomTimeZoneNoFallback",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "S",
                    type: "string"
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
                    type: "string"
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
                    type: "string"
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
                    type: "string"
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
                    type: "string"
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
                    type: "string"
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
                    type: "string"
                }
                
            ]
        },
        
        "GregorianZonedDateTimeFormatter.formatIsoDatetimeWithCustomTimeZone": {
            func: GregorianZonedDateTimeFormatterDemo.formatIsoDatetimeWithCustomTimeZone,
            // For avoiding webpacking minifying issues:
            funcName: "GregorianZonedDateTimeFormatter.formatIsoDatetimeWithCustomTimeZone",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "DateLength",
                    type: "DateLength"
                },
                
                {
                    name: "TimeLength",
                    type: "TimeLength"
                },
                
                {
                    name: "Year",
                    type: "number"
                },
                
                {
                    name: "Month",
                    type: "number"
                },
                
                {
                    name: "Day",
                    type: "number"
                },
                
                {
                    name: "Hour",
                    type: "number"
                },
                
                {
                    name: "Minute",
                    type: "number"
                },
                
                {
                    name: "Second",
                    type: "number"
                },
                
                {
                    name: "Nanosecond",
                    type: "number"
                },
                
                {
                    name: "S",
                    type: "string"
                }
                
            ]
        },
        
        "ZonedDateTimeFormatter.formatDatetimeWithCustomTimeZone": {
            func: ZonedDateTimeFormatterDemo.formatDatetimeWithCustomTimeZone,
            // For avoiding webpacking minifying issues:
            funcName: "ZonedDateTimeFormatter.formatDatetimeWithCustomTimeZone",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "DateLength",
                    type: "DateLength"
                },
                
                {
                    name: "TimeLength",
                    type: "TimeLength"
                },
                
                {
                    name: "Year",
                    type: "number"
                },
                
                {
                    name: "Month",
                    type: "number"
                },
                
                {
                    name: "Day",
                    type: "number"
                },
                
                {
                    name: "Hour",
                    type: "number"
                },
                
                {
                    name: "Minute",
                    type: "number"
                },
                
                {
                    name: "Second",
                    type: "number"
                },
                
                {
                    name: "Nanosecond",
                    type: "number"
                },
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "S",
                    type: "string"
                }
                
            ]
        },
        
        "ZonedDateTimeFormatter.formatIsoDatetimeWithCustomTimeZone": {
            func: ZonedDateTimeFormatterDemo.formatIsoDatetimeWithCustomTimeZone,
            // For avoiding webpacking minifying issues:
            funcName: "ZonedDateTimeFormatter.formatIsoDatetimeWithCustomTimeZone",
            parameters: [
                
                {
                    name: "Name",
                    type: "string"
                },
                
                {
                    name: "DateLength",
                    type: "DateLength"
                },
                
                {
                    name: "TimeLength",
                    type: "TimeLength"
                },
                
                {
                    name: "Year",
                    type: "number"
                },
                
                {
                    name: "Month",
                    type: "number"
                },
                
                {
                    name: "Day",
                    type: "number"
                },
                
                {
                    name: "Hour",
                    type: "number"
                },
                
                {
                    name: "Minute",
                    type: "number"
                },
                
                {
                    name: "Second",
                    type: "number"
                },
                
                {
                    name: "Nanosecond",
                    type: "number"
                },
                
                {
                    name: "S",
                    type: "string"
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
                    type: "AnyCalendarKind"
                }
                
            ]
        }
        },
};