import { DateTimeFormatterGregorian } from "icu4x"
import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
import { Time } from "icu4x"
export function formatIso(selfLocaleName, selfLength, selfTimePrecision, selfAlignment, selfYearStyle, isoDateYear, isoDateMonth, isoDateDay, timeHour, timeMinute, timeSecond, timeSubsecond) {
    
    let selfLocale = Locale.fromString(selfLocaleName);
    
    let self = DateTimeFormatterGregorian.createYmdt(selfLocale,selfLength,selfTimePrecision,selfAlignment,selfYearStyle);
    
    let isoDate = new IsoDate(isoDateYear,isoDateMonth,isoDateDay);
    
    let time = new Time(timeHour,timeMinute,timeSecond,timeSubsecond);
    
    let out = self.formatIso(isoDate,time);
    

    return out;
}
