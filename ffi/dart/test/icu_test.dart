import 'package:icu/icu.dart';
import 'package:test/test.dart';

void main() {
  Logger.initSimpleLogger();

  test('SignedFixedDecimal.toString', () {
    final x = SignedFixedDecimal.fromDoubleWithLowerMagnitude(1.49403, -7);
    expect(x.toString(), '1.4940300');
  });

  test('LocaleFallbacker', () {
    final iterator = LocaleFallbacker()
        .forConfig(
            LocaleFallbackConfig(priority: LocaleFallbackPriority.region))
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
    final formatter =
        ListFormatter.andWithLength(Locale.fromString('es'), ListLength.wide);
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
        ]);
  });

  test('DateTime formatting', () {
    final zonedDateTime =
        ZonedDateTimeParser().tryIsoFromStr('2025-01-15T14:32:12.34+01[Europe/Zurich]');

    var locale = Locale.fromString('de');

    expect(
        ZonedDateTimeFormatter.withLength(locale, DateTimeLength.long)
            .formatIso(zonedDateTime.date, zonedDateTime.time, zonedDateTime.zone),
        '15. Januar 2025, 14:32:12 MEZ');

    expect(
        ZonedDateTimeFormatter.withLength(locale, DateTimeLength.short)
            .formatIso(zonedDateTime.date, zonedDateTime.time, zonedDateTime.zone),
        '15.01.25, 14:32:12 MEZ');
  });
}
