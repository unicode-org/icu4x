import { Calendar } from "../lib/Calendar.mjs"
import { DataProvider } from "../lib/DataProvider.mjs"
import { DateTime } from "../lib/DateTime.mjs"
import { DateTimeFormatter } from "../lib/DateTimeFormatter.mjs"
import { IsoDateTime } from "../lib/IsoDateTime.mjs"
import { Locale } from "../lib/Locale.mjs"
export function formatDatetime(name: string, dateLength: DateLength, timeLength: TimeLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number, name: string);
export function formatIsoDatetime(name: string, dateLength: DateLength, timeLength: TimeLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number);
