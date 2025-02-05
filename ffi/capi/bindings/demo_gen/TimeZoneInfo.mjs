import { TimeZoneInfo } from "icu4x"
export function timeZoneId(timeZoneInfoBcp47Id, timeZoneInfoOffsetSeconds, timeZoneInfoDst) {
    
    let timeZoneInfo = new TimeZoneInfo(timeZoneInfoBcp47Id,timeZoneInfoOffsetSeconds,timeZoneInfoDst);
    
    let out = timeZoneInfo.timeZoneId;
    

    return out;
}
