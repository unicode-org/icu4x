import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
import { NeoDateFormatterGregorian } from "icu4x"
export function formatIso(neoDateFormatterGregorianLocaleName, neoDateFormatterGregorianLength, neoDateFormatterGregorianAlignment, neoDateFormatterGregorianYearStyle, dateYear, dateMonth, dateDay) {
    
    let neoDateFormatterGregorianLocale = Locale.fromString(neoDateFormatterGregorianLocaleName);
    
    let neoDateFormatterGregorian = NeoDateFormatterGregorian.createYmd(neoDateFormatterGregorianLocale,neoDateFormatterGregorianLength,neoDateFormatterGregorianAlignment,neoDateFormatterGregorianYearStyle);
    
    let date = new IsoDate(dateYear,dateMonth,dateDay);
    
    let out = neoDateFormatterGregorian.formatIso(date);
    

    return out;
}
