import { Calendar } from "./js/Calendar.mjs"
import { DataProvider } from "./js/DataProvider.mjs"
import { DateTime } from "./js/DateTime.mjs"
import { IsoDateTime } from "./js/IsoDateTime.mjs"
import { Locale } from "./js/Locale.mjs"
import { Time } from "./js/Time.mjs"
import { TimeFormatter } from "./js/TimeFormatter.mjs"
export function formatTime(name: string, length: TimeLength, hour: number, minute: number, second: number, nanosecond: number);
export function formatDatetime(name: string, length: TimeLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number, name: string);
export function formatIsoDatetime(name: string, length: TimeLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number);
