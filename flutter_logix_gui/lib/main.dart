import 'dart:convert';
import 'dart:io';

import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter_logix_gui/circuit/circuit_description.dart';
import 'package:flutter_logix_gui/home.dart';
import 'package:path_provider/path_provider.dart';

void main() async {
  // Directory directory = await getApplicationDocumentsDirectory();
  final stdlib_dir = Directory("assets/stdlib_components/");

  final ComponentLibrary components = {};

  // iterate over all files in the stdlib directory
  final files = stdlib_dir.listSync();
  for (final file in files) {
    if (file is File) {
      final text = await file.readAsString();
      final jsonObj = await jsonDecode(text);
      if (jsonObj is List<dynamic>) {
        for (final compJson in jsonObj) {
          final comp = ComponentDescription.fromJson(compJson);
          components[comp.type] = comp;
        }
      }
    }
  }

  final lib = Library(
    circuits: {},
    components: components,
  );

  runApp(MyApp(library: lib));
}

class MyApp extends StatelessWidget {
  const MyApp({
    super.key,
    required this.library,
  });

  final Library library;

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Logix',
      debugShowCheckedModeBanner: false,
      scrollBehavior: MyCustomScrollBehavior(),
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: Home(library: library),
    );
  }
}

//
// Enable scrolling with mouse dragging
class MyCustomScrollBehavior extends MaterialScrollBehavior {
  @override
  Set<PointerDeviceKind> get dragDevices => {
        PointerDeviceKind.touch,
        PointerDeviceKind.mouse,
      };
}
