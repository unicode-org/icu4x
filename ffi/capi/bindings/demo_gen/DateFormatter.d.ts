import { Calendar } from "icu4x"
import { Date } from "icu4x"
import { DateFormatter } from "icu4x"
import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
import { Time } from "icu4x"
export function formatDate(name: string, length: DateTimeLength, year: number, month: number, day: number, name_1: string);
export function formatIsoDate(name: string, length: DateTimeLength, year: number, month: number, day: number);
export function formatDatetime(name: string, length: DateTimeLength, year: number, month: number, day: number, name_1: string, hour: number, minute: number, second: number, nanosecond: number);
export function formatIsoDatetime(name: string, length: DateTimeLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number);
