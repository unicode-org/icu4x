import { Calendar } from "../lib/Calendar.mjs"
import { DataProvider } from "../lib/DataProvider.mjs"
import { DateTime } from "../lib/DateTime.mjs"
import { IsoDateTime } from "../lib/IsoDateTime.mjs"
import { Locale } from "../lib/Locale.mjs"
import { Time } from "../lib/Time.mjs"
import { TimeFormatter } from "../lib/TimeFormatter.mjs"
export function formatTime(name: string, length: TimeLength, hour: number, minute: number, second: number, nanosecond: number);
export function formatDatetime(name: string, length: TimeLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number, name: string);
export function formatIsoDatetime(name: string, length: TimeLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number);
