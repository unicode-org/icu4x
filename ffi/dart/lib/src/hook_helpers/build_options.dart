import 'dart:convert' show JsonEncoder;
import 'dart:io';

import 'package:collection/collection.dart';
import 'package:hooks/hooks.dart' show HookInputUserDefines;
import 'package:path/path.dart' as path;

enum BuildModeEnum { local, checkout, fetch }

class BuildOptions {
  final BuildModeEnum buildMode;
  final Uri? localPath;
  final Uri? checkoutPath;

  BuildOptions({required this.buildMode, this.localPath, this.checkoutPath});

  Map<String, dynamic> toMap() {
    return {
      'buildMode': buildMode.name,
      if (localPath != null) 'localPath': localPath.toString(),
      if (checkoutPath != null) 'checkoutPath': checkoutPath.toString(),
    };
  }

  factory BuildOptions.fromDefines(HookInputUserDefines defines) {
    return BuildOptions(
      buildMode:
          BuildModeEnum.values.firstWhereOrNull(
            (element) => element.name == defines['buildMode'],
          ) ??
          BuildModeEnum.fetch,
      localPath: defines.path('localPath'),
      checkoutPath: defines.path('checkoutPath'),
    );
  }

  static String getPath(Directory dir, String p) =>
      path.canonicalize(path.absolute(dir.path, p));

  String toJson() => const JsonEncoder.withIndent('  ').convert(toMap());
}
