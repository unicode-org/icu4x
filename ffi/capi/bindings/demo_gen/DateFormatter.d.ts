import { Calendar } from "./Calendar.mjs"
import { DataProvider } from "./DataProvider.mjs"
import { Date } from "./Date.mjs"
import { DateFormatter } from "./DateFormatter.mjs"
import { DateTime } from "./DateTime.mjs"
import { IsoDate } from "./IsoDate.mjs"
import { IsoDateTime } from "./IsoDateTime.mjs"
import { Locale } from "./Locale.mjs"
export function formatDate(name: string, dateLength: DateLength, year: number, month: number, day: number, name: string);
export function formatIsoDate(name: string, dateLength: DateLength, year: number, month: number, day: number);
export function formatDatetime(name: string, dateLength: DateLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number, name: string);
export function formatIsoDatetime(name: string, dateLength: DateLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number);
