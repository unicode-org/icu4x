import { CaseMapper } from "./CaseMapper.mjs"
import { DataProvider } from "./DataProvider.mjs"
import { Locale } from "./Locale.mjs"
import { TitlecaseOptions } from "./js/TitlecaseOptions.mjs"
export function lowercase(s: string, name: string);
export function uppercase(s: string, name: string);
export function titlecaseSegmentWithOnlyCaseData(s: string, name: string, leading_adjustment: LeadingAdjustment, trailing_case: TrailingCase);
export function fold(s: string);
export function foldTurkic(s: string);
