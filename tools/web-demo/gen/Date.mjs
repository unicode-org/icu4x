import { Calendar } from "icu4x"
import { Date } from "icu4x"
export function rataDie(selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) {
    
    let selfCalendar = new Calendar(selfCalendarKind);
    
    let self = Date.fromIsoInCalendar(selfIsoYear,selfIsoMonth,selfIsoDay,selfCalendar);
    
    let out = self.rataDie;
    

    return out;
}
export function dayOfYear(selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) {
    
    let selfCalendar = new Calendar(selfCalendarKind);
    
    let self = Date.fromIsoInCalendar(selfIsoYear,selfIsoMonth,selfIsoDay,selfCalendar);
    
    let out = self.dayOfYear;
    

    return out;
}
export function dayOfMonth(selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) {
    
    let selfCalendar = new Calendar(selfCalendarKind);
    
    let self = Date.fromIsoInCalendar(selfIsoYear,selfIsoMonth,selfIsoDay,selfCalendar);
    
    let out = self.dayOfMonth;
    

    return out;
}
export function dayOfWeek(selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) {
    
    let selfCalendar = new Calendar(selfCalendarKind);
    
    let self = Date.fromIsoInCalendar(selfIsoYear,selfIsoMonth,selfIsoDay,selfCalendar);
    
    let out = self.dayOfWeek;
    
    out = out?.value || 'None';;
    

    return out;
}
export function ordinalMonth(selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) {
    
    let selfCalendar = new Calendar(selfCalendarKind);
    
    let self = Date.fromIsoInCalendar(selfIsoYear,selfIsoMonth,selfIsoDay,selfCalendar);
    
    let out = self.ordinalMonth;
    

    return out;
}
export function monthCode(selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) {
    
    let selfCalendar = new Calendar(selfCalendarKind);
    
    let self = Date.fromIsoInCalendar(selfIsoYear,selfIsoMonth,selfIsoDay,selfCalendar);
    
    let out = self.monthCode;
    

    return out;
}
export function monthNumber(selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) {
    
    let selfCalendar = new Calendar(selfCalendarKind);
    
    let self = Date.fromIsoInCalendar(selfIsoYear,selfIsoMonth,selfIsoDay,selfCalendar);
    
    let out = self.monthNumber;
    

    return out;
}
export function monthIsLeap(selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) {
    
    let selfCalendar = new Calendar(selfCalendarKind);
    
    let self = Date.fromIsoInCalendar(selfIsoYear,selfIsoMonth,selfIsoDay,selfCalendar);
    
    let out = self.monthIsLeap;
    
    out = out ? 'true' : 'false';;
    

    return out;
}
export function eraYearOrRelatedIso(selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) {
    
    let selfCalendar = new Calendar(selfCalendarKind);
    
    let self = Date.fromIsoInCalendar(selfIsoYear,selfIsoMonth,selfIsoDay,selfCalendar);
    
    let out = self.eraYearOrRelatedIso;
    

    return out;
}
export function extendedYear(selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) {
    
    let selfCalendar = new Calendar(selfCalendarKind);
    
    let self = Date.fromIsoInCalendar(selfIsoYear,selfIsoMonth,selfIsoDay,selfCalendar);
    
    let out = self.extendedYear;
    

    return out;
}
export function era(selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) {
    
    let selfCalendar = new Calendar(selfCalendarKind);
    
    let self = Date.fromIsoInCalendar(selfIsoYear,selfIsoMonth,selfIsoDay,selfCalendar);
    
    let out = self.era;
    

    return out;
}
export function monthsInYear(selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) {
    
    let selfCalendar = new Calendar(selfCalendarKind);
    
    let self = Date.fromIsoInCalendar(selfIsoYear,selfIsoMonth,selfIsoDay,selfCalendar);
    
    let out = self.monthsInYear;
    

    return out;
}
export function daysInMonth(selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) {
    
    let selfCalendar = new Calendar(selfCalendarKind);
    
    let self = Date.fromIsoInCalendar(selfIsoYear,selfIsoMonth,selfIsoDay,selfCalendar);
    
    let out = self.daysInMonth;
    

    return out;
}
export function daysInYear(selfIsoYear, selfIsoMonth, selfIsoDay, selfCalendarKind) {
    
    let selfCalendar = new Calendar(selfCalendarKind);
    
    let self = Date.fromIsoInCalendar(selfIsoYear,selfIsoMonth,selfIsoDay,selfCalendar);
    
    let out = self.daysInYear;
    

    return out;
}
