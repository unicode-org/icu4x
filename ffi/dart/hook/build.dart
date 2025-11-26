// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'dart:io';

import 'package:code_assets/code_assets.dart';
import 'package:crypto/crypto.dart' show sha256;
import 'package:hooks/hooks.dart';
import 'package:icu4x/src/hook_helpers/hashes.dart' show fileHashes, version;

void main(List<String> args) async {
  await build(args, (input, output) async {
    BuildOptions buildOptions;
    try {
      buildOptions = BuildOptions.fromDefines(input.userDefines);
    } catch (e) {
      throw ArgumentError('''
Error: $e


Set the build mode with either `fetch`, `local`, or `checkout` by writing into your pubspec:

* fetch: Fetch the precompiled binary from a CDN.
```
hooks:
  user_defines:
    icu4x:
      buildMode: fetch
```

* local: Use a locally existing binary.
```
hooks:
  user_defines:
    icu4x:
      buildMode: local
      localPath: path/to/dylib.so
```

* checkout: Build a fresh library from a local git checkout of the icu4x repository.
```
hooks:
  user_defines:
    icu4x:
      buildMode: checkout
      checkoutPath: path/to/checkout
```

''');
    }
    print('Read build options: $buildOptions');
    final buildMode = switch (buildOptions.buildMode) {
      BuildModeEnum.local => LocalMode(input, buildOptions.localPath),
      BuildModeEnum.checkout => CheckoutMode(input, buildOptions.checkoutPath),
      BuildModeEnum.fetch => FetchMode(input),
    };

    final builtLibrary = await buildMode.build();

    output.assets.code.add(
      CodeAsset(
        package: input.packageName,
        name: 'src/bindings/lib.g.dart',
        linkMode: DynamicLoadingBundled(),
        file: builtLibrary,
      ),
      routing:
          buildOptions.buildMode != BuildModeEnum.local &&
              input.config.linkingEnabled
          ? ToLinkHook(input.packageName)
          : const ToAppBundle(),
    );
    output.dependencies.addAll(buildMode.dependencies);
    output.dependencies.add(input.packageRoot.resolve('pubspec.yaml'));
  });
}

enum BuildModeEnum { local, checkout, fetch }

class BuildOptions {
  final BuildModeEnum buildMode;
  final Uri? localPath;
  final Uri? checkoutPath;

  BuildOptions({required this.buildMode, this.localPath, this.checkoutPath});

  factory BuildOptions.fromDefines(HookInputUserDefines defines) {
    return BuildOptions(
      buildMode: BuildModeEnum.values.firstWhere(
        (element) => element.name == defines['buildMode'],
        orElse: () => BuildModeEnum.fetch,
      ),
      localPath: defines.path('localPath'),
      checkoutPath: defines.path('checkoutPath'),
    );
  }
}

sealed class BuildMode {
  final BuildInput input;

  const BuildMode(this.input);

  List<Uri> get dependencies;

  Future<Uri> build();
}

final class FetchMode extends BuildMode {
  FetchMode(super.input);
  final httpClient = HttpClient();

  @override
  Future<Uri> build() async {
    print('Running in `fetch` mode');
    final rustTarget = _asRustTarget(input.config.code);
    final libraryType = input.config.buildStatic
        ? 'static-with_data'
        : 'dynamic';
    print('Fetching pre-built binary for $version, $rustTarget, $libraryType');
    final dylibRemoteUri = Uri.parse(
      'https://github.com/unicode-org/icu4x/releases/'
      'download/$version/icu4x-2-$libraryType-$rustTarget',
    );
    final request = await httpClient.getUrl(dylibRemoteUri);
    final response = await request.close();
    if (response.statusCode != 200) {
      throw ArgumentError('The request to $dylibRemoteUri failed');
    }
    final bytes = await response.fold<List<int>>([], (a, b) => a..addAll(b));
    final fileHash = sha256.convert(bytes).toString();
    final expectedFileHash = fileHashes[(rustTarget, libraryType)];
    if (fileHash != expectedFileHash) {
      throw Exception(
        'The pre-built binary for the target $rustTarget-$libraryType at '
        '$dylibRemoteUri has a hash of $fileHash, which does not match '
        '$expectedFileHash fixed in the build hook of package:icu4x.',
      );
    }
    final library = File.fromUri(
      input.outputDirectory.resolve(input.config.filename('icu4x')),
    );
    await library.writeAsBytes(bytes);
    return library.uri;
  }

  @override
  List<Uri> get dependencies => [];
}

final class LocalMode extends BuildMode {
  final Uri? localPath;
  LocalMode(super.input, this.localPath);

  String get _localLibraryPath {
    if (localPath != null) {
      return localPath!.toFilePath(windows: Platform.isWindows);
    }
    throw ArgumentError(
      '`localPath` is not set in the build options. '
      'If the `buildMode` is set to `local`, the '
      '`localPath` key must contain the path to '
      'the binary.',
    );
  }

  @override
  Future<Uri> build() async {
    print('Running in `local` mode');
    final targetOS = input.config.code.targetOS;
    final dylibFileName = targetOS.dylibFileName('icu4x');
    final dylibFileUri = input.outputDirectory.resolve(dylibFileName);
    final file = File(_localLibraryPath);
    if (!(await file.exists())) {
      throw FileSystemException('Could not find binary.', _localLibraryPath);
    }
    await file.copy(dylibFileUri.toFilePath(windows: Platform.isWindows));
    return dylibFileUri;
  }

  @override
  List<Uri> get dependencies => [Uri.file(_localLibraryPath)];
}

final class CheckoutMode extends BuildMode {
  final Uri? checkoutPath;

  CheckoutMode(super.input, this.checkoutPath);

  @override
  Future<Uri> build() async {
    print('Running in `checkout` mode');
    if (checkoutPath == null) {
      throw ArgumentError(
        'Specify the ICU4X checkout folder with the `checkoutPath` key in your '
        'pubspec build options.',
      );
    }
    if (!File.fromUri(checkoutPath!.resolve('Cargo.lock')).existsSync()) {
      throw ArgumentError(
        'The `Cargo.lock` file could not by found at $checkoutPath',
      );
    }
    final out = input.outputDirectory.resolve(input.config.filename('icu4x'));
    final rustTarget = _asRustTarget(input.config.code);
    final buildStatic = input.config.buildStatic;
    final workingDirectory = Directory.fromUri(checkoutPath!);

    final isNoStd = _isNoStdTarget(rustTarget);

    final nightly =
        Platform.environment['PINNED_CI_NIGHTLY'] ?? 'nightly-2025-09-27';

    if (buildStatic || isNoStd) {
      await runProcess('rustup', [
        'toolchain',
        'install',
        '--no-self-update',
        nightly,
        '--component',
        'rust-src',
      ], workingDirectory: workingDirectory);
    }

    await runProcess('rustup', [
      'target',
      'add',
      rustTarget,
      if (buildStatic || isNoStd) ...['--toolchain', nightly],
    ], workingDirectory: workingDirectory);

    final additionalFeatures = isNoStd
        ? ['libc_alloc', 'looping_panic_handler']
        : ['simple_logger'];
    await runProcess(
      'cargo',
      [
        if (buildStatic || isNoStd) '+$nightly',
        'rustc',
        '--manifest-path=ffi/capi/Cargo.toml',
        '--crate-type=${buildStatic ? 'staticlib' : 'cdylib'}',
        '--release',
        '--config=profile.release.panic="abort"',
        '--config=profile.release.codegen-units=1',
        '--no-default-features',
        '--features=${{
          ...['default_components', 'experimental', 'buffer_provider', 'compiled_data'],
          ...additionalFeatures,
        }.join(',')}',
        if (isNoStd) '-Zbuild-std=core,alloc',
        if (buildStatic || isNoStd) ...['-Zbuild-std=std,panic_abort'],
        '--target=$rustTarget',
        '--',
        '--emit',
        'link=${out.toFilePath(windows: Platform.isWindows)}',
      ],
      workingDirectory: workingDirectory,
      environment: {
        if (isNoStd) 'RUSTFLAGS': '-Zunstable-options -Cpanic=immediate-abort',
      },
    );
    return out;
  }

  @override
  List<Uri> get dependencies => [checkoutPath!.resolve('Cargo.lock')];
}

bool _isNoStdTarget(String target) =>
    ['riscv64-linux-android', 'riscv64gc-unknown-linux-gnu'].contains(target);

String _asRustTarget(CodeConfig code) {
  if (code.targetOS == OS.iOS &&
      code.targetArchitecture == Architecture.arm64 &&
      code.iOS.targetSdk == IOSSdk.iPhoneSimulator) {
    return 'aarch64-apple-ios-sim';
  }
  return switch ((code.targetOS, code.targetArchitecture)) {
    (OS.android, Architecture.arm) => 'armv7-linux-androideabi',
    (OS.android, Architecture.arm64) => 'aarch64-linux-android',
    (OS.android, Architecture.ia32) => 'i686-linux-android',
    (OS.android, Architecture.riscv64) => 'riscv64-linux-android',
    (OS.android, Architecture.x64) => 'x86_64-linux-android',
    (OS.fuchsia, Architecture.arm64) => 'aarch64-unknown-fuchsia',
    (OS.fuchsia, Architecture.x64) => 'x86_64-unknown-fuchsia',
    (OS.iOS, Architecture.arm64) => 'aarch64-apple-ios',
    (OS.iOS, Architecture.x64) => 'x86_64-apple-ios',
    (OS.linux, Architecture.arm) => 'armv7-unknown-linux-gnueabihf',
    (OS.linux, Architecture.arm64) => 'aarch64-unknown-linux-gnu',
    (OS.linux, Architecture.ia32) => 'i686-unknown-linux-gnu',
    (OS.linux, Architecture.riscv32) => 'riscv32gc-unknown-linux-gnu',
    (OS.linux, Architecture.riscv64) => 'riscv64gc-unknown-linux-gnu',
    (OS.linux, Architecture.x64) => 'x86_64-unknown-linux-gnu',
    (OS.macOS, Architecture.arm64) => 'aarch64-apple-darwin',
    (OS.macOS, Architecture.x64) => 'x86_64-apple-darwin',
    (OS.windows, Architecture.arm64) => 'aarch64-pc-windows-msvc',
    (OS.windows, Architecture.ia32) => 'i686-pc-windows-msvc',
    (OS.windows, Architecture.x64) => 'x86_64-pc-windows-msvc',
    (_, _) => throw UnimplementedError('Target $code not available for rust'),
  };
}

extension on BuildConfig {
  bool get buildStatic =>
      code.linkModePreference == LinkModePreference.static || linkingEnabled;

  String Function(String) get filename => buildStatic
      ? code.targetOS.staticlibFileName
      : code.targetOS.dylibFileName;
}

Future<void> runProcess(
  String executable,
  List<String> arguments, {
  Directory? workingDirectory,
  bool dryRun = false,
  Map<String, String>? environment,
}) async {
  print('----------');
  print('Running `$executable $arguments` in $workingDirectory');
  if (!dryRun) {
    final processResult = await Process.run(
      executable,
      arguments,
      workingDirectory: workingDirectory?.path,
    );
    print('stdout:');
    print(processResult.stdout);
    if ((processResult.stderr as String).isNotEmpty) {
      print('stderr:');
      print(processResult.stderr);
    }
    if (processResult.exitCode != 0) {
      throw ProcessException(executable, arguments, '', processResult.exitCode);
    }
  } else {
    print('Not running, as --dry-run is set.');
  }
  print('==========');
}
