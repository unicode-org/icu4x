import 'package:icu/icu.dart';
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
    Rune a = 'a'.runes.first;
    Rune emoji = '💡'.runes.first;

    final emojiSet = CodePointSetData.emoji();
    expect(emojiSet.contains(a), false);
    expect(emojiSet.contains(emoji), true);

    Rune upperA = CaseMapper().simpleUppercase(a);
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
    final zonedDateTimeIso = ZonedIsoDateTime.fromString(
      '2025-01-15T14:32:12.34+01[Europe/Zurich]',
      IanaParser(),
      VariantOffsetsCalculator(),
    );

    final zonedDateTimeBuddhist = ZonedDateTime.fromString(
      '2026-01-15T05:32:12.34+07[Asia/Bangkok][u-ca=buddhist]',
      Calendar.forKind(CalendarKind.buddhist),
      IanaParser(),
      VariantOffsetsCalculator(),
    );

    var locale = Locale.fromString('de-u-ca-islamic');

    ///// DateFormatter /////

    expect(DateFormatter.md(locale).formatIso(zonedDateTimeIso.date), '14.07.');

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
      'Mi., 14. Raj. 1446 AH, 14:32:12',
    );

    expect(
      DateTimeFormatter.ymdet(
        locale,
        length: DateTimeLength.long,
        timePrecision: TimePrecision.minute,
      ).formatIso(zonedDateTimeIso.date, zonedDateTimeIso.time),
      'Mittwoch, 14. Radschab 1446 AH, 14:32',
    );

    expect(
      () => DateTimeFormatter.ymdet(locale).formatSameCalendar(
        zonedDateTimeBuddhist.date,
        zonedDateTimeBuddhist.time,
      ),
      throwsA(
        DateTimeMismatchedCalendarError(
          thisKind: CalendarKind.hijriObservationalMecca,
          dateKind: CalendarKind.buddhist,
        ),
      ),
    );

    expect(
      DateTimeFormatter.ymdet(locale).formatSameCalendar(
        zonedDateTimeBuddhist.date.toCalendar(
          Calendar.forKind(CalendarKind.hijriObservationalMecca),
        ),
        zonedDateTimeBuddhist.time,
      ),
      'Do., 25. Raj. 1447 AH, 05:32:12',
    );

    expect(
      DateTimeFormatter.ymdet(locale).formatIso(
        zonedDateTimeBuddhist.date.toIso(),
        zonedDateTimeBuddhist.time,
      ),
      'Do., 25. Raj. 1447 AH, 05:32:12',
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
      '14.07. Mitteleuropäische Zeit',
    );

    expect(
      () => ZonedDateFormatter.genericLong(locale, DateFormatter.ym(locale)),
      throwsA(DateTimeFormatterLoadError.invalidDateFields),
    );

    expect(
      () => ZonedDateFormatter.genericLong(
        locale,
        DateFormatter.ymd(locale),
      ).formatIso(zonedDateTimeIso.date, TimeZoneInfo.utc()),
      throwsA(DateTimeWriteError.missingInputField),
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
      'Mi., 14. Raj. 1446 AH, 14:32:12 Mitteleuropäische Zeit',
    );

    expect(
      () => ZonedDateTimeFormatter.genericLong(
        locale,
        DateTimeFormatter.ymdet(locale),
      ).formatIso(
        zonedDateTimeIso.date,
        zonedDateTimeIso.time,
        TimeZoneInfo.utc(),
      ),
      throwsA(DateTimeWriteError.missingInputField),
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
      '14. Radschab 1446 AH, 14:32:12 MEZ',
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
      '14.07.46 AH, 14:32:12 MEZ',
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
