import { Locale } from "icu4x"
import { Time } from "icu4x"
import { NoCalendarFormatter } from "icu4x"
export function format(timeFormatterLocaleName, timeFormatterLength, valueHour, valueMinute, valueSecond, valueSubsecond) {
    
    let timeFormatterLocale = Locale.fromString(timeFormatterLocaleName);
    
    let timeFormatter = NoCalendarFormatter.createWithLength(timeFormatterLocale,timeFormatterLength);
    
    let value = new Time(valueHour,valueMinute,valueSecond,valueSubsecond);
    
    let out = timeFormatter.format(value);
    

    return out;
}
