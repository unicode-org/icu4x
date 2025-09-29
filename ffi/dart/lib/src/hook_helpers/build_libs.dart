// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import 'dart:io';

import 'package:args/args.dart';

Future<void> main(List<String> args) async {
  const fileKey = 'file';
  const targetKey = 'target';
  const compileTypeKey = 'compile_type';
  const cargoFeaturesKey = 'cargo_features';
  const workingDirectoryKey = 'working_directory';
  final argParser = ArgParser()
    ..addOption(fileKey, mandatory: true)
    ..addOption(compileTypeKey, allowed: ['static', 'dynamic'], mandatory: true)
    ..addOption(targetKey, mandatory: true)
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

  await buildLib(
    parsed.option(targetKey)!,
    parsed.option(compileTypeKey)! == 'static',
    (parsed.option(workingDirectoryKey) != null
            ? Directory(parsed.option(workingDirectoryKey)!)
            : null) ??
        File.fromUri(Platform.script).parent,
    parsed.multiOption(cargoFeaturesKey),
    Uri.file(parsed.option(fileKey)!),
  );
}

Future buildLib(
  String rustTarget,
  bool buildStatic,
  Directory startingPoint,
  List<String> cargoFeatures,
  Uri out,
) async {
  // We assume that the first folder to contain a cargo.toml above the
  // output directory is the directory containing the ICU4X code.
  var workingDirectory = startingPoint;
  while (!File.fromUri(
    workingDirectory.uri.resolve('Cargo.toml'),
  ).existsSync()) {
    workingDirectory = workingDirectory.parent;
  }

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
      '--features=${{...cargoFeatures, ...additionalFeatures}.join(',')}',
      if (isNoStd) '-Zbuild-std=core,alloc',
      if (buildStatic || isNoStd) ...['-Zbuild-std=std,panic_abort'],
      '--target=$rustTarget',
      '--',
      '--emit',
      'link=${out.toFilePath(windows: Platform.isWindows)}',
    ],
    workingDirectory: workingDirectory,
    environment: {if (isNoStd) 'RUSTFLAGS': '-Zunstable-options -Cpanic=immediate-abort'},
  );
}

bool _isNoStdTarget(String target) =>
    ['riscv64-linux-android', 'riscv64gc-unknown-linux-gnu'].contains(target);

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
