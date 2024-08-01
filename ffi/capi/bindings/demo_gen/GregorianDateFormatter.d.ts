import { DataProvider } from "./js/DataProvider.mjs"
import { GregorianDateFormatter } from "./js/GregorianDateFormatter.mjs"
import { IsoDate } from "./js/IsoDate.mjs"
import { IsoDateTime } from "./js/IsoDateTime.mjs"
import { Locale } from "./js/Locale.mjs"
export function formatIsoDate(name: string, length: DateLength, year: number, month: number, day: number);
export function formatIsoDatetime(name: string, length: DateLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number);
