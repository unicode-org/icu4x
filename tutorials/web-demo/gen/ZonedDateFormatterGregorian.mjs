import { DateFormatterGregorian } from "icu4x"
import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
import { TimeZone } from "icu4x"
import { TimeZoneInfo } from "icu4x"
import { UtcOffset } from "icu4x"
import { ZonedDateFormatterGregorian } from "icu4x"
export function formatIso(selfLocaleName, selfFormatterLocaleName, selfFormatterLength, selfFormatterAlignment, selfFormatterYearStyle, isoDateYear, isoDateMonth, isoDateDay, zoneIdId, zoneOffsetOffset, zoneVariant) {
    
    let selfLocale = Locale.fromString(selfLocaleName);
    
    let selfFormatterLocale = Locale.fromString(selfFormatterLocaleName);
    
    let selfFormatter = DateFormatterGregorian.createYmd(selfFormatterLocale,selfFormatterLength,selfFormatterAlignment,selfFormatterYearStyle);
    
    let self = ZonedDateFormatterGregorian.createGenericShort(selfLocale,selfFormatter);
    
    let isoDate = new IsoDate(isoDateYear,isoDateMonth,isoDateDay);
    
    let zoneId = TimeZone.createFromBcp47(zoneIdId);
    
    let zoneOffset = UtcOffset.fromString(zoneOffsetOffset);
    
    let zone = new TimeZoneInfo(zoneId,zoneOffset,zoneVariant);
    
    let out = self.formatIso(isoDate,zone);
    

    return out;
}
