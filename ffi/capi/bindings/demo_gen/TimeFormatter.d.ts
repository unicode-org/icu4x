import { Calendar } from "icu4x"
import { DataProvider } from "icu4x"
import { DateTime } from "icu4x"
import { IsoDateTime } from "icu4x"
import { Locale } from "icu4x"
import { Time } from "icu4x"
import { TimeFormatter } from "icu4x"
export function formatTime(name: string, length: TimeLength, hour: number, minute: number, second: number, nanosecond: number);
export function formatDatetime(name: string, length: TimeLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number, name: string);
export function formatIsoDatetime(name: string, length: TimeLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number);
