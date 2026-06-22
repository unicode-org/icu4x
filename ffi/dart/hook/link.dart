// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'package:code_assets/code_assets.dart'
    show CodeAsset, HookConfigCodeConfig, LinkInputCodeAssets, OS;
import 'package:hooks/hooks.dart' show link;
import 'package:logging/logging.dart' show Level, Logger;
import 'package:native_toolchain_c/native_toolchain_c.dart'
    show CLinker, LinkerOptions;
import 'package:record_use/record_use.dart' as record_use;

/// Run the linker to turn a static into a treeshaken dynamic library.
Future<void> main(List<String> args) async {
  await link(args, (input, output) async {
    print('Start linking');
    final CodeAsset staticLib;
    try {
      staticLib = input.assets.code.firstWhere(
        (asset) => asset.id == 'package:icu4x/src/bindings/lib.g.dart',
      );
    } catch (e) {
      // No static lib built, so assume a dynamic one was already bundled.
      return;
    }

    final usedSymbols = input
        // ignore: experimental_member_use
        .recordedUses
        ?.instances[const record_use.Class(
          '_DiplomatFfiUse',
          record_use.Library('package:icu4x/src/bindings/lib.g.dart'),
        )]
        ?.whereType<record_use.InstanceConstantReference>()
        .map((instance) => instance.instanceConstant)
        .whereType<record_use.InstanceConstant>()
        .map(
          (instanceConstant) =>
              instanceConstant.fields['symbol'] as record_use.StringConstant,
        )
        .map((stringConstant) => stringConstant.value);

    print('''
### Using symbols:
  ${usedSymbols?.join('\n')}
### End using symbols
''');

    await CLinker.library(
      name: 'icu4x',
      packageName: input.packageName,
      assetName: 'src/bindings/lib.g.dart',
      sources: [staticLib.file!.toFilePath()],
      libraries:
          // On Windows, icu4x.lib is lacking /DEFAULTLIB directives to advise
          // the linker on what libraries to link against. To make up for that,
          // the libraries used have to be provided to the linker explicitly.
          input.config.code.targetOS == OS.windows
          ? const ['MSVCRT', 'ws2_32', 'userenv', 'ntdll']
          : const [],
      linkerOptions: LinkerOptions.treeshake(symbolsToKeep: usedSymbols),
    ).run(
      input: input,
      output: output,
      logger: Logger('')
        ..level = Level.ALL
        ..onRecord.listen((record) => print(record.message)),
    );
  });
}
