import 'dart:io';

import 'package:code_assets/code_assets.dart';
import 'package:crypto/crypto.dart' show sha256;
import 'package:hooks/hooks.dart';
import 'package:icu4x/src/hook_helpers/build_libs.dart' show buildLib;
import 'package:icu4x/src/hook_helpers/build_options.dart'
    show BuildModeEnum, BuildOptions;
import 'package:icu4x/src/hook_helpers/hashes.dart' show fileHashes;
import 'package:icu4x/src/hook_helpers/shared.dart' show assetId, package;
import 'package:icu4x/src/hook_helpers/version.dart' show version;

void main(List<String> args) async {
  await build(args, (input, output) async {
    BuildOptions buildOptions;
    try {
      buildOptions = BuildOptions.fromDefines(input.userDefines);
      print('Got build options: ${buildOptions.toJson()}');
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
    print('Read build options: ${buildOptions.toJson()}');
    final buildMode = switch (buildOptions.buildMode) {
      BuildModeEnum.local => LocalMode(input, buildOptions.localPath),
      BuildModeEnum.checkout => CheckoutMode(input, buildOptions.checkoutPath),
      BuildModeEnum.fetch => FetchMode(input),
    };

    final builtLibrary = await buildMode.build();

    output.assets.code.add(
      CodeAsset(
        package: package,
        name: assetId,
        linkMode: DynamicLoadingBundled(),
        file: builtLibrary,
      ),
      routing:
          buildOptions.buildMode != BuildModeEnum.local &&
              input.config.linkingEnabled
          ? const ToLinkHook(package)
          : const ToAppBundle(),
    );
    output.addDependencies(buildMode.dependencies);
    output.addDependency(input.packageRoot.resolve('pubspec.yaml'));
  });
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
    final targetOS = input.config.code.targetOS;
    final targetArchitecture = input.config.code.targetArchitecture;
    final libraryType = input.config.buildStatic ? 'static_data' : 'dynamic';
    final target = [targetOS, targetArchitecture, libraryType].join('_');
    print('Fetching pre-built binary for $version and $target');
    final dylibRemoteUri = Uri.parse(
      'https://github.com/unicode-org/icu4x/releases/download/$version/$target',
    );
    final library = await fetchToFile(
      dylibRemoteUri,
      input.outputDirectory.resolve(input.config.filename('icu4x')),
    );

    final bytes = await library.readAsBytes();
    final fileHash = sha256.convert(bytes).toString();
    final expectedFileHash =
        fileHashes[(targetOS, targetArchitecture, libraryType)];
    if (fileHash != expectedFileHash) {
      throw Exception(
        'The pre-built binary for the target $target at $dylibRemoteUri has a'
        ' hash of $fileHash, which does not match $expectedFileHash fixed in'
        ' the build hook of package:icu4x.',
      );
    }
    return library.uri;
  }

  Future<File> fetchToFile(Uri uri, Uri fileUri) async {
    final request = await httpClient.getUrl(uri);
    final response = await request.close();
    if (response.statusCode != 200) {
      throw ArgumentError('The request to $uri failed');
    }
    final file = File.fromUri(fileUri);
    await file.create();
    await response.pipe(file.openWrite());
    return file;
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
    final builtLib = await buildLib(
      input.config.code.targetOS,
      input.config.code.targetArchitecture,
      input.config.buildStatic,
      input.config.code.targetOS == OS.iOS &&
          input.config.code.iOS.targetSdk == IOSSdk.iPhoneSimulator,
      Directory.fromUri(checkoutPath!),
      [
        'default_components',
        'experimental',

        'buffer_provider',
        'compiled_data',
      ],
    );
    return builtLib.uri;
  }

  @override
  List<Uri> get dependencies => [checkoutPath!.resolve('Cargo.lock')];
}

extension on BuildConfig {
  bool get buildStatic =>
      code.linkModePreference == LinkModePreference.static || linkingEnabled;

  String Function(String) get filename => buildStatic
      ? code.targetOS.staticlibFileName
      : code.targetOS.dylibFileName;
}
