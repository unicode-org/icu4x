// generated by diplomat-tool
import type { IsoTimeZoneFormat } from "./IsoTimeZoneFormat"
import type { IsoTimeZoneMinuteDisplay } from "./IsoTimeZoneMinuteDisplay"
import type { IsoTimeZoneSecondDisplay } from "./IsoTimeZoneSecondDisplay"
import type { pointer, char } from "./diplomat-runtime.d.ts";

export class IsoTimeZoneOptions {
    get format() : IsoTimeZoneFormat;
    set format(value: IsoTimeZoneFormat); 
    get minutes() : IsoTimeZoneMinuteDisplay;
    set minutes(value: IsoTimeZoneMinuteDisplay); 
    get seconds() : IsoTimeZoneSecondDisplay;
    set seconds(value: IsoTimeZoneSecondDisplay); 

    

}