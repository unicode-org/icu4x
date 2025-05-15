import { DateFormatterGregorian } from "icu4x"
import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
export function formatIso(selfLocaleName, selfLength, selfAlignment, selfYearStyle, isoDateYear, isoDateMonth, isoDateDay) {
    
    let selfLocale = Locale.fromString(selfLocaleName);
    
    let self = DateFormatterGregorian.createYmd(selfLocale,selfLength,selfAlignment,selfYearStyle);
    
    let isoDate = new IsoDate(isoDateYear,isoDateMonth,isoDateDay);
    
    let out = self.formatIso(isoDate);
    

    return out;
}
