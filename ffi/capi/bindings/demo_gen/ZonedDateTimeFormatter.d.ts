import { Calendar } from "../lib/Calendar.mjs"
import { CustomTimeZone } from "../lib/CustomTimeZone.mjs"
import { DataProvider } from "../lib/DataProvider.mjs"
import { DateTime } from "../lib/DateTime.mjs"
import { IsoDateTime } from "../lib/IsoDateTime.mjs"
import { Locale } from "../lib/Locale.mjs"
import { ZonedDateTimeFormatter } from "../lib/ZonedDateTimeFormatter.mjs"
export function formatDatetimeWithCustomTimeZone(name: string, dateLength: DateLength, timeLength: TimeLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number, name: string, s: string);
export function formatIsoDatetimeWithCustomTimeZone(name: string, dateLength: DateLength, timeLength: TimeLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number, s: string);
