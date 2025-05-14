import { DateTimeFormatter } from "icu4x"
import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
import { Time } from "icu4x"
export function formatIso(selfLocaleName: string, selfLength: DateTimeLength, selfTimePrecision: TimePrecision, selfAlignment: DateTimeAlignment, selfYearStyle: YearStyle, isoDateYear: number, isoDateMonth: number, isoDateDay: number, timeHour: number, timeMinute: number, timeSecond: number, timeSubsecond: number);
