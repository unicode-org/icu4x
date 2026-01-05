import * as icu from "icu";
export * as icu from "icu";
import RenderTerminiLocaleCanonicalizer from "./LocaleCanonicalizer.mjs";
import RenderTerminiWordSegmenter from "./WordSegmenter.mjs";

const displayBool = (out) => out ? 'true' : 'false';
const displayOrdering = (out) => out == 0 ? '==' : out == 1 ? '>' : '<';
const displayChar = (out) => String.fromCharCode(out);
const displayByte = (out) => '0x' + out.toString(16);
const displayOptionalEnum = (out) => out?.value || 'None';

let termini = Object.assign({
    "GeneralCategoryGroup.contains": {
        func: (selfMask, val) => icu.GeneralCategoryGroup.fromFields({
            mask: selfMask
        }).contains(val),
        // For avoiding webpacking minifying issues:
        funcName: "GeneralCategoryGroup.contains",
        expr: (selfMask, val) => "icu.GeneralCategoryGroup.fromFields({\n    mask: selfMask\n}).contains(val)".replace(/([\( ])selfMask([,\) \n])/, '$1' + selfMask + '$2').replace(/([\( ])val([,\) \n])/, '$1' + val + '$2'),
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
                typeUse: "enumerator",
                values: ["Unassigned", "UppercaseLetter", "LowercaseLetter", "TitlecaseLetter", "ModifierLetter", "OtherLetter", "NonspacingMark", "SpacingMark", "EnclosingMark", "DecimalNumber", "LetterNumber", "OtherNumber", "SpaceSeparator", "LineSeparator", "ParagraphSeparator", "Control", "Format", "PrivateUse", "Surrogate", "DashPunctuation", "OpenPunctuation", "ClosePunctuation", "ConnectorPunctuation", "InitialPunctuation", "FinalPunctuation", "OtherPunctuation", "MathSymbol", "CurrencySymbol", "ModifierSymbol", "OtherSymbol"]
            }
            
        ]
    },

    "CaseMapper.lowercase": {
        func: (s, localeName) => new icu.CaseMapper().lowercase(s, icu.Locale.fromString(localeName)),
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.lowercase",
        expr: (s, localeName) => "new icu.CaseMapper().lowercase(s, icu.Locale.fromString(localeName))".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2').replace(/([\( ])localeName([,\) \n])/, '$1' + localeName + '$2'),
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
        func: (s, localeName) => new icu.CaseMapper().uppercase(s, icu.Locale.fromString(localeName)),
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.uppercase",
        expr: (s, localeName) => "new icu.CaseMapper().uppercase(s, icu.Locale.fromString(localeName))".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2').replace(/([\( ])localeName([,\) \n])/, '$1' + localeName + '$2'),
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
        func: (s, localeName, optionsLeadingAdjustment, optionsTrailingCase) => new icu.CaseMapper().titlecaseSegmentWithOnlyCaseData(s, icu.Locale.fromString(localeName), icu.TitlecaseOptions.fromFields({
            leadingAdjustment: optionsLeadingAdjustment,
            trailingCase: optionsTrailingCase
        })),
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.titlecaseSegmentWithOnlyCaseData",
        expr: (s, localeName, optionsLeadingAdjustment, optionsTrailingCase) => "new icu.CaseMapper().titlecaseSegmentWithOnlyCaseData(s, icu.Locale.fromString(localeName), icu.TitlecaseOptions.fromFields({\n    leadingAdjustment: optionsLeadingAdjustment,\n    trailingCase: optionsTrailingCase\n}))".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2').replace(/([\( ])localeName([,\) \n])/, '$1' + localeName + '$2').replace(/([\( ])optionsLeadingAdjustment([,\) \n])/, '$1' + optionsLeadingAdjustment + '$2').replace(/([\( ])optionsTrailingCase([,\) \n])/, '$1' + optionsTrailingCase + '$2'),
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
                typeUse: "enumerator",
                values: ["Auto", "None", "ToCased"]
            },
            
            {
                name: "options_trailingCase",
                type: "TrailingCase",
                typeUse: "enumerator",
                values: ["Lower", "Unchanged"]
            }
            
        ]
    },

    "CaseMapper.fold": {
        func: (s) => new icu.CaseMapper().fold(s),
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.fold",
        expr: (s) => "new icu.CaseMapper().fold(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "CaseMapper.foldTurkic": {
        func: (s) => new icu.CaseMapper().foldTurkic(s),
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.foldTurkic",
        expr: (s) => "new icu.CaseMapper().foldTurkic(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "CaseMapper.simpleLowercase": {
        func: (ch) => new icu.CaseMapper().simpleLowercase(ch),
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.simpleLowercase",
        expr: (ch) => "new icu.CaseMapper().simpleLowercase(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
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
        func: (ch) => new icu.CaseMapper().simpleUppercase(ch),
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.simpleUppercase",
        expr: (ch) => "new icu.CaseMapper().simpleUppercase(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
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
        func: (ch) => new icu.CaseMapper().simpleTitlecase(ch),
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.simpleTitlecase",
        expr: (ch) => "new icu.CaseMapper().simpleTitlecase(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
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
        func: (ch) => new icu.CaseMapper().simpleFold(ch),
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.simpleFold",
        expr: (ch) => "new icu.CaseMapper().simpleFold(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
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
        func: (ch) => new icu.CaseMapper().simpleFoldTurkic(ch),
        // For avoiding webpacking minifying issues:
        funcName: "CaseMapper.simpleFoldTurkic",
        expr: (ch) => "new icu.CaseMapper().simpleFoldTurkic(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
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
        func: (s, localeName, optionsLeadingAdjustment, optionsTrailingCase) => new icu.TitlecaseMapper().titlecaseSegment(s, icu.Locale.fromString(localeName), icu.TitlecaseOptions.fromFields({
            leadingAdjustment: optionsLeadingAdjustment,
            trailingCase: optionsTrailingCase
        })),
        // For avoiding webpacking minifying issues:
        funcName: "TitlecaseMapper.titlecaseSegment",
        expr: (s, localeName, optionsLeadingAdjustment, optionsTrailingCase) => "new icu.TitlecaseMapper().titlecaseSegment(s, icu.Locale.fromString(localeName), icu.TitlecaseOptions.fromFields({\n    leadingAdjustment: optionsLeadingAdjustment,\n    trailingCase: optionsTrailingCase\n}))".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2').replace(/([\( ])localeName([,\) \n])/, '$1' + localeName + '$2').replace(/([\( ])optionsLeadingAdjustment([,\) \n])/, '$1' + optionsLeadingAdjustment + '$2').replace(/([\( ])optionsTrailingCase([,\) \n])/, '$1' + optionsTrailingCase + '$2'),
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
                typeUse: "enumerator",
                values: ["Auto", "None", "ToCased"]
            },
            
            {
                name: "options_trailingCase",
                type: "TrailingCase",
                typeUse: "enumerator",
                values: ["Lower", "Unchanged"]
            }
            
        ]
    },

    "Collator.compare": {
        func: (selfLocaleName, selfOptionsStrength, selfOptionsAlternateHandling, selfOptionsMaxVariable, selfOptionsCaseLevel, left, right) => new icu.Collator(icu.Locale.fromString(selfLocaleName), icu.CollatorOptions.fromFields({
            strength: selfOptionsStrength,
            alternateHandling: selfOptionsAlternateHandling,
            maxVariable: selfOptionsMaxVariable,
            caseLevel: selfOptionsCaseLevel
        })).compare(left, right),
        // For avoiding webpacking minifying issues:
        funcName: "Collator.compare",
        expr: (selfLocaleName, selfOptionsStrength, selfOptionsAlternateHandling, selfOptionsMaxVariable, selfOptionsCaseLevel, left, right) => "new icu.Collator(icu.Locale.fromString(selfLocaleName), icu.CollatorOptions.fromFields({\n    strength: selfOptionsStrength,\n    alternateHandling: selfOptionsAlternateHandling,\n    maxVariable: selfOptionsMaxVariable,\n    caseLevel: selfOptionsCaseLevel\n})).compare(left, right)".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfOptionsStrength([,\) \n])/, '$1' + selfOptionsStrength + '$2').replace(/([\( ])selfOptionsAlternateHandling([,\) \n])/, '$1' + selfOptionsAlternateHandling + '$2').replace(/([\( ])selfOptionsMaxVariable([,\) \n])/, '$1' + selfOptionsMaxVariable + '$2').replace(/([\( ])selfOptionsCaseLevel([,\) \n])/, '$1' + selfOptionsCaseLevel + '$2').replace(/([\( ])left([,\) \n])/, '$1' + left + '$2').replace(/([\( ])right([,\) \n])/, '$1' + right + '$2'),
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
                typeUse: "enumerator",
                values: ["Primary", "Secondary", "Tertiary", "Quaternary", "Identical"]
            },
            
            {
                name: "self_options_alternateHandling",
                type: "CollatorAlternateHandling",
                typeUse: "enumerator",
                values: ["NonIgnorable", "Shifted"]
            },
            
            {
                name: "self_options_maxVariable",
                type: "CollatorMaxVariable",
                typeUse: "enumerator",
                values: ["Space", "Punctuation", "Symbol", "Currency"]
            },
            
            {
                name: "self_options_caseLevel",
                type: "CollatorCaseLevel",
                typeUse: "enumerator",
                values: ["Off", "On"]
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
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).rataDie,
        // For avoiding webpacking minifying issues:
        funcName: "Date.rataDie",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).rataDie".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
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
                typeUse: "enumerator",
                values: ["Iso", "Gregorian", "Buddhist", "Japanese", "JapaneseExtended", "Ethiopian", "EthiopianAmeteAlem", "Indian", "Coptic", "Dangi", "Chinese", "Hebrew", "HijriTabularTypeIIFriday", "HijriSimulatedMecca", "HijriTabularTypeIIThursday", "HijriUmmAlQura", "Persian", "Roc"]
            }
            
        ]
    },

    "Date.dayOfYear": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).dayOfYear,
        // For avoiding webpacking minifying issues:
        funcName: "Date.dayOfYear",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).dayOfYear".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
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
                typeUse: "enumerator",
                values: ["Iso", "Gregorian", "Buddhist", "Japanese", "JapaneseExtended", "Ethiopian", "EthiopianAmeteAlem", "Indian", "Coptic", "Dangi", "Chinese", "Hebrew", "HijriTabularTypeIIFriday", "HijriSimulatedMecca", "HijriTabularTypeIIThursday", "HijriUmmAlQura", "Persian", "Roc"]
            }
            
        ]
    },

    "Date.dayOfMonth": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).dayOfMonth,
        // For avoiding webpacking minifying issues:
        funcName: "Date.dayOfMonth",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).dayOfMonth".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
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
                typeUse: "enumerator",
                values: ["Iso", "Gregorian", "Buddhist", "Japanese", "JapaneseExtended", "Ethiopian", "EthiopianAmeteAlem", "Indian", "Coptic", "Dangi", "Chinese", "Hebrew", "HijriTabularTypeIIFriday", "HijriSimulatedMecca", "HijriTabularTypeIIThursday", "HijriUmmAlQura", "Persian", "Roc"]
            }
            
        ]
    },

    "Date.ordinalMonth": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).ordinalMonth,
        // For avoiding webpacking minifying issues:
        funcName: "Date.ordinalMonth",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).ordinalMonth".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
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
                typeUse: "enumerator",
                values: ["Iso", "Gregorian", "Buddhist", "Japanese", "JapaneseExtended", "Ethiopian", "EthiopianAmeteAlem", "Indian", "Coptic", "Dangi", "Chinese", "Hebrew", "HijriTabularTypeIIFriday", "HijriSimulatedMecca", "HijriTabularTypeIIThursday", "HijriUmmAlQura", "Persian", "Roc"]
            }
            
        ]
    },

    "Date.monthCode": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).monthCode,
        // For avoiding webpacking minifying issues:
        funcName: "Date.monthCode",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).monthCode".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
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
                typeUse: "enumerator",
                values: ["Iso", "Gregorian", "Buddhist", "Japanese", "JapaneseExtended", "Ethiopian", "EthiopianAmeteAlem", "Indian", "Coptic", "Dangi", "Chinese", "Hebrew", "HijriTabularTypeIIFriday", "HijriSimulatedMecca", "HijriTabularTypeIIThursday", "HijriUmmAlQura", "Persian", "Roc"]
            }
            
        ]
    },

    "Date.monthNumber": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).monthNumber,
        // For avoiding webpacking minifying issues:
        funcName: "Date.monthNumber",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).monthNumber".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
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
                typeUse: "enumerator",
                values: ["Iso", "Gregorian", "Buddhist", "Japanese", "JapaneseExtended", "Ethiopian", "EthiopianAmeteAlem", "Indian", "Coptic", "Dangi", "Chinese", "Hebrew", "HijriTabularTypeIIFriday", "HijriSimulatedMecca", "HijriTabularTypeIIThursday", "HijriUmmAlQura", "Persian", "Roc"]
            }
            
        ]
    },

    "Date.monthIsLeap": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).monthIsLeap,
        // For avoiding webpacking minifying issues:
        funcName: "Date.monthIsLeap",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).monthIsLeap".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
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
                typeUse: "enumerator",
                values: ["Iso", "Gregorian", "Buddhist", "Japanese", "JapaneseExtended", "Ethiopian", "EthiopianAmeteAlem", "Indian", "Coptic", "Dangi", "Chinese", "Hebrew", "HijriTabularTypeIIFriday", "HijriSimulatedMecca", "HijriTabularTypeIIThursday", "HijriUmmAlQura", "Persian", "Roc"]
            }
            
        ]
    },

    "Date.eraYearOrRelatedIso": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).eraYearOrRelatedIso,
        // For avoiding webpacking minifying issues:
        funcName: "Date.eraYearOrRelatedIso",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).eraYearOrRelatedIso".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
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
                typeUse: "enumerator",
                values: ["Iso", "Gregorian", "Buddhist", "Japanese", "JapaneseExtended", "Ethiopian", "EthiopianAmeteAlem", "Indian", "Coptic", "Dangi", "Chinese", "Hebrew", "HijriTabularTypeIIFriday", "HijriSimulatedMecca", "HijriTabularTypeIIThursday", "HijriUmmAlQura", "Persian", "Roc"]
            }
            
        ]
    },

    "Date.extendedYear": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).extendedYear,
        // For avoiding webpacking minifying issues:
        funcName: "Date.extendedYear",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).extendedYear".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
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
                typeUse: "enumerator",
                values: ["Iso", "Gregorian", "Buddhist", "Japanese", "JapaneseExtended", "Ethiopian", "EthiopianAmeteAlem", "Indian", "Coptic", "Dangi", "Chinese", "Hebrew", "HijriTabularTypeIIFriday", "HijriSimulatedMecca", "HijriTabularTypeIIThursday", "HijriUmmAlQura", "Persian", "Roc"]
            }
            
        ]
    },

    "Date.era": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).era,
        // For avoiding webpacking minifying issues:
        funcName: "Date.era",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).era".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
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
                typeUse: "enumerator",
                values: ["Iso", "Gregorian", "Buddhist", "Japanese", "JapaneseExtended", "Ethiopian", "EthiopianAmeteAlem", "Indian", "Coptic", "Dangi", "Chinese", "Hebrew", "HijriTabularTypeIIFriday", "HijriSimulatedMecca", "HijriTabularTypeIIThursday", "HijriUmmAlQura", "Persian", "Roc"]
            }
            
        ]
    },

    "Date.monthsInYear": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).monthsInYear,
        // For avoiding webpacking minifying issues:
        funcName: "Date.monthsInYear",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).monthsInYear".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
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
                typeUse: "enumerator",
                values: ["Iso", "Gregorian", "Buddhist", "Japanese", "JapaneseExtended", "Ethiopian", "EthiopianAmeteAlem", "Indian", "Coptic", "Dangi", "Chinese", "Hebrew", "HijriTabularTypeIIFriday", "HijriSimulatedMecca", "HijriTabularTypeIIThursday", "HijriUmmAlQura", "Persian", "Roc"]
            }
            
        ]
    },

    "Date.daysInMonth": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).daysInMonth,
        // For avoiding webpacking minifying issues:
        funcName: "Date.daysInMonth",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).daysInMonth".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
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
                typeUse: "enumerator",
                values: ["Iso", "Gregorian", "Buddhist", "Japanese", "JapaneseExtended", "Ethiopian", "EthiopianAmeteAlem", "Indian", "Coptic", "Dangi", "Chinese", "Hebrew", "HijriTabularTypeIIFriday", "HijriSimulatedMecca", "HijriTabularTypeIIThursday", "HijriUmmAlQura", "Persian", "Roc"]
            }
            
        ]
    },

    "Date.daysInYear": {
        func: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).daysInYear,
        // For avoiding webpacking minifying issues:
        funcName: "Date.daysInYear",
        expr: (selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) => "icu.Date.fromIsoInCalendar(selfIsoYear, selfIsoMonth, selfIsoDay, new icu.Calendar(selfCalendarKind)).daysInYear".replace(/([\( ])selfIsoYear([,\) \n])/, '$1' + selfIsoYear + '$2').replace(/([\( ])selfIsoMonth([,\) \n])/, '$1' + selfIsoMonth + '$2').replace(/([\( ])selfIsoDay([,\) \n])/, '$1' + selfIsoDay + '$2').replace(/([\( ])selfCalendarKind([,\) \n])/, '$1' + selfCalendarKind + '$2'),
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
                typeUse: "enumerator",
                values: ["Iso", "Gregorian", "Buddhist", "Japanese", "JapaneseExtended", "Ethiopian", "EthiopianAmeteAlem", "Indian", "Coptic", "Dangi", "Chinese", "Hebrew", "HijriTabularTypeIIFriday", "HijriSimulatedMecca", "HijriTabularTypeIIThursday", "HijriUmmAlQura", "Persian", "Roc"]
            }
            
        ]
    },

    "DateFormatter.formatIso": {
        func: (selfLocaleName, selfLength, selfAlignment, selfYearStyle, isoDateYear, isoDateMonth, isoDateDay) => icu.DateFormatter.createYmd(icu.Locale.fromString(selfLocaleName), selfLength, selfAlignment, selfYearStyle).formatIso(new icu.IsoDate(isoDateYear, isoDateMonth, isoDateDay)),
        // For avoiding webpacking minifying issues:
        funcName: "DateFormatter.formatIso",
        expr: (selfLocaleName, selfLength, selfAlignment, selfYearStyle, isoDateYear, isoDateMonth, isoDateDay) => "icu.DateFormatter.createYmd(icu.Locale.fromString(selfLocaleName), selfLength, selfAlignment, selfYearStyle).formatIso(new icu.IsoDate(isoDateYear, isoDateMonth, isoDateDay))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfLength([,\) \n])/, '$1' + selfLength + '$2').replace(/([\( ])selfAlignment([,\) \n])/, '$1' + selfAlignment + '$2').replace(/([\( ])selfYearStyle([,\) \n])/, '$1' + selfYearStyle + '$2').replace(/([\( ])isoDateYear([,\) \n])/, '$1' + isoDateYear + '$2').replace(/([\( ])isoDateMonth([,\) \n])/, '$1' + isoDateMonth + '$2').replace(/([\( ])isoDateDay([,\) \n])/, '$1' + isoDateDay + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_length",
                type: "DateTimeLength",
                typeUse: "enumerator",
                values: ["Long", "Medium", "Short"]
            },
            
            {
                name: "self_alignment",
                type: "DateTimeAlignment",
                typeUse: "enumerator",
                values: ["Auto", "Column"]
            },
            
            {
                name: "self_yearStyle",
                type: "YearStyle",
                typeUse: "enumerator",
                values: ["Auto", "Full", "WithEra"]
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
        func: (selfLocaleName, selfLength, selfAlignment, selfYearStyle, isoDateYear, isoDateMonth, isoDateDay) => icu.DateFormatterGregorian.createYmd(icu.Locale.fromString(selfLocaleName), selfLength, selfAlignment, selfYearStyle).formatIso(new icu.IsoDate(isoDateYear, isoDateMonth, isoDateDay)),
        // For avoiding webpacking minifying issues:
        funcName: "DateFormatterGregorian.formatIso",
        expr: (selfLocaleName, selfLength, selfAlignment, selfYearStyle, isoDateYear, isoDateMonth, isoDateDay) => "icu.DateFormatterGregorian.createYmd(icu.Locale.fromString(selfLocaleName), selfLength, selfAlignment, selfYearStyle).formatIso(new icu.IsoDate(isoDateYear, isoDateMonth, isoDateDay))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfLength([,\) \n])/, '$1' + selfLength + '$2').replace(/([\( ])selfAlignment([,\) \n])/, '$1' + selfAlignment + '$2').replace(/([\( ])selfYearStyle([,\) \n])/, '$1' + selfYearStyle + '$2').replace(/([\( ])isoDateYear([,\) \n])/, '$1' + isoDateYear + '$2').replace(/([\( ])isoDateMonth([,\) \n])/, '$1' + isoDateMonth + '$2').replace(/([\( ])isoDateDay([,\) \n])/, '$1' + isoDateDay + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_length",
                type: "DateTimeLength",
                typeUse: "enumerator",
                values: ["Long", "Medium", "Short"]
            },
            
            {
                name: "self_alignment",
                type: "DateTimeAlignment",
                typeUse: "enumerator",
                values: ["Auto", "Column"]
            },
            
            {
                name: "self_yearStyle",
                type: "YearStyle",
                typeUse: "enumerator",
                values: ["Auto", "Full", "WithEra"]
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
        func: (selfLocaleName, selfLength, selfTimePrecision, selfAlignment, selfYearStyle, isoDateYear, isoDateMonth, isoDateDay, timeHour, timeMinute, timeSecond, timeSubsecond) => icu.DateTimeFormatter.createYmdt(icu.Locale.fromString(selfLocaleName), selfLength, selfTimePrecision, selfAlignment, selfYearStyle).formatIso(new icu.IsoDate(isoDateYear, isoDateMonth, isoDateDay), new icu.Time(timeHour, timeMinute, timeSecond, timeSubsecond)),
        // For avoiding webpacking minifying issues:
        funcName: "DateTimeFormatter.formatIso",
        expr: (selfLocaleName, selfLength, selfTimePrecision, selfAlignment, selfYearStyle, isoDateYear, isoDateMonth, isoDateDay, timeHour, timeMinute, timeSecond, timeSubsecond) => "icu.DateTimeFormatter.createYmdt(icu.Locale.fromString(selfLocaleName), selfLength, selfTimePrecision, selfAlignment, selfYearStyle).formatIso(new icu.IsoDate(isoDateYear, isoDateMonth, isoDateDay), new icu.Time(timeHour, timeMinute, timeSecond, timeSubsecond))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfLength([,\) \n])/, '$1' + selfLength + '$2').replace(/([\( ])selfTimePrecision([,\) \n])/, '$1' + selfTimePrecision + '$2').replace(/([\( ])selfAlignment([,\) \n])/, '$1' + selfAlignment + '$2').replace(/([\( ])selfYearStyle([,\) \n])/, '$1' + selfYearStyle + '$2').replace(/([\( ])isoDateYear([,\) \n])/, '$1' + isoDateYear + '$2').replace(/([\( ])isoDateMonth([,\) \n])/, '$1' + isoDateMonth + '$2').replace(/([\( ])isoDateDay([,\) \n])/, '$1' + isoDateDay + '$2').replace(/([\( ])timeHour([,\) \n])/, '$1' + timeHour + '$2').replace(/([\( ])timeMinute([,\) \n])/, '$1' + timeMinute + '$2').replace(/([\( ])timeSecond([,\) \n])/, '$1' + timeSecond + '$2').replace(/([\( ])timeSubsecond([,\) \n])/, '$1' + timeSubsecond + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_length",
                type: "DateTimeLength",
                typeUse: "enumerator",
                values: ["Long", "Medium", "Short"]
            },
            
            {
                name: "self_timePrecision",
                type: "TimePrecision",
                typeUse: "enumerator",
                values: ["Hour", "Minute", "MinuteOptional", "Second", "Subsecond1", "Subsecond2", "Subsecond3", "Subsecond4", "Subsecond5", "Subsecond6", "Subsecond7", "Subsecond8", "Subsecond9"]
            },
            
            {
                name: "self_alignment",
                type: "DateTimeAlignment",
                typeUse: "enumerator",
                values: ["Auto", "Column"]
            },
            
            {
                name: "self_yearStyle",
                type: "YearStyle",
                typeUse: "enumerator",
                values: ["Auto", "Full", "WithEra"]
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
        func: (selfLocaleName, selfLength, selfTimePrecision, selfAlignment, selfYearStyle, isoDateYear, isoDateMonth, isoDateDay, timeHour, timeMinute, timeSecond, timeSubsecond) => icu.DateTimeFormatterGregorian.createYmdt(icu.Locale.fromString(selfLocaleName), selfLength, selfTimePrecision, selfAlignment, selfYearStyle).formatIso(new icu.IsoDate(isoDateYear, isoDateMonth, isoDateDay), new icu.Time(timeHour, timeMinute, timeSecond, timeSubsecond)),
        // For avoiding webpacking minifying issues:
        funcName: "DateTimeFormatterGregorian.formatIso",
        expr: (selfLocaleName, selfLength, selfTimePrecision, selfAlignment, selfYearStyle, isoDateYear, isoDateMonth, isoDateDay, timeHour, timeMinute, timeSecond, timeSubsecond) => "icu.DateTimeFormatterGregorian.createYmdt(icu.Locale.fromString(selfLocaleName), selfLength, selfTimePrecision, selfAlignment, selfYearStyle).formatIso(new icu.IsoDate(isoDateYear, isoDateMonth, isoDateDay), new icu.Time(timeHour, timeMinute, timeSecond, timeSubsecond))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfLength([,\) \n])/, '$1' + selfLength + '$2').replace(/([\( ])selfTimePrecision([,\) \n])/, '$1' + selfTimePrecision + '$2').replace(/([\( ])selfAlignment([,\) \n])/, '$1' + selfAlignment + '$2').replace(/([\( ])selfYearStyle([,\) \n])/, '$1' + selfYearStyle + '$2').replace(/([\( ])isoDateYear([,\) \n])/, '$1' + isoDateYear + '$2').replace(/([\( ])isoDateMonth([,\) \n])/, '$1' + isoDateMonth + '$2').replace(/([\( ])isoDateDay([,\) \n])/, '$1' + isoDateDay + '$2').replace(/([\( ])timeHour([,\) \n])/, '$1' + timeHour + '$2').replace(/([\( ])timeMinute([,\) \n])/, '$1' + timeMinute + '$2').replace(/([\( ])timeSecond([,\) \n])/, '$1' + timeSecond + '$2').replace(/([\( ])timeSubsecond([,\) \n])/, '$1' + timeSubsecond + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_length",
                type: "DateTimeLength",
                typeUse: "enumerator",
                values: ["Long", "Medium", "Short"]
            },
            
            {
                name: "self_timePrecision",
                type: "TimePrecision",
                typeUse: "enumerator",
                values: ["Hour", "Minute", "MinuteOptional", "Second", "Subsecond1", "Subsecond2", "Subsecond3", "Subsecond4", "Subsecond5", "Subsecond6", "Subsecond7", "Subsecond8", "Subsecond9"]
            },
            
            {
                name: "self_alignment",
                type: "DateTimeAlignment",
                typeUse: "enumerator",
                values: ["Auto", "Column"]
            },
            
            {
                name: "self_yearStyle",
                type: "YearStyle",
                typeUse: "enumerator",
                values: ["Auto", "Full", "WithEra"]
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
        func: (selfLocaleName, selfGroupingStrategy, valueV) => icu.DecimalFormatter.createWithGroupingStrategy(icu.Locale.fromString(selfLocaleName), selfGroupingStrategy).format(icu.Decimal.fromString(valueV)),
        // For avoiding webpacking minifying issues:
        funcName: "DecimalFormatter.format",
        expr: (selfLocaleName, selfGroupingStrategy, valueV) => "icu.DecimalFormatter.createWithGroupingStrategy(icu.Locale.fromString(selfLocaleName), selfGroupingStrategy).format(icu.Decimal.fromString(valueV))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfGroupingStrategy([,\) \n])/, '$1' + selfGroupingStrategy + '$2').replace(/([\( ])valueV([,\) \n])/, '$1' + valueV + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_groupingStrategy",
                type: "DecimalGroupingStrategy",
                typeUse: "enumerator",
                values: ["Auto", "Never", "Always", "Min2"]
            },
            
            {
                name: "value_v",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "ExemplarCharacters.containsStr": {
        func: (selfLocaleName, s) => icu.ExemplarCharacters.createMain(icu.Locale.fromString(selfLocaleName)).containsStr(s),
        // For avoiding webpacking minifying issues:
        funcName: "ExemplarCharacters.containsStr",
        expr: (selfLocaleName, s) => "icu.ExemplarCharacters.createMain(icu.Locale.fromString(selfLocaleName)).containsStr(s)".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
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
        func: (selfLocaleName, cp) => icu.ExemplarCharacters.createMain(icu.Locale.fromString(selfLocaleName)).contains(cp),
        // For avoiding webpacking minifying issues:
        funcName: "ExemplarCharacters.contains",
        expr: (selfLocaleName, cp) => "icu.ExemplarCharacters.createMain(icu.Locale.fromString(selfLocaleName)).contains(cp)".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])cp([,\) \n])/, '$1' + cp + '$2'),
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
        func: (selfV, magnitude) => icu.Decimal.fromString(selfV).digitAt(magnitude),
        // For avoiding webpacking minifying issues:
        funcName: "Decimal.digitAt",
        expr: (selfV, magnitude) => "icu.Decimal.fromString(selfV).digitAt(magnitude)".replace(/([\( ])selfV([,\) \n])/, '$1' + selfV + '$2').replace(/([\( ])magnitude([,\) \n])/, '$1' + magnitude + '$2'),
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
        func: (selfV) => icu.Decimal.fromString(selfV).magnitudeStart,
        // For avoiding webpacking minifying issues:
        funcName: "Decimal.magnitudeStart",
        expr: (selfV) => "icu.Decimal.fromString(selfV).magnitudeStart".replace(/([\( ])selfV([,\) \n])/, '$1' + selfV + '$2'),
        parameters: [
            
            {
                name: "self_v",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Decimal.magnitudeEnd": {
        func: (selfV) => icu.Decimal.fromString(selfV).magnitudeEnd,
        // For avoiding webpacking minifying issues:
        funcName: "Decimal.magnitudeEnd",
        expr: (selfV) => "icu.Decimal.fromString(selfV).magnitudeEnd".replace(/([\( ])selfV([,\) \n])/, '$1' + selfV + '$2'),
        parameters: [
            
            {
                name: "self_v",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Decimal.nonzeroMagnitudeStart": {
        func: (selfV) => icu.Decimal.fromString(selfV).nonzeroMagnitudeStart,
        // For avoiding webpacking minifying issues:
        funcName: "Decimal.nonzeroMagnitudeStart",
        expr: (selfV) => "icu.Decimal.fromString(selfV).nonzeroMagnitudeStart".replace(/([\( ])selfV([,\) \n])/, '$1' + selfV + '$2'),
        parameters: [
            
            {
                name: "self_v",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Decimal.nonzeroMagnitudeEnd": {
        func: (selfV) => icu.Decimal.fromString(selfV).nonzeroMagnitudeEnd,
        // For avoiding webpacking minifying issues:
        funcName: "Decimal.nonzeroMagnitudeEnd",
        expr: (selfV) => "icu.Decimal.fromString(selfV).nonzeroMagnitudeEnd".replace(/([\( ])selfV([,\) \n])/, '$1' + selfV + '$2'),
        parameters: [
            
            {
                name: "self_v",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Decimal.isZero": {
        func: (selfV) => icu.Decimal.fromString(selfV).isZero,
        // For avoiding webpacking minifying issues:
        funcName: "Decimal.isZero",
        expr: (selfV) => "icu.Decimal.fromString(selfV).isZero".replace(/([\( ])selfV([,\) \n])/, '$1' + selfV + '$2'),
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
        func: (selfV) => icu.Decimal.fromString(selfV).sign,
        // For avoiding webpacking minifying issues:
        funcName: "Decimal.sign",
        expr: (selfV) => "icu.Decimal.fromString(selfV).sign".replace(/([\( ])selfV([,\) \n])/, '$1' + selfV + '$2'),
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
        func: (selfLocaleName, selfLength, list) => icu.ListFormatter.createAndWithLength(icu.Locale.fromString(selfLocaleName), selfLength).format(list),
        // For avoiding webpacking minifying issues:
        funcName: "ListFormatter.format",
        expr: (selfLocaleName, selfLength, list) => "icu.ListFormatter.createAndWithLength(icu.Locale.fromString(selfLocaleName), selfLength).format(list)".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfLength([,\) \n])/, '$1' + selfLength + '$2').replace(/([\( ])list([,\) \n])/, '$1' + list + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_length",
                type: "ListLength",
                typeUse: "enumerator",
                values: ["Wide", "Short", "Narrow"]
            },
            
            {
                name: "list",
                type: "Array<string>",
                typeUse: "Array<string>"
            }
            
        ]
    },

    "Locale.basename": {
        func: (selfName) => icu.Locale.fromString(selfName).basename,
        // For avoiding webpacking minifying issues:
        funcName: "Locale.basename",
        expr: (selfName) => "icu.Locale.fromString(selfName).basename".replace(/([\( ])selfName([,\) \n])/, '$1' + selfName + '$2'),
        parameters: [
            
            {
                name: "self_name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Locale.getUnicodeExtension": {
        func: (selfName, s) => icu.Locale.fromString(selfName).getUnicodeExtension(s),
        // For avoiding webpacking minifying issues:
        funcName: "Locale.getUnicodeExtension",
        expr: (selfName, s) => "icu.Locale.fromString(selfName).getUnicodeExtension(s)".replace(/([\( ])selfName([,\) \n])/, '$1' + selfName + '$2').replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
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
        func: (selfName) => icu.Locale.fromString(selfName).language,
        // For avoiding webpacking minifying issues:
        funcName: "Locale.language",
        expr: (selfName) => "icu.Locale.fromString(selfName).language".replace(/([\( ])selfName([,\) \n])/, '$1' + selfName + '$2'),
        parameters: [
            
            {
                name: "self_name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Locale.region": {
        func: (selfName) => icu.Locale.fromString(selfName).region,
        // For avoiding webpacking minifying issues:
        funcName: "Locale.region",
        expr: (selfName) => "icu.Locale.fromString(selfName).region".replace(/([\( ])selfName([,\) \n])/, '$1' + selfName + '$2'),
        parameters: [
            
            {
                name: "self_name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Locale.script": {
        func: (selfName) => icu.Locale.fromString(selfName).script,
        // For avoiding webpacking minifying issues:
        funcName: "Locale.script",
        expr: (selfName) => "icu.Locale.fromString(selfName).script".replace(/([\( ])selfName([,\) \n])/, '$1' + selfName + '$2'),
        parameters: [
            
            {
                name: "self_name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Locale.normalize": {
        func: (s) => icu.Locale.normalize(s),
        // For avoiding webpacking minifying issues:
        funcName: "Locale.normalize",
        expr: (s) => "icu.Locale.normalize(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Locale.toString": {
        func: (selfName) => icu.Locale.fromString(selfName).toString(),
        // For avoiding webpacking minifying issues:
        funcName: "Locale.toString",
        expr: (selfName) => "icu.Locale.fromString(selfName).toString()".replace(/([\( ])selfName([,\) \n])/, '$1' + selfName + '$2'),
        parameters: [
            
            {
                name: "self_name",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Locale.normalizingEq": {
        func: (selfName, other) => icu.Locale.fromString(selfName).normalizingEq(other),
        // For avoiding webpacking minifying issues:
        funcName: "Locale.normalizingEq",
        expr: (selfName, other) => "icu.Locale.fromString(selfName).normalizingEq(other)".replace(/([\( ])selfName([,\) \n])/, '$1' + selfName + '$2').replace(/([\( ])other([,\) \n])/, '$1' + other + '$2'),
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
        func: (selfName, other) => icu.Locale.fromString(selfName).compareToString(other),
        // For avoiding webpacking minifying issues:
        funcName: "Locale.compareToString",
        expr: (selfName, other) => "icu.Locale.fromString(selfName).compareToString(other)".replace(/([\( ])selfName([,\) \n])/, '$1' + selfName + '$2').replace(/([\( ])other([,\) \n])/, '$1' + other + '$2'),
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
        func: (selfName, otherName) => icu.Locale.fromString(selfName).compareTo(icu.Locale.fromString(otherName)),
        // For avoiding webpacking minifying issues:
        funcName: "Locale.compareTo",
        expr: (selfName, otherName) => "icu.Locale.fromString(selfName).compareTo(icu.Locale.fromString(otherName))".replace(/([\( ])selfName([,\) \n])/, '$1' + selfName + '$2').replace(/([\( ])otherName([,\) \n])/, '$1' + otherName + '$2'),
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
        func: (localeName) => new icu.LocaleDirectionality().get(icu.Locale.fromString(localeName)),
        // For avoiding webpacking minifying issues:
        funcName: "LocaleDirectionality.get",
        expr: (localeName) => "new icu.LocaleDirectionality().get(icu.Locale.fromString(localeName))".replace(/([\( ])localeName([,\) \n])/, '$1' + localeName + '$2'),
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
        func: (s) => icu.ComposingNormalizer.createNfc().normalize(s),
        // For avoiding webpacking minifying issues:
        funcName: "ComposingNormalizer.normalize",
        expr: (s) => "icu.ComposingNormalizer.createNfc().normalize(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "ComposingNormalizer.isNormalized": {
        func: (s) => icu.ComposingNormalizer.createNfc().isNormalized(s),
        // For avoiding webpacking minifying issues:
        funcName: "ComposingNormalizer.isNormalized",
        expr: (s) => "icu.ComposingNormalizer.createNfc().isNormalized(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
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
        func: (s) => icu.ComposingNormalizer.createNfc().isNormalizedUpTo(s),
        // For avoiding webpacking minifying issues:
        funcName: "ComposingNormalizer.isNormalizedUpTo",
        expr: (s) => "icu.ComposingNormalizer.createNfc().isNormalizedUpTo(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "DecomposingNormalizer.normalize": {
        func: (s) => icu.DecomposingNormalizer.createNfd().normalize(s),
        // For avoiding webpacking minifying issues:
        funcName: "DecomposingNormalizer.normalize",
        expr: (s) => "icu.DecomposingNormalizer.createNfd().normalize(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "DecomposingNormalizer.isNormalized": {
        func: (s) => icu.DecomposingNormalizer.createNfd().isNormalized(s),
        // For avoiding webpacking minifying issues:
        funcName: "DecomposingNormalizer.isNormalized",
        expr: (s) => "icu.DecomposingNormalizer.createNfd().isNormalized(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
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
        func: (s) => icu.DecomposingNormalizer.createNfd().isNormalizedUpTo(s),
        // For avoiding webpacking minifying issues:
        funcName: "DecomposingNormalizer.isNormalizedUpTo",
        expr: (s) => "icu.DecomposingNormalizer.createNfd().isNormalizedUpTo(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "CanonicalCombiningClassMap.get": {
        func: (ch) => new icu.CanonicalCombiningClassMap().get(ch),
        // For avoiding webpacking minifying issues:
        funcName: "CanonicalCombiningClassMap.get",
        expr: (ch) => "new icu.CanonicalCombiningClassMap().get(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "CanonicalComposition.compose": {
        func: (starter, second) => new icu.CanonicalComposition().compose(starter, second),
        // For avoiding webpacking minifying issues:
        funcName: "CanonicalComposition.compose",
        expr: (starter, second) => "new icu.CanonicalComposition().compose(starter, second)".replace(/([\( ])starter([,\) \n])/, '$1' + starter + '$2').replace(/([\( ])second([,\) \n])/, '$1' + second + '$2'),
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
        func: (selfLocaleName, opXV) => icu.PluralRules.createCardinal(icu.Locale.fromString(selfLocaleName)).categoryFor(icu.PluralOperands.fromFixedDecimal(icu.Decimal.fromString(opXV))),
        // For avoiding webpacking minifying issues:
        funcName: "PluralRules.categoryFor",
        expr: (selfLocaleName, opXV) => "icu.PluralRules.createCardinal(icu.Locale.fromString(selfLocaleName)).categoryFor(icu.PluralOperands.fromFixedDecimal(icu.Decimal.fromString(opXV)))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])opXV([,\) \n])/, '$1' + opXV + '$2'),
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
        func: (s) => icu.EmojiSetData.createBasic().containsStr(s),
        // For avoiding webpacking minifying issues:
        funcName: "EmojiSetData.containsStr",
        expr: (s) => "icu.EmojiSetData.createBasic().containsStr(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
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
        func: (cp) => icu.EmojiSetData.createBasic().contains(cp),
        // For avoiding webpacking minifying issues:
        funcName: "EmojiSetData.contains",
        expr: (cp) => "icu.EmojiSetData.createBasic().contains(cp)".replace(/([\( ])cp([,\) \n])/, '$1' + cp + '$2'),
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
        func: (ch) => new icu.ScriptWithExtensions().getScriptVal(ch),
        // For avoiding webpacking minifying issues:
        funcName: "ScriptWithExtensions.getScriptVal",
        expr: (ch) => "new icu.ScriptWithExtensions().getScriptVal(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "ScriptWithExtensions.hasScript": {
        func: (ch, script) => new icu.ScriptWithExtensions().hasScript(ch, script),
        // For avoiding webpacking minifying issues:
        funcName: "ScriptWithExtensions.hasScript",
        expr: (ch, script) => "new icu.ScriptWithExtensions().hasScript(ch, script)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2').replace(/([\( ])script([,\) \n])/, '$1' + script + '$2'),
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
        func: (selfLocaleName, selfLength, selfTimePrecision, selfAlignment, timeHour, timeMinute, timeSecond, timeSubsecond) => new icu.TimeFormatter(icu.Locale.fromString(selfLocaleName), selfLength, selfTimePrecision, selfAlignment).format(new icu.Time(timeHour, timeMinute, timeSecond, timeSubsecond)),
        // For avoiding webpacking minifying issues:
        funcName: "TimeFormatter.format",
        expr: (selfLocaleName, selfLength, selfTimePrecision, selfAlignment, timeHour, timeMinute, timeSecond, timeSubsecond) => "new icu.TimeFormatter(icu.Locale.fromString(selfLocaleName), selfLength, selfTimePrecision, selfAlignment).format(new icu.Time(timeHour, timeMinute, timeSecond, timeSubsecond))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfLength([,\) \n])/, '$1' + selfLength + '$2').replace(/([\( ])selfTimePrecision([,\) \n])/, '$1' + selfTimePrecision + '$2').replace(/([\( ])selfAlignment([,\) \n])/, '$1' + selfAlignment + '$2').replace(/([\( ])timeHour([,\) \n])/, '$1' + timeHour + '$2').replace(/([\( ])timeMinute([,\) \n])/, '$1' + timeMinute + '$2').replace(/([\( ])timeSecond([,\) \n])/, '$1' + timeSecond + '$2').replace(/([\( ])timeSubsecond([,\) \n])/, '$1' + timeSubsecond + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_length",
                type: "DateTimeLength",
                typeUse: "enumerator",
                values: ["Long", "Medium", "Short"]
            },
            
            {
                name: "self_timePrecision",
                type: "TimePrecision",
                typeUse: "enumerator",
                values: ["Hour", "Minute", "MinuteOptional", "Second", "Subsecond1", "Subsecond2", "Subsecond3", "Subsecond4", "Subsecond5", "Subsecond6", "Subsecond7", "Subsecond8", "Subsecond9"]
            },
            
            {
                name: "self_alignment",
                type: "DateTimeAlignment",
                typeUse: "enumerator",
                values: ["Auto", "Column"]
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
        func: (selfId) => icu.TimeZone.createFromBcp47(selfId).isUnknown(),
        // For avoiding webpacking minifying issues:
        funcName: "TimeZone.isUnknown",
        expr: (selfId) => "icu.TimeZone.createFromBcp47(selfId).isUnknown()".replace(/([\( ])selfId([,\) \n])/, '$1' + selfId + '$2'),
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
        func: (selfLocaleName, zoneIdId, zoneOffsetOffset, zoneVariant) => icu.TimeZoneFormatter.createSpecificLong(icu.Locale.fromString(selfLocaleName)).format(new icu.TimeZoneInfo(icu.TimeZone.createFromBcp47(zoneIdId), icu.UtcOffset.fromString(zoneOffsetOffset), zoneVariant)),
        // For avoiding webpacking minifying issues:
        funcName: "TimeZoneFormatter.format",
        expr: (selfLocaleName, zoneIdId, zoneOffsetOffset, zoneVariant) => "icu.TimeZoneFormatter.createSpecificLong(icu.Locale.fromString(selfLocaleName)).format(new icu.TimeZoneInfo(icu.TimeZone.createFromBcp47(zoneIdId), icu.UtcOffset.fromString(zoneOffsetOffset), zoneVariant))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])zoneIdId([,\) \n])/, '$1' + zoneIdId + '$2').replace(/([\( ])zoneOffsetOffset([,\) \n])/, '$1' + zoneOffsetOffset + '$2').replace(/([\( ])zoneVariant([,\) \n])/, '$1' + zoneVariant + '$2'),
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
                typeUse: "enumerator",
                values: ["Standard", "Daylight"]
            }
            
        ]
    },

    "UtcOffset.seconds": {
        func: (selfOffset) => icu.UtcOffset.fromString(selfOffset).seconds,
        // For avoiding webpacking minifying issues:
        funcName: "UtcOffset.seconds",
        expr: (selfOffset) => "icu.UtcOffset.fromString(selfOffset).seconds".replace(/([\( ])selfOffset([,\) \n])/, '$1' + selfOffset + '$2'),
        parameters: [
            
            {
                name: "self_offset",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "UtcOffset.isNonNegative": {
        func: (selfOffset) => icu.UtcOffset.fromString(selfOffset).isNonNegative,
        // For avoiding webpacking minifying issues:
        funcName: "UtcOffset.isNonNegative",
        expr: (selfOffset) => "icu.UtcOffset.fromString(selfOffset).isNonNegative".replace(/([\( ])selfOffset([,\) \n])/, '$1' + selfOffset + '$2'),
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
        func: (selfOffset) => icu.UtcOffset.fromString(selfOffset).isZero,
        // For avoiding webpacking minifying issues:
        funcName: "UtcOffset.isZero",
        expr: (selfOffset) => "icu.UtcOffset.fromString(selfOffset).isZero".replace(/([\( ])selfOffset([,\) \n])/, '$1' + selfOffset + '$2'),
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
        func: (selfOffset) => icu.UtcOffset.fromString(selfOffset).hoursPart,
        // For avoiding webpacking minifying issues:
        funcName: "UtcOffset.hoursPart",
        expr: (selfOffset) => "icu.UtcOffset.fromString(selfOffset).hoursPart".replace(/([\( ])selfOffset([,\) \n])/, '$1' + selfOffset + '$2'),
        parameters: [
            
            {
                name: "self_offset",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "UtcOffset.minutesPart": {
        func: (selfOffset) => icu.UtcOffset.fromString(selfOffset).minutesPart,
        // For avoiding webpacking minifying issues:
        funcName: "UtcOffset.minutesPart",
        expr: (selfOffset) => "icu.UtcOffset.fromString(selfOffset).minutesPart".replace(/([\( ])selfOffset([,\) \n])/, '$1' + selfOffset + '$2'),
        parameters: [
            
            {
                name: "self_offset",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "UtcOffset.secondsPart": {
        func: (selfOffset) => icu.UtcOffset.fromString(selfOffset).secondsPart,
        // For avoiding webpacking minifying issues:
        funcName: "UtcOffset.secondsPart",
        expr: (selfOffset) => "icu.UtcOffset.fromString(selfOffset).secondsPart".replace(/([\( ])selfOffset([,\) \n])/, '$1' + selfOffset + '$2'),
        parameters: [
            
            {
                name: "self_offset",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "WeekInformation.firstWeekday": {
        func: (selfLocaleName) => new icu.WeekInformation(icu.Locale.fromString(selfLocaleName)).firstWeekday,
        // For avoiding webpacking minifying issues:
        funcName: "WeekInformation.firstWeekday",
        expr: (selfLocaleName) => "new icu.WeekInformation(icu.Locale.fromString(selfLocaleName)).firstWeekday".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2'),
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
        func: (selfLocaleName, day) => new icu.WeekInformation(icu.Locale.fromString(selfLocaleName)).isWeekend(day),
        // For avoiding webpacking minifying issues:
        funcName: "WeekInformation.isWeekend",
        expr: (selfLocaleName, day) => "new icu.WeekInformation(icu.Locale.fromString(selfLocaleName)).isWeekend(day)".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])day([,\) \n])/, '$1' + day + '$2'),
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
                typeUse: "enumerator",
                values: ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"]
            }
            
        ]
    },

    "ZonedDateFormatter.formatIso": {
        func: (selfLocaleName, selfFormatterLocaleName, selfFormatterLength, selfFormatterAlignment, selfFormatterYearStyle, isoDateYear, isoDateMonth, isoDateDay, zoneIdId, zoneOffsetOffset, zoneVariant) => icu.ZonedDateFormatter.createSpecificLong(icu.Locale.fromString(selfLocaleName), icu.DateFormatter.createYmd(icu.Locale.fromString(selfFormatterLocaleName), selfFormatterLength, selfFormatterAlignment, selfFormatterYearStyle)).formatIso(new icu.IsoDate(isoDateYear, isoDateMonth, isoDateDay), new icu.TimeZoneInfo(icu.TimeZone.createFromBcp47(zoneIdId), icu.UtcOffset.fromString(zoneOffsetOffset), zoneVariant)),
        // For avoiding webpacking minifying issues:
        funcName: "ZonedDateFormatter.formatIso",
        expr: (selfLocaleName, selfFormatterLocaleName, selfFormatterLength, selfFormatterAlignment, selfFormatterYearStyle, isoDateYear, isoDateMonth, isoDateDay, zoneIdId, zoneOffsetOffset, zoneVariant) => "icu.ZonedDateFormatter.createSpecificLong(icu.Locale.fromString(selfLocaleName), icu.DateFormatter.createYmd(icu.Locale.fromString(selfFormatterLocaleName), selfFormatterLength, selfFormatterAlignment, selfFormatterYearStyle)).formatIso(new icu.IsoDate(isoDateYear, isoDateMonth, isoDateDay), new icu.TimeZoneInfo(icu.TimeZone.createFromBcp47(zoneIdId), icu.UtcOffset.fromString(zoneOffsetOffset), zoneVariant))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfFormatterLocaleName([,\) \n])/, '$1' + selfFormatterLocaleName + '$2').replace(/([\( ])selfFormatterLength([,\) \n])/, '$1' + selfFormatterLength + '$2').replace(/([\( ])selfFormatterAlignment([,\) \n])/, '$1' + selfFormatterAlignment + '$2').replace(/([\( ])selfFormatterYearStyle([,\) \n])/, '$1' + selfFormatterYearStyle + '$2').replace(/([\( ])isoDateYear([,\) \n])/, '$1' + isoDateYear + '$2').replace(/([\( ])isoDateMonth([,\) \n])/, '$1' + isoDateMonth + '$2').replace(/([\( ])isoDateDay([,\) \n])/, '$1' + isoDateDay + '$2').replace(/([\( ])zoneIdId([,\) \n])/, '$1' + zoneIdId + '$2').replace(/([\( ])zoneOffsetOffset([,\) \n])/, '$1' + zoneOffsetOffset + '$2').replace(/([\( ])zoneVariant([,\) \n])/, '$1' + zoneVariant + '$2'),
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
                typeUse: "enumerator",
                values: ["Long", "Medium", "Short"]
            },
            
            {
                name: "self_formatter_alignment",
                type: "DateTimeAlignment",
                typeUse: "enumerator",
                values: ["Auto", "Column"]
            },
            
            {
                name: "self_formatter_yearStyle",
                type: "YearStyle",
                typeUse: "enumerator",
                values: ["Auto", "Full", "WithEra"]
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
                typeUse: "enumerator",
                values: ["Standard", "Daylight"]
            }
            
        ]
    },

    "ZonedDateFormatterGregorian.formatIso": {
        func: (selfLocaleName, selfFormatterLocaleName, selfFormatterLength, selfFormatterAlignment, selfFormatterYearStyle, isoDateYear, isoDateMonth, isoDateDay, zoneIdId, zoneOffsetOffset, zoneVariant) => icu.ZonedDateFormatterGregorian.createSpecificLong(icu.Locale.fromString(selfLocaleName), icu.DateFormatterGregorian.createYmd(icu.Locale.fromString(selfFormatterLocaleName), selfFormatterLength, selfFormatterAlignment, selfFormatterYearStyle)).formatIso(new icu.IsoDate(isoDateYear, isoDateMonth, isoDateDay), new icu.TimeZoneInfo(icu.TimeZone.createFromBcp47(zoneIdId), icu.UtcOffset.fromString(zoneOffsetOffset), zoneVariant)),
        // For avoiding webpacking minifying issues:
        funcName: "ZonedDateFormatterGregorian.formatIso",
        expr: (selfLocaleName, selfFormatterLocaleName, selfFormatterLength, selfFormatterAlignment, selfFormatterYearStyle, isoDateYear, isoDateMonth, isoDateDay, zoneIdId, zoneOffsetOffset, zoneVariant) => "icu.ZonedDateFormatterGregorian.createSpecificLong(icu.Locale.fromString(selfLocaleName), icu.DateFormatterGregorian.createYmd(icu.Locale.fromString(selfFormatterLocaleName), selfFormatterLength, selfFormatterAlignment, selfFormatterYearStyle)).formatIso(new icu.IsoDate(isoDateYear, isoDateMonth, isoDateDay), new icu.TimeZoneInfo(icu.TimeZone.createFromBcp47(zoneIdId), icu.UtcOffset.fromString(zoneOffsetOffset), zoneVariant))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfFormatterLocaleName([,\) \n])/, '$1' + selfFormatterLocaleName + '$2').replace(/([\( ])selfFormatterLength([,\) \n])/, '$1' + selfFormatterLength + '$2').replace(/([\( ])selfFormatterAlignment([,\) \n])/, '$1' + selfFormatterAlignment + '$2').replace(/([\( ])selfFormatterYearStyle([,\) \n])/, '$1' + selfFormatterYearStyle + '$2').replace(/([\( ])isoDateYear([,\) \n])/, '$1' + isoDateYear + '$2').replace(/([\( ])isoDateMonth([,\) \n])/, '$1' + isoDateMonth + '$2').replace(/([\( ])isoDateDay([,\) \n])/, '$1' + isoDateDay + '$2').replace(/([\( ])zoneIdId([,\) \n])/, '$1' + zoneIdId + '$2').replace(/([\( ])zoneOffsetOffset([,\) \n])/, '$1' + zoneOffsetOffset + '$2').replace(/([\( ])zoneVariant([,\) \n])/, '$1' + zoneVariant + '$2'),
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
                typeUse: "enumerator",
                values: ["Long", "Medium", "Short"]
            },
            
            {
                name: "self_formatter_alignment",
                type: "DateTimeAlignment",
                typeUse: "enumerator",
                values: ["Auto", "Column"]
            },
            
            {
                name: "self_formatter_yearStyle",
                type: "YearStyle",
                typeUse: "enumerator",
                values: ["Auto", "Full", "WithEra"]
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
                typeUse: "enumerator",
                values: ["Standard", "Daylight"]
            }
            
        ]
    },

    "ZonedDateTimeFormatter.formatIso": {
        func: (selfLocaleName, selfFormatterLocaleName, selfFormatterLength, selfFormatterTimePrecision, selfFormatterAlignment, selfFormatterYearStyle, isoDateYear, isoDateMonth, isoDateDay, timeHour, timeMinute, timeSecond, timeSubsecond, zoneIdId, zoneOffsetOffset, zoneVariant) => icu.ZonedDateTimeFormatter.createSpecificLong(icu.Locale.fromString(selfLocaleName), icu.DateTimeFormatter.createYmdt(icu.Locale.fromString(selfFormatterLocaleName), selfFormatterLength, selfFormatterTimePrecision, selfFormatterAlignment, selfFormatterYearStyle)).formatIso(new icu.IsoDate(isoDateYear, isoDateMonth, isoDateDay), new icu.Time(timeHour, timeMinute, timeSecond, timeSubsecond), new icu.TimeZoneInfo(icu.TimeZone.createFromBcp47(zoneIdId), icu.UtcOffset.fromString(zoneOffsetOffset), zoneVariant)),
        // For avoiding webpacking minifying issues:
        funcName: "ZonedDateTimeFormatter.formatIso",
        expr: (selfLocaleName, selfFormatterLocaleName, selfFormatterLength, selfFormatterTimePrecision, selfFormatterAlignment, selfFormatterYearStyle, isoDateYear, isoDateMonth, isoDateDay, timeHour, timeMinute, timeSecond, timeSubsecond, zoneIdId, zoneOffsetOffset, zoneVariant) => "icu.ZonedDateTimeFormatter.createSpecificLong(icu.Locale.fromString(selfLocaleName), icu.DateTimeFormatter.createYmdt(icu.Locale.fromString(selfFormatterLocaleName), selfFormatterLength, selfFormatterTimePrecision, selfFormatterAlignment, selfFormatterYearStyle)).formatIso(new icu.IsoDate(isoDateYear, isoDateMonth, isoDateDay), new icu.Time(timeHour, timeMinute, timeSecond, timeSubsecond), new icu.TimeZoneInfo(icu.TimeZone.createFromBcp47(zoneIdId), icu.UtcOffset.fromString(zoneOffsetOffset), zoneVariant))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfFormatterLocaleName([,\) \n])/, '$1' + selfFormatterLocaleName + '$2').replace(/([\( ])selfFormatterLength([,\) \n])/, '$1' + selfFormatterLength + '$2').replace(/([\( ])selfFormatterTimePrecision([,\) \n])/, '$1' + selfFormatterTimePrecision + '$2').replace(/([\( ])selfFormatterAlignment([,\) \n])/, '$1' + selfFormatterAlignment + '$2').replace(/([\( ])selfFormatterYearStyle([,\) \n])/, '$1' + selfFormatterYearStyle + '$2').replace(/([\( ])isoDateYear([,\) \n])/, '$1' + isoDateYear + '$2').replace(/([\( ])isoDateMonth([,\) \n])/, '$1' + isoDateMonth + '$2').replace(/([\( ])isoDateDay([,\) \n])/, '$1' + isoDateDay + '$2').replace(/([\( ])timeHour([,\) \n])/, '$1' + timeHour + '$2').replace(/([\( ])timeMinute([,\) \n])/, '$1' + timeMinute + '$2').replace(/([\( ])timeSecond([,\) \n])/, '$1' + timeSecond + '$2').replace(/([\( ])timeSubsecond([,\) \n])/, '$1' + timeSubsecond + '$2').replace(/([\( ])zoneIdId([,\) \n])/, '$1' + zoneIdId + '$2').replace(/([\( ])zoneOffsetOffset([,\) \n])/, '$1' + zoneOffsetOffset + '$2').replace(/([\( ])zoneVariant([,\) \n])/, '$1' + zoneVariant + '$2'),
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
                typeUse: "enumerator",
                values: ["Long", "Medium", "Short"]
            },
            
            {
                name: "self_formatter_timePrecision",
                type: "TimePrecision",
                typeUse: "enumerator",
                values: ["Hour", "Minute", "MinuteOptional", "Second", "Subsecond1", "Subsecond2", "Subsecond3", "Subsecond4", "Subsecond5", "Subsecond6", "Subsecond7", "Subsecond8", "Subsecond9"]
            },
            
            {
                name: "self_formatter_alignment",
                type: "DateTimeAlignment",
                typeUse: "enumerator",
                values: ["Auto", "Column"]
            },
            
            {
                name: "self_formatter_yearStyle",
                type: "YearStyle",
                typeUse: "enumerator",
                values: ["Auto", "Full", "WithEra"]
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
                typeUse: "enumerator",
                values: ["Standard", "Daylight"]
            }
            
        ]
    },

    "ZonedDateTimeFormatterGregorian.formatIso": {
        func: (selfLocaleName, selfFormatterLocaleName, selfFormatterLength, selfFormatterTimePrecision, selfFormatterAlignment, selfFormatterYearStyle, isoDateYear, isoDateMonth, isoDateDay, timeHour, timeMinute, timeSecond, timeSubsecond, zoneIdId, zoneOffsetOffset, zoneVariant) => icu.ZonedDateTimeFormatterGregorian.createSpecificLong(icu.Locale.fromString(selfLocaleName), icu.DateTimeFormatterGregorian.createYmdt(icu.Locale.fromString(selfFormatterLocaleName), selfFormatterLength, selfFormatterTimePrecision, selfFormatterAlignment, selfFormatterYearStyle)).formatIso(new icu.IsoDate(isoDateYear, isoDateMonth, isoDateDay), new icu.Time(timeHour, timeMinute, timeSecond, timeSubsecond), new icu.TimeZoneInfo(icu.TimeZone.createFromBcp47(zoneIdId), icu.UtcOffset.fromString(zoneOffsetOffset), zoneVariant)),
        // For avoiding webpacking minifying issues:
        funcName: "ZonedDateTimeFormatterGregorian.formatIso",
        expr: (selfLocaleName, selfFormatterLocaleName, selfFormatterLength, selfFormatterTimePrecision, selfFormatterAlignment, selfFormatterYearStyle, isoDateYear, isoDateMonth, isoDateDay, timeHour, timeMinute, timeSecond, timeSubsecond, zoneIdId, zoneOffsetOffset, zoneVariant) => "icu.ZonedDateTimeFormatterGregorian.createSpecificLong(icu.Locale.fromString(selfLocaleName), icu.DateTimeFormatterGregorian.createYmdt(icu.Locale.fromString(selfFormatterLocaleName), selfFormatterLength, selfFormatterTimePrecision, selfFormatterAlignment, selfFormatterYearStyle)).formatIso(new icu.IsoDate(isoDateYear, isoDateMonth, isoDateDay), new icu.Time(timeHour, timeMinute, timeSecond, timeSubsecond), new icu.TimeZoneInfo(icu.TimeZone.createFromBcp47(zoneIdId), icu.UtcOffset.fromString(zoneOffsetOffset), zoneVariant))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfFormatterLocaleName([,\) \n])/, '$1' + selfFormatterLocaleName + '$2').replace(/([\( ])selfFormatterLength([,\) \n])/, '$1' + selfFormatterLength + '$2').replace(/([\( ])selfFormatterTimePrecision([,\) \n])/, '$1' + selfFormatterTimePrecision + '$2').replace(/([\( ])selfFormatterAlignment([,\) \n])/, '$1' + selfFormatterAlignment + '$2').replace(/([\( ])selfFormatterYearStyle([,\) \n])/, '$1' + selfFormatterYearStyle + '$2').replace(/([\( ])isoDateYear([,\) \n])/, '$1' + isoDateYear + '$2').replace(/([\( ])isoDateMonth([,\) \n])/, '$1' + isoDateMonth + '$2').replace(/([\( ])isoDateDay([,\) \n])/, '$1' + isoDateDay + '$2').replace(/([\( ])timeHour([,\) \n])/, '$1' + timeHour + '$2').replace(/([\( ])timeMinute([,\) \n])/, '$1' + timeMinute + '$2').replace(/([\( ])timeSecond([,\) \n])/, '$1' + timeSecond + '$2').replace(/([\( ])timeSubsecond([,\) \n])/, '$1' + timeSubsecond + '$2').replace(/([\( ])zoneIdId([,\) \n])/, '$1' + zoneIdId + '$2').replace(/([\( ])zoneOffsetOffset([,\) \n])/, '$1' + zoneOffsetOffset + '$2').replace(/([\( ])zoneVariant([,\) \n])/, '$1' + zoneVariant + '$2'),
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
                typeUse: "enumerator",
                values: ["Long", "Medium", "Short"]
            },
            
            {
                name: "self_formatter_timePrecision",
                type: "TimePrecision",
                typeUse: "enumerator",
                values: ["Hour", "Minute", "MinuteOptional", "Second", "Subsecond1", "Subsecond2", "Subsecond3", "Subsecond4", "Subsecond5", "Subsecond6", "Subsecond7", "Subsecond8", "Subsecond9"]
            },
            
            {
                name: "self_formatter_alignment",
                type: "DateTimeAlignment",
                typeUse: "enumerator",
                values: ["Auto", "Column"]
            },
            
            {
                name: "self_formatter_yearStyle",
                type: "YearStyle",
                typeUse: "enumerator",
                values: ["Auto", "Full", "WithEra"]
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
                typeUse: "enumerator",
                values: ["Standard", "Daylight"]
            }
            
        ]
    },

    "ZonedTimeFormatter.format": {
        func: (selfLocaleName, selfLength, selfTimePrecision, selfAlignment, timeHour, timeMinute, timeSecond, timeSubsecond, zoneIdId, zoneOffsetOffset, zoneVariant) => icu.ZonedTimeFormatter.createSpecificLong(icu.Locale.fromString(selfLocaleName), selfLength, selfTimePrecision, selfAlignment).format(new icu.Time(timeHour, timeMinute, timeSecond, timeSubsecond), new icu.TimeZoneInfo(icu.TimeZone.createFromBcp47(zoneIdId), icu.UtcOffset.fromString(zoneOffsetOffset), zoneVariant)),
        // For avoiding webpacking minifying issues:
        funcName: "ZonedTimeFormatter.format",
        expr: (selfLocaleName, selfLength, selfTimePrecision, selfAlignment, timeHour, timeMinute, timeSecond, timeSubsecond, zoneIdId, zoneOffsetOffset, zoneVariant) => "icu.ZonedTimeFormatter.createSpecificLong(icu.Locale.fromString(selfLocaleName), selfLength, selfTimePrecision, selfAlignment).format(new icu.Time(timeHour, timeMinute, timeSecond, timeSubsecond), new icu.TimeZoneInfo(icu.TimeZone.createFromBcp47(zoneIdId), icu.UtcOffset.fromString(zoneOffsetOffset), zoneVariant))".replace(/([\( ])selfLocaleName([,\) \n])/, '$1' + selfLocaleName + '$2').replace(/([\( ])selfLength([,\) \n])/, '$1' + selfLength + '$2').replace(/([\( ])selfTimePrecision([,\) \n])/, '$1' + selfTimePrecision + '$2').replace(/([\( ])selfAlignment([,\) \n])/, '$1' + selfAlignment + '$2').replace(/([\( ])timeHour([,\) \n])/, '$1' + timeHour + '$2').replace(/([\( ])timeMinute([,\) \n])/, '$1' + timeMinute + '$2').replace(/([\( ])timeSecond([,\) \n])/, '$1' + timeSecond + '$2').replace(/([\( ])timeSubsecond([,\) \n])/, '$1' + timeSubsecond + '$2').replace(/([\( ])zoneIdId([,\) \n])/, '$1' + zoneIdId + '$2').replace(/([\( ])zoneOffsetOffset([,\) \n])/, '$1' + zoneOffsetOffset + '$2').replace(/([\( ])zoneVariant([,\) \n])/, '$1' + zoneVariant + '$2'),
        parameters: [
            
            {
                name: "self_locale_name",
                type: "string",
                typeUse: "string"
            },
            
            {
                name: "self_length",
                type: "DateTimeLength",
                typeUse: "enumerator",
                values: ["Long", "Medium", "Short"]
            },
            
            {
                name: "self_timePrecision",
                type: "TimePrecision",
                typeUse: "enumerator",
                values: ["Hour", "Minute", "MinuteOptional", "Second", "Subsecond1", "Subsecond2", "Subsecond3", "Subsecond4", "Subsecond5", "Subsecond6", "Subsecond7", "Subsecond8", "Subsecond9"]
            },
            
            {
                name: "self_alignment",
                type: "DateTimeAlignment",
                typeUse: "enumerator",
                values: ["Auto", "Column"]
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
                typeUse: "enumerator",
                values: ["Standard", "Daylight"]
            }
            
        ]
    },

    "CalendarKind.create": {
        func: (localeName) => icu.CalendarKind.create(icu.Locale.fromString(localeName)),
        // For avoiding webpacking minifying issues:
        funcName: "CalendarKind.create",
        expr: (localeName) => "icu.CalendarKind.create(icu.Locale.fromString(localeName))".replace(/([\( ])localeName([,\) \n])/, '$1' + localeName + '$2'),
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
        func: (digits) => icu.TimePrecision.fromSubsecondDigits(digits),
        // For avoiding webpacking minifying issues:
        funcName: "TimePrecision.fromSubsecondDigits",
        expr: (digits) => "icu.TimePrecision.fromSubsecondDigits(digits)".replace(/([\( ])digits([,\) \n])/, '$1' + digits + '$2'),
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
        func: (s) => icu.PluralCategory.getForCldrString(s),
        // For avoiding webpacking minifying issues:
        funcName: "PluralCategory.getForCldrString",
        expr: (s) => "icu.PluralCategory.getForCldrString(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
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
        func: (ch) => icu.BidiClass.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "BidiClass.forChar",
        expr: (ch) => "icu.BidiClass.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "BidiClass.tryFromStr": {
        func: (s) => icu.BidiClass.tryFromStr(s),
        // For avoiding webpacking minifying issues:
        funcName: "BidiClass.tryFromStr",
        expr: (s) => "icu.BidiClass.tryFromStr(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "CanonicalCombiningClass.forChar": {
        func: (ch) => icu.CanonicalCombiningClass.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "CanonicalCombiningClass.forChar",
        expr: (ch) => "icu.CanonicalCombiningClass.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "CanonicalCombiningClass.tryFromStr": {
        func: (s) => icu.CanonicalCombiningClass.tryFromStr(s),
        // For avoiding webpacking minifying issues:
        funcName: "CanonicalCombiningClass.tryFromStr",
        expr: (s) => "icu.CanonicalCombiningClass.tryFromStr(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "EastAsianWidth.forChar": {
        func: (ch) => icu.EastAsianWidth.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "EastAsianWidth.forChar",
        expr: (ch) => "icu.EastAsianWidth.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "EastAsianWidth.tryFromStr": {
        func: (s) => icu.EastAsianWidth.tryFromStr(s),
        // For avoiding webpacking minifying issues:
        funcName: "EastAsianWidth.tryFromStr",
        expr: (s) => "icu.EastAsianWidth.tryFromStr(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "GeneralCategory.forChar": {
        func: (ch) => icu.GeneralCategory.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "GeneralCategory.forChar",
        expr: (ch) => "icu.GeneralCategory.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "GeneralCategory.tryFromStr": {
        func: (s) => icu.GeneralCategory.tryFromStr(s),
        // For avoiding webpacking minifying issues:
        funcName: "GeneralCategory.tryFromStr",
        expr: (s) => "icu.GeneralCategory.tryFromStr(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "GraphemeClusterBreak.forChar": {
        func: (ch) => icu.GraphemeClusterBreak.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "GraphemeClusterBreak.forChar",
        expr: (ch) => "icu.GraphemeClusterBreak.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "GraphemeClusterBreak.tryFromStr": {
        func: (s) => icu.GraphemeClusterBreak.tryFromStr(s),
        // For avoiding webpacking minifying issues:
        funcName: "GraphemeClusterBreak.tryFromStr",
        expr: (s) => "icu.GraphemeClusterBreak.tryFromStr(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "HangulSyllableType.forChar": {
        func: (ch) => icu.HangulSyllableType.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "HangulSyllableType.forChar",
        expr: (ch) => "icu.HangulSyllableType.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "HangulSyllableType.tryFromStr": {
        func: (s) => icu.HangulSyllableType.tryFromStr(s),
        // For avoiding webpacking minifying issues:
        funcName: "HangulSyllableType.tryFromStr",
        expr: (s) => "icu.HangulSyllableType.tryFromStr(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "IndicConjunctBreak.forChar": {
        func: (ch) => icu.IndicConjunctBreak.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "IndicConjunctBreak.forChar",
        expr: (ch) => "icu.IndicConjunctBreak.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "IndicConjunctBreak.tryFromStr": {
        func: (s) => icu.IndicConjunctBreak.tryFromStr(s),
        // For avoiding webpacking minifying issues:
        funcName: "IndicConjunctBreak.tryFromStr",
        expr: (s) => "icu.IndicConjunctBreak.tryFromStr(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "IndicSyllabicCategory.forChar": {
        func: (ch) => icu.IndicSyllabicCategory.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "IndicSyllabicCategory.forChar",
        expr: (ch) => "icu.IndicSyllabicCategory.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "IndicSyllabicCategory.tryFromStr": {
        func: (s) => icu.IndicSyllabicCategory.tryFromStr(s),
        // For avoiding webpacking minifying issues:
        funcName: "IndicSyllabicCategory.tryFromStr",
        expr: (s) => "icu.IndicSyllabicCategory.tryFromStr(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "JoiningGroup.forChar": {
        func: (ch) => icu.JoiningGroup.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "JoiningGroup.forChar",
        expr: (ch) => "icu.JoiningGroup.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "JoiningGroup.tryFromStr": {
        func: (s) => icu.JoiningGroup.tryFromStr(s),
        // For avoiding webpacking minifying issues:
        funcName: "JoiningGroup.tryFromStr",
        expr: (s) => "icu.JoiningGroup.tryFromStr(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "JoiningType.forChar": {
        func: (ch) => icu.JoiningType.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "JoiningType.forChar",
        expr: (ch) => "icu.JoiningType.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "JoiningType.tryFromStr": {
        func: (s) => icu.JoiningType.tryFromStr(s),
        // For avoiding webpacking minifying issues:
        funcName: "JoiningType.tryFromStr",
        expr: (s) => "icu.JoiningType.tryFromStr(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "LineBreak.forChar": {
        func: (ch) => icu.LineBreak.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "LineBreak.forChar",
        expr: (ch) => "icu.LineBreak.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "LineBreak.tryFromStr": {
        func: (s) => icu.LineBreak.tryFromStr(s),
        // For avoiding webpacking minifying issues:
        funcName: "LineBreak.tryFromStr",
        expr: (s) => "icu.LineBreak.tryFromStr(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "NumericType.forChar": {
        func: (ch) => icu.NumericType.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "NumericType.forChar",
        expr: (ch) => "icu.NumericType.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "NumericType.tryFromStr": {
        func: (s) => icu.NumericType.tryFromStr(s),
        // For avoiding webpacking minifying issues:
        funcName: "NumericType.tryFromStr",
        expr: (s) => "icu.NumericType.tryFromStr(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "Script.forChar": {
        func: (ch) => icu.Script.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "Script.forChar",
        expr: (ch) => "icu.Script.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "Script.tryFromStr": {
        func: (s) => icu.Script.tryFromStr(s),
        // For avoiding webpacking minifying issues:
        funcName: "Script.tryFromStr",
        expr: (s) => "icu.Script.tryFromStr(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "SentenceBreak.forChar": {
        func: (ch) => icu.SentenceBreak.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "SentenceBreak.forChar",
        expr: (ch) => "icu.SentenceBreak.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "SentenceBreak.tryFromStr": {
        func: (s) => icu.SentenceBreak.tryFromStr(s),
        // For avoiding webpacking minifying issues:
        funcName: "SentenceBreak.tryFromStr",
        expr: (s) => "icu.SentenceBreak.tryFromStr(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "VerticalOrientation.forChar": {
        func: (ch) => icu.VerticalOrientation.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "VerticalOrientation.forChar",
        expr: (ch) => "icu.VerticalOrientation.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "VerticalOrientation.tryFromStr": {
        func: (s) => icu.VerticalOrientation.tryFromStr(s),
        // For avoiding webpacking minifying issues:
        funcName: "VerticalOrientation.tryFromStr",
        expr: (s) => "icu.VerticalOrientation.tryFromStr(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "WordBreak.forChar": {
        func: (ch) => icu.WordBreak.forChar(ch),
        // For avoiding webpacking minifying issues:
        funcName: "WordBreak.forChar",
        expr: (ch) => "icu.WordBreak.forChar(ch)".replace(/([\( ])ch([,\) \n])/, '$1' + ch + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "ch",
                type: "codepoint",
                typeUse: "codepoint"
            }
            
        ]
    },

    "WordBreak.tryFromStr": {
        func: (s) => icu.WordBreak.tryFromStr(s),
        // For avoiding webpacking minifying issues:
        funcName: "WordBreak.tryFromStr",
        expr: (s) => "icu.WordBreak.tryFromStr(s)".replace(/([\( ])s([,\) \n])/, '$1' + s + '$2'),
        display: displayOptionalEnum,
        parameters: [
            
            {
                name: "s",
                type: "string",
                typeUse: "string"
            }
            
        ]
    },

    "SegmenterWordType.isWordLike": {
        func: (self) => new icu.SegmenterWordType(self).isWordLike,
        // For avoiding webpacking minifying issues:
        funcName: "SegmenterWordType.isWordLike",
        expr: (self) => "new icu.SegmenterWordType(self).isWordLike".replace(/([\( ])self([,\) \n])/, '$1' + self + '$2'),
        display: displayBool,
        parameters: [
            
            {
                name: "self",
                type: "SegmenterWordType",
                typeUse: "enumerator",
                values: ["None", "Number", "Letter"]
            }
            
        ]
    },

    "TimeZoneVariant.fromRearguardIsdst": {
        func: (isdst) => icu.TimeZoneVariant.fromRearguardIsdst(isdst),
        // For avoiding webpacking minifying issues:
        funcName: "TimeZoneVariant.fromRearguardIsdst",
        expr: (isdst) => "icu.TimeZoneVariant.fromRearguardIsdst(isdst)".replace(/([\( ])isdst([,\) \n])/, '$1' + isdst + '$2'),
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