import { IsoTimeZoneFormat } from "./IsoTimeZoneFormat";
import { IsoTimeZoneMinuteDisplay } from "./IsoTimeZoneMinuteDisplay";
import { IsoTimeZoneSecondDisplay } from "./IsoTimeZoneSecondDisplay";

/**
 */
export class IsoTimeZoneOptions {
  format: IsoTimeZoneFormat;
  minutes: IsoTimeZoneMinuteDisplay;
  seconds: IsoTimeZoneSecondDisplay;
}
