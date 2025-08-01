import 'package:flutter/material.dart';
import 'package:dart_icu4x/dart_icu4x.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  var _charProperties = <UnicodeCharProperties>[];

  void _getCharProperties() {
    final charProperties = getUnicodeCharProperties(
      offset: BigInt.from(0),
      limit: BigInt.from(20),
    );
    setState(() {
      _charProperties = charProperties;
    });
  }

  @override
  void initState() {
    super.initState();
    _getCharProperties();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: ListView.separated(
          shrinkWrap: true,
          itemCount: _charProperties.length,
          itemBuilder: (context, index) {
            final char = _charProperties[index];
            return ListTile(
              title: Text(char.character),
              subtitle: Text(char.codePoint.toString()),
              trailing: Text(char.generalCategory.toString()),
            );
          },
          separatorBuilder: (context, index) => const Divider(),
        ),
      ),
    );
  }
}
