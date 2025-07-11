import 'dart:io';

import 'package:icu4x/src/hook_helpers/build_options.dart'
    show BuildModeEnum, BuildOptions;
import 'package:yaml_edit/yaml_edit.dart' show YamlEditor;

Future<void> main(List<String> args) async {
  final pubspecPath = args[0];
  final buildModeString = args[1];
  final pathString = args.length > 2 ? args[2] : null;
  final buildMode = BuildModeEnum.values.firstWhere(
    (mode) => mode.name == buildModeString,
  );
  final buildOptions = switch (buildMode) {
    BuildModeEnum.local => BuildOptions(
      buildMode: buildMode,
      localDylibPath: Uri.file(pathString!),
    ),
    BuildModeEnum.checkout => BuildOptions(
      buildMode: buildMode,
      checkoutPath: Uri.directory(pathString!),
    ),
    BuildModeEnum.fetch => BuildOptions(buildMode: buildMode),
  };
  print('Writing build options: ${buildOptions.toJson()}');

  final pubspecContents = await File(pubspecPath).readAsString();
  final yamlEditor = YamlEditor(pubspecContents);
  yamlEditor.update(['hooks', 'user_defines'], {'icu4x': buildOptions.toMap()});
  await File(pubspecPath).writeAsString(yamlEditor.toString());
}
