import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
import { NeoDateTimeFormatter } from "icu4x"
import { Time } from "icu4x"
export function formatIso(name: string, length: NeoDateTimeLength, timePrecision: TimePrecision, alignment: DateTimeAlignment, year: number, month: number, day: number, hour: number, minute: number, second: number, nanosecond: number);
