import { CaseMapper } from "./js/CaseMapper.mjs"
import { DataProvider } from "./js/DataProvider.mjs"
import { Locale } from "./js/Locale.mjs"
import { TitlecaseOptions } from "./js/TitlecaseOptions.mjs"
export function lowercase(s: string, name: string);
export function uppercase(s: string, name: string);
export function titlecaseSegmentWithOnlyCaseData(s: string, name: string, leading_adjustment: LeadingAdjustment, trailing_case: TrailingCase);
export function fold(s: string);
export function foldTurkic(s: string);
