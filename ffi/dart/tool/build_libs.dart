// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'dart:io';

import 'package:args/args.dart';
import 'package:native_assets_cli/code_assets.dart';
import 'package:path/path.dart' as path;

const crateName = 'icu_capi';

Future<void> main(List<String> args) async {
  final defaultFeatures = ['default_components', 'compiled_data'];
  var fileKey = 'file';
  var osKey = 'os';
  var architectureKey = 'architecture';
  var simulatorKey = 'simulator';
  var compileTypeKey = 'compile_type';
  var cargoFeaturesKey = 'cargo_features';
  final argParser =
      ArgParser()
        ..addOption(fileKey, mandatory: true)
        ..addOption(
          compileTypeKey,
          allowed: ['static', 'dynamic'],
          mandatory: true,
        )
        ..addFlag(simulatorKey, defaultsTo: false)
        ..addOption(osKey, mandatory: true)
        ..addOption(architectureKey, mandatory: true)
        ..addMultiOption(cargoFeaturesKey, defaultsTo: defaultFeatures);

  ArgResults parsed;
  try {
    parsed = argParser.parse(args);
  } catch (e) {
    print('Error parsing $args');
    print(argParser.usage);
    exit(1);
  }
  final buildStatic = parsed.option(compileTypeKey)! == 'static';
  final simulator = parsed.flag(simulatorKey);
  final targetOS = OS.values.firstWhere((o) => o.name == parsed.option(osKey)!);
  final targetArchitecture = Architecture.values.firstWhere(
    (o) => o.name == parsed.option(architectureKey)!,
  );

  final out = Uri.file(parsed.option(fileKey)!);

  final cargoFeatures = parsed.multiOption(cargoFeaturesKey);

  // We assume that the first folder to contain a cargo.toml above the
  // output directory is the directory containing the ICU4X code.
  Directory icu4xPath = recurseToParentRustCrate(File.fromUri(Platform.script));

  final libFileName = targetOS.filename(buildStatic)(
    crateName.replaceAll('-', '_'),
  );
  await buildLib(
    targetOS,
    targetArchitecture,
    buildStatic,
    simulator,
    out,
    libFileName,
    icu4xPath.path,
    cargoFeatures: cargoFeatures,
  );
}

Directory recurseToParentRustCrate(FileSystemEntity startingPoint) {
  var folder =
      startingPoint is Directory ? startingPoint : startingPoint.parent;
  while (!File.fromUri(folder.uri.resolve('Cargo.toml')).existsSync()) {
    folder = folder.parent;
    if (folder.parent == folder) {
      throw ArgumentError(
        'Running in the wrong directory, as no Cargo.toml exists above $startingPoint',
      );
    }
  }
  return folder;
}

Future<Uri> buildLibraryFromInput(
  BuildInput input,
  String workingDirectory,
) async {
  final crateNameFixed = crateName.replaceAll('-', '_');
  final libFileName = input.config.code.targetOS.filename(
    input.config.buildStatic,
  )(crateNameFixed);
  final libFileUri = input.outputDirectory.resolve(libFileName);
  await buildLib(
    input.config.code.targetOS,
    input.config.code.targetArchitecture,
    input.config.buildStatic,
    input.config.code.targetOS == OS.iOS &&
        input.config.code.iOS.targetSdk == IOSSdk.iPhoneSimulator,
    libFileUri,
    libFileName,
    workingDirectory,
  );
  return libFileUri;
}

// Copied from Dart's package:intl4x build.dart, see
// https://github.com/dart-lang/i18n/blob/main/pkgs/intl4x/hook/build.dart
Future<void> buildLib(
  OS targetOS,
  Architecture targetArchitecture,
  bool buildStatic,
  bool isSimulator,
  Uri libFileUri,
  String libFileName,
  String workingDirectory, {
  List<String> cargoFeatures = const [],
}) async {
  final isNoStd = _isNoStdTarget((targetOS, targetArchitecture));
  final target = _asRustTarget(targetOS, targetArchitecture, isSimulator);
  if (!isNoStd) {
    final rustArguments = ['target', 'add', target];
    await runProcess(
      'rustup',
      rustArguments,
      workingDirectory: workingDirectory,
    );
  }
  final features = [
    'default_components',
    'icu_collator',
    'icu_datetime',
    'icu_list',
    'icu_decimal',
    'icu_plurals',
    'compiled_data',
    'buffer_provider',
    'experimental_components',
  ];
  final stdFeatures = ['logging', 'simple_logger'];
  final noStdFeatures = ['libc_alloc', 'panic_handler'];
  final arguments = [
    if (buildStatic || isNoStd) '+nightly',
    'rustc',
    '-p=$crateName',
    '--crate-type=${buildStatic ? 'staticlib' : 'cdylib'}',
    '--release',
    '--config=profile.release.panic="abort"',
    '--config=profile.release.codegen-units=1',
    '--no-default-features',
    '--features=${[...features, ...(isNoStd ? noStdFeatures : stdFeatures)].join(',')}',
    if (isNoStd) '-Zbuild-std=core,alloc',
    if (buildStatic || isNoStd) ...[
      '-Zbuild-std=std,panic_abort',
      '-Zbuild-std-features=panic_immediate_abort',
    ],
    '--target=$target',
  ];
  await runProcess('cargo', arguments, workingDirectory: workingDirectory);

  final builtPath = path.join(
    workingDirectory,
    'target',
    target,
    'release',
    libFileName,
  );
  final file = File(builtPath);
  if (!(await file.exists())) {
    throw FileSystemException('Building the dylib failed', builtPath);
  }
  await file.copy(libFileUri.toFilePath(windows: Platform.isWindows));
}

String _asRustTarget(OS os, Architecture? architecture, bool isSimulator) {
  if (os == OS.iOS && architecture == Architecture.arm64 && isSimulator) {
    return 'aarch64-apple-ios-sim';
  }
  return switch ((os, architecture)) {
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
    (_, _) =>
      throw UnimplementedError(
        'Target ${(os, architecture)} not available for rust',
      ),
  };
}

bool _isNoStdTarget((OS os, Architecture? architecture) arg) => [
  (OS.android, Architecture.riscv64),
  (OS.linux, Architecture.riscv64),
].contains(arg);

extension on BuildConfig {
  bool get buildStatic =>
      code.linkModePreference == LinkModePreference.static || linkingEnabled;
}

extension on OS {
  String Function(String) filename(bool buildStatic) =>
      buildStatic ? staticlibFileName : dylibFileName;
}

Future<void> runProcess(
  String executable,
  List<String> arguments, {
  String? workingDirectory,
  bool dryRun = false,
}) async {
  print('----------');
  print('Running `$executable $arguments` in $workingDirectory');
  if (!dryRun) {
    final processResult = await Process.run(
      executable,
      arguments,
      workingDirectory: workingDirectory,
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
