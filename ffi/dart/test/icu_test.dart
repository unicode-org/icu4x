import 'package:icu4x/icu4x.dart';
import 'package:test/test.dart';

void main() {
  Logger.initSimpleLogger();

  test('Decimal.toString', () {
    final x = Decimal.fromDoubleWithLowerMagnitude(1.49403, -7);
    expect(x.toString(), '1.4940300');
  });

  test('LocaleFallbacker', () {
    final iterator = LocaleFallbacker()
        .forConfig(
          LocaleFallbackConfig(priority: LocaleFallbackPriority.region),
        )
        .fallbackForLocale(Locale.fromString('de-CH-u-ca-japanese'));
    expect(iterator.moveNext(), true);
    expect(iterator.current, Locale.fromString('de-CH'));
    expect(iterator.moveNext(), true);
    expect(iterator.current, Locale.fromString('und-CH'));
    expect(iterator.moveNext(), false);
  });

  test('Properties', () {
    final a = 'a'.runes.first;
    final emoji = '💡'.runes.first;

    final emojiSet = CodePointSetData.emoji();
    expect(emojiSet.contains(a), false);
    expect(emojiSet.contains(emoji), true);

    final upperA = CaseMapper().simpleUppercase(a);
    expect(String.fromCharCode(upperA), 'A');
  });

  test('ListFormatter', () {
    final formatter = ListFormatter.andWithLength(
      Locale.fromString('es'),
      ListLength.wide,
    );
    final list = ['España', 'Francia', 'Suiza', 'Italia'];

    expect(formatter.format(list), 'España, Francia, Suiza e Italia');
  });

  test('Locale ordering', () {
    expect(
      [
        Locale.fromString('en-GB'),
        Locale.fromString('de'),
        Locale.fromString('az'),
      ]..sort(),
      [
        Locale.fromString('az'),
        Locale.fromString('de'),
        Locale.fromString('en-GB'),
      ],
    );
  });

  test('Locale extensions', () {
    final locale = Locale.fromString('en-GB');
    expect(locale.getUnicodeExtension('ca'), null);
    expect(locale.setUnicodeExtension('ca', 'gregory'), true);
    expect(locale.setUnicodeExtension('ca', 'gregorian'), false);
    expect(locale.setUnicodeExtension('calendar', 'gregory'), false);
    expect(locale.getUnicodeExtension('ca'), 'gregory');
    expect(locale.toString(), 'en-GB-u-ca-gregory');
    expect(locale.setUnicodeExtension('ka', 'gregory'), true);
    expect(locale.toString(), 'en-GB-u-ca-gregory-ka-gregory');
  });

  test('Time zones', () {
    final iter = IanaParserExtended().iterAll();
    iter.moveNext();
    expect(iter.current.canonical, 'Africa/Abidjan');
  });

  test('Dates', () {
    final date = IsoDate(2022, 8, 26);
    expect(date.weekOfYear().weekNumber, 34);

    final weekInfo = WeekInformation(Locale.fromString('de'));
    expect(weekInfo.firstWeekday, Weekday.monday);
    expect(weekInfo.isWeekend(Weekday.sunday), isTrue);

    final weekend = weekInfo.weekend;
    expect(weekend.moveNext(), true);
    expect(weekend.current, Weekday.saturday);
    expect(weekend.moveNext(), true);
    expect(weekend.current, Weekday.sunday);
    expect(weekend.moveNext(), false);
  });

  test('DateTime formatting', () {
    final zonedDateTimeIso = ZonedIsoDateTime.strictFromString(
      '2025-01-15T14:32:12.34+01[Europe/Zurich]',
      IanaParser(),
    );

    final zonedDateTimeBuddhist = ZonedDateTime.strictFromString(
      '2026-01-15T05:32:12.34+07[Asia/Bangkok][u-ca=buddhist]',
      Calendar(CalendarKind.buddhist),
      IanaParser(),
    );

    final locale = Locale.fromString('de-u-ca-islamic-umalqura');

    ///// DateFormatter /////

    expect(DateFormatter.md(locale).formatIso(zonedDateTimeIso.date), '15.07.');

    ///// TimeFormatter /////

    expect(
      TimeFormatter(
        locale,
        timePrecision: TimePrecision.minuteOptional,
      ).format(zonedDateTimeIso.time),
      '14:32',
    );

    ///// DateTimeFormatter /////

    expect(
      DateTimeFormatter.ymdet(
        locale,
      ).formatIso(zonedDateTimeIso.date, zonedDateTimeIso.time),
      'Mi., 15.07.1446 AH, 14:32:12',
    );

    expect(
      DateTimeFormatter.ymdet(
        locale,
        length: DateTimeLength.long,
        timePrecision: TimePrecision.minute,
      ).formatIso(zonedDateTimeIso.date, zonedDateTimeIso.time),
      'Mittwoch, 15. Radschab 1446 AH, 14:32',
    );

    expect(
      () => DateTimeFormatter.ymdet(locale).formatSameCalendar(
        zonedDateTimeBuddhist.date,
        zonedDateTimeBuddhist.time,
      ),
      throwsA(
        DateTimeMismatchedCalendarError(
          thisKind: CalendarKind.hijriUmmAlQura,
          dateKind: CalendarKind.buddhist,
        ),
      ),
    );

    expect(
      DateTimeFormatter.ymdet(locale).formatSameCalendar(
        zonedDateTimeBuddhist.date.toCalendar(
          Calendar(CalendarKind.hijriUmmAlQura),
        ),
        zonedDateTimeBuddhist.time,
      ),
      'Do., 26.07.1447 AH, 05:32:12',
    );

    expect(
      DateTimeFormatter.ymdet(locale).formatIso(
        zonedDateTimeBuddhist.date.toIso(),
        zonedDateTimeBuddhist.time,
      ),
      'Do., 26.07.1447 AH, 05:32:12',
    );

    ///// DateTimeFormatterGregorian /////

    expect(
      DateTimeFormatterGregorian.ymdet(
        locale,
      ).formatIso(zonedDateTimeIso.date, zonedDateTimeIso.time),
      'Mi., 15.01.2025, 14:32:12',
    );

    expect(
      DateTimeFormatterGregorian.ymdet(
        locale,
        length: DateTimeLength.long,
        timePrecision: TimePrecision.minute,
      ).formatIso(zonedDateTimeIso.date, zonedDateTimeIso.time),
      'Mittwoch, 15. Januar 2025, 14:32',
    );

    ///// TimeZoneFormatter /////

    expect(
      TimeZoneFormatter.genericLong(locale).format(zonedDateTimeIso.zone),
      'Mitteleuropäische Zeit',
    );

    ///// ZonedDateFormatter /////

    expect(
      ZonedDateFormatter.genericLong(
        locale,
        DateFormatter.md(locale),
      ).formatIso(zonedDateTimeIso.date, zonedDateTimeIso.zone),
      '15.07. Mitteleuropäische Zeit',
    );

    expect(
      () => ZonedDateFormatter.genericLong(locale, DateFormatter.ym(locale)),
      throwsA(DateTimeFormatterLoadError.invalidDateFields),
    );

    expect(
      ZonedDateFormatter.genericLong(
        locale,
        DateFormatter.ymd(locale),
      ).formatIso(zonedDateTimeIso.date, TimeZoneInfo.utc()),
      // Note: this fills in noon for the ZoneNameTimestamp
      '15.07.1446 AH Koordinierte Weltzeit',
    );

    ///// ZonedTimeFormatter /////

    expect(
      ZonedTimeFormatter.specificLong(
        locale,
        timePrecision: TimePrecision.minuteOptional,
      ).format(zonedDateTimeIso.time, zonedDateTimeIso.zone),
      '14:32 Mitteleuropäische Normalzeit',
    );

    ///// ZonedDateTimeFormatter /////

    expect(
      ZonedDateTimeFormatter.genericLong(
        locale,
        DateTimeFormatter.ymdet(locale),
      ).formatIso(
        zonedDateTimeIso.date,
        zonedDateTimeIso.time,
        zonedDateTimeIso.zone,
      ),
      'Mi., 15.07.1446 AH, 14:32:12 Mitteleuropäische Zeit',
    );

    expect(
      ZonedDateTimeFormatter.specificLong(
        locale,
        DateTimeFormatter.ymdet(locale),
      ).formatIso(
        zonedDateTimeIso.date,
        zonedDateTimeIso.time,
        TimeZoneInfo.utc(),
      ),
      'Mi., 15.07.1446 AH, 14:32:12 Koordinierte Weltzeit',
    );

    expect(
      ZonedDateTimeFormatter.specificShort(
        locale,
        DateTimeFormatter.ymdt(locale, length: DateTimeLength.long),
      ).formatIso(
        zonedDateTimeIso.date,
        zonedDateTimeIso.time,
        zonedDateTimeIso.zone,
      ),
      '15. Radschab 1446 AH, 14:32:12 MEZ',
    );

    expect(
      ZonedDateTimeFormatter.specificShort(
        locale,
        DateTimeFormatter.ymdt(locale, length: DateTimeLength.short),
      ).formatIso(
        zonedDateTimeIso.date,
        zonedDateTimeIso.time,
        zonedDateTimeIso.zone,
      ),
      '15.07.46 AH, 14:32:12 MEZ',
    );

    expect(
      ZonedDateTimeFormatter.genericLong(
        locale,
        DateTimeFormatter.mdt(locale),
      ).formatIso(
        zonedDateTimeIso.date,
        zonedDateTimeIso.time,
        TimeZoneInfo(
          TimeZone.fromBcp47('uslax'),
          offset: UtcOffset.fromSeconds(-420),
        ),
      ),
      '15.07., 14:32:12 GMT-00:07',
    );

    final customZDT = ZonedIsoDateTime.fromEpochMillisecondsAndUtcOffset(
      1746140981731, // 2025-05-01T23:09:41.731Z
      UtcOffset.fromString('+02'),
    );
    expect(
      ZonedDateTimeFormatter.genericShort(
        Locale.fromString('en'),
        DateTimeFormatter.ymdt(
          Locale.fromString('en'),
          timePrecision: TimePrecision.subsecond3,
        ),
      ).formatIso(customZDT.date, customZDT.time, customZDT.zone),
      'May 2, 2025, 1:09:41.731 AM GMT+2',
    );

    ///// ZonedDateTimeFormatterGregorian /////

    expect(
      ZonedDateTimeFormatterGregorian.genericLong(
        locale,
        DateTimeFormatterGregorian.ymdet(locale),
      ).formatIso(
        zonedDateTimeIso.date,
        zonedDateTimeIso.time,
        zonedDateTimeIso.zone,
      ),
      'Mi., 15.01.2025, 14:32:12 Mitteleuropäische Zeit',
    );
  });
}
