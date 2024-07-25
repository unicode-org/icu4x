import 'package:icu/icu.dart';
import 'package:test/test.dart';

void main() {
  Logger.initSimpleLogger();

  test('FixedDecimal.toString', () {
    final x = FixedDecimal.fromDoubleWithLowerMagnitude(1.49403, -7);
    expect(x.toString(), '1.4940300');
  });

  test('LocaleFallbacker', () {
    final iterator = LocaleFallbacker(DataProvider.compiled())
        .forConfig(LocaleFallbackConfig(priority: LocaleFallbackPriority.region))
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

    final emojiSet = CodePointSetData.emoji(DataProvider.compiled());
    expect(emojiSet.contains(a), false);
    expect(emojiSet.contains(emoji), true);

    Rune upperA = CaseMapper(DataProvider.compiled()).simpleUppercase(a);
    expect(String.fromCharCode(upperA), 'A');
  });

  test('ListFormatter', () {
    final formatter = ListFormatter.andWithLength(
        DataProvider.compiled(), Locale.fromString('es'), ListLength.wide);
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
}
