import 'package:hooks/hooks.dart' show link;
import 'package:icu4x/hook.dart' show treeshakeLibrary;

/// Run the linker to turn a static into a treeshaken dynamic library.
Future<void> main(List<String> args) async {
  await link(args, (input, output) async => treeshakeLibrary(input, output));
}
