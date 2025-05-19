import { CaseMapper } from "icu4x"
import { Locale } from "icu4x"
import { TitlecaseOptions } from "icu4x"
export function lowercase(s: string, localeName: string);
export function uppercase(s: string, localeName: string);
export function titlecaseSegmentWithOnlyCaseData(s: string, localeName: string, optionsLeadingAdjustment: LeadingAdjustment, optionsTrailingCase: TrailingCase);
export function fold(s: string);
export function foldTurkic(s: string);
export function simpleLowercase(ch: codepoint);
export function simpleUppercase(ch: codepoint);
export function simpleTitlecase(ch: codepoint);
export function simpleFold(ch: codepoint);
export function simpleFoldTurkic(ch: codepoint);
