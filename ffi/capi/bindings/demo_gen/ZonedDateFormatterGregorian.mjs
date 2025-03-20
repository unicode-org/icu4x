import { DateFormatterGregorian } from "icu4x"
import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
import { TimeZone } from "icu4x"
import { TimeZoneInfo } from "icu4x"
import { UtcOffset } from "icu4x"
import { ZonedDateFormatterGregorian } from "icu4x"
export function formatIso(zonedDateFormatterGregorianLocaleName, zonedDateFormatterGregorianFormatterLocaleName, zonedDateFormatterGregorianFormatterLength, zonedDateFormatterGregorianFormatterAlignment, zonedDateFormatterGregorianFormatterYearStyle, dateYear, dateMonth, dateDay, zoneTimeZoneIdId, zoneOffsetOffset, zoneZoneVariant) {
    
    let zonedDateFormatterGregorianLocale = Locale.fromString(zonedDateFormatterGregorianLocaleName);
    
    let zonedDateFormatterGregorianFormatterLocale = Locale.fromString(zonedDateFormatterGregorianFormatterLocaleName);
    
    let zonedDateFormatterGregorianFormatter = DateFormatterGregorian.createYmd(zonedDateFormatterGregorianFormatterLocale,zonedDateFormatterGregorianFormatterLength,zonedDateFormatterGregorianFormatterAlignment,zonedDateFormatterGregorianFormatterYearStyle);
    
    let zonedDateFormatterGregorian = ZonedDateFormatterGregorian.createGenericShort(zonedDateFormatterGregorianLocale,zonedDateFormatterGregorianFormatter);
    
    let date = new IsoDate(dateYear,dateMonth,dateDay);
    
    let zoneTimeZoneId = TimeZone.createFromBcp47(zoneTimeZoneIdId);
    
    let zoneOffset = UtcOffset.fromString(zoneOffsetOffset);
    
    let zone = new TimeZoneInfo(zoneTimeZoneId,zoneOffset,zoneZoneVariant);
    
    let out = zonedDateFormatterGregorian.formatIso(date,zone);
    

    return out;
}
