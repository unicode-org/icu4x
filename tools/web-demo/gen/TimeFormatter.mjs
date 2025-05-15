import { Locale } from "icu4x"
import { Time } from "icu4x"
import { TimeFormatter } from "icu4x"
export function format(selfLocaleName, selfLength, selfTimePrecision, selfAlignment, timeHour, timeMinute, timeSecond, timeSubsecond) {
    
    let selfLocale = Locale.fromString(selfLocaleName);
    
    let self = new TimeFormatter(selfLocale,selfLength,selfTimePrecision,selfAlignment);
    
    let time = new Time(timeHour,timeMinute,timeSecond,timeSubsecond);
    
    let out = self.format(time);
    

    return out;
}
