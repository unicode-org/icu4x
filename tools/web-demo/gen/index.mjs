export * as lib from "icu4x";
import RenderTerminiLocaleCanonicalizer from "./LocaleCanonicalizer.mjs";
import RenderTerminiWordSegmenter from "./WordSegmenter.mjs";
import { BidiClass } from "icu4x"
import { Calendar } from "icu4x"
import { CalendarKind } from "icu4x"
import { CanonicalCombiningClass } from "icu4x"
import { CanonicalCombiningClassMap } from "icu4x"
import { CanonicalComposition } from "icu4x"
import { CaseMapper } from "icu4x"
import { Collator } from "icu4x"
import { CollatorOptions } from "icu4x"
import { ComposingNormalizer } from "icu4x"
import { Date } from "icu4x"
import { DateFormatter } from "icu4x"
import { DateFormatterGregorian } from "icu4x"
import { DateTimeFormatter } from "icu4x"
import { DateTimeFormatterGregorian } from "icu4x"
import { Decimal } from "icu4x"
import { DecimalFormatter } from "icu4x"
import { DecomposingNormalizer } from "icu4x"
import { EastAsianWidth } from "icu4x"
import { EmojiSetData } from "icu4x"
import { ExemplarCharacters } from "icu4x"
import { GeneralCategory } from "icu4x"
import { GeneralCategoryGroup } from "icu4x"
import { GraphemeClusterBreak } from "icu4x"
import { HangulSyllableType } from "icu4x"
import { IndicSyllabicCategory } from "icu4x"
import { IsoDate } from "icu4x"
import { JoiningType } from "icu4x"
import { LineBreak } from "icu4x"
import { ListFormatter } from "icu4x"
import { Locale } from "icu4x"
import { LocaleDirectionality } from "icu4x"
import { PluralCategory } from "icu4x"
import { PluralOperands } from "icu4x"
import { PluralRules } from "icu4x"
import { Script } from "icu4x"
import { ScriptWithExtensions } from "icu4x"
import { SegmenterWordType } from "icu4x"
import { SentenceBreak } from "icu4x"
import { Time } from "icu4x"
import { TimeFormatter } from "icu4x"
import { TimePrecision } from "icu4x"
import { TimeZone } from "icu4x"
import { TimeZoneFormatter } from "icu4x"
import { TimeZoneInfo } from "icu4x"
import { TimeZoneVariant } from "icu4x"
import { TitlecaseMapper } from "icu4x"
import { TitlecaseOptions } from "icu4x"
import { UtcOffset } from "icu4x"
import { VerticalOrientation } from "icu4x"
import { WeekInformation } from "icu4x"
import { WordBreak } from "icu4x"
import { ZonedDateFormatter } from "icu4x"
import { ZonedDateFormatterGregorian } from "icu4x"
import { ZonedDateTimeFormatter } from "icu4x"
import { ZonedDateTimeFormatterGregorian } from "icu4x"
import { ZonedTimeFormatter } from "icu4x"

const displayBool = (out) => out ? 'true' : 'false';
const displayOrdering = (out) => out == 0 ? '==' : out == 1 ? '>' : '<';
const displayChar = (out) => String.fromCharCode(out);
const displayByte = (out) => '0x' + out.toString(16);
const displayOptionalEnum = (out) => out?.value || 'None';

let termini = Object.assign({
    "GeneralCategoryGroup.contains": {
        func: (selfMask, val) => GeneralCategoryGroup.fromFields({
            mask: selfMask
        }).contains(val),
        // For avoiding webpacking minifying issues:
        funcName: "GeneralCategoryGroup.contains",
        expr: (selfMask, val) => "GeneralCategoryGroup.fromFields({\n    mask: selfMask\n}).contains(val)".replace(/([\( ])selfMask([,\) \n])/, '$1' + selfMask + '$2').replace(/([\( ])val([,\) \n])/, '$1' + val + '$2'),
        display: displayBool,
        parameters: [
            
            {
                name: "self_mask",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "val",
                type: "GeneralCategory",
                typeUse: "enumerator"
            }
            
        ]
    },

    "CaseMapper.lowercase": {
        func: (s, localeName) => new CaseMapper().lowercase(s, Locale.fromString(localeName)),
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.lowercase",
        expr: (s, localeName) => "new CaseMapper().lowercase(s, Locale.fromString(localeName))".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2').replace(/([\( ])localeName([,\) \n])/, '$1' + localeName + '$2'),
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "locale_name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "CaseMapper.uppercase": {
        func: (s, localeName) => new CaseMapper().uppercase(s, Locale.fromString(localeName)),
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.uppercase",
        expr: (s, localeName) => "new CaseMapper().uppercase(s, Locale.fromString(localeName))".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2').replace(/([\( ])localeName([,\) \n])/, '$1' + localeName + '$2'),
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "locale_name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "CaseMapper.titlecaseSegmentWithOnlyCaseData": {
        func: (s, localeName, optionsLeadingAdjustment, optionsTrailingCase) => new CaseMapper().titlecaseSegmentWithOnlyCaseData(s, Locale.fromString(localeName), TitlecaseOptions.fromFields({
            leadingAdjustment: optionsLeadingAdjustment,
            trailingCase: optionsTrailingCase
        })),
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.titlecaseSegmentWithOnlyCaseData",
        expr: (s, localeName, optionsLeadingAdjustment, optionsTrailingCase) => "new CaseMapper().titlecaseSegmentWithOnlyCaseData(s, Locale.fromString(localeName), TitlecaseOptions.fromFields({\n    leadingAdjustment: optionsLeadingAdjustment,\n    trailingCase: optionsTrailingCase\n}))".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2').replace(/([\( ])localeName([,\) \n])/, '$1' + localeName + '$2').replace(/([\( ])optionsLeadingAdjustment([,\) \n])/, '$1' + optionsLeadingAdjustment + '$2').replace(/([\( ])optionsTrailingCase([,\) \n])/, '$1' + optionsTrailingCase + '$2'),
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "options_leadingAdjustment",
                type: "LeadingAdjustment",
                typeUse: "enumerator"
            },
            
            {
                name: "options_trailingCase",
                type: "TrailingCase",
                typeUse: "enumerator"
            }
            
        ]
    },

    "CaseMapper.fold": {
        func: (s) => new CaseMapper().fold(s),
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.fold",
        expr: (s) => "new CaseMapper().fold(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "CaseMapper.foldTurkic": {
        func: (s) => new CaseMapper().foldTurkic(s),
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.foldTurkic",
        expr: (s) => "new CaseMapper().foldTurkic(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "CaseMapper.simpleLowercase": {
        func: (ch) => new CaseMapper().simpleLowercase(ch),
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.simpleLowercase",
        expr: (ch) => "new CaseMapper().simpleLowercase(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayChar,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "CaseMapper.simpleUppercase": {
        func: (ch) => new CaseMapper().simpleUppercase(ch),
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.simpleUppercase",
        expr: (ch) => "new CaseMapper().simpleUppercase(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayChar,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "CaseMapper.simpleTitlecase": {
        func: (ch) => new CaseMapper().simpleTitlecase(ch),
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.simpleTitlecase",
        expr: (ch) => "new CaseMapper().simpleTitlecase(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayChar,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "CaseMapper.simpleFold": {
        func: (ch) => new CaseMapper().simpleFold(ch),
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.simpleFold",
        expr: (ch) => "new CaseMapper().simpleFold(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayChar,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "CaseMapper.simpleFoldTurkic": {
        func: (ch) => new CaseMapper().simpleFoldTurkic(ch),
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.simpleFoldTurkic",
        expr: (ch) => "new CaseMapper().simpleFoldTurkic(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayChar,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "TitlecaseMapper.titlecaseSegment": {
        func: (s, localeName, optionsLeadingAdjustment, optionsTrailingCase) => new TitlecaseMapper().titlecaseSegment(s, Locale.fromString(localeName), TitlecaseOptions.fromFields({
            leadingAdjustment: optionsLeadingAdjustment,
            trailingCase: optionsTrailingCase
        })),
        // For avoiding webpacking minifying issues:
        funcName: "TitlecaseMapper.titlecaseSegment",
        expr: (s, localeName, optionsLeadingAdjustment, optionsTrailingCase) => "new TitlecaseMapper().titlecaseSegment(s, Locale.fromString(localeName), TitlecaseOptions.fromFields({\n    leadingAdjustment: optionsLeadingAdjustment,\n    trailingCase: optionsTrailingCase\n}))".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2').replace(/([\( ])localeName([,\) \n])/, '$1' + localeName + '$2').replace(/([\( ])optionsLeadingAdjustment([,\) \n])/, '$1' + optionsLeadingAdjustment + '$2').replace(/([\( ])optionsTrailingCase([,\) \n])/, '$1' + optionsTrailingCase + '$2'),
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "options_leadingAdjustment",
                type: "LeadingAdjustment",
                typeUse: "enumerator"
            },
            
            {
                name: "options_trailingCase",
                type: "TrailingCase",
                typeUse: "enumerator"
            }
            
        ]
    },

    "Collator.compare": {
        func: (selfLocaleName, selfOptionsStrength, selfOptionsAlternateHandling, selfOptionsMaxVariable, selfOptionsCaseLevel, left, right) => new Collator(Locale.fromString(selfLocaleName), CollatorOptions.fromFields({
            strength: selfOptionsStrength,
            alternateHandling: selfOptionsAlternateHandling,
            maxVariable: selfOptionsMaxVariable,
            caseLevel: selfOptionsCaseLevel
        })).compare(left, right),
        // For avoiding webpacking minifying issues:
        funcName: "Collator.compare",
        expr: (selfLocaleName, selfOptionsStrength, selfOptionsAlternateHandling, selfOptionsMaxVariable, selfOptionsCaseLevel, left, right) => "new Collator(Locale.fromString(selfLocaleName), CollatorOptions.fromFields({\n    strength: selfOptionsStrength,\n    alternateHandling: selfOptionsAlternateHandling,\n    maxVariable: selfOptionsMaxVariable,\n    caseLevel: selfOptionsCaseLevel\n})).compare(left, right)".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfOptionsStrength([,\) \n])/, '$1' + selfOptionsStrength + '$2').replace(/([\( ])selfOptionsAlternateHandling([,\) \n])/, '$1' + selfOptionsAlternateHandling + '$2').replace(/([\( ])selfOptionsMaxVariable([,\) \n])/, '$1' + selfOptionsMaxVariable + '$2').replace(/([\( ])selfOptionsCaseLevel([,\) \n])/, '$1' + selfOptionsCaseLevel + '$2').replace(/([\( ])left([,\) \n])/, '$1' + left + '$2').replace(/([\( ])right([,\) \n])/, '$1' + right + '$2'),
        display: displayOrdering,
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_options_strength",
                type: "CollatorStrength",
                typeUse: "enumerator"
            },
            
            {
                name: "self_options_alternateHandling",
                type: "CollatorAlternateHandling",
                typeUse: "enumerator"
            },
            
            {
                name: "self_options_maxVariable",
                type: "CollatorMaxVariable",
                typeUse: "enumerator"
            },
            
            {
                name: "self_options_caseLevel",
                type: "CollatorCaseLevel",
                typeUse: "enumerator"
            },
            
            {
                name: "left",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "right",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Date.rataDie": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).rataDie,
        // For avoiding webpacking minifying issues:
        funcName: "Date.rataDie",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).rataDie".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
        parameters: [
            
            {
                name: "self_isoYear",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoMonth",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoDay",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_calendar_kind",
                type: "CalendarKind",
                typeUse: "enumerator"
            }
            
        ]
    },

    "Date.dayOfYear": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).dayOfYear,
        // For avoiding webpacking minifying issues:
        funcName: "Date.dayOfYear",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).dayOfYear".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
        parameters: [
            
            {
                name: "self_isoYear",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoMonth",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoDay",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_calendar_kind",
                type: "CalendarKind",
                typeUse: "enumerator"
            }
            
        ]
    },

    "Date.dayOfMonth": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).dayOfMonth,
        // For avoiding webpacking minifying issues:
        funcName: "Date.dayOfMonth",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).dayOfMonth".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
        parameters: [
            
            {
                name: "self_isoYear",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoMonth",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoDay",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_calendar_kind",
                type: "CalendarKind",
                typeUse: "enumerator"
            }
            
        ]
    },

    "Date.dayOfWeek": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).dayOfWeek,
        // For avoiding webpacking minifying issues:
        funcName: "Date.dayOfWeek",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).dayOfWeek".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "self_isoYear",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoMonth",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoDay",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_calendar_kind",
                type: "CalendarKind",
                typeUse: "enumerator"
            }
            
        ]
    },

    "Date.ordinalMonth": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).ordinalMonth,
        // For avoiding webpacking minifying issues:
        funcName: "Date.ordinalMonth",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).ordinalMonth".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
        parameters: [
            
            {
                name: "self_isoYear",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoMonth",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoDay",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_calendar_kind",
                type: "CalendarKind",
                typeUse: "enumerator"
            }
            
        ]
    },

    "Date.monthCode": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).monthCode,
        // For avoiding webpacking minifying issues:
        funcName: "Date.monthCode",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).monthCode".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
        parameters: [
            
            {
                name: "self_isoYear",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoMonth",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoDay",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_calendar_kind",
                type: "CalendarKind",
                typeUse: "enumerator"
            }
            
        ]
    },

    "Date.monthNumber": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).monthNumber,
        // For avoiding webpacking minifying issues:
        funcName: "Date.monthNumber",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).monthNumber".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
        parameters: [
            
            {
                name: "self_isoYear",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoMonth",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoDay",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_calendar_kind",
                type: "CalendarKind",
                typeUse: "enumerator"
            }
            
        ]
    },

    "Date.monthIsLeap": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).monthIsLeap,
        // For avoiding webpacking minifying issues:
        funcName: "Date.monthIsLeap",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).monthIsLeap".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
        display: displayBool,
        parameters: [
            
            {
                name: "self_isoYear",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoMonth",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoDay",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_calendar_kind",
                type: "CalendarKind",
                typeUse: "enumerator"
            }
            
        ]
    },

    "Date.eraYearOrRelatedIso": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).eraYearOrRelatedIso,
        // For avoiding webpacking minifying issues:
        funcName: "Date.eraYearOrRelatedIso",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).eraYearOrRelatedIso".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
        parameters: [
            
            {
                name: "self_isoYear",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoMonth",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoDay",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_calendar_kind",
                type: "CalendarKind",
                typeUse: "enumerator"
            }
            
        ]
    },

    "Date.extendedYear": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).extendedYear,
        // For avoiding webpacking minifying issues:
        funcName: "Date.extendedYear",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).extendedYear".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
        parameters: [
            
            {
                name: "self_isoYear",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoMonth",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoDay",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_calendar_kind",
                type: "CalendarKind",
                typeUse: "enumerator"
            }
            
        ]
    },

    "Date.era": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).era,
        // For avoiding webpacking minifying issues:
        funcName: "Date.era",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).era".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
        parameters: [
            
            {
                name: "self_isoYear",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoMonth",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoDay",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_calendar_kind",
                type: "CalendarKind",
                typeUse: "enumerator"
            }
            
        ]
    },

    "Date.monthsInYear": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).monthsInYear,
        // For avoiding webpacking minifying issues:
        funcName: "Date.monthsInYear",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).monthsInYear".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
        parameters: [
            
            {
                name: "self_isoYear",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoMonth",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoDay",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_calendar_kind",
                type: "CalendarKind",
                typeUse: "enumerator"
            }
            
        ]
    },

    "Date.daysInMonth": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).daysInMonth,
        // For avoiding webpacking minifying issues:
        funcName: "Date.daysInMonth",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).daysInMonth".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
        parameters: [
            
            {
                name: "self_isoYear",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoMonth",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoDay",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_calendar_kind",
                type: "CalendarKind",
                typeUse: "enumerator"
            }
            
        ]
    },

    "Date.daysInYear": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).daysInYear,
        // For avoiding webpacking minifying issues:
        funcName: "Date.daysInYear",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new Calendar(selfCalendarKind)).daysInYear".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
        parameters: [
            
            {
                name: "self_isoYear",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoMonth",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_isoDay",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "self_calendar_kind",
                type: "CalendarKind",
                typeUse: "enumerator"
            }
            
        ]
    },

    "DateFormatter.formatIso": {
        func: (selfLocaleName, selfLength, selfAlignment, selfYearStyle, isoDateYear, isoDateMonth, isoDateDay) => DateFormatter.createYmd(Locale.fromString(selfLocaleName), selfLength, selfAlignment, selfYearStyle).formatIso(new IsoDate(isoDateYear, isoDateMonth, isoDateDay)),
        // For avoiding webpacking minifying issues:
        funcName: "DateFormatter.formatIso",
        expr: (selfLocaleName, selfLength, selfAlignment, selfYearStyle, isoDateYear, isoDateMonth, isoDateDay) => "DateFormatter.createYmd(Locale.fromString(selfLocaleName), selfLength, selfAlignment, selfYearStyle).formatIso(new IsoDate(isoDateYear, isoDateMonth, isoDateDay))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfLength([,\) \n])/, '$1' + selfLength + '$2').replace(/([\( ])selfAlignment([,\) \n])/, '$1' + selfAlignment + '$2').replace(/([\( ])selfYearStyle([,\) \n])/, '$1' + selfYearStyle + '$2').replace(/([\( ])isoDateYear([,\) \n])/, '$1' + isoDateYear + '$2').replace(/([\( ])isoDateMonth([,\) \n])/, '$1' + isoDateMonth + '$2').replace(/([\( ])isoDateDay([,\) \n])/, '$1' + isoDateDay + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_length",
                type: "DateTimeLength",
                typeUse: "enumerator"
            },
            
            {
                name: "self_alignment",
                type: "DateTimeAlignment",
                typeUse: "enumerator"
            },
            
            {
                name: "self_yearStyle",
                type: "YearStyle",
                typeUse: "enumerator"
            },
            
            {
                name: "isoDate_year",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "isoDate_month",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "isoDate_day",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "DateFormatterGregorian.formatIso": {
        func: (selfLocaleName, selfLength, selfAlignment, selfYearStyle, isoDateYear, isoDateMonth, isoDateDay) => DateFormatterGregorian.createYmd(Locale.fromString(selfLocaleName), selfLength, selfAlignment, selfYearStyle).formatIso(new IsoDate(isoDateYear, isoDateMonth, isoDateDay)),
        // For avoiding webpacking minifying issues:
        funcName: "DateFormatterGregorian.formatIso",
        expr: (selfLocaleName, selfLength, selfAlignment, selfYearStyle, isoDateYear, isoDateMonth, isoDateDay) => "DateFormatterGregorian.createYmd(Locale.fromString(selfLocaleName), selfLength, selfAlignment, selfYearStyle).formatIso(new IsoDate(isoDateYear, isoDateMonth, isoDateDay))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfLength([,\) \n])/, '$1' + selfLength + '$2').replace(/([\( ])selfAlignment([,\) \n])/, '$1' + selfAlignment + '$2').replace(/([\( ])selfYearStyle([,\) \n])/, '$1' + selfYearStyle + '$2').replace(/([\( ])isoDateYear([,\) \n])/, '$1' + isoDateYear + '$2').replace(/([\( ])isoDateMonth([,\) \n])/, '$1' + isoDateMonth + '$2').replace(/([\( ])isoDateDay([,\) \n])/, '$1' + isoDateDay + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_length",
                type: "DateTimeLength",
                typeUse: "enumerator"
            },
            
            {
                name: "self_alignment",
                type: "DateTimeAlignment",
                typeUse: "enumerator"
            },
            
            {
                name: "self_yearStyle",
                type: "YearStyle",
                typeUse: "enumerator"
            },
            
            {
                name: "isoDate_year",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "isoDate_month",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "isoDate_day",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "DateTimeFormatter.formatIso": {
        func: (selfLocaleName, selfLength, selfTimePrecision, selfAlignment, selfYearStyle, isoDateYear, isoDateMonth, isoDateDay, timeHour, timeMinute, timeSecond, timeSubsecond) => DateTimeFormatter.createYmdt(Locale.fromString(selfLocaleName), selfLength, selfTimePrecision, selfAlignment, selfYearStyle).formatIso(new IsoDate(isoDateYear, isoDateMonth, isoDateDay), new Time(timeHour, timeMinute, timeSecond, timeSubsecond)),
        // For avoiding webpacking minifying issues:
        funcName: "DateTimeFormatter.formatIso",
        expr: (selfLocaleName, selfLength, selfTimePrecision, selfAlignment, selfYearStyle, isoDateYear, isoDateMonth, isoDateDay, timeHour, timeMinute, timeSecond, timeSubsecond) => "DateTimeFormatter.createYmdt(Locale.fromString(selfLocaleName), selfLength, selfTimePrecision, selfAlignment, selfYearStyle).formatIso(new IsoDate(isoDateYear, isoDateMonth, isoDateDay), new Time(timeHour, timeMinute, timeSecond, timeSubsecond))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfLength([,\) \n])/, '$1' + selfLength + '$2').replace(/([\( ])selfTimePrecision([,\) \n])/, '$1' + selfTimePrecision + '$2').replace(/([\( ])selfAlignment([,\) \n])/, '$1' + selfAlignment + '$2').replace(/([\( ])selfYearStyle([,\) \n])/, '$1' + selfYearStyle + '$2').replace(/([\( ])isoDateYear([,\) \n])/, '$1' + isoDateYear + '$2').replace(/([\( ])isoDateMonth([,\) \n])/, '$1' + isoDateMonth + '$2').replace(/([\( ])isoDateDay([,\) \n])/, '$1' + isoDateDay + '$2').replace(/([\( ])timeHour([,\) \n])/, '$1' + timeHour + '$2').replace(/([\( ])timeMinute([,\) \n])/, '$1' + timeMinute + '$2').replace(/([\( ])timeSecond([,\) \n])/, '$1' + timeSecond + '$2').replace(/([\( ])timeSubsecond([,\) \n])/, '$1' + timeSubsecond + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_length",
                type: "DateTimeLength",
                typeUse: "enumerator"
            },
            
            {
                name: "self_timePrecision",
                type: "TimePrecision",
                typeUse: "enumerator"
            },
            
            {
                name: "self_alignment",
                type: "DateTimeAlignment",
                typeUse: "enumerator"
            },
            
            {
                name: "self_yearStyle",
                type: "YearStyle",
                typeUse: "enumerator"
            },
            
            {
                name: "isoDate_year",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "isoDate_month",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "isoDate_day",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_hour",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_minute",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_second",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_subsecond",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "DateTimeFormatterGregorian.formatIso": {
        func: (selfLocaleName, selfLength, selfTimePrecision, selfAlignment, selfYearStyle, isoDateYear, isoDateMonth, isoDateDay, timeHour, timeMinute, timeSecond, timeSubsecond) => DateTimeFormatterGregorian.createYmdt(Locale.fromString(selfLocaleName), selfLength, selfTimePrecision, selfAlignment, selfYearStyle).formatIso(new IsoDate(isoDateYear, isoDateMonth, isoDateDay), new Time(timeHour, timeMinute, timeSecond, timeSubsecond)),
        // For avoiding webpacking minifying issues:
        funcName: "DateTimeFormatterGregorian.formatIso",
        expr: (selfLocaleName, selfLength, selfTimePrecision, selfAlignment, selfYearStyle, isoDateYear, isoDateMonth, isoDateDay, timeHour, timeMinute, timeSecond, timeSubsecond) => "DateTimeFormatterGregorian.createYmdt(Locale.fromString(selfLocaleName), selfLength, selfTimePrecision, selfAlignment, selfYearStyle).formatIso(new IsoDate(isoDateYear, isoDateMonth, isoDateDay), new Time(timeHour, timeMinute, timeSecond, timeSubsecond))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfLength([,\) \n])/, '$1' + selfLength + '$2').replace(/([\( ])selfTimePrecision([,\) \n])/, '$1' + selfTimePrecision + '$2').replace(/([\( ])selfAlignment([,\) \n])/, '$1' + selfAlignment + '$2').replace(/([\( ])selfYearStyle([,\) \n])/, '$1' + selfYearStyle + '$2').replace(/([\( ])isoDateYear([,\) \n])/, '$1' + isoDateYear + '$2').replace(/([\( ])isoDateMonth([,\) \n])/, '$1' + isoDateMonth + '$2').replace(/([\( ])isoDateDay([,\) \n])/, '$1' + isoDateDay + '$2').replace(/([\( ])timeHour([,\) \n])/, '$1' + timeHour + '$2').replace(/([\( ])timeMinute([,\) \n])/, '$1' + timeMinute + '$2').replace(/([\( ])timeSecond([,\) \n])/, '$1' + timeSecond + '$2').replace(/([\( ])timeSubsecond([,\) \n])/, '$1' + timeSubsecond + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_length",
                type: "DateTimeLength",
                typeUse: "enumerator"
            },
            
            {
                name: "self_timePrecision",
                type: "TimePrecision",
                typeUse: "enumerator"
            },
            
            {
                name: "self_alignment",
                type: "DateTimeAlignment",
                typeUse: "enumerator"
            },
            
            {
                name: "self_yearStyle",
                type: "YearStyle",
                typeUse: "enumerator"
            },
            
            {
                name: "isoDate_year",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "isoDate_month",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "isoDate_day",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_hour",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_minute",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_second",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_subsecond",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "DecimalFormatter.format": {
        func: (selfLocaleName, selfGroupingStrategy, valueV) => DecimalFormatter.createWithGroupingStrategy(Locale.fromString(selfLocaleName), selfGroupingStrategy).format(Decimal.fromString(valueV)),
        // For avoiding webpacking minifying issues:
        funcName: "DecimalFormatter.format",
        expr: (selfLocaleName, selfGroupingStrategy, valueV) => "DecimalFormatter.createWithGroupingStrategy(Locale.fromString(selfLocaleName), selfGroupingStrategy).format(Decimal.fromString(valueV))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfGroupingStrategy([,\) \n])/, '$1' + selfGroupingStrategy + '$2').replace(/([\( ])valueV([,\) \n])/, '$1' + valueV + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_groupingStrategy",
                type: "DecimalGroupingStrategy",
                typeUse: "enumerator"
            },
            
            {
                name: "value_v",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "ExemplarCharacters.containsStr": {
        func: (selfLocaleName, s) => ExemplarCharacters.createMain(Locale.fromString(selfLocaleName)).containsStr(s),
        // For avoiding webpacking minifying issues:
        funcName: "ExemplarCharacters.containsStr",
        expr: (selfLocaleName, s) => "ExemplarCharacters.createMain(Locale.fromString(selfLocaleName)).containsStr(s)".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        display: displayBool,
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "ExemplarCharacters.contains": {
        func: (selfLocaleName, cp) => ExemplarCharacters.createMain(Locale.fromString(selfLocaleName)).contains(cp),
        // For avoiding webpacking minifying issues:
        funcName: "ExemplarCharacters.contains",
        expr: (selfLocaleName, cp) => "ExemplarCharacters.createMain(Locale.fromString(selfLocaleName)).contains(cp)".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])cp([,\) \n])/, '$1' + cp + '$2'),
        display: displayBool,
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "cp",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "Decimal.digitAt": {
        func: (selfV, magnitude) => Decimal.fromString(selfV).digitAt(magnitude),
        // For avoiding webpacking minifying issues:
        funcName: "Decimal.digitAt",
        expr: (selfV, magnitude) => "Decimal.fromString(selfV).digitAt(magnitude)".replace(/([\( ])selfV([,\) \n])/, '$1' + selfV + '$2').replace(/([\( ])magnitude([,\) \n])/, '$1' + magnitude + '$2'),
        parameters: [
            
            {
                name: "self_v",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "magnitude",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "Decimal.magnitudeStart": {
        func: (selfV) => Decimal.fromString(selfV).magnitudeStart,
        // For avoiding webpacking minifying issues:
        funcName: "Decimal.magnitudeStart",
        expr: (selfV) => "Decimal.fromString(selfV).magnitudeStart".replace(/([\( ])selfV([,\) \n])/, '$1' + selfV + '$2'),
        parameters: [
            
            {
                name: "self_v",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Decimal.magnitudeEnd": {
        func: (selfV) => Decimal.fromString(selfV).magnitudeEnd,
        // For avoiding webpacking minifying issues:
        funcName: "Decimal.magnitudeEnd",
        expr: (selfV) => "Decimal.fromString(selfV).magnitudeEnd".replace(/([\( ])selfV([,\) \n])/, '$1' + selfV + '$2'),
        parameters: [
            
            {
                name: "self_v",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Decimal.nonzeroMagnitudeStart": {
        func: (selfV) => Decimal.fromString(selfV).nonzeroMagnitudeStart,
        // For avoiding webpacking minifying issues:
        funcName: "Decimal.nonzeroMagnitudeStart",
        expr: (selfV) => "Decimal.fromString(selfV).nonzeroMagnitudeStart".replace(/([\( ])selfV([,\) \n])/, '$1' + selfV + '$2'),
        parameters: [
            
            {
                name: "self_v",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Decimal.nonzeroMagnitudeEnd": {
        func: (selfV) => Decimal.fromString(selfV).nonzeroMagnitudeEnd,
        // For avoiding webpacking minifying issues:
        funcName: "Decimal.nonzeroMagnitudeEnd",
        expr: (selfV) => "Decimal.fromString(selfV).nonzeroMagnitudeEnd".replace(/([\( ])selfV([,\) \n])/, '$1' + selfV + '$2'),
        parameters: [
            
            {
                name: "self_v",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Decimal.isZero": {
        func: (selfV) => Decimal.fromString(selfV).isZero,
        // For avoiding webpacking minifying issues:
        funcName: "Decimal.isZero",
        expr: (selfV) => "Decimal.fromString(selfV).isZero".replace(/([\( ])selfV([,\) \n])/, '$1' + selfV + '$2'),
        display: displayBool,
        parameters: [
            
            {
                name: "self_v",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Decimal.sign": {
        func: (selfV) => Decimal.fromString(selfV).sign,
        // For avoiding webpacking minifying issues:
        funcName: "Decimal.sign",
        expr: (selfV) => "Decimal.fromString(selfV).sign".replace(/([\( ])selfV([,\) \n])/, '$1' + selfV + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "self_v",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "ListFormatter.format": {
        func: (selfLocaleName, selfLength, list) => ListFormatter.createAndWithLength(Locale.fromString(selfLocaleName), selfLength).format(list),
        // For avoiding webpacking minifying issues:
        funcName: "ListFormatter.format",
        expr: (selfLocaleName, selfLength, list) => "ListFormatter.createAndWithLength(Locale.fromString(selfLocaleName), selfLength).format(list)".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfLength([,\) \n])/, '$1' + selfLength + '$2').replace(/([\( ])list([,\) \n])/, '$1' + list + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_length",
                type: "ListLength",
                typeUse: "enumerator"
            },
            
            {
                name: "list",
                type: "Array<string>",
                typeUse: "Array<string>"
            }
            
        ]
    },

    "Locale.basename": {
        func: (selfName) => Locale.fromString(selfName).basename,
        // For avoiding webpacking minifying issues:
        funcName: "Locale.basename",
        expr: (selfName) => "Locale.fromString(selfName).basename".replace(/([\( ])selfName([,\) \n])/, '$1' + selfName + '$2'),
        parameters: [
            
            {
                name: "self_name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Locale.getUnicodeExtension": {
        func: (selfName, s) => Locale.fromString(selfName).getUnicodeExtension(s),
        // For avoiding webpacking minifying issues:
        funcName: "Locale.getUnicodeExtension",
        expr: (selfName, s) => "Locale.fromString(selfName).getUnicodeExtension(s)".replace(/([\( ])selfName([,\) \n])/, '$1' + selfName + '$2').replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        parameters: [
            
            {
                name: "self_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Locale.language": {
        func: (selfName) => Locale.fromString(selfName).language,
        // For avoiding webpacking minifying issues:
        funcName: "Locale.language",
        expr: (selfName) => "Locale.fromString(selfName).language".replace(/([\( ])selfName([,\) \n])/, '$1' + selfName + '$2'),
        parameters: [
            
            {
                name: "self_name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Locale.region": {
        func: (selfName) => Locale.fromString(selfName).region,
        // For avoiding webpacking minifying issues:
        funcName: "Locale.region",
        expr: (selfName) => "Locale.fromString(selfName).region".replace(/([\( ])selfName([,\) \n])/, '$1' + selfName + '$2'),
        parameters: [
            
            {
                name: "self_name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Locale.script": {
        func: (selfName) => Locale.fromString(selfName).script,
        // For avoiding webpacking minifying issues:
        funcName: "Locale.script",
        expr: (selfName) => "Locale.fromString(selfName).script".replace(/([\( ])selfName([,\) \n])/, '$1' + selfName + '$2'),
        parameters: [
            
            {
                name: "self_name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Locale.normalize": {
        func: (s) => Locale.normalize(s),
        // For avoiding webpacking minifying issues:
        funcName: "Locale.normalize",
        expr: (s) => "Locale.normalize(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Locale.toString": {
        func: (selfName) => Locale.fromString(selfName).toString(),
        // For avoiding webpacking minifying issues:
        funcName: "Locale.toString",
        expr: (selfName) => "Locale.fromString(selfName).toString()".replace(/([\( ])selfName([,\) \n])/, '$1' + selfName + '$2'),
        parameters: [
            
            {
                name: "self_name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Locale.normalizingEq": {
        func: (selfName, other) => Locale.fromString(selfName).normalizingEq(other),
        // For avoiding webpacking minifying issues:
        funcName: "Locale.normalizingEq",
        expr: (selfName, other) => "Locale.fromString(selfName).normalizingEq(other)".replace(/([\( ])selfName([,\) \n])/, '$1' + selfName + '$2').replace(/([\( ])other([,\) \n])/, '$1' + other + '$2'),
        display: displayBool,
        parameters: [
            
            {
                name: "self_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "other",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Locale.compareToString": {
        func: (selfName, other) => Locale.fromString(selfName).compareToString(other),
        // For avoiding webpacking minifying issues:
        funcName: "Locale.compareToString",
        expr: (selfName, other) => "Locale.fromString(selfName).compareToString(other)".replace(/([\( ])selfName([,\) \n])/, '$1' + selfName + '$2').replace(/([\( ])other([,\) \n])/, '$1' + other + '$2'),
        display: displayOrdering,
        parameters: [
            
            {
                name: "self_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "other",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Locale.compareTo": {
        func: (selfName, otherName) => Locale.fromString(selfName).compareTo(Locale.fromString(otherName)),
        // For avoiding webpacking minifying issues:
        funcName: "Locale.compareTo",
        expr: (selfName, otherName) => "Locale.fromString(selfName).compareTo(Locale.fromString(otherName))".replace(/([\( ])selfName([,\) \n])/, '$1' + selfName + '$2').replace(/([\( ])otherName([,\) \n])/, '$1' + otherName + '$2'),
        display: displayOrdering,
        parameters: [
            
            {
                name: "self_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "other_name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "LocaleDirectionality.get": {
        func: (localeName) => new LocaleDirectionality().get(Locale.fromString(localeName)),
        // For avoiding webpacking minifying issues:
        funcName: "LocaleDirectionality.get",
        expr: (localeName) => "new LocaleDirectionality().get(Locale.fromString(localeName))".replace(/([\( ])localeName([,\) \n])/, '$1' + localeName + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "locale_name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "ComposingNormalizer.normalize": {
        func: (s) => ComposingNormalizer.createNfc().normalize(s),
        // For avoiding webpacking minifying issues:
        funcName: "ComposingNormalizer.normalize",
        expr: (s) => "ComposingNormalizer.createNfc().normalize(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "ComposingNormalizer.isNormalized": {
        func: (s) => ComposingNormalizer.createNfc().isNormalized(s),
        // For avoiding webpacking minifying issues:
        funcName: "ComposingNormalizer.isNormalized",
        expr: (s) => "ComposingNormalizer.createNfc().isNormalized(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        display: displayBool,
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "ComposingNormalizer.isNormalizedUpTo": {
        func: (s) => ComposingNormalizer.createNfc().isNormalizedUpTo(s),
        // For avoiding webpacking minifying issues:
        funcName: "ComposingNormalizer.isNormalizedUpTo",
        expr: (s) => "ComposingNormalizer.createNfc().isNormalizedUpTo(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "DecomposingNormalizer.normalize": {
        func: (s) => DecomposingNormalizer.createNfd().normalize(s),
        // For avoiding webpacking minifying issues:
        funcName: "DecomposingNormalizer.normalize",
        expr: (s) => "DecomposingNormalizer.createNfd().normalize(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "DecomposingNormalizer.isNormalized": {
        func: (s) => DecomposingNormalizer.createNfd().isNormalized(s),
        // For avoiding webpacking minifying issues:
        funcName: "DecomposingNormalizer.isNormalized",
        expr: (s) => "DecomposingNormalizer.createNfd().isNormalized(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        display: displayBool,
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "DecomposingNormalizer.isNormalizedUpTo": {
        func: (s) => DecomposingNormalizer.createNfd().isNormalizedUpTo(s),
        // For avoiding webpacking minifying issues:
        funcName: "DecomposingNormalizer.isNormalizedUpTo",
        expr: (s) => "DecomposingNormalizer.createNfd().isNormalizedUpTo(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "CanonicalCombiningClassMap.get": {
        func: (ch) => new CanonicalCombiningClassMap().get(ch),
        // For avoiding webpacking minifying issues:
        funcName: "CanonicalCombiningClassMap.get",
        expr: (ch) => "new CanonicalCombiningClassMap().get(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "CanonicalComposition.compose": {
        func: (starter, second) => new CanonicalComposition().compose(starter, second),
        // For avoiding webpacking minifying issues:
        funcName: "CanonicalComposition.compose",
        expr: (starter, second) => "new CanonicalComposition().compose(starter, second)".replace(/([\( ])starter([,\) \n])/, '$1' + starter + '$2').replace(/([\( ])second([,\) \n])/, '$1' + second + '$2'),
        display: displayChar,
        parameters: [
            
            {
                name: "starter",
                type: "codepoint",
                typeUse: "codepoint"
            },
            
            {
                name: "second",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "PluralRules.categoryFor": {
        func: (selfLocaleName, opXV) => PluralRules.createCardinal(Locale.fromString(selfLocaleName)).categoryFor(PluralOperands.fromFixedDecimal(Decimal.fromString(opXV))),
        // For avoiding webpacking minifying issues:
        funcName: "PluralRules.categoryFor",
        expr: (selfLocaleName, opXV) => "PluralRules.createCardinal(Locale.fromString(selfLocaleName)).categoryFor(PluralOperands.fromFixedDecimal(Decimal.fromString(opXV)))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])opXV([,\) \n])/, '$1' + opXV + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "op_x_v",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "EmojiSetData.containsStr": {
        func: (s) => EmojiSetData.createBasic().containsStr(s),
        // For avoiding webpacking minifying issues:
        funcName: "EmojiSetData.containsStr",
        expr: (s) => "EmojiSetData.createBasic().containsStr(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        display: displayBool,
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "EmojiSetData.contains": {
        func: (cp) => EmojiSetData.createBasic().contains(cp),
        // For avoiding webpacking minifying issues:
        funcName: "EmojiSetData.contains",
        expr: (cp) => "EmojiSetData.createBasic().contains(cp)".replace(/([\( ])cp([,\) \n])/, '$1' + cp + '$2'),
        display: displayBool,
        parameters: [
            
            {
                name: "cp",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "ScriptWithExtensions.getScriptVal": {
        func: (ch) => new ScriptWithExtensions().getScriptVal(ch),
        // For avoiding webpacking minifying issues:
        funcName: "ScriptWithExtensions.getScriptVal",
        expr: (ch) => "new ScriptWithExtensions().getScriptVal(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "ScriptWithExtensions.hasScript": {
        func: (ch, script) => new ScriptWithExtensions().hasScript(ch, script),
        // For avoiding webpacking minifying issues:
        funcName: "ScriptWithExtensions.hasScript",
        expr: (ch, script) => "new ScriptWithExtensions().hasScript(ch, script)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2').replace(/([\( ])script([,\) \n])/, '$1' + script + '$2'),
        display: displayBool,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            },
            
            {
                name: "script",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "TimeFormatter.format": {
        func: (selfLocaleName, selfLength, selfTimePrecision, selfAlignment, timeHour, timeMinute, timeSecond, timeSubsecond) => new TimeFormatter(Locale.fromString(selfLocaleName), selfLength, selfTimePrecision, selfAlignment).format(new Time(timeHour, timeMinute, timeSecond, timeSubsecond)),
        // For avoiding webpacking minifying issues:
        funcName: "TimeFormatter.format",
        expr: (selfLocaleName, selfLength, selfTimePrecision, selfAlignment, timeHour, timeMinute, timeSecond, timeSubsecond) => "new TimeFormatter(Locale.fromString(selfLocaleName), selfLength, selfTimePrecision, selfAlignment).format(new Time(timeHour, timeMinute, timeSecond, timeSubsecond))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfLength([,\) \n])/, '$1' + selfLength + '$2').replace(/([\( ])selfTimePrecision([,\) \n])/, '$1' + selfTimePrecision + '$2').replace(/([\( ])selfAlignment([,\) \n])/, '$1' + selfAlignment + '$2').replace(/([\( ])timeHour([,\) \n])/, '$1' + timeHour + '$2').replace(/([\( ])timeMinute([,\) \n])/, '$1' + timeMinute + '$2').replace(/([\( ])timeSecond([,\) \n])/, '$1' + timeSecond + '$2').replace(/([\( ])timeSubsecond([,\) \n])/, '$1' + timeSubsecond + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_length",
                type: "DateTimeLength",
                typeUse: "enumerator"
            },
            
            {
                name: "self_timePrecision",
                type: "TimePrecision",
                typeUse: "enumerator"
            },
            
            {
                name: "self_alignment",
                type: "DateTimeAlignment",
                typeUse: "enumerator"
            },
            
            {
                name: "time_hour",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_minute",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_second",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_subsecond",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "TimeZone.isUnknown": {
        func: (selfId) => TimeZone.createFromBcp47(selfId).isUnknown(),
        // For avoiding webpacking minifying issues:
        funcName: "TimeZone.isUnknown",
        expr: (selfId) => "TimeZone.createFromBcp47(selfId).isUnknown()".replace(/([\( ])selfId([,\) \n])/, '$1' + selfId + '$2'),
        display: displayBool,
        parameters: [
            
            {
                name: "self_id",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "TimeZoneFormatter.format": {
        func: (selfLocaleName, zoneIdId, zoneOffsetOffset, zoneVariant) => TimeZoneFormatter.createGenericShort(Locale.fromString(selfLocaleName)).format(new TimeZoneInfo(TimeZone.createFromBcp47(zoneIdId), UtcOffset.fromString(zoneOffsetOffset), zoneVariant)),
        // For avoiding webpacking minifying issues:
        funcName: "TimeZoneFormatter.format",
        expr: (selfLocaleName, zoneIdId, zoneOffsetOffset, zoneVariant) => "TimeZoneFormatter.createGenericShort(Locale.fromString(selfLocaleName)).format(new TimeZoneInfo(TimeZone.createFromBcp47(zoneIdId), UtcOffset.fromString(zoneOffsetOffset), zoneVariant))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])zoneIdId([,\) \n])/, '$1' + zoneIdId + '$2').replace(/([\( ])zoneOffsetOffset([,\) \n])/, '$1' + zoneOffsetOffset + '$2').replace(/([\( ])zoneVariant([,\) \n])/, '$1' + zoneVariant + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "zone_id_id",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "zone_offset_offset",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "zone_variant",
                type: "TimeZoneVariant",
                typeUse: "enumerator"
            }
            
        ]
    },

    "UtcOffset.seconds": {
        func: (selfOffset) => UtcOffset.fromString(selfOffset).seconds,
        // For avoiding webpacking minifying issues:
        funcName: "UtcOffset.seconds",
        expr: (selfOffset) => "UtcOffset.fromString(selfOffset).seconds".replace(/([\( ])selfOffset([,\) \n])/, '$1' + selfOffset + '$2'),
        parameters: [
            
            {
                name: "self_offset",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "UtcOffset.isNonNegative": {
        func: (selfOffset) => UtcOffset.fromString(selfOffset).isNonNegative,
        // For avoiding webpacking minifying issues:
        funcName: "UtcOffset.isNonNegative",
        expr: (selfOffset) => "UtcOffset.fromString(selfOffset).isNonNegative".replace(/([\( ])selfOffset([,\) \n])/, '$1' + selfOffset + '$2'),
        display: displayBool,
        parameters: [
            
            {
                name: "self_offset",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "UtcOffset.isZero": {
        func: (selfOffset) => UtcOffset.fromString(selfOffset).isZero,
        // For avoiding webpacking minifying issues:
        funcName: "UtcOffset.isZero",
        expr: (selfOffset) => "UtcOffset.fromString(selfOffset).isZero".replace(/([\( ])selfOffset([,\) \n])/, '$1' + selfOffset + '$2'),
        display: displayBool,
        parameters: [
            
            {
                name: "self_offset",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "UtcOffset.hoursPart": {
        func: (selfOffset) => UtcOffset.fromString(selfOffset).hoursPart,
        // For avoiding webpacking minifying issues:
        funcName: "UtcOffset.hoursPart",
        expr: (selfOffset) => "UtcOffset.fromString(selfOffset).hoursPart".replace(/([\( ])selfOffset([,\) \n])/, '$1' + selfOffset + '$2'),
        parameters: [
            
            {
                name: "self_offset",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "UtcOffset.minutesPart": {
        func: (selfOffset) => UtcOffset.fromString(selfOffset).minutesPart,
        // For avoiding webpacking minifying issues:
        funcName: "UtcOffset.minutesPart",
        expr: (selfOffset) => "UtcOffset.fromString(selfOffset).minutesPart".replace(/([\( ])selfOffset([,\) \n])/, '$1' + selfOffset + '$2'),
        parameters: [
            
            {
                name: "self_offset",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "UtcOffset.secondsPart": {
        func: (selfOffset) => UtcOffset.fromString(selfOffset).secondsPart,
        // For avoiding webpacking minifying issues:
        funcName: "UtcOffset.secondsPart",
        expr: (selfOffset) => "UtcOffset.fromString(selfOffset).secondsPart".replace(/([\( ])selfOffset([,\) \n])/, '$1' + selfOffset + '$2'),
        parameters: [
            
            {
                name: "self_offset",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "WeekInformation.firstWeekday": {
        func: (selfLocaleName) => new WeekInformation(Locale.fromString(selfLocaleName)).firstWeekday,
        // For avoiding webpacking minifying issues:
        funcName: "WeekInformation.firstWeekday",
        expr: (selfLocaleName) => "new WeekInformation(Locale.fromString(selfLocaleName)).firstWeekday".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "WeekInformation.isWeekend": {
        func: (selfLocaleName, day) => new WeekInformation(Locale.fromString(selfLocaleName)).isWeekend(day),
        // For avoiding webpacking minifying issues:
        funcName: "WeekInformation.isWeekend",
        expr: (selfLocaleName, day) => "new WeekInformation(Locale.fromString(selfLocaleName)).isWeekend(day)".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])day([,\) \n])/, '$1' + day + '$2'),
        display: displayBool,
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "day",
                type: "Weekday",
                typeUse: "enumerator"
            }
            
        ]
    },

    "ZonedDateFormatter.formatIso": {
        func: (selfLocaleName, selfFormatterLocaleName, selfFormatterLength, selfFormatterAlignment, selfFormatterYearStyle, isoDateYear, isoDateMonth, isoDateDay, zoneIdId, zoneOffsetOffset, zoneVariant) => ZonedDateFormatter.createGenericShort(Locale.fromString(selfLocaleName), DateFormatter.createYmd(Locale.fromString(selfFormatterLocaleName), selfFormatterLength, selfFormatterAlignment, selfFormatterYearStyle)).formatIso(new IsoDate(isoDateYear, isoDateMonth, isoDateDay), new TimeZoneInfo(TimeZone.createFromBcp47(zoneIdId), UtcOffset.fromString(zoneOffsetOffset), zoneVariant)),
        // For avoiding webpacking minifying issues:
        funcName: "ZonedDateFormatter.formatIso",
        expr: (selfLocaleName, selfFormatterLocaleName, selfFormatterLength, selfFormatterAlignment, selfFormatterYearStyle, isoDateYear, isoDateMonth, isoDateDay, zoneIdId, zoneOffsetOffset, zoneVariant) => "ZonedDateFormatter.createGenericShort(Locale.fromString(selfLocaleName), DateFormatter.createYmd(Locale.fromString(selfFormatterLocaleName), selfFormatterLength, selfFormatterAlignment, selfFormatterYearStyle)).formatIso(new IsoDate(isoDateYear, isoDateMonth, isoDateDay), new TimeZoneInfo(TimeZone.createFromBcp47(zoneIdId), UtcOffset.fromString(zoneOffsetOffset), zoneVariant))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfFormatterLocaleName([,\) \n])/, '$1' + selfFormatterLocaleName + '$2').replace(/([\( ])selfFormatterLength([,\) \n])/, '$1' + selfFormatterLength + '$2').replace(/([\( ])selfFormatterAlignment([,\) \n])/, '$1' + selfFormatterAlignment + '$2').replace(/([\( ])selfFormatterYearStyle([,\) \n])/, '$1' + selfFormatterYearStyle + '$2').replace(/([\( ])isoDateYear([,\) \n])/, '$1' + isoDateYear + '$2').replace(/([\( ])isoDateMonth([,\) \n])/, '$1' + isoDateMonth + '$2').replace(/([\( ])isoDateDay([,\) \n])/, '$1' + isoDateDay + '$2').replace(/([\( ])zoneIdId([,\) \n])/, '$1' + zoneIdId + '$2').replace(/([\( ])zoneOffsetOffset([,\) \n])/, '$1' + zoneOffsetOffset + '$2').replace(/([\( ])zoneVariant([,\) \n])/, '$1' + zoneVariant + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_formatter_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_formatter_length",
                type: "DateTimeLength",
                typeUse: "enumerator"
            },
            
            {
                name: "self_formatter_alignment",
                type: "DateTimeAlignment",
                typeUse: "enumerator"
            },
            
            {
                name: "self_formatter_yearStyle",
                type: "YearStyle",
                typeUse: "enumerator"
            },
            
            {
                name: "isoDate_year",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "isoDate_month",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "isoDate_day",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "zone_id_id",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "zone_offset_offset",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "zone_variant",
                type: "TimeZoneVariant",
                typeUse: "enumerator"
            }
            
        ]
    },

    "ZonedDateFormatterGregorian.formatIso": {
        func: (selfLocaleName, selfFormatterLocaleName, selfFormatterLength, selfFormatterAlignment, selfFormatterYearStyle, isoDateYear, isoDateMonth, isoDateDay, zoneIdId, zoneOffsetOffset, zoneVariant) => ZonedDateFormatterGregorian.createGenericShort(Locale.fromString(selfLocaleName), DateFormatterGregorian.createYmd(Locale.fromString(selfFormatterLocaleName), selfFormatterLength, selfFormatterAlignment, selfFormatterYearStyle)).formatIso(new IsoDate(isoDateYear, isoDateMonth, isoDateDay), new TimeZoneInfo(TimeZone.createFromBcp47(zoneIdId), UtcOffset.fromString(zoneOffsetOffset), zoneVariant)),
        // For avoiding webpacking minifying issues:
        funcName: "ZonedDateFormatterGregorian.formatIso",
        expr: (selfLocaleName, selfFormatterLocaleName, selfFormatterLength, selfFormatterAlignment, selfFormatterYearStyle, isoDateYear, isoDateMonth, isoDateDay, zoneIdId, zoneOffsetOffset, zoneVariant) => "ZonedDateFormatterGregorian.createGenericShort(Locale.fromString(selfLocaleName), DateFormatterGregorian.createYmd(Locale.fromString(selfFormatterLocaleName), selfFormatterLength, selfFormatterAlignment, selfFormatterYearStyle)).formatIso(new IsoDate(isoDateYear, isoDateMonth, isoDateDay), new TimeZoneInfo(TimeZone.createFromBcp47(zoneIdId), UtcOffset.fromString(zoneOffsetOffset), zoneVariant))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfFormatterLocaleName([,\) \n])/, '$1' + selfFormatterLocaleName + '$2').replace(/([\( ])selfFormatterLength([,\) \n])/, '$1' + selfFormatterLength + '$2').replace(/([\( ])selfFormatterAlignment([,\) \n])/, '$1' + selfFormatterAlignment + '$2').replace(/([\( ])selfFormatterYearStyle([,\) \n])/, '$1' + selfFormatterYearStyle + '$2').replace(/([\( ])isoDateYear([,\) \n])/, '$1' + isoDateYear + '$2').replace(/([\( ])isoDateMonth([,\) \n])/, '$1' + isoDateMonth + '$2').replace(/([\( ])isoDateDay([,\) \n])/, '$1' + isoDateDay + '$2').replace(/([\( ])zoneIdId([,\) \n])/, '$1' + zoneIdId + '$2').replace(/([\( ])zoneOffsetOffset([,\) \n])/, '$1' + zoneOffsetOffset + '$2').replace(/([\( ])zoneVariant([,\) \n])/, '$1' + zoneVariant + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_formatter_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_formatter_length",
                type: "DateTimeLength",
                typeUse: "enumerator"
            },
            
            {
                name: "self_formatter_alignment",
                type: "DateTimeAlignment",
                typeUse: "enumerator"
            },
            
            {
                name: "self_formatter_yearStyle",
                type: "YearStyle",
                typeUse: "enumerator"
            },
            
            {
                name: "isoDate_year",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "isoDate_month",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "isoDate_day",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "zone_id_id",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "zone_offset_offset",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "zone_variant",
                type: "TimeZoneVariant",
                typeUse: "enumerator"
            }
            
        ]
    },

    "ZonedDateTimeFormatter.formatIso": {
        func: (selfLocaleName, selfFormatterLocaleName, selfFormatterLength, selfFormatterTimePrecision, selfFormatterAlignment, selfFormatterYearStyle, isoDateYear, isoDateMonth, isoDateDay, timeHour, timeMinute, timeSecond, timeSubsecond, zoneIdId, zoneOffsetOffset, zoneVariant) => ZonedDateTimeFormatter.createGenericShort(Locale.fromString(selfLocaleName), DateTimeFormatter.createYmdt(Locale.fromString(selfFormatterLocaleName), selfFormatterLength, selfFormatterTimePrecision, selfFormatterAlignment, selfFormatterYearStyle)).formatIso(new IsoDate(isoDateYear, isoDateMonth, isoDateDay), new Time(timeHour, timeMinute, timeSecond, timeSubsecond), new TimeZoneInfo(TimeZone.createFromBcp47(zoneIdId), UtcOffset.fromString(zoneOffsetOffset), zoneVariant)),
        // For avoiding webpacking minifying issues:
        funcName: "ZonedDateTimeFormatter.formatIso",
        expr: (selfLocaleName, selfFormatterLocaleName, selfFormatterLength, selfFormatterTimePrecision, selfFormatterAlignment, selfFormatterYearStyle, isoDateYear, isoDateMonth, isoDateDay, timeHour, timeMinute, timeSecond, timeSubsecond, zoneIdId, zoneOffsetOffset, zoneVariant) => "ZonedDateTimeFormatter.createGenericShort(Locale.fromString(selfLocaleName), DateTimeFormatter.createYmdt(Locale.fromString(selfFormatterLocaleName), selfFormatterLength, selfFormatterTimePrecision, selfFormatterAlignment, selfFormatterYearStyle)).formatIso(new IsoDate(isoDateYear, isoDateMonth, isoDateDay), new Time(timeHour, timeMinute, timeSecond, timeSubsecond), new TimeZoneInfo(TimeZone.createFromBcp47(zoneIdId), UtcOffset.fromString(zoneOffsetOffset), zoneVariant))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfFormatterLocaleName([,\) \n])/, '$1' + selfFormatterLocaleName + '$2').replace(/([\( ])selfFormatterLength([,\) \n])/, '$1' + selfFormatterLength + '$2').replace(/([\( ])selfFormatterTimePrecision([,\) \n])/, '$1' + selfFormatterTimePrecision + '$2').replace(/([\( ])selfFormatterAlignment([,\) \n])/, '$1' + selfFormatterAlignment + '$2').replace(/([\( ])selfFormatterYearStyle([,\) \n])/, '$1' + selfFormatterYearStyle + '$2').replace(/([\( ])isoDateYear([,\) \n])/, '$1' + isoDateYear + '$2').replace(/([\( ])isoDateMonth([,\) \n])/, '$1' + isoDateMonth + '$2').replace(/([\( ])isoDateDay([,\) \n])/, '$1' + isoDateDay + '$2').replace(/([\( ])timeHour([,\) \n])/, '$1' + timeHour + '$2').replace(/([\( ])timeMinute([,\) \n])/, '$1' + timeMinute + '$2').replace(/([\( ])timeSecond([,\) \n])/, '$1' + timeSecond + '$2').replace(/([\( ])timeSubsecond([,\) \n])/, '$1' + timeSubsecond + '$2').replace(/([\( ])zoneIdId([,\) \n])/, '$1' + zoneIdId + '$2').replace(/([\( ])zoneOffsetOffset([,\) \n])/, '$1' + zoneOffsetOffset + '$2').replace(/([\( ])zoneVariant([,\) \n])/, '$1' + zoneVariant + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_formatter_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_formatter_length",
                type: "DateTimeLength",
                typeUse: "enumerator"
            },
            
            {
                name: "self_formatter_timePrecision",
                type: "TimePrecision",
                typeUse: "enumerator"
            },
            
            {
                name: "self_formatter_alignment",
                type: "DateTimeAlignment",
                typeUse: "enumerator"
            },
            
            {
                name: "self_formatter_yearStyle",
                type: "YearStyle",
                typeUse: "enumerator"
            },
            
            {
                name: "isoDate_year",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "isoDate_month",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "isoDate_day",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_hour",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_minute",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_second",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_subsecond",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "zone_id_id",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "zone_offset_offset",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "zone_variant",
                type: "TimeZoneVariant",
                typeUse: "enumerator"
            }
            
        ]
    },

    "ZonedDateTimeFormatterGregorian.formatIso": {
        func: (selfLocaleName, selfFormatterLocaleName, selfFormatterLength, selfFormatterTimePrecision, selfFormatterAlignment, selfFormatterYearStyle, isoDateYear, isoDateMonth, isoDateDay, timeHour, timeMinute, timeSecond, timeSubsecond, zoneIdId, zoneOffsetOffset, zoneVariant) => ZonedDateTimeFormatterGregorian.createGenericShort(Locale.fromString(selfLocaleName), DateTimeFormatterGregorian.createYmdt(Locale.fromString(selfFormatterLocaleName), selfFormatterLength, selfFormatterTimePrecision, selfFormatterAlignment, selfFormatterYearStyle)).formatIso(new IsoDate(isoDateYear, isoDateMonth, isoDateDay), new Time(timeHour, timeMinute, timeSecond, timeSubsecond), new TimeZoneInfo(TimeZone.createFromBcp47(zoneIdId), UtcOffset.fromString(zoneOffsetOffset), zoneVariant)),
        // For avoiding webpacking minifying issues:
        funcName: "ZonedDateTimeFormatterGregorian.formatIso",
        expr: (selfLocaleName, selfFormatterLocaleName, selfFormatterLength, selfFormatterTimePrecision, selfFormatterAlignment, selfFormatterYearStyle, isoDateYear, isoDateMonth, isoDateDay, timeHour, timeMinute, timeSecond, timeSubsecond, zoneIdId, zoneOffsetOffset, zoneVariant) => "ZonedDateTimeFormatterGregorian.createGenericShort(Locale.fromString(selfLocaleName), DateTimeFormatterGregorian.createYmdt(Locale.fromString(selfFormatterLocaleName), selfFormatterLength, selfFormatterTimePrecision, selfFormatterAlignment, selfFormatterYearStyle)).formatIso(new IsoDate(isoDateYear, isoDateMonth, isoDateDay), new Time(timeHour, timeMinute, timeSecond, timeSubsecond), new TimeZoneInfo(TimeZone.createFromBcp47(zoneIdId), UtcOffset.fromString(zoneOffsetOffset), zoneVariant))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfFormatterLocaleName([,\) \n])/, '$1' + selfFormatterLocaleName + '$2').replace(/([\( ])selfFormatterLength([,\) \n])/, '$1' + selfFormatterLength + '$2').replace(/([\( ])selfFormatterTimePrecision([,\) \n])/, '$1' + selfFormatterTimePrecision + '$2').replace(/([\( ])selfFormatterAlignment([,\) \n])/, '$1' + selfFormatterAlignment + '$2').replace(/([\( ])selfFormatterYearStyle([,\) \n])/, '$1' + selfFormatterYearStyle + '$2').replace(/([\( ])isoDateYear([,\) \n])/, '$1' + isoDateYear + '$2').replace(/([\( ])isoDateMonth([,\) \n])/, '$1' + isoDateMonth + '$2').replace(/([\( ])isoDateDay([,\) \n])/, '$1' + isoDateDay + '$2').replace(/([\( ])timeHour([,\) \n])/, '$1' + timeHour + '$2').replace(/([\( ])timeMinute([,\) \n])/, '$1' + timeMinute + '$2').replace(/([\( ])timeSecond([,\) \n])/, '$1' + timeSecond + '$2').replace(/([\( ])timeSubsecond([,\) \n])/, '$1' + timeSubsecond + '$2').replace(/([\( ])zoneIdId([,\) \n])/, '$1' + zoneIdId + '$2').replace(/([\( ])zoneOffsetOffset([,\) \n])/, '$1' + zoneOffsetOffset + '$2').replace(/([\( ])zoneVariant([,\) \n])/, '$1' + zoneVariant + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_formatter_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_formatter_length",
                type: "DateTimeLength",
                typeUse: "enumerator"
            },
            
            {
                name: "self_formatter_timePrecision",
                type: "TimePrecision",
                typeUse: "enumerator"
            },
            
            {
                name: "self_formatter_alignment",
                type: "DateTimeAlignment",
                typeUse: "enumerator"
            },
            
            {
                name: "self_formatter_yearStyle",
                type: "YearStyle",
                typeUse: "enumerator"
            },
            
            {
                name: "isoDate_year",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "isoDate_month",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "isoDate_day",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_hour",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_minute",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_second",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_subsecond",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "zone_id_id",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "zone_offset_offset",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "zone_variant",
                type: "TimeZoneVariant",
                typeUse: "enumerator"
            }
            
        ]
    },

    "ZonedTimeFormatter.format": {
        func: (selfLocaleName, selfLength, selfTimePrecision, selfAlignment, timeHour, timeMinute, timeSecond, timeSubsecond, zoneIdId, zoneOffsetOffset, zoneVariant) => ZonedTimeFormatter.createGenericShort(Locale.fromString(selfLocaleName), selfLength, selfTimePrecision, selfAlignment).format(new Time(timeHour, timeMinute, timeSecond, timeSubsecond), new TimeZoneInfo(TimeZone.createFromBcp47(zoneIdId), UtcOffset.fromString(zoneOffsetOffset), zoneVariant)),
        // For avoiding webpacking minifying issues:
        funcName: "ZonedTimeFormatter.format",
        expr: (selfLocaleName, selfLength, selfTimePrecision, selfAlignment, timeHour, timeMinute, timeSecond, timeSubsecond, zoneIdId, zoneOffsetOffset, zoneVariant) => "ZonedTimeFormatter.createGenericShort(Locale.fromString(selfLocaleName), selfLength, selfTimePrecision, selfAlignment).format(new Time(timeHour, timeMinute, timeSecond, timeSubsecond), new TimeZoneInfo(TimeZone.createFromBcp47(zoneIdId), UtcOffset.fromString(zoneOffsetOffset), zoneVariant))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfLength([,\) \n])/, '$1' + selfLength + '$2').replace(/([\( ])selfTimePrecision([,\) \n])/, '$1' + selfTimePrecision + '$2').replace(/([\( ])selfAlignment([,\) \n])/, '$1' + selfAlignment + '$2').replace(/([\( ])timeHour([,\) \n])/, '$1' + timeHour + '$2').replace(/([\( ])timeMinute([,\) \n])/, '$1' + timeMinute + '$2').replace(/([\( ])timeSecond([,\) \n])/, '$1' + timeSecond + '$2').replace(/([\( ])timeSubsecond([,\) \n])/, '$1' + timeSubsecond + '$2').replace(/([\( ])zoneIdId([,\) \n])/, '$1' + zoneIdId + '$2').replace(/([\( ])zoneOffsetOffset([,\) \n])/, '$1' + zoneOffsetOffset + '$2').replace(/([\( ])zoneVariant([,\) \n])/, '$1' + zoneVariant + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_length",
                type: "DateTimeLength",
                typeUse: "enumerator"
            },
            
            {
                name: "self_timePrecision",
                type: "TimePrecision",
                typeUse: "enumerator"
            },
            
            {
                name: "self_alignment",
                type: "DateTimeAlignment",
                typeUse: "enumerator"
            },
            
            {
                name: "time_hour",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_minute",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_second",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "time_subsecond",
                type: "number",
                typeUse: "number"
            },
            
            {
                name: "zone_id_id",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "zone_offset_offset",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "zone_variant",
                type: "TimeZoneVariant",
                typeUse: "enumerator"
            }
            
        ]
    },

    "CalendarKind.create": {
        func: (localeName) => CalendarKind.create(Locale.fromString(localeName)),
        // For avoiding webpacking minifying issues:
        funcName: "CalendarKind.create",
        expr: (localeName) => "CalendarKind.create(Locale.fromString(localeName))".replace(/([\( ])localeName([,\) \n])/, '$1' + localeName + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "locale_name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "TimePrecision.fromSubsecondDigits": {
        func: (digits) => TimePrecision.fromSubsecondDigits(digits),
        // For avoiding webpacking minifying issues:
        funcName: "TimePrecision.fromSubsecondDigits",
        expr: (digits) => "TimePrecision.fromSubsecondDigits(digits)".replace(/([\( ])digits([,\) \n])/, '$1' + digits + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "digits",
                type: "number",
                typeUse: "number"
            }
            
        ]
    },

    "PluralCategory.getForCldrString": {
        func: (s) => PluralCategory.getForCldrString(s),
        // For avoiding webpacking minifying issues:
        funcName: "PluralCategory.getForCldrString",
        expr: (s) => "PluralCategory.getForCldrString(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "BidiClass.forChar": {
        func: (ch) => BidiClass.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "BidiClass.forChar",
        expr: (ch) => "BidiClass.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "CanonicalCombiningClass.forChar": {
        func: (ch) => CanonicalCombiningClass.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "CanonicalCombiningClass.forChar",
        expr: (ch) => "CanonicalCombiningClass.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "EastAsianWidth.forChar": {
        func: (ch) => EastAsianWidth.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "EastAsianWidth.forChar",
        expr: (ch) => "EastAsianWidth.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "GeneralCategory.forChar": {
        func: (ch) => GeneralCategory.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "GeneralCategory.forChar",
        expr: (ch) => "GeneralCategory.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "GraphemeClusterBreak.forChar": {
        func: (ch) => GraphemeClusterBreak.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "GraphemeClusterBreak.forChar",
        expr: (ch) => "GraphemeClusterBreak.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "HangulSyllableType.forChar": {
        func: (ch) => HangulSyllableType.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "HangulSyllableType.forChar",
        expr: (ch) => "HangulSyllableType.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "IndicSyllabicCategory.forChar": {
        func: (ch) => IndicSyllabicCategory.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "IndicSyllabicCategory.forChar",
        expr: (ch) => "IndicSyllabicCategory.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "JoiningType.forChar": {
        func: (ch) => JoiningType.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "JoiningType.forChar",
        expr: (ch) => "JoiningType.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "LineBreak.forChar": {
        func: (ch) => LineBreak.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "LineBreak.forChar",
        expr: (ch) => "LineBreak.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "Script.forChar": {
        func: (ch) => Script.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "Script.forChar",
        expr: (ch) => "Script.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "SentenceBreak.forChar": {
        func: (ch) => SentenceBreak.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "SentenceBreak.forChar",
        expr: (ch) => "SentenceBreak.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "VerticalOrientation.forChar": {
        func: (ch) => VerticalOrientation.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "VerticalOrientation.forChar",
        expr: (ch) => "VerticalOrientation.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "WordBreak.forChar": {
        func: (ch) => WordBreak.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "WordBreak.forChar",
        expr: (ch) => "WordBreak.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "SegmenterWordType.isWordLike": {
        func: (self) => new SegmenterWordType(self).isWordLike,
        // For avoiding webpacking minifying issues:
        funcName: "SegmenterWordType.isWordLike",
        expr: (self) => "new SegmenterWordType(self).isWordLike".replace(/([\( ])self([,\) \n])/, '$1' + self + '$2'),
        display: displayBool,
        parameters: [
            
            {
                name: "self",
                type: "SegmenterWordType",
                typeUse: "enumerator"
            }
            
        ]
    },

    "TimeZoneVariant.fromRearguardIsdst": {
        func: (isdst) => TimeZoneVariant.fromRearguardIsdst(isdst),
        // For avoiding webpacking minifying issues:
        funcName: "TimeZoneVariant.fromRearguardIsdst",
        expr: (isdst) => "TimeZoneVariant.fromRearguardIsdst(isdst)".replace(/([\( ])isdst([,\) \n])/, '$1' + isdst + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "isdst",
                type: "boolean",
                typeUse: "boolean"
            }
            
        ]
    }
}, RenderTerminiLocaleCanonicalizer, RenderTerminiWordSegmenter);

export const RenderInfo = {
    "termini": termini
};