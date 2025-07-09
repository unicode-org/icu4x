import 'dart:convert';
import 'dart:io';

import 'package:code_assets/code_assets.dart'
    show HookConfigCodeConfig, LinkInputCodeAssets, OS;
import 'package:collection/collection.dart' show IterableExtension;
import 'package:hooks/hooks.dart' show LinkInput, LinkOutputBuilder;
import 'package:icu4x/src/hook_helpers/shared.dart' show assetId, package;
import 'package:logging/logging.dart';
import 'package:native_toolchain_c/native_toolchain_c.dart';
import 'package:record_use/record_use.dart' as record_use;

const diplomatFfiUseIdentifier = record_use.Identifier(
  importUri: 'package:icu4x/src/bindings/lib.g.dart',
  name: '_DiplomatFfiUse',
);

Future<void> treeshakeLibrary(
  LinkInput input,
  LinkOutputBuilder output, {
  Map<String, Set<String>?> symbolsToKeep = const {},
}) async {
  print('Start linking');
  final staticLib = input.assets.code.firstWhereOrNull(
    (asset) => asset.id == 'package:$package/$assetId',
  );
  if (staticLib == null) {
    // No static lib built, so assume a dynamic one was already bundled.
    return;
  }

  final usages = input.usages;

  output.addDependency(staticLib.file!);

  final symbols = usages
      ?.constantsOf(diplomatFfiUseIdentifier)
      .map((instance) => instance['symbol'] as String);

  final usedSymbols = symbols?.whereNot(
    (symbol) => symbolsToKeep.entries.any(
      (entry) => _isUnusedSymbol(symbol, entry.key, entry.value),
    ),
  );

  print('''
### Using symbols:
${usedSymbols?.join('\n')}
### End using symbols
''');

  await CLinker.library(
    name: 'icu4x',
    packageName: 'icu4x',
    assetName: assetId,
    sources: [staticLib.file!.toFilePath()],
    libraries:
        // On Windows, icu4x.lib is lacking /DEFAULTLIB directives to advise
        // the linker on what libraries to link against. To make up for that,
        // the libraries used have to be provided to the linker explicitly.
        input.config.code.targetOS == OS.windows
            ? const ['MSVCRT', 'ws2_32', 'userenv', 'ntdll']
            : const [],
    linkerOptions: LinkerOptions.treeshake(symbols: usedSymbols),
  ).run(
    input: input,
    output: output,
    logger:
        Logger('')
          ..level = Level.ALL
          ..onRecord.listen((record) => print(record.message)),
  );
}

bool _isUnusedSymbol(String symbol, String prefix, Set<String>? usedSymbols) =>
    symbol.startsWith(prefix) && !(usedSymbols?.contains(symbol) ?? true);

extension on LinkInput {
  record_use.RecordedUsages? get usages {
    if (recordedUsagesFile == null) {
      return null;
    }
    final usagesContent = File.fromUri(recordedUsagesFile!).readAsStringSync();
    final usagesJson = jsonDecode(usagesContent) as Map<String, dynamic>;
    return record_use.RecordedUsages.fromJson(usagesJson);
  }
}
