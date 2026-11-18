// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'package:code_assets/code_assets.dart';
import 'package:prebuilt_code_assets/prebuilt_code_assets.dart';
import 'package:record_use/record_use.dart' as record_use;

import 'hashes.dart' show fileHashes, version;

/// Shared build and link hook specification for `package:icu4x`.
final PrebuiltLibrary icu4xLibrary = PrebuiltLibrary(
  name: 'icu4x',
  packageName: 'icu4x',
  assetName: 'src/bindings/lib.g.dart',
  fallbackToBuildOnFetchFailure: false,
  strictBuildOptions: true,
  releaseConfig: PrebuiltReleaseConfig.rustTargets(
    owner: 'unicode-org',
    repo: 'icu4x',
    version: version,
    targetHashes: fileHashes,
    libraryName: 'icu4x',
    staticLibraryType: 'static-with_data',
    dynamicLibraryType: 'dynamic-with_data',
    formatAssetName: (rustTarget, libraryType) =>
        'icu4x-2-$libraryType-$rustTarget',
  ),
  buildFromSource: const CargoSourceBuilder(
    libraryName: 'icu4x',
    manifestPath: 'ffi/capi/Cargo.toml',
    noDefaultFeatures: true,
    features: [
      'default_components',
      'experimental',
      'buffer_provider',
      'compiled_data',
    ],
    conditionalFeatures: _conditionalFeatures,
    cargoConfigFlags: [
      '--config=profile.release.panic="abort"',
      '--config=profile.release.codegen-units=1',
    ],
    nightlyToolchainForStatic: 'nightly-2026-06-01',
    ensureRustupTarget: true,
  ).build,
  usedSymbols: SymbolsResolvers.fromMethodPrefix(
    const record_use.Library('package:icu4x/src/bindings/lib.g.dart'),
  ),
  libraries: (targetOS) => switch (targetOS) {
    // On Windows, icu4x.lib is lacking /DEFAULTLIB directives to advise
    // the linker on what libraries to link against. To make up for that,
    // the libraries used have to be provided to the linker explicitly.
    OS.windows => const ['MSVCRT', 'ws2_32', 'userenv', 'ntdll'],
    // On Android, libm (math library) is not linked by default, but math
    // functions like `expf` referenced in Rust libicu4x require libm.
    OS.android => const ['m'],
    _ => const [],
  },
);

List<String> _conditionalFeatures(bool isNoStd) => isNoStd
    ? const ['libc_alloc', 'looping_panic_handler']
    : const ['simple_logger'];
