import { Locale } from "icu4x"
import { Time } from "icu4x"
import { TimeFormatter } from "icu4x"
import { TimeZone } from "icu4x"
import { TimeZoneInfo } from "icu4x"
import { UtcOffset } from "icu4x"
import { ZonedTimeFormatter } from "icu4x"
export function format(zonedTimeFormatterLocaleName: string, zonedTimeFormatterFormatterLocaleName: string, zonedTimeFormatterFormatterLength: DateTimeLength, zonedTimeFormatterFormatterTimePrecision: TimePrecision, zonedTimeFormatterFormatterAlignment: DateTimeAlignment, timeHour: number, timeMinute: number, timeSecond: number, timeSubsecond: number, zoneTimeZoneIdId: string, zoneOffsetOffset: string, zoneZoneVariant: TimeZoneVariant);
