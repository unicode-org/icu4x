import { GregorianDateTimeFormatter } from "icu4x"
import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
import { Time } from "icu4x"
export function formatIso(gregorianDateTimeFormatterLocaleName: string, gregorianDateTimeFormatterLength: DateTimeLength, dateYear: number, dateMonth: number, dateDay: number, timeHour: number, timeMinute: number, timeSecond: number, timeNanosecond: number);
