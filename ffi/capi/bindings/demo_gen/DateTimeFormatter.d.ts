import { Calendar } from "icu4x"
import { DataProvider } from "icu4x"
import { DateTime } from "icu4x"
import { DateTimeFormatter } from "icu4x"
import { IsoDateTime } from "icu4x"
import { Locale } from "icu4x"
export function formatDatetime(name: string, dateLength: DateLength, timeLength: TimeLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number, name: string);
export function formatIsoDatetime(name: string, dateLength: DateLength, timeLength: TimeLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number);
