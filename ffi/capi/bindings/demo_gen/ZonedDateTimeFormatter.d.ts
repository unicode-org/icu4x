import { Calendar } from "./js/Calendar.mjs"
import { CustomTimeZone } from "./js/CustomTimeZone.mjs"
import { DataProvider } from "./js/DataProvider.mjs"
import { DateTime } from "./js/DateTime.mjs"
import { IsoDateTime } from "./js/IsoDateTime.mjs"
import { Locale } from "./js/Locale.mjs"
import { ZonedDateTimeFormatter } from "./js/ZonedDateTimeFormatter.mjs"
export function formatDatetimeWithCustomTimeZone(name: string, dateLength: DateLength, timeLength: TimeLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number, name: string, s: string);
export function formatIsoDatetimeWithCustomTimeZone(name: string, dateLength: DateLength, timeLength: TimeLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number, s: string);
