import { CustomTimeZone } from "../lib/CustomTimeZone.mjs"
import { DataProvider } from "../lib/DataProvider.mjs"
import { GregorianZonedDateTimeFormatter } from "../lib/GregorianZonedDateTimeFormatter.mjs"
import { IsoDateTime } from "../lib/IsoDateTime.mjs"
import { Locale } from "../lib/Locale.mjs"
export function formatIsoDatetimeWithCustomTimeZone(name: string, dateLength: DateLength, timeLength: TimeLength, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number, s: string);
