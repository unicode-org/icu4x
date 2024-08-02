import { DataProvider } from "../lib/DataProvider.mjs"
import { GregorianDateFormatter } from "../lib/GregorianDateFormatter.mjs"
import { IsoDate } from "../lib/IsoDate.mjs"
import { IsoDateTime } from "../lib/IsoDateTime.mjs"
import { Locale } from "../lib/Locale.mjs"
export function formatIsoDate(name: string, length: DateLength, year: number, month: number, day: number);
export function formatIsoDatetime(name: string, length: DateLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number);
