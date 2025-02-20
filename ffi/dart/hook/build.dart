// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'dart:io';

import 'package:native_assets_cli/code_assets.dart';

import '../tool/build_libs.dart'
    show buildLibraryFromInput, recurseToParentRustCrate;

void main(List<String> args) async {
  await build(args, (input, output) async {
    // We assume that the first folder to contain a cargo.toml above the
    // output directory is the directory containing the ICU4X code.
    final icu4xPath = recurseToParentRustCrate(
      Directory.fromUri(input.outputDirectory),
    );

    final lib = await buildLibraryFromInput(input, icu4xPath.path);

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
