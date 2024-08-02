import { Calendar } from "../lib/Calendar.mjs"
import { DataProvider } from "../lib/DataProvider.mjs"
import { Date } from "../lib/Date.mjs"
import { DateFormatter } from "../lib/DateFormatter.mjs"
import { DateTime } from "../lib/DateTime.mjs"
import { IsoDate } from "../lib/IsoDate.mjs"
import { IsoDateTime } from "../lib/IsoDateTime.mjs"
import { Locale } from "../lib/Locale.mjs"
export function formatDate(name: string, dateLength: DateLength, year: number, month: number, day: number, name: string);
export function formatIsoDate(name: string, dateLength: DateLength, year: number, month: number, day: number);
export function formatDatetime(name: string, dateLength: DateLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number, name: string);
export function formatIsoDatetime(name: string, dateLength: DateLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number);
