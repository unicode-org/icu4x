import { Calendar } from "icu4x"
import { DataProvider } from "icu4x"
import { DateTime } from "icu4x"
import { IsoDateTime } from "icu4x"
import { Locale } from "icu4x"
import { MetazoneCalculator } from "icu4x"
import { TimeZone } from "icu4x"
import { ZoneOffsetCalculator } from "icu4x"
import { ZonedDateTimeFormatter } from "icu4x"
export function formatDatetimeWithCustomTimeZone(name: string, length: DateTimeLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number, name: string, s: string);
export function formatIsoDatetimeWithCustomTimeZone(name: string, length: DateTimeLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number, s: string);
