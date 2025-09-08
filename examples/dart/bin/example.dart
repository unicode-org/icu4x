import 'package:icu4x/icu4x.dart' show Decimal;

void main(List<String> arguments) {
  final f = 1.49403;
  final x = Decimal.fromDoubleWithLowerMagnitude(f, -7);
  print('$f formatted is: $x!');
}
