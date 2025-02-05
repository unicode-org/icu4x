import { DateTimeFieldSetBuilder } from "icu4x"
import { DateTimeFormatter } from "icu4x"
import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
import { Time } from "icu4x"
export function formatIso(dateTimeFormatterLocaleName, dateTimeFormatterBuilderLength, dateTimeFormatterBuilderDateFields, dateTimeFormatterBuilderTimePrecision, dateTimeFormatterBuilderZoneStyle, dateTimeFormatterBuilderAlignment, dateTimeFormatterBuilderYearStyle, dateYear, dateMonth, dateDay, timeHour, timeMinute, timeSecond, timeNanosecond) {
    
    let dateTimeFormatterLocale = Locale.fromString(dateTimeFormatterLocaleName);
    
    let dateTimeFormatterBuilder = DateTimeFieldSetBuilder.fromFields({
        length: dateTimeFormatterBuilderLength,
        dateFields: dateTimeFormatterBuilderDateFields,
        timePrecision: dateTimeFormatterBuilderTimePrecision,
        zoneStyle: dateTimeFormatterBuilderZoneStyle,
        alignment: dateTimeFormatterBuilderAlignment,
        yearStyle: dateTimeFormatterBuilderYearStyle
    });
    
    let dateTimeFormatter = DateTimeFormatter.createFromBuilder(dateTimeFormatterLocale,dateTimeFormatterBuilder);
    
    let date = new IsoDate(dateYear,dateMonth,dateDay);
    
    let time = new Time(timeHour,timeMinute,timeSecond,timeNanosecond);
    
    let out = dateTimeFormatter.formatIso(date,time);
    

    return out;
}
