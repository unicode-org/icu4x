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

  test('DateTime formatting', () {
    final zonedDateTimeIso = ZonedIsoDateTime.fromString(
      '2025-01-15T14:32:12.34+01[Europe/Zurich]',
      IanaParser(),
      UtcOffsetCalculator(),
    );

    final zonedDateTimeBuddhist = ZonedDateTime.fromString(
      '2026-01-15T05:32:12.34+07[Asia/Bangkok][u-ca=buddhist]',
      Calendar.forKind(AnyCalendarKind.buddhist),
      IanaParser(),
      UtcOffsetCalculator(),
    );

    var locale = Locale.fromString('de-u-ca-islamic');

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
          thisKind: AnyCalendarKind.hijriObservational,
          dateKind: AnyCalendarKind.buddhist,
        ),
      ),
    );

    expect(
      DateTimeFormatter.ymdet(locale).formatSameCalendar(
        zonedDateTimeBuddhist.date.toCalendar(
          Calendar.forKind(AnyCalendarKind.hijriObservational),
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

    expect(
      DateTimeFormatter.ymdet(locale)
          .withZoneGenericLong(locale)
          .formatIso(
            zonedDateTimeIso.date,
            zonedDateTimeIso.time,
            zonedDateTimeIso.zone,
          ),
      'Mi., 14. Raj. 1446 AH, 14:32:12 Mitteleuropäische Zeit',
    );

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

    expect(
      ZonedDateTimeFormatter.withLength(locale, DateTimeLength.long).formatIso(
        zonedDateTimeIso.date,
        zonedDateTimeIso.time,
        zonedDateTimeIso.zone,
      ),
      '14. Radschab 1446 AH, 14:32:12 MEZ',
    );

    expect(
      ZonedDateTimeFormatter.withLength(locale, DateTimeLength.short).formatIso(
        zonedDateTimeIso.date,
        zonedDateTimeIso.time,
        zonedDateTimeIso.zone,
      ),
      '14.07.46 AH, 14:32:12 MEZ',
    );
  });
}
