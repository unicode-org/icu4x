// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'package:native_assets_cli/code_assets.dart';

import '../tool/build_libs.dart'
    show buildLibraryFromInput;

void main(List<String> args) async {
  await build(args, (input, output) async {
    final lib = await buildLibraryFromInput(input);

    output.assets.code.add(
      CodeAsset(
        package: input.packageName,
        name: 'src/lib.g.dart',
        linkMode: DynamicLoadingBundled(),
        os: input.config.code.targetOS,
        architecture: input.config.code.targetArchitecture,
        file: lib.uri,
      ),
    );
  });
}
