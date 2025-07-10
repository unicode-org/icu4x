// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'dart:io';

import 'package:args/args.dart';
import 'package:code_assets/code_assets.dart';
import 'package:path/path.dart' as path;

Future<void> main(List<String> args) async {
  const fileKey = 'file';
  const osKey = 'os';
  const architectureKey = 'architecture';
  const simulatorKey = 'simulator';
  const compileTypeKey = 'compile_type';
  const cargoFeaturesKey = 'cargo_features';
  const workingDirectoryKey = 'working_directory';
  final argParser = ArgParser()
    ..addOption(fileKey, mandatory: true)
    ..addOption(compileTypeKey, allowed: ['static', 'dynamic'], mandatory: true)
    ..addFlag(simulatorKey, defaultsTo: false)
    ..addOption(osKey, mandatory: true)
    ..addOption(architectureKey, mandatory: true)
    ..addOption(workingDirectoryKey)
    ..addMultiOption(
      cargoFeaturesKey,
      defaultsTo: ['default_components', 'compiled_data'],
    );

  ArgResults parsed;
  try {
    parsed = argParser.parse(args);
  } catch (e) {
    print('Error parsing $args');
    print(argParser.usage);
    exit(1);
  }

  final lib = await buildLib(
    OS.values.firstWhere((o) => o.name == parsed.option(osKey)!),
    Architecture.values.firstWhere(
      (o) => o.name == parsed.option(architectureKey)!,
    ),
    parsed.option(compileTypeKey)! == 'static',
    parsed.flag(simulatorKey),
    (parsed.option(workingDirectoryKey) != null
            ? Directory(parsed.option(workingDirectoryKey)!)
            : null) ??
        File.fromUri(Platform.script).parent,
    parsed.multiOption(cargoFeaturesKey),
  );
  if (!lib.existsSync()) {
    throw FileSystemException('Building the dylib failed', lib.path);
  }
  final file = Uri.file(
    parsed.option(fileKey)!,
  ).toFilePath(windows: Platform.isWindows);
  File(file).parent.createSync(recursive: true);
  await lib.copy(file);
}

Future<File> buildLib(
  OS targetOS,
  Architecture targetArchitecture,
  bool buildStatic,
  bool isSimulator,
  Directory startingPoint,
  List<String> cargoFeatures,
) async {
  // We assume that the first folder to contain a cargo.toml above the
  // output directory is the directory containing the ICU4X code.
  var workingDirectory = startingPoint;
  while (!File.fromUri(
    workingDirectory.uri.resolve('Cargo.toml'),
  ).existsSync()) {
    workingDirectory = workingDirectory.parent;
  }

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

  final additionalFeatures = isNoStd
      ? ['libc_alloc', 'looping_panic_handler']
      : ['logging', 'simple_logger'];
  await runProcess('cargo', [
    if (buildStatic || isNoStd) '+nightly',
    'rustc',
    '--manifest-path=ffi/capi/Cargo.toml',
    '--crate-type=${buildStatic ? 'staticlib' : 'cdylib'}',
    '--release',
    '--config=profile.release.panic="abort"',
    '--config=profile.release.codegen-units=1',
    '--no-default-features',
    '--features=${{...cargoFeatures, ...additionalFeatures}.join(',')}',
    if (isNoStd) '-Zbuild-std=core,alloc',
    if (buildStatic || isNoStd) ...[
      '-Zbuild-std=std,panic_abort',
      '-Zbuild-std-features=panic_immediate_abort',
    ],
    '--target=$target',
  ], workingDirectory: workingDirectory);

  final file = File(
    path.join(
      workingDirectory.path,
      'target',
      target,
      'release',
      (buildStatic ? targetOS.staticlibFileName : targetOS.dylibFileName)(
        'icu_capi',
      ),
    ),
  );
  if (!(await file.exists())) {
    throw FileSystemException('Building the dylib failed', file.path);
  }
  return file;
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
    (_, _) => throw UnimplementedError(
      'Target ${(os, architecture)} not available for rust',
    ),
  };
}

bool _isNoStdTarget((OS os, Architecture? architecture) arg) => [
  (OS.android, Architecture.riscv64),
  (OS.linux, Architecture.riscv64),
].contains(arg);

Future<void> runProcess(
  String executable,
  List<String> arguments, {
  Directory? workingDirectory,
  bool dryRun = false,
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
