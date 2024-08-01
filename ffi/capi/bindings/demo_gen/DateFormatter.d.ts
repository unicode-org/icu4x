import { Calendar } from "./js/Calendar.mjs"
import { DataProvider } from "./js/DataProvider.mjs"
import { Date } from "./js/Date.mjs"
import { DateFormatter } from "./js/DateFormatter.mjs"
import { DateTime } from "./js/DateTime.mjs"
import { IsoDate } from "./js/IsoDate.mjs"
import { IsoDateTime } from "./js/IsoDateTime.mjs"
import { Locale } from "./js/Locale.mjs"
export function formatDate(name: string, dateLength: DateLength, year: number, month: number, day: number, name: string);
export function formatIsoDate(name: string, dateLength: DateLength, year: number, month: number, day: number);
export function formatDatetime(name: string, dateLength: DateLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number, name: string);
export function formatIsoDatetime(name: string, dateLength: DateLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number);
