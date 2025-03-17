import { Calendar } from "icu4x"
import { Date } from "icu4x"
import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
import { NeoDateFormatter } from "icu4x"
export function formatIso(neoDateFormatterLocaleName: string, neoDateFormatterLength: DateTimeLength, neoDateFormatterAlignment: DateTimeAlignment, neoDateFormatterYearStyle: YearStyle, dateYear: number, dateMonth: number, dateDay: number);
export function formatSameCalendar(neoDateFormatterLocaleName: string, neoDateFormatterLength: DateTimeLength, neoDateFormatterAlignment: DateTimeAlignment, neoDateFormatterYearStyle: YearStyle, dateYear: number, dateMonth: number, dateDay: number, dateCalendarLocaleName: string);
